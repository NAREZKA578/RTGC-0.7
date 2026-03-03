use glow::Context;
use std::rc::Rc;

pub struct Renderer {
    gl: Rc<Context>,
}

impl Renderer {
    pub fn new(gl: Context) -> Result<Self, Box<dyn std::error::Error>> {
        let gl = Rc::new(gl);
        
        unsafe {
            gl.enable(glow::DEPTH_TEST);
            gl.depth_func(glow::LESS);
            gl.enable(glow::CULL_FACE);
            gl.cull_face(glow::BACK);
        }
        
        Ok(Self { gl })
    }
    
    pub fn render(&self) -> Result<(), Box<dyn std::error::Error>> {
        // Render scene here
        Ok(())
    }
}