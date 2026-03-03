// 3D Models Module for Off-Road Truck Simulator
// Contains procedural generation of detailed models

use nalgebra::Vector3;

#[derive(Debug, Clone)]
pub struct Vertex {
    pub position: Vector3<f32>,
    pub normal: Vector3<f32>,
    pub tex_coords: (f32, f32),
}

#[derive(Debug, Clone)]
pub struct Mesh {
    pub vertices: Vec<Vertex>,
    pub indices: Vec<u32>,
    pub material: Material,
}

#[derive(Debug, Clone)]
pub struct Material {
    pub albedo: [f32; 3],
    pub roughness: f32,
    pub metallic: f32,
}

#[derive(Debug)]
pub struct Model {
    pub meshes: Vec<Mesh>,
    pub name: String,
}

impl Vertex {
    pub fn new(position: Vector3<f32>, normal: Vector3<f32>, tex_coords: (f32, f32)) -> Self {
        Self {
            position,
            normal,
            tex_coords,
        }
    }
}

impl Mesh {
    pub fn new(vertices: Vec<Vertex>, indices: Vec<u32>, material: Material) -> Self {
        Self {
            vertices,
            indices,
            material,
        }
    }
    
    pub fn generate_cube(center: Vector3<f32>, size: Vector3<f32>) -> Self {
        let half_size = size * 0.5;
        let mut vertices = Vec::new();
        let mut indices = Vec::new();
        
        // Define cube vertices
        let positions = [
            // Front face
            center + Vector3::new(-half_size.x, -half_size.y,  half_size.z), // 0
            center + Vector3::new( half_size.x, -half_size.y,  half_size.z), // 1
            center + Vector3::new( half_size.x,  half_size.y,  half_size.z), // 2
            center + Vector3::new(-half_size.x,  half_size.y,  half_size.z), // 3
            // Back face
            center + Vector3::new(-half_size.x, -half_size.y, -half_size.z), // 4
            center + Vector3::new( half_size.x, -half_size.y, -half_size.z), // 5
            center + Vector3::new( half_size.x,  half_size.y, -half_size.z), // 6
            center + Vector3::new(-half_size.x,  half_size.y, -half_size.z), // 7
        ];
        
        // Define normals for each face
        let normals = [
            Vector3::new(0.0, 0.0, 1.0),  // Front
            Vector3::new(0.0, 0.0, -1.0), // Back
            Vector3::new(1.0, 0.0, 0.0),  // Right
            Vector3::new(-1.0, 0.0, 0.0), // Left
            Vector3::new(0.0, 1.0, 0.0),  // Top
            Vector3::new(0.0, -1.0, 0.0), // Bottom
        ];
        
        // Add vertices with proper normals and texture coordinates
        for (face_idx, &normal) in normals.iter().enumerate() {
            let v_start = face_idx * 4;
            for i in 0..4 {
                let pos_idx = match face_idx {
                    0 => [0, 1, 2, 3], // Front
                    1 => [5, 4, 7, 6], // Back
                    2 => [1, 5, 6, 2], // Right
                    3 => [4, 0, 3, 7], // Left
                    4 => [3, 2, 6, 7], // Top
                    5 => [4, 5, 1, 0], // Bottom
                    _ => [0, 1, 2, 3],
                }[i];
                
                let tex_coords = match i {
                    0 => (0.0, 0.0),
                    1 => (1.0, 0.0),
                    2 => (1.0, 1.0),
                    3 => (0.0, 1.0),
                    _ => (0.0, 0.0),
                };
                
                vertices.push(Vertex::new(positions[pos_idx], normal, tex_coords));
            }
        }
        
        // Define indices for triangles
        for face in 0..6 {
            let base = (face * 4) as u32;
            indices.extend_from_slice(&[
                base, base + 1, base + 2,  // First triangle
                base, base + 2, base + 3,  // Second triangle
            ]);
        }
        
        let material = Material {
            albedo: [0.8, 0.8, 0.8],
            roughness: 0.5,
            metallic: 0.0,
        };
        
        Self::new(vertices, indices, material)
    }
    
    pub fn generate_cylinder(center: Vector3<f32>, radius: f32, height: f32, segments: usize) -> Self {
        let mut vertices = Vec::new();
        let mut indices = Vec::new();
        
        // Generate vertices for cylinder
        let height_half = height * 0.5;
        
        // Top cap center
        vertices.push(Vertex::new(
            center + Vector3::new(0.0, height_half, 0.0),
            Vector3::new(0.0, 1.0, 0.0),
            (0.5, 0.5)
        ));
        
        // Bottom cap center
        vertices.push(Vertex::new(
            center + Vector3::new(0.0, -height_half, 0.0),
            Vector3::new(0.0, -1.0, 0.0),
            (0.5, 0.5)
        ));
        
        // Generate circular vertices
        for i in 0..segments {
            let angle = (i as f32) * 2.0 * std::f32::consts::PI / (segments as f32);
            let x = radius * angle.cos();
            let z = radius * angle.sin();
            
            // Top circle
            vertices.push(Vertex::new(
                center + Vector3::new(x, height_half, z),
                Vector3::new(x, 0.0, z).normalize(),
                (angle / (2.0 * std::f32::consts::PI), 0.0)
            ));
            
            // Bottom circle
            vertices.push(Vertex::new(
                center + Vector3::new(x, -height_half, z),
                Vector3::new(x, 0.0, z).normalize(),
                (angle / (2.0 * std::f32::consts::PI), 1.0)
            ));
        }
        
        // Generate indices for top cap
        for i in 2..(segments + 1) {
            indices.extend_from_slice(&[0, i as u32, (i + 1) as u32]);
        }
        indices.extend_from_slice(&[0, (segments + 1) as u32, 2]); // Close the loop
        
        // Generate indices for bottom cap
        for i in 2..(segments + 1) {
            indices.extend_from_slice(&[1, (i + 1) as u32, i as u32]);
        }
        indices.extend_from_slice(&[1, 2, (segments + 1) as u32]); // Close the loop
        
        // Generate indices for sides
        for i in 0..segments {
            let top_current = (2 + i * 2) as u32;
            let top_next = (2 + ((i + 1) % segments) * 2) as u32;
            let bottom_current = (2 + i * 2 + 1) as u32;
            let bottom_next = (2 + ((i + 1) % segments) * 2 + 1) as u32;
            
            // First triangle
            indices.extend_from_slice(&[top_current, bottom_current, top_next]);
            // Second triangle
            indices.extend_from_slice(&[bottom_current, bottom_next, top_next]);
        }
        
        let material = Material {
            albedo: [0.6, 0.6, 0.6],
            roughness: 0.3,
            metallic: 0.1,
        };
        
        Self::new(vertices, indices, material)
    }
    
    pub fn generate_sphere(center: Vector3<f32>, radius: f32, segments: usize) -> Self {
        let mut vertices = Vec::new();
        let mut indices = Vec::new();
        
        // Generate vertices
        for y in 0..=segments {
            let phi = (y as f32) * std::f32::consts::PI / (segments as f32);
            for x in 0..=segments {
                let theta = (x as f32) * 2.0 * std::f32::consts::PI / (segments as f32);
                
                let sin_phi = phi.sin();
                let cos_phi = phi.cos();
                let sin_theta = theta.sin();
                let cos_theta = theta.cos();
                
                let x_pos = radius * sin_phi * cos_theta;
                let y_pos = radius * cos_phi;
                let z_pos = radius * sin_phi * sin_theta;
                
                let normal = Vector3::new(x_pos, y_pos, z_pos).normalize();
                let tex_coord_x = (theta / (2.0 * std::f32::consts::PI));
                let tex_coord_y = phi / std::f32::consts::PI;
                
                vertices.push(Vertex::new(
                    center + Vector3::new(x_pos, y_pos, z_pos),
                    normal,
                    (tex_coord_x, tex_coord_y)
                ));
            }
        }
        
        // Generate indices
        for y in 0..segments {
            for x in 0..segments {
                let current = (y * (segments + 1) + x) as u32;
                let next = (y * (segments + 1) + x + 1) as u32;
                let next_row = ((y + 1) * (segments + 1) + x) as u32;
                let next_row_next = ((y + 1) * (segments + 1) + x + 1) as u32;
                
                indices.extend_from_slice(&[current, next_row, next]);
                indices.extend_from_slice(&[next, next_row, next_row_next]);
            }
        }
        
        let material = Material {
            albedo: [0.9, 0.5, 0.2],
            roughness: 0.4,
            metallic: 0.05,
        };
        
        Self::new(vertices, indices, material)
    }
}

impl Model {
    pub fn new(meshes: Vec<Mesh>, name: String) -> Self {
        Self { meshes, name }
    }
    
    pub fn generate_truck_model(color: [f32; 3]) -> Self {
        let mut meshes = Vec::new();
        
        // Main cabin (box)
        let cabin_material = Material {
            albedo: color,
            roughness: 0.3,
            metallic: 0.2,
        };
        
        let cabin_mesh = Mesh::generate_cube(
            Vector3::new(0.0, 1.0, 0.0),
            Vector3::new(3.0, 2.0, 1.5)
        );
        let mut cabin_mesh_with_material = cabin_mesh;
        cabin_mesh_with_material.material = cabin_material;
        meshes.push(cabin_mesh_with_material);
        
        // Cargo bed (box)
        let cargo_bed_material = Material {
            albedo: [0.4, 0.4, 0.4],
            roughness: 0.5,
            metallic: 0.1,
        };
        
        let cargo_bed_mesh = Mesh::generate_cube(
            Vector3::new(0.0, 1.0, -2.0),
            Vector3::new(3.0, 1.0, 3.0)
        );
        let mut cargo_bed_mesh_with_material = cargo_bed_mesh;
        cargo_bed_mesh_with_material.material = cargo_bed_material;
        meshes.push(cargo_bed_mesh_with_material);
        
        // Cabin roof (box)
        let roof_mesh = Mesh::generate_cube(
            Vector3::new(0.0, 2.2, 0.2),
            Vector3::new(2.5, 0.4, 1.0)
        );
        let mut roof_mesh_with_material = roof_mesh;
        roof_mesh_with_material.material = cabin_material.clone();
        meshes.push(roof_mesh_with_material);
        
        // Wheels (cylinders)
        let wheel_material = Material {
            albedo: [0.1, 0.1, 0.1],
            roughness: 0.9,
            metallic: 0.05,
        };
        
        let wheel_positions = [
            Vector3::new(-1.2, -0.5, 1.5),  // Front left
            Vector3::new(1.2, -0.5, 1.5),   // Front right
            Vector3::new(-1.2, -0.5, -1.5), // Rear left
            Vector3::new(1.2, -0.5, -1.5),  // Rear right
        ];
        
        for pos in &wheel_positions {
            let mut wheel_mesh = Mesh::generate_cylinder(*pos, 0.6, 0.3, 16);
            wheel_mesh.material = wheel_material.clone();
            meshes.push(wheel_mesh);
        }
        
        // Headlights
        let headlight_material = Material {
            albedo: [0.9, 0.9, 0.8],
            roughness: 0.1,
            metallic: 0.8,
        };
        
        let headlight_positions = [
            Vector3::new(-0.8, 0.8, 0.76),
            Vector3::new(0.8, 0.8, 0.76),
        ];
        
        for pos in &headlight_positions {
            let mut headlight_mesh = Mesh::generate_sphere(*pos, 0.15, 8);
            headlight_mesh.material = headlight_material.clone();
            meshes.push(headlight_mesh);
        }
        
        Self::new(meshes, "truck".to_string())
    }
    
    pub fn generate_cargo_model(size: f32, color: [f32; 3]) -> Self {
        let mut meshes = Vec::new();
        
        let cargo_material = Material {
            albedo: color,
            roughness: 0.6,
            metallic: 0.0,
        };
        
        let mut cargo_mesh = Mesh::generate_cube(
            Vector3::new(0.0, size/2.0, 0.0),
            Vector3::new(size, size, size)
        );
        cargo_mesh.material = cargo_material;
        meshes.push(cargo_mesh);
        
        Self::new(meshes, "cargo".to_string())
    }
    
    pub fn generate_delivery_point_model(size: f32) -> Self {
        let mut meshes = Vec::new();
        
        // Base platform
        let platform_material = Material {
            albedo: [0.0, 0.6, 0.0], // Green
            roughness: 0.7,
            metallic: 0.0,
        };
        
        let mut platform_mesh = Mesh::generate_cube(
            Vector3::new(0.0, 0.1, 0.0),
            Vector3::new(size, 0.2, size)
        );
        platform_mesh.material = platform_material;
        meshes.push(platform_mesh);
        
        // Central marker
        let marker_material = Material {
            albedo: [1.0, 1.0, 1.0], // White
            roughness: 0.3,
            metallic: 0.2,
        };
        
        let mut marker_mesh = Mesh::generate_cylinder(
            Vector3::new(0.0, size/2.0 + 0.1, 0.0),
            size/4.0,
            size,
            16
        );
        marker_mesh.material = marker_material;
        meshes.push(marker_mesh);
        
        Self::new(meshes, "delivery_point".to_string())
    }
    
    pub fn generate_terrain_chunk(width: f32, depth: f32, height_data: &[Vec<f32>]) -> Self {
        let mut vertices = Vec::new();
        let mut indices = Vec::new();
        
        let rows = height_data.len();
        let cols = if rows > 0 { height_data[0].len() } else { 0 };
        
        if rows == 0 || cols == 0 {
            return Self::new(vec![], "empty_terrain".to_string());
        }
        
        let cell_width = width / (cols as f32 - 1.0).max(1.0);
        let cell_depth = depth / (rows as f32 - 1.0).max(1.0);
        
        // Generate vertices
        for (row, height_row) in height_data.iter().enumerate() {
            for (col, &height) in height_row.iter().enumerate() {
                let x = col as f32 * cell_width - width / 2.0;
                let z = row as f32 * cell_depth - depth / 2.0;
                let y = height;
                
                // Calculate normal (simplified)
                let normal = Vector3::new(0.0, 1.0, 0.0);
                
                let vertex = Vertex::new(
                    Vector3::new(x, y, z),
                    normal,
                    (col as f32 / (cols - 1) as f32, row as f32 / (rows - 1) as f32)
                );
                vertices.push(vertex);
            }
        }
        
        // Generate indices
        for row in 0..(rows - 1) {
            for col in 0..(cols - 1) {
                let index = (row * cols + col) as u32;
                
                // First triangle
                indices.push(index);
                indices.push(index + cols);
                indices.push(index + 1);
                
                // Second triangle
                indices.push(index + 1);
                indices.push(index + cols);
                indices.push(index + cols + 1);
            }
        }
        
        let material = Material {
            albedo: [0.4, 0.3, 0.2], // Brown earth tone
            roughness: 0.8,
            metallic: 0.0,
        };
        
        let mesh = Mesh::new(vertices, indices, material);
        
        Self::new(vec![mesh], "terrain_chunk".to_string())
    }
}