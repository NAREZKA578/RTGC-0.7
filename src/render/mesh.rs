use crate::render::rhi::*;

pub struct Mesh {
    pub vertices: Vec<Vertex>,
    pub indices: Vec<u32>,
    pub vertex_buffer: Option<Box<dyn Buffer>>,
    pub index_buffer: Option<Box<dyn Buffer>>,
    pub bounds: Bounds,
}

pub struct Vertex {
    pub position: [f32; 3],
    pub normal: [f32; 3],
    pub tangent: [f32; 3],
    pub uv: [f32; 2],
    pub color: [f32; 4],
}

pub struct Bounds {
    pub min: [f32; 3],
    pub max: [f32; 3],
}

impl Mesh {
    pub fn new(vertices: Vec<Vertex>, indices: Vec<u32>) -> Self {
        let mut min = [f32::MAX, f32::MAX, f32::MAX];
        let mut max = [f32::MIN, f32::MIN, f32::MIN];

        for vertex in &vertices {
            min[0] = min[0].min(vertex.position[0]);
            min[1] = min[1].min(vertex.position[1]);
            min[2] = min[2].min(vertex.position[2]);

            max[0] = max[0].max(vertex.position[0]);
            max[1] = max[1].max(vertex.position[1]);
            max[2] = max[2].max(vertex.position[2]);
        }

        Self {
            vertices,
            indices,
            vertex_buffer: None,
            index_buffer: None,
            bounds: Bounds { min, max },
        }
    }

    pub fn upload_to_gpu(&mut self, device: &dyn Device) -> Result<(), crate::render::rhi::RHIError> {
        // Create vertex buffer
        let vb_desc = BufferDesc {
            size: (self.vertices.len() * std::mem::size_of::<Vertex>()) as u64,
            usage: BufferUsage::Vertex,
            memory_type: MemoryType::Upload,
        };
        let vertex_buffer = device.create_buffer(&vb_desc)?;
        self.vertex_buffer = Some(Box::new(vertex_buffer));

        // Create index buffer
        let ib_desc = BufferDesc {
            size: (self.indices.len() * std::mem::size_of::<u32>()) as u64,
            usage: BufferUsage::Index,
            memory_type: MemoryType::Upload,
        };
        let index_buffer = device.create_buffer(&ib_desc)?;
        self.index_buffer = Some(Box::new(index_buffer));

        Ok(())
    }

    pub fn calculate_tangents(&mut self) {
        // Initialize tangents to zero
        for vertex in &mut self.vertices {
            vertex.tangent = [0.0, 0.0, 0.0];
        }

        // Calculate tangent for each triangle
        for i in (0..self.indices.len()).step_by(3) {
            let i1 = self.indices[i] as usize;
            let i2 = self.indices[i + 1] as usize;
            let i3 = self.indices[i + 2] as usize;

            let v1 = &self.vertices[i1];
            let v2 = &self.vertices[i2];
            let v3 = &self.vertices[i3];

            let pos1 = glam::Vec3::from_slice(&v1.position);
            let pos2 = glam::Vec3::from_slice(&v2.position);
            let pos3 = glam::Vec3::from_slice(&v3.position);

            let uv1 = glam::Vec2::from_slice(&v1.uv);
            let uv2 = glam::Vec2::from_slice(&v2.uv);
            let uv3 = glam::Vec2::from_slice(&v3.uv);

            let edge1 = pos2 - pos1;
            let edge2 = pos3 - pos1;
            let delta_uv1 = uv2 - uv1;
            let delta_uv2 = uv3 - uv1;

            let r = 1.0 / (delta_uv1.x * delta_uv2.y - delta_uv2.x * delta_uv1.y + 1e-10);

            let tangent = (edge1 * delta_uv2.y - edge2 * delta_uv1.y) * r;

            self.vertices[i1].tangent = [
                self.vertices[i1].tangent[0] + tangent.x,
                self.vertices[i1].tangent[1] + tangent.y,
                self.vertices[i1].tangent[2] + tangent.z,
            ];
            self.vertices[i2].tangent = [
                self.vertices[i2].tangent[0] + tangent.x,
                self.vertices[i2].tangent[1] + tangent.y,
                self.vertices[i2].tangent[2] + tangent.z,
            ];
            self.vertices[i3].tangent = [
                self.vertices[i3].tangent[0] + tangent.x,
                self.vertices[i3].tangent[1] + tangent.y,
                self.vertices[i3].tangent[2] + tangent.z,
            ];
        }

        // Normalize tangents
        for vertex in &mut self.vertices {
            let tangent = glam::Vec3::from_slice(&vertex.tangent).normalize();
            vertex.tangent = tangent.to_array();
        }
    }
}