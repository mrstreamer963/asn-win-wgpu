use std::sync::Arc;

use asn_logger::*;
use asn_winit::WinitWindow;
use wgpu::ExperimentalFeatures;

use crate::{
    StateError,
    data::{LOG_MODULE_NAME, MIN_WINDOW_SIZE},
};

pub struct WgpuGraphContext {
    pub window: Arc<WinitWindow>,
    pub device: wgpu::Device,
    pub queue: wgpu::Queue,
    pub surface_format: wgpu::TextureFormat,
    pub surface: wgpu::Surface<'static>,
    pub config: wgpu::SurfaceConfiguration,
}

impl WgpuGraphContext {
    pub async fn new(window: Arc<WinitWindow>) -> Result<Self, StateError> {
        m_trace!("new...");

        // if size.width < MIN_WINDOW_SIZE || size.height < MIN_WINDOW_SIZE {
        //     return Err(StateError::InvalidWindowSize {
        //         width: size.width,
        //         height: size.height,
        //     });
        // }

        // Create GPU instance
        // wgpu 29: Instance::new takes descriptor by value, not by reference
        let instance = wgpu::Instance::new(wgpu::InstanceDescriptor {
            #[cfg(not(target_arch = "wasm32"))]
            backends: wgpu::Backends::PRIMARY,
            #[cfg(target_arch = "wasm32")]
            backends: wgpu::Backends::GL,
            flags: wgpu::InstanceFlags::default(),
            memory_budget_thresholds: wgpu::MemoryBudgetThresholds::default(),
            backend_options: wgpu::BackendOptions::default(),
            display: None,
        });

        m_trace!("instance ok");

        // Create surface
        let surface = instance
            .create_surface(window.clone())
            .map_err(|e| StateError::SurfaceCreation(e.to_string()))?;

        m_trace!("surface ok");

        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::default(),
                compatible_surface: Some(&surface),
                force_fallback_adapter: false,
            })
            .await
            .map_err(|_| StateError::NoAdapter)?;

        m_trace!("adapter ok");

        // Create device and queue with better feature handling
        let required_features = wgpu::Features::empty();
        let (device, queue) = adapter
            .request_device(&wgpu::DeviceDescriptor {
                label: Some("ASN WGPU Device"),
                required_features,
                required_limits: if cfg!(target_arch = "wasm32") {
                    wgpu::Limits::downlevel_webgl2_defaults()
                } else {
                    wgpu::Limits::default()
                },
                memory_hints: Default::default(),
                trace: wgpu::Trace::Off,
                experimental_features: ExperimentalFeatures::disabled(),
            })
            .await
            .map_err(|e| StateError::DeviceCreation(e.to_string()))?;

        m_trace!("device, queue ok");

        // Configure surface with better error handling
        let surface_caps = surface.get_capabilities(&adapter);
        m_trace!("surface_caps: {:?}", surface_caps);

        let surface_format = surface_caps
            .formats
            .iter()
            .find(|f| f.is_srgb())
            .copied()
            .unwrap_or(surface_caps.formats[0]);

        m_trace!("surface_format: {:?}", surface_format);

        let size = window.inner_size();
        m_trace!("window size: {size:?}");

        let width = size.width.max(1);
        let height = size.height.max(1);

        let config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: surface_format,
            width,
            height,
            present_mode: surface_caps
                .present_modes
                .first()
                .copied()
                .unwrap_or(wgpu::PresentMode::Fifo),
            alpha_mode: surface_caps
                .alpha_modes
                .iter()
                .find(|mode| **mode == wgpu::CompositeAlphaMode::Opaque)
                .copied()
                .unwrap_or(wgpu::CompositeAlphaMode::Auto),
            view_formats: vec![],
            desired_maximum_frame_latency: 2,
        };

        m_trace!("config ready");

        let mut w = WgpuGraphContext {
            window,
            device,
            queue,
            surface_format,
            surface,
            config,
        };

        w.resize(width, height)?;

        Ok(w)
    }

    pub fn resize(&mut self, width: u32, height: u32) -> Result<(), StateError> {
        m_trace!("resize {width} {height}");

        // Size validation
        if width < MIN_WINDOW_SIZE || height < MIN_WINDOW_SIZE {
            return Err(StateError::InvalidWindowSize { width, height });
        }

        self.config.width = width;
        self.config.height = height;
        self.surface.configure(&self.device, &self.config);

        Ok(())
    }
}
