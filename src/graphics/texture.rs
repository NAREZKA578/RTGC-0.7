use glow::Context;
use std::rc::Rc;

pub struct Texture {
    gl: Rc<Context>,
    texture: glow::Texture,
}

impl Texture {
    pub fn new(gl: &Context, data: &[u8], width: u32, height: u32) -> Result<Self, Box<dyn std::error::Error>> {
        let gl = Rc::new(gl.clone());
        
        unsafe {
            let texture = gl.create_texture()?;
            gl.bind_texture(glow::TEXTURE_2D, Some(texture));
            
            gl.tex_image_2d(
                glow::TEXTURE_2D,
                0,
                glow::RGB as i32,
                width as i32,
                height as i32,
                0,
                glow::RGB,
                glow::UNSIGNED_BYTE,
                Some(data),
            );
            
            gl.generate_mipmap(glow::TEXTURE_2D);
            
            gl.tex_parameter_i32(glow::TEXTURE_2D, glow::TEXTURE_MIN_FILTER, glow::LINEAR as i32);
            gl.tex_parameter_i32(glow::TEXTURE_2D, glow::TEXTURE_MAG_FILTER, glow::LINEAR as i32);
            
            Ok(Texture { gl, texture })
        }
    }
    
    pub fn bind(&self) {
        unsafe {
            self.gl.bind_texture(glow::TEXTURE_2D, Some(self.texture));
        }
    }
    
    pub fn unbind(&self) {
        unsafe {
            self.gl.bind_texture(glow::TEXTURE_2D, None);
        }
    }
}

impl Drop for Texture {
    fn drop(&mut self) {
        unsafe {
            self.gl.delete_texture(self.texture);
        }
    }
}