pub mod renderer;
pub mod camera;
pub mod shader;
pub mod mesh;
pub mod texture;
pub mod models;
pub mod lod_system;
pub mod texture_streaming;
pub mod rhi;

use winit::window::Window;
use std::sync::Arc;
use glow::Context;
use crate::graphics::renderer::{Renderer, MenuState};
use crate::graphics::rhi::{RhiFactory, RhiConfig, IDevice, GraphicsBackend};

pub struct GraphicsContext {
    pub renderer: Renderer,
    gl: Context,
    window: Arc<Window>,
    rhi_config: RhiConfig,
}

impl GraphicsContext {
    pub fn new(window: Arc<Window>) -> Result<Self, Box<dyn std::error::Error>> {
        unsafe {
            let gl = glow::Context::from_loader_function(|s| {
                window.get_proc_address(s) as *const _
            });
            
            let renderer = Renderer::new(gl.clone())?;
            
            Ok(Self {
                renderer,
                gl,
                window,
                rhi_config: RhiConfig::default(),
            })
        }
    }
    
    /// Initialize the RHI with Vulkan or DirectX 12 backend
    pub fn initialize_rhi(&mut self) -> Result<Box<dyn IDevice>, Box<dyn std::error::Error>> {
        let config = RhiConfig {
            backend: RhiFactory::get_preferred_backend(),
            enable_validation: cfg!(debug_assertions),
            enable_debug_layers: cfg!(debug_assertions),
            max_frames_in_flight: 3,
            descriptor_pool_size: 1024,
        };
        
        log::info!("Initializing RHI with {} backend", config.backend.as_str());
        
        let device = RhiFactory::create_device(config.backend)?;
        log::info!("RHI device created: {}", device.get_device_name());
        
        Ok(device)
    }
    
    pub fn render(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        unsafe {
            self.gl.clear_color(0.1, 0.2, 0.3, 1.0);
            self.gl.clear(glow::COLOR_BUFFER_BIT | glow::DEPTH_BUFFER_BIT);
        }
        
        self.renderer.render()?;
        
        Ok(())
    }
    
    pub fn get_rhi_config(&self) -> &RhiConfig {
        &self.rhi_config
    }
    
    pub fn set_rhi_config(&mut self, config: RhiConfig) {
        self.rhi_config = config;
    }
}