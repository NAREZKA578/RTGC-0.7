use glow::Context;
use std::rc::Rc;

pub struct Shader {
    gl: Rc<Context>,
    program: glow::Program,
}

impl Shader {
    pub fn new(
        gl: &Context,
        vertex_shader_source: &str,
        fragment_shader_source: &str,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        let gl = Rc::new(gl.clone());
        
        unsafe {
            let vertex_shader = compile_shader(&gl, glow::VERTEX_SHADER, vertex_shader_source)?;
            let fragment_shader = compile_shader(&gl, glow::FRAGMENT_SHADER, fragment_shader_source)?;
            
            let program = gl.create_program()?;
            gl.attach_shader(program, vertex_shader);
            gl.attach_shader(program, fragment_shader);
            gl.link_program(program);
            
            if !gl.get_program_link_status(program) {
                return Err(
                    format!("Failed to link shader program: {}", gl.get_program_info_log(program)).into()
                );
            }
            
            gl.delete_shader(vertex_shader);
            gl.delete_shader(fragment_shader);
            
            Ok(Shader { gl, program })
        }
    }
    
    pub fn bind(&self) {
        unsafe {
            self.gl.use_program(Some(self.program));
        }
    }
    
    pub fn unbind(&self) {
        unsafe {
            self.gl.use_program(None);
        }
    }
}

unsafe fn compile_shader(
    gl: &Context,
    shader_type: u32,
    source: &str,
) -> Result<glow::Shader, Box<dyn std::error::Error>> {
    let shader = gl.create_shader(shader_type)?;
    gl.shader_source(shader, source);
    gl.compile_shader(shader);
    
    if !gl.get_shader_compile_status(shader) {
        return Err(format!("Failed to compile shader: {}", gl.get_shader_info_log(shader)).into());
    }
    
    Ok(shader)
}

impl Drop for Shader {
    fn drop(&mut self) {
        unsafe {
            self.gl.delete_program(self.program);
        }
    }
}