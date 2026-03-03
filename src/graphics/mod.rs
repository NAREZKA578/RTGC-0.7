pub mod renderer;
pub mod camera;
pub mod shader;
pub mod mesh;
pub mod texture;
pub mod models;

use winit::window::Window;
use std::sync::Arc;
use glow::Context;
use crate::graphics::renderer::Renderer;

pub struct GraphicsContext {
    pub renderer: Renderer,
    gl: Context,
    window: Arc<Window>,
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
            })
        }
    }
    
    pub fn render(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        unsafe {
            self.gl.clear_color(0.1, 0.2, 0.3, 1.0);
            self.gl.clear(glow::COLOR_BUFFER_BIT | glow::DEPTH_BUFFER_BIT);
        }
        
        self.renderer.render()?;
        
        Ok(())
    }
}