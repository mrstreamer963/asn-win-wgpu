use asn_core::cgmath::{Matrix4, SquareMatrix};
use asn_gui_core::TAsnGuiElement;
use asn_wgpu::wgpu::util::DeviceExt;
use asn_wgpu::{WgpuFrameContext, WgpuGraphContext, wgpu};

use crate::data::rgba_handler::RgbaHandler;
use crate::data::texture::{AsnTextureFormat, WgpuTexture};
use crate::data::utils::{get_render_pipeline, get_texture_bind_group_layout};
use crate::data::{DEFAULT_CLEAR_COLOR, INDICES, SHADER_SOURCE, VERTICES};

mod data;

pub struct WgpuMap {
    render_pipeline: wgpu::RenderPipeline,
    vertex_buffer: wgpu::Buffer,
    index_buffer: wgpu::Buffer,
    diffuse_bind_group: wgpu::BindGroup,
    num_indices: u32,
    map_handler: RgbaHandler,
    map_texture: WgpuTexture,
    tiles_texture: WgpuTexture,
    tiles_info_buffer: wgpu::Buffer, // Uniform-буфер с информацией о тайлах
    mvp_matrix_buffer: wgpu::Buffer, // Uniform-буфер для MVP-матрицы
    is_map_updated: bool,
}

impl WgpuMap {
    pub fn update_map(&mut self, map_indices: &[u32]) {
        // Получаем ширину карты из текущего обработчика
        let map_width = self.map_handler.width();

        // Устанавливаем индексы тайлов в RGBA-формате
        self.map_handler
            .set_tile_indices(map_indices, map_width)
            .unwrap();
        self.is_map_updated = true;
    }

    /// Обновляет MVP-матрицу
    pub fn update_mvp_matrix(&self, gcx: &WgpuGraphContext, mvp_matrix: [[f32; 4]; 4]) {
        gcx.queue.write_buffer(
            &self.mvp_matrix_buffer,
            0,
            bytemuck::cast_slice(&[mvp_matrix]),
        );
    }
}

impl TAsnGuiElement for WgpuMap {
    type GraphContext = WgpuGraphContext;
    type FrameContext = WgpuFrameContext;

    fn update(&mut self, gcx: &Self::GraphContext) {
        if !self.is_map_updated {
            return;
        }

        // Обновляем текстуру напрямую
        self.map_texture.update_from_rgba(
            &gcx.queue,
            bytemuck::cast_slice(self.map_handler.data()),
            self.map_handler.width(),
            self.map_handler.height(),
            AsnTextureFormat::Rgba32Uint,
        );

        self.is_map_updated = false;
    }

    fn draw(&mut self, fcx: &mut Self::FrameContext) {
        // m_trace!("draw");
        let mut render_pass = fcx.encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: Some("Render Pass"),
            color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                view: &fcx.view,
                resolve_target: None,
                ops: wgpu::Operations {
                    load: wgpu::LoadOp::Clear(DEFAULT_CLEAR_COLOR),
                    store: wgpu::StoreOp::Store,
                },
                depth_slice: None,
            })],
            depth_stencil_attachment: None,
            occlusion_query_set: None,
            timestamp_writes: None,
            // wgpu 29: new required field
            multiview_mask: None,
        });

        render_pass.set_pipeline(&self.render_pipeline);
        render_pass.set_bind_group(0, &self.diffuse_bind_group, &[]);
        render_pass.set_vertex_buffer(0, self.vertex_buffer.slice(..));
        render_pass.set_index_buffer(self.index_buffer.slice(..), wgpu::IndexFormat::Uint16);
        render_pass.draw_indexed(0..self.num_indices, 0, 0..1);
    }
}

/// Параметры для создания карты тайлов
pub struct MapTilesParams<'a> {
    /// Байты тайлов карты
    pub map_tiles_bytes: &'a [u8],
    /// Ширина тайлов
    pub tiles_width: u32,
    /// Высота тайлов
    pub tiles_height: u32,
}

/// Параметры карты
pub struct MapParams<'a> {
    /// Ширина карты
    pub map_width: u32,
    /// Высота карты
    pub map_height: u32,
    /// Индексы тайлов на карте
    pub tile_indices: &'a [u32],
}

pub fn get_map(
    gcx: &WgpuGraphContext,
    tiles_params: &MapTilesParams,
    map_params: &MapParams,
) -> WgpuMap {
    let device = &gcx.device;
    let queue = &gcx.queue;
    let format = gcx.surface_format;

    // Создаем текстуру тайлов
    let tiles_texture = WgpuTexture::from_bytes(
        device,
        queue,
        tiles_params.map_tiles_bytes,
        "map-tiles-texture.png",
    )
    .unwrap();

    // Создаем uniform-буфер с информацией о тайлах и карте
    let tiles_info_data = [
        tiles_params.tiles_width as f32,
        tiles_params.tiles_height as f32,
        map_params.map_width as f32,
        map_params.map_height as f32,
    ];
    let tiles_info_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        label: Some("Tiles Info Buffer"),
        contents: bytemuck::cast_slice(&tiles_info_data),
        usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
    });

    // Создаем uniform-буфер для MVP-матрицы
    let mvp_matrix: [[f32; 4]; 4] = Matrix4::<f32>::identity().into();
    let mvp_matrix_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        label: Some("MVP Matrix Buffer"),
        contents: bytemuck::cast_slice(&[mvp_matrix]),
        usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
    });

    let mut map_handler = RgbaHandler::new(map_params.map_width, map_params.map_height);
    map_handler
        .set_tile_indices(map_params.tile_indices, tiles_params.tiles_width)
        .unwrap();

    let map_texture = WgpuTexture::from_rgba(
        device,
        queue,
        bytemuck::cast_slice(map_handler.data()),
        map_handler.width(),
        map_handler.height(),
        "MAP_TEXTURE_0",
        AsnTextureFormat::Rgba32Uint,
    )
    .unwrap();

    let vertex_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        label: Some("Map Vertex Buffer"),
        contents: bytemuck::cast_slice(VERTICES),
        usage: wgpu::BufferUsages::VERTEX,
    });

    let index_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        label: Some("Map Index Buffer"),
        contents: bytemuck::cast_slice(INDICES),
        usage: wgpu::BufferUsages::INDEX,
    });

    let num_indices = INDICES.len() as u32;

    let texture_bind_group_layout = get_texture_bind_group_layout(&device);

    let diffuse_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
        layout: &texture_bind_group_layout,
        entries: &[
            // Uniform-буфер с информацией о тайлах
            wgpu::BindGroupEntry {
                binding: 0,
                resource: wgpu::BindingResource::Buffer(
                    tiles_info_buffer.as_entire_buffer_binding(),
                ),
            },
            // Текстура тайлов
            wgpu::BindGroupEntry {
                binding: 1,
                resource: wgpu::BindingResource::TextureView(&tiles_texture.view),
            },
            wgpu::BindGroupEntry {
                binding: 2,
                resource: wgpu::BindingResource::Sampler(&tiles_texture.sampler),
            },
            // Текстура карты
            wgpu::BindGroupEntry {
                binding: 3,
                resource: wgpu::BindingResource::TextureView(&map_texture.view),
            },
            wgpu::BindGroupEntry {
                binding: 4,
                resource: wgpu::BindingResource::Sampler(&map_texture.sampler),
            },
            // Uniform-буфер для MVP-матрицы
            wgpu::BindGroupEntry {
                binding: 5,
                resource: wgpu::BindingResource::Buffer(
                    mvp_matrix_buffer.as_entire_buffer_binding(),
                ),
            },
        ],
        label: Some("map_diffuse_bind_group"),
    });

    let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
        label: Some("Map Shader"),
        source: wgpu::ShaderSource::Wgsl(SHADER_SOURCE.into()),
    });

    let render_pipeline = get_render_pipeline(device, format, &texture_bind_group_layout, shader);

    WgpuMap {
        render_pipeline,
        vertex_buffer,
        index_buffer,
        diffuse_bind_group,
        num_indices,
        map_handler,
        map_texture,
        tiles_texture,
        tiles_info_buffer,
        mvp_matrix_buffer,
        is_map_updated: false,
    }
}
