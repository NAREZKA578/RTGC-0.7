use glow::Context;
use std::rc::Rc;
use std::collections::HashMap;
use nalgebra::{Vector3, Matrix4, UnitQuaternion};
use crate::graphics::{camera::Camera, mesh::Mesh, shader::Shader, texture::Texture};
use crate::graphics::models::{Model as ModelGen, Vertex as ModelVertex};
use crate::graphics::lod_system::{LodManager, LodObject};
use crate::graphics::texture_streaming::TextureStreamingSystem;

#[derive(Debug, Clone)]
pub struct Vertex {
    pub position: [f32; 3],
    pub normal: [f32; 3],
    pub tex_coords: [f32; 2],
}

pub struct Model {
    pub meshes: Vec<Mesh>,
    pub textures: Vec<Texture>,
}

pub struct Renderer {
    gl: Rc<Context>,
    pub shader: Shader,
    pub camera: Camera,
    models: HashMap<String, Model>,
    current_city_index: usize,
    pub menu_state: MenuState,
    pub lod_manager: LodManager,
    pub texture_streaming: TextureStreamingSystem,
}

#[derive(Debug, Clone, PartialEq)]
pub enum MenuState {
    Loading,
    MainMenu,
    CitySelection,
    InGame,
    WorldCreation,
    Settings,
}

impl Renderer {
    pub fn new(gl: Context) -> Result<Self, Box<dyn std::error::Error>> {
        let gl = Rc::new(gl);
        
        unsafe {
            gl.enable(glow::DEPTH_TEST);
            gl.depth_func(glow::LESS);
            gl.enable(glow::CULL_FACE);
            gl.cull_face(glow::BACK);
            
            // Create default shader
            let vertex_shader = r#"#version 330 core
            layout (location = 0) in vec3 a_position;
            layout (location = 1) in vec3 a_normal;
            layout (location = 2) in vec2 a_tex_coords;
            
            uniform mat4 u_model;
            uniform mat4 u_view;
            uniform mat4 u_projection;
            
            out vec3 frag_position;
            out vec3 frag_normal;
            out vec2 frag_tex_coords;
            
            void main() {
                vec4 world_pos = u_model * vec4(a_position, 1.0);
                gl_Position = u_projection * u_view * world_pos;
                
                frag_position = world_pos.xyz;
                frag_normal = mat3(transpose(inverse(u_model))) * a_normal;
                frag_tex_coords = a_tex_coords;
            }"#;
            
            let fragment_shader = r#"#version 330 core
            in vec3 frag_position;
            in vec3 frag_normal;
            in vec2 frag_tex_coords;
            
            out vec4 FragColor;
            
            uniform vec3 u_light_pos;
            uniform vec3 u_view_pos;
            uniform vec3 u_light_color;
            
            void main() {
                // Ambient
                float ambient_strength = 0.1;
                vec3 ambient = ambient_strength * u_light_color;
                
                // Diffuse
                vec3 norm = normalize(frag_normal);
                vec3 light_dir = normalize(u_light_pos - frag_position);
                float diff = max(dot(norm, light_dir), 0.0);
                vec3 diffuse = diff * u_light_color;
                
                // Specular
                float specular_strength = 0.5;
                vec3 view_dir = normalize(u_view_pos - frag_position);
                vec3 reflect_dir = reflect(-light_dir, norm);
                float spec = pow(max(dot(view_dir, reflect_dir), 0.0), 32.0);
                vec3 specular = specular_strength * spec * u_light_color;
                
                vec3 result = (ambient + diffuse + specular) * vec3(0.5, 0.5, 1.0);
                FragColor = vec4(result, 1.0);
            }"#;
            
            let shader = Shader::new(&gl, vertex_shader, fragment_shader)?;
        }
        
        let camera = Camera::new(
            Vector3::new(0.0, 0.0, 3.0),
            Vector3::new(0.0, 0.0, 0.0),
            Vector3::new(0.0, 1.0, 0.0),
            45.0,
            800.0 / 600.0,
            0.1,
            100.0,
        );
        
        Ok(Self {
            gl,
            shader,
            camera,
            models: HashMap::new(),
            current_city_index: 0,
            menu_state: MenuState::Loading,
            lod_manager: LodManager::new(),
            texture_streaming: TextureStreamingSystem::new(128, 10.0, 5), // Max 128 textures, 10 units per tile, 5 tile radius
        })
    }
    
    pub fn render(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        unsafe {
            self.gl.clear_color(0.1, 0.2, 0.3, 1.0);
            self.gl.clear(glow::COLOR_BUFFER_BIT | glow::DEPTH_BUFFER_BIT);
        }

        // Update LOD system based on camera position
        self.lod_manager.update_all_lods(&self.camera.position);

        // Update texture streaming based on camera position
        self.texture_streaming.update_camera_position(nalgebra::Vector2::new(
            self.camera.position.x,
            self.camera.position.z,
        ));

        match self.menu_state {
            MenuState::Loading => self.render_loading_screen()?,
            MenuState::MainMenu => self.render_main_menu()?,
            MenuState::CitySelection => self.render_city_selection()?,
            MenuState::InGame => self.render_game()?,
            MenuState::WorldCreation => self.render_world_creation()?,
            MenuState::Settings => self.render_settings()?,
        }

        Ok(())
    }
    
    pub fn update_camera_for_frame(&mut self, truck_position: Vector3<f32>, truck_rotation: UnitQuaternion<f32>) {
        self.camera.update_for_truck(truck_position, truck_rotation);
    }
    
    fn render_loading_screen(&self) -> Result<(), Box<dyn std::error::Error>> {
        // Render loading screen
        println!("Loading...");
        Ok(())
    }
    
    fn render_main_menu(&self) -> Result<(), Box<dyn std::error::Error>> {
        // Render main menu with text
        println!("Main Menu");
        Ok(())
    }
    
    fn render_city_selection(&self) -> Result<(), Box<dyn std::error::Error>> {
        // Render city selection screen
        println!("City Selection");
        Ok(())
    }
    
    fn render_game(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        // Render the actual game scene with proper OpenGL rendering
        
        // Get visible objects from LOD system
        let visible_objects = self.lod_manager.get_objects_in_view(&self.camera.position, 100.0);
        
        // Use the shader
        self.shader.bind();
        
        // Set up view and projection matrices
        let projection = self.camera.projection_matrix();
        let view = self.camera.view_matrix();
        
        unsafe {
            let u_projection = self.gl.get_uniform_location(self.shader.program, "u_projection").unwrap();
            let u_view = self.gl.get_uniform_location(self.shader.program, "u_view").unwrap();
            let u_light_pos = self.gl.get_uniform_location(self.shader.program, "u_light_pos").unwrap();
            let u_view_pos = self.gl.get_uniform_location(self.shader.program, "u_view_pos").unwrap();
            let u_light_color = self.gl.get_uniform_location(self.shader.program, "u_light_color").unwrap();
            
            self.gl.uniform_matrix_4_f32_slice(Some(&u_projection), false, projection.as_slice());
            self.gl.uniform_matrix_4_f32_slice(Some(&u_view), false, view.as_slice());
            self.gl.uniform_3_f32(Some(&u_light_pos), 5.0, 5.0, 5.0);
            self.gl.uniform_3_f32(Some(&u_view_pos), self.camera.position.x, self.camera.position.y, self.camera.position.z);
            self.gl.uniform_3_f32(Some(&u_light_color), 1.0, 1.0, 1.0);
        }
        
        // Render each visible object using appropriate LOD model
        for (_index, lod_model) in visible_objects {
            match lod_model {
                crate::graphics::lod_system::LodModel::HighPoly { vertices, indices } => {
                    // Create and render a temporary mesh for high-poly model
                    let mesh = Mesh::new(&self.gl, &vertices, &indices)?;
                    mesh.draw();
                },
                crate::graphics::lod_system::LodModel::MediumPoly { vertices, indices } => {
                    // Create and render a temporary mesh for medium-poly model
                    let mesh = Mesh::new(&self.gl, &vertices, &indices)?;
                    mesh.draw();
                },
                crate::graphics::lod_system::LodModel::LowPoly { vertices, indices } => {
                    // Create and render a temporary mesh for low-poly model
                    let mesh = Mesh::new(&self.gl, &vertices, &indices)?;
                    mesh.draw();
                },
                crate::graphics::lod_system::LodModel::Billboard { texture_id, size } => {
                    // For billboards, we'd need to create quad geometry and apply the texture
                    // This is a simplified approach - in a real engine, you'd want to optimize this
                    // For now, just skip rendering since we don't have a texture reference here
                },
            }
        }
        
        // Also render models from the traditional model system
        for (_, model) in &self.models {
            for mesh in &model.meshes {
                mesh.draw();
            }
        }
        
        Ok(())
    }
    
    fn render_world_creation(&self) -> Result<(), Box<dyn std::error::Error>> {
        // Render world creation screen
        println!("World Creation");
        Ok(())
    }
    
    fn render_settings(&self) -> Result<(), Box<dyn std::error::Error>> {
        // Render settings screen
        println!("Settings");
        Ok(())
    }
    
    pub fn load_model(&mut self, name: String, model: Model) {
        self.models.insert(name, model);
    }
    
    pub fn render_model(&self, model_name: &str) -> Result<(), Box<dyn std::error::Error>> {
        if let Some(model) = self.models.get(model_name) {
            self.shader.bind();
            
            let projection = self.camera.projection_matrix();
            let view = self.camera.view_matrix();
            
            unsafe {
                let u_projection = self.gl.get_uniform_location(self.shader.program, "u_projection").unwrap();
                let u_view = self.gl.get_uniform_location(self.shader.program, "u_view").unwrap();
                
                self.gl.uniform_matrix_4_f32_slice(Some(&u_projection), false, projection.as_slice());
                self.gl.uniform_matrix_4_f32_slice(Some(&u_view), false, view.as_slice());
            }
            
            for mesh in &model.meshes {
                mesh.draw();
            }
        }
        
        Ok(())
    }
    
    pub fn set_camera(&mut self, camera: Camera) {
        self.camera = camera;
    }
    
    pub fn next_city(&mut self) {
        self.current_city_index = (self.current_city_index + 1) % 14; // 14 Siberian cities
    }
    
    pub fn prev_city(&mut self) {
        if self.current_city_index == 0 {
            self.current_city_index = 13;
        } else {
            self.current_city_index -= 1;
        }
    }
}