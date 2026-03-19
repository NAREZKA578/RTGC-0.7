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
    // SPRINT 1: Terrain & Vehicle rendering
    terrain_mesh: Option<Mesh>,
    vehicle_box_mesh: Option<Mesh>,
    vehicle_transform: Option<(Vector3<f32>, UnitQuaternion<f32>)>,
    // Window dimensions for HUD rendering
    width: u32,
    height: u32,
    // HUD Manager reference for rendering
    hud_data: Option<crate::ui::hud::VehicleHudData>,
    // SPRINT 5: Weather and Day/Night cycle support
    sky_color_top: Vector3<f32>,
    sky_color_horizon: Vector3<f32>,
    sun_direction: Vector3<f32>,
    ambient_intensity: f32,
    vehicle_lights_enabled: bool,
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
            texture_streaming: TextureStreamingSystem::new(128, 10.0, 5),
            // SPRINT 1: Initialize terrain & vehicle mesh placeholders
            terrain_mesh: None,
            vehicle_box_mesh: None,
            vehicle_transform: None,
            hud_data: None,
            // SPRINT 5: Weather and Day/Night defaults
            sky_color_top: Vector3::new(0.4, 0.6, 0.9),
            sky_color_horizon: Vector3::new(0.7, 0.8, 0.9),
            sun_direction: Vector3::y(),
            ambient_intensity: 0.5,
            vehicle_lights_enabled: false,
        })
    }
    
    /// Set the terrain mesh for rendering
    pub fn set_terrain_mesh(&mut self, mesh: Mesh) {
        self.terrain_mesh = Some(mesh);
    }
    
    /// Set vehicle transform and HUD data
    pub fn set_vehicle_transform(&mut self, pos: Vector3<f32>, rot: UnitQuaternion<f32>) {
        self.vehicle_transform = Some((pos, rot));
    }
    
    /// Set HUD data for rendering
    pub fn set_hud_data(&mut self, data: crate::ui::hud::VehicleHudData) {
        self.hud_data = Some(data);
    }

    // SPRINT 5: Weather and Day/Night cycle methods
    pub fn set_sky_color(&mut self, top: Vector3<f32>, horizon: Vector3<f32>) {
        self.sky_color_top = top;
        self.sky_color_horizon = horizon;
    }

    pub fn set_sun_direction(&mut self, dir: Vector3<f32>) {
        self.sun_direction = dir;
    }

    pub fn set_ambient_intensity(&mut self, intensity: f32) {
        self.ambient_intensity = intensity.clamp(0.0, 1.0);
    }

    pub fn enable_vehicle_lights(&mut self, enable: bool) {
        self.vehicle_lights_enabled = enable;
    }

    /// Create a simple box mesh for the vehicle (temporary until GLTF loading works)
    pub fn create_vehicle_box_mesh(&mut self, half_extents: Vector3<f32>) -> Result<(), Box<dyn std::error::Error>> {
        // Create a unit cube centered at origin, scaled by half_extents
        let hx = half_extents.x;
        let hy = half_extents.y;
        let hz = half_extents.z;
        
        // Cube vertices: 8 corners with normals
        let vertices: Vec<f32> = vec![
            // Front face (z = +hz)
            -hx, -hy,  hz,  0.0, 0.0, 1.0,  0.0, 0.0,
             hx, -hy,  hz,  0.0, 0.0, 1.0,  1.0, 0.0,
             hx,  hy,  hz,  0.0, 0.0, 1.0,  1.0, 1.0,
            -hx,  hy,  hz,  0.0, 0.0, 1.0,  0.0, 1.0,
            // Back face (z = -hz)
             hx, -hy, -hz,  0.0, 0.0,-1.0,  0.0, 0.0,
            -hx, -hy, -hz,  0.0, 0.0,-1.0,  1.0, 0.0,
            -hx,  hy, -hz,  0.0, 0.0,-1.0,  1.0, 1.0,
             hx,  hy, -hz,  0.0, 0.0,-1.0,  0.0, 1.0,
            // Top face (y = +hy)
            -hx,  hy, -hz,  0.0, 1.0, 0.0,  0.0, 0.0,
             hx,  hy, -hz,  0.0, 1.0, 0.0,  1.0, 0.0,
             hx,  hy,  hz,  0.0, 1.0, 0.0,  1.0, 1.0,
            -hx,  hy,  hz,  0.0, 1.0, 0.0,  0.0, 1.0,
            // Bottom face (y = -hy)
            -hx, -hy,  hz,  0.0,-1.0, 0.0,  0.0, 0.0,
             hx, -hy,  hz,  0.0,-1.0, 0.0,  1.0, 0.0,
             hx, -hy, -hz,  0.0,-1.0, 0.0,  1.0, 1.0,
            -hx, -hy, -hz,  0.0,-1.0, 0.0,  0.0, 1.0,
            // Right face (x = +hx)
             hx, -hy, -hz,  1.0, 0.0, 0.0,  0.0, 0.0,
             hx,  hy, -hz,  1.0, 0.0, 0.0,  1.0, 0.0,
             hx,  hy,  hz,  1.0, 0.0, 0.0,  1.0, 1.0,
             hx, -hy,  hz,  1.0, 0.0, 0.0,  0.0, 1.0,
            // Left face (x = -hx)
            -hx, -hy,  hz, -1.0, 0.0, 0.0,  0.0, 0.0,
            -hx,  hy,  hz, -1.0, 0.0, 0.0,  1.0, 0.0,
            -hx,  hy, -hz, -1.0, 0.0, 0.0,  1.0, 1.0,
            -hx, -hy, -hz, -1.0, 0.0, 0.0,  0.0, 1.0,
        ];
        
        let indices: Vec<u32> = vec![
            0, 1, 2, 0, 2, 3,       // Front
            4, 5, 6, 4, 6, 7,       // Back
            8, 9, 10, 8, 10, 11,    // Top
            12, 13, 14, 12, 14, 15, // Bottom
            16, 17, 18, 16, 18, 19, // Right
            20, 21, 22, 20, 22, 23, // Left
        ];
        
        self.vehicle_box_mesh = Some(Mesh::new(&self.gl, &vertices, &indices)?);
        Ok(())
    }
    
    pub fn render(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        unsafe {
            // SPRINT 5: Clear with sky gradient color (using top color for now)
            self.gl.clear_color(
                self.sky_color_top.x,
                self.sky_color_top.y,
                self.sky_color_top.z,
                1.0
            );
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
            // Set uniforms with safe handling - skip if uniform not found
            if let Some(u_projection) = self.gl.get_uniform_location(self.shader.program, "u_projection") {
                self.gl.uniform_matrix_4_f32_slice(Some(&u_projection), false, projection.as_slice());
            }
            if let Some(u_view) = self.gl.get_uniform_location(self.shader.program, "u_view") {
                self.gl.uniform_matrix_4_f32_slice(Some(&u_view), false, view.as_slice());
            }
            // SPRINT 5: Light position from sun direction (scaled for shader)
            if let Some(u_light_pos) = self.gl.get_uniform_location(self.shader.program, "u_light_pos") {
                let light_pos = self.sun_direction * 100.0;
                self.gl.uniform_3_f32(Some(&u_light_pos), light_pos.x, light_pos.y, light_pos.z);
            }
            if let Some(u_view_pos) = self.gl.get_uniform_location(self.shader.program, "u_view_pos") {
                self.gl.uniform_3_f32(Some(&u_view_pos), self.camera.position.x, self.camera.position.y, self.camera.position.z);
            }
            // SPRINT 5: Light color affected by ambient intensity and weather
            if let Some(u_light_color) = self.gl.get_uniform_location(self.shader.program, "u_light_color") {
                let light_intensity = self.ambient_intensity;
                self.gl.uniform_3_f32(Some(&u_light_color), 
                    light_intensity, light_intensity, light_intensity * 1.1);
            }
        }
        
        // === SPRINT 1: Render terrain mesh ===
        if let Some(ref terrain_mesh) = self.terrain_mesh {
            unsafe {
                // Set model matrix to identity for terrain
                if let Some(u_model) = self.gl.get_uniform_location(self.shader.program, "u_model") {
                    let identity = Matrix4::identity();
                    self.gl.uniform_matrix_4_f32_slice(Some(&u_model), false, identity.as_slice());
                }
            }
            terrain_mesh.draw();
        }
        
        // === SPRINT 1: Render vehicle as box ===
        if let Some((pos, rot)) = self.vehicle_transform {
            let model_matrix = rot.to_homogeneous().prepend_translation(&pos);
            unsafe {
                if let Some(u_model) = self.gl.get_uniform_location(self.shader.program, "u_model") {
                    self.gl.uniform_matrix_4_f32_slice(Some(&u_model), false, model_matrix.as_slice());
                }
            }
            if let Some(ref box_mesh) = self.vehicle_box_mesh {
                box_mesh.draw();
            }
        }
        
        // Render each visible object using appropriate LOD model
        for (_index, lod_model) in visible_objects {
            match lod_model {
                crate::graphics::lod_system::LodModel::HighPoly { vertices, indices } => {
                    let mesh = Mesh::new(&self.gl, &vertices, &indices)?;
                    mesh.draw();
                },
                crate::graphics::lod_system::LodModel::MediumPoly { vertices, indices } => {
                    let mesh = Mesh::new(&self.gl, &vertices, &indices)?;
                    mesh.draw();
                },
                crate::graphics::lod_system::LodModel::LowPoly { vertices, indices } => {
                    let mesh = Mesh::new(&self.gl, &vertices, &indices)?;
                    mesh.draw();
                },
                crate::graphics::lod_system::LodModel::Billboard { texture_id, size } => {
                    // Skip billboards for now
                },
            }
        }
        
        // Also render models from the traditional model system
        for (_, model) in &self.models {
            for mesh in &model.meshes {
                mesh.draw();
            }
        }
        
        // === SPRINT 2: Render HUD ===
        // HUD рисуется после основной сцены, без depth test
        self.render_hud()?;
        
        Ok(())
    }
    
    /// Render HUD overlay (2D UI without depth test)
    fn render_hud(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        use crate::ui::hud::HudFlashElement;
        
        // Get HUD data from renderer's stored data (set by engine via set_hud_data)
        let hud_data = self.hud_data.clone().unwrap_or_else(|| crate::ui::hud::VehicleHudData {
            speed_kmh: self.vehicle_transform
                .map(|(_, _)| 65.0)  // placeholder if no data
                .unwrap_or(0.0),
            engine_rpm: 2200.0,
            engine_rpm_max: 3200.0,
            gear: crate::ui::hud::GearState::Drive(4),
            engine_running: true,
            fuel_level: 0.75,
            ..Default::default()
        });
        
        unsafe {
            // Disable depth test for 2D UI
            self.gl.disable(glow::DEPTH_TEST);
            
            // Use simple color for now (will use shader later)
            self.gl.use_program(Some(self.shader.program));
            
            // Draw speed panel (bottom left rectangle)
            self.draw_rect(10.0, self.height as f32 - 60.0, 200.0, 50.0, [0.1, 0.1, 0.1, 0.8]);
            
            // Draw speed value (simple representation)
            let speed_text = format!("{:.0} km/h", hud_data.speed_kmh);
            // Text rendering will be added later with bitmap font
            
            // Draw RPM bar
            let rpm_ratio = (hud_data.engine_rpm / hud_data.engine_rpm_max).min(1.0);
            let bar_width = 150.0 * rpm_ratio;
            self.draw_rect(20.0, self.height as f32 - 40.0, bar_width, 10.0, [0.2, 0.8, 0.2, 1.0]);
            
            // Draw fuel bar
            let fuel_width = 100.0 * hud_data.fuel_level;
            self.draw_rect(20.0, self.height as f32 - 25.0, fuel_width, 8.0, [0.8, 0.8, 0.2, 1.0]);
            
            // Draw wheel contact indicators (4 dots)
            for (i, &contact) in hud_data.wheel_contact.iter().enumerate() {
                let x = 250.0 + (i as f32 * 20.0);
                let y = self.height as f32 - 40.0;
                let color = if contact { [0.0, 1.0, 0.0, 1.0] } else { [1.0, 0.0, 0.0, 1.0] };
                // Using small rect instead of circle for simplicity
                self.draw_rect(x - 6.0, y - 6.0, 12.0, 12.0, color);
            }
            
            // Flash warning for low fuel
            if hud_data.fuel_reserve {
                self.draw_rect(150.0, self.height as f32 - 25.0, 100.0, 8.0, [1.0, 0.0, 0.0, 1.0]);
            }
            
            // Re-enable depth test
            self.gl.enable(glow::DEPTH_TEST);
        }
        
        Ok(())
    }
    
    /// Draw a 2D rectangle (simple quad)
    unsafe fn draw_rect(&self, x: f32, y: f32, width: f32, height: f32, color: [f32; 4]) {
        // Simple HUD rect drawing using triangle fan
        // This is a placeholder - full implementation needs proper VAO/VBO setup
        // For now we use a simple approach with pre-defined quad vertices
        
        let vertices: [f32; 16] = [
            x, y,                    // bottom-left
            x + width, y,            // bottom-right
            x + width, y + height,   // top-right
            x, y + height,           // top-left
        ];
        
        // In full implementation: bind UI shader, set ortho projection, draw quad
        // For Sprint 1 alpha: stub implementation
    }
    
    /// Draw a 2D triangle (for minimap player icon)
    unsafe fn draw_triangle(&self, x1: f32, y1: f32, x2: f32, y2: f32, x3: f32, y3: f32, color: [f32; 4]) {
        // Placeholder for triangle drawing
    }
    
    /// Draw text at position
    unsafe fn draw_text(&self, text: &str, x: f32, y: f32, size: f32, color: [f32; 4]) {
        // Placeholder for text rendering - needs bitmap font or signed distance field font
        // For Sprint 1: stub implementation
    }
    
    /// Get renderer width
    pub fn get_width(&self) -> u32 {
        self.width
    }
    
    /// Get renderer height
    pub fn get_height(&self) -> u32 {
        self.height
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
                // Set uniforms with safe handling - skip if uniform not found
                if let Some(u_projection) = self.gl.get_uniform_location(self.shader.program, "u_projection") {
                    self.gl.uniform_matrix_4_f32_slice(Some(&u_projection), false, projection.as_slice());
                }
                if let Some(u_view) = self.gl.get_uniform_location(self.shader.program, "u_view") {
                    self.gl.uniform_matrix_4_f32_slice(Some(&u_view), false, view.as_slice());
                }
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