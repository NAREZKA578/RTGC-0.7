use crate::render::{
    mesh::Mesh,
    material::Material,
    camera::Camera,
    rhi::{Buffer, BufferUsage, MemoryType, Device},
};

pub struct Scene {
    pub entities: Vec<Entity>,
    pub lights: Vec<Light>,
    pub cameras: Vec<Camera>,
    pub skybox: Option<Skybox>,
}

pub struct Entity {
    pub mesh: Mesh,
    pub material: Material,
    pub transform: Transform,
    pub visible: bool,
}

pub struct Transform {
    pub position: [f32; 3],
    pub rotation: [f32; 3], // Euler angles in radians
    pub scale: [f32; 3],
}

impl Transform {
    pub fn new() -> Self {
        Self {
            position: [0.0, 0.0, 0.0],
            rotation: [0.0, 0.0, 0.0],
            scale: [1.0, 1.0, 1.0],
        }
    }

    pub fn translation_matrix(&self) -> glam::Mat4 {
        glam::Mat4::from_translation(glam::Vec3::from(self.position))
    }

    pub fn rotation_matrix(&self) -> glam::Mat4 {
        glam::Mat4::from_rotation_x(self.rotation[0]) *
        glam::Mat4::from_rotation_y(self.rotation[1]) *
        glam::Mat4::from_rotation_z(self.rotation[2])
    }

    pub fn scale_matrix(&self) -> glam::Mat4 {
        glam::Mat4::from_scale(glam::Vec3::from(self.scale))
    }

    pub fn model_matrix(&self) -> glam::Mat4 {
        self.translation_matrix() * self.rotation_matrix() * self.scale_matrix()
    }
}

pub struct Light {
    pub light_type: LightType,
    pub position: [f32; 3],
    pub direction: [f32; 3],
    pub color: [f32; 3],
    pub intensity: f32,
    pub range: f32,
    pub inner_cone_angle: f32,
    pub outer_cone_angle: f32,
}

pub enum LightType {
    Directional,
    Point,
    Spot,
}

pub struct Skybox {
    pub texture: Option<String>, // Path to cube map texture
    pub enabled: bool,
}

impl Scene {
    pub fn new() -> Self {
        Self {
            entities: Vec::new(),
            lights: Vec::new(),
            cameras: Vec::new(),
            skybox: None,
        }
    }

    pub fn add_entity(&mut self, entity: Entity) {
        self.entities.push(entity);
    }

    pub fn add_light(&mut self, light: Light) {
        self.lights.push(light);
    }

    pub fn add_camera(&mut self, camera: Camera) {
        self.cameras.push(camera);
    }

    pub fn update(&mut self, delta_time: f32) {
        // Update animations, particle systems, etc.
        for entity in &mut self.entities {
            // Update entity-specific logic
        }
    }

    pub fn upload_to_gpu(&mut self, device: &dyn Device) -> Result<(), crate::render::rhi::RHIError> {
        for entity in &mut self.entities {
            // Upload mesh data to GPU
            entity.mesh.upload_to_gpu(device)?;
            
            // Create uniform buffer for transform
            let transform_ub_desc = crate::render::rhi::BufferDesc {
                size: std::mem::size_of::<TransformUniforms>() as u64,
                usage: BufferUsage::Uniform,
                memory_type: MemoryType::HostVisible,
            };
            
            // We would create and populate the uniform buffer here
            // For now, just uploading the mesh data
        }
        
        Ok(())
    }
}

pub struct TransformUniforms {
    pub model_matrix: [[f32; 4]; 4],
    pub normal_matrix: [[f32; 4]; 4],
}