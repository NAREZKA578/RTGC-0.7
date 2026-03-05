use crate::render::rhi::*;
use crate::render::texture::Texture;

pub struct Material {
    pub albedo: Option<Texture>,
    pub normal: Option<Texture>,
    pub metallic: Option<Texture>,
    pub roughness: Option<Texture>,
    pub ao: Option<Texture>,
    pub emissive: Option<Texture>,
    pub pipeline: Option<Box<dyn PipelineState>>,
    pub uniforms: MaterialUniforms,
}

pub struct MaterialUniforms {
    pub albedo_factor: [f32; 4],
    pub metallic_factor: f32,
    pub roughness_factor: f32,
    pub normal_scale: f32,
    pub emissive_factor: [f32; 3],
}

impl Material {
    pub fn new() -> Self {
        Self {
            albedo: None,
            normal: None,
            metallic: None,
            roughness: None,
            ao: None,
            emissive: None,
            pipeline: None,
            uniforms: MaterialUniforms {
                albedo_factor: [1.0, 1.0, 1.0, 1.0],
                metallic_factor: 0.0,
                roughness_factor: 1.0,
                normal_scale: 1.0,
                emissive_factor: [0.0, 0.0, 0.0],
            },
        }
    }

    pub fn create_pipeline(&mut self, device: &dyn Device, shader_desc: &PipelineDesc) -> Result<(), RHIError> {
        let pipeline = device.create_pipeline(shader_desc)?;
        self.pipeline = Some(Box::new(pipeline));
        Ok(())
    }

    pub fn bind(&self, cmd_buffer: &mut dyn CommandBuffer) {
        if let Some(ref pipeline) = self.pipeline {
            cmd_buffer.set_pipeline(pipeline.as_ref());
        }
    }
}