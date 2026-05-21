use web_time::Instant;

use crate::StateError;

pub struct WgpuFrameContext {
    pub output: wgpu::SurfaceTexture,
    pub encoder: wgpu::CommandEncoder,
    pub view: wgpu::TextureView,
    pub frame_start: Instant,
}

impl WgpuFrameContext {
    pub fn new(
        surface: &wgpu::Surface<'static>,
        device: &wgpu::Device,
    ) -> Result<WgpuFrameContext, StateError> {
        // wgpu 29: get_current_texture returns CurrentSurfaceTexture enum
        let output = match surface.get_current_texture() {
            wgpu::CurrentSurfaceTexture::Success(frame) => frame,
            wgpu::CurrentSurfaceTexture::Timeout | wgpu::CurrentSurfaceTexture::Occluded => {
                return Err(StateError::TextureError(
                    "Surface timeout or occluded".to_string(),
                ));
            }
            wgpu::CurrentSurfaceTexture::Outdated
            | wgpu::CurrentSurfaceTexture::Suboptimal(_)
            | wgpu::CurrentSurfaceTexture::Lost => {
                return Err(StateError::TextureError(
                    "Surface outdated, suboptimal, or lost".to_string(),
                ));
            }
            wgpu::CurrentSurfaceTexture::Validation => {
                return Err(StateError::TextureError(
                    "Surface validation error".to_string(),
                ));
            }
        };

        let view = output
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());

        let encoder = device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
            label: Some("Render Encoder"),
        });

        let frame_start = Instant::now();

        Ok(Self {
            output,
            encoder,
            view,
            frame_start,
        })
    }
}
