//! Модуль рендеринга для игрового движка на Rust
//! 
//! Реализует:
//! - Базовый рендерер на wgpu
//! - Камеру с перспективной проекцией
//! - Загрузку и рендеринг мешей

use wgpu::{Device, Queue, Surface, SurfaceConfiguration, TextureView};
use glam::{Mat4, Vec3, Quat};

/// Камера для рендеринга
#[derive(Clone, Debug)]
pub struct Camera {
    /// Позиция камеры
    pub position: Vec3,
    /// Направление взгляда (target)
    pub target: Vec3,
    /// Вектор вверх
    pub up: Vec3,
    /// Поле зрения (градусы)
    pub fov: f32,
    /// Соотношение сторон
    pub aspect: f32,
    /// Ближняя плоскость отсечения
    pub near: f32,
    /// Дальняя плоскость отсечения
    pub far: f32,
}

impl Camera {
    pub fn new(position: Vec3, target: Vec3, up: Vec3) -> Self {
        Self {
            position,
            target,
            up,
            fov: 60.0,
            aspect: 16.0 / 9.0,
            near: 0.1,
            far: 1000.0,
        }
    }

    /// Матрица вида (view matrix)
    pub fn view_matrix(&self) -> Mat4 {
        Mat4::look_at_rh(self.position, self.target, self.up)
    }

    /// Матрица проекции (projection matrix)
    pub fn projection_matrix(&self) -> Mat4 {
        Mat4::perspective_rh(
            self.fov.to_radians(),
            self.aspect,
            self.near,
            self.far,
        )
    }

    /// Матрица вида-проекции
    pub fn view_projection_matrix(&self) -> Mat4 {
        self.projection_matrix() * self.view_matrix()
    }

    /// Движение камеры вперед/назад
    pub fn move_forward(&mut self, distance: f32) {
        let direction = (self.target - self.position).normalize();
        self.position += direction * distance;
        self.target += direction * distance;
    }

    /// Движение камеры влево/вправо
    pub fn move_right(&mut self, distance: f32) {
        let direction = (self.target - self.position).normalize();
        let right = direction.cross(self.up).normalize();
        self.position += right * distance;
        self.target += right * distance;
    }

    /// Поворот камеры по горизонтали
    pub fn rotate_horizontal(&mut self, angle: f32) {
        let offset = self.position - self.target;
        let rotation = Quat::from_rotation_y(angle);
        let rotated = rotation * offset;
        self.position = self.target + rotated;
    }

    /// Поворот камеры по вертикали
    pub fn rotate_vertical(&mut self, angle: f32) {
        let offset = self.position - self.target;
        let right = (self.target - self.position).normalize().cross(self.up).normalize();
        
        // Ограничиваем угол чтобы не перевернуться
        let pitch_angle = angle.clamp(-std::f32::consts::FRAC_PI_2 + 0.1, std::f32::consts::FRAC_PI_2 - 0.1);
        
        let rotation = Quat::from_axis_angle(right, pitch_angle);
        let rotated = rotation * offset;
        self.position = self.target + rotated;
    }
}

/// Простой меш
#[derive(Clone, Debug)]
pub struct Mesh {
    /// Название меша
    pub name: String,
    /// Вершины (позиция + нормаль + UV)
    pub vertices: Vec<Vertex>,
    /// Индексы
    pub indices: Vec<u32>,
}

/// Вершина меша
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct Vertex {
    pub position: [f32; 3],
    pub normal: [f32; 3],
    pub tex_coords: [f32; 2],
}

impl Vertex {
    pub fn desc() -> wgpu::VertexBufferLayout<'static> {
        wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<Vertex>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &[
                wgpu::VertexAttribute {
                    offset: 0,
                    shader_location: 0,
                    format: wgpu::VertexFormat::Float32x3,
                },
                wgpu::VertexAttribute {
                    offset: std::mem::size_of::<[f32; 3]>() as wgpu::BufferAddress,
                    shader_location: 1,
                    format: wgpu::VertexFormat::Float32x3,
                },
                wgpu::VertexAttribute {
                    offset: std::mem::size_of::<[f32; 6]>() as wgpu::BufferAddress,
                    shader_location: 2,
                    format: wgpu::VertexFormat::Float32x2,
                },
            ],
        }
    }
}

/// Основной рендерер
pub struct Renderer {
    device: Device,
    queue: Queue,
    surface_config: Option<SurfaceConfiguration>,
    camera: Camera,
    meshes: Vec<Mesh>,
}

impl Renderer {
    pub fn new(device: Device, queue: Queue) -> Self {
        Self {
            device,
            queue,
            surface_config: None,
            camera: Camera::new(
                Vec3::new(0.0, 5.0, 10.0),
                Vec3::ZERO,
                Vec3::Y,
            ),
            meshes: Vec::new(),
        }
    }

    /// Настройка поверхности
    pub fn configure_surface(&mut self, surface: &Surface, width: u32, height: u32) {
        let config = SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: surface.get_preferred_format(self.device.features()).unwrap(),
            width,
            height,
            present_mode: wgpu::PresentMode::Fifo,
            alpha_mode: wgpu::CompositeAlphaMode::Auto,
            view_formats: vec![],
        };
        
        surface.configure(&self.device, &config);
        self.surface_config = Some(config);
        
        // Обновляем aspect ratio камеры
        if let Some(cam) = self.camera.as_mut() {
            cam.aspect = width as f32 / height as f32;
        }
    }

    /// Добавление меша в сцену
    pub fn add_mesh(&mut self, mesh: Mesh) {
        self.meshes.push(mesh);
    }

    /// Создание куба
    pub fn create_cube(size: f32) -> Mesh {
        let s = size / 2.0;
        
        let vertices = vec![
            // Front face
            Vertex { position: [-s, -s, s], normal: [0.0, 0.0, 1.0], tex_coords: [0.0, 0.0] },
            Vertex { position: [s, -s, s], normal: [0.0, 0.0, 1.0], tex_coords: [1.0, 0.0] },
            Vertex { position: [s, s, s], normal: [0.0, 0.0, 1.0], tex_coords: [1.0, 1.0] },
            Vertex { position: [-s, s, s], normal: [0.0, 0.0, 1.0], tex_coords: [0.0, 1.0] },
            // Back face
            Vertex { position: [-s, -s, -s], normal: [0.0, 0.0, -1.0], tex_coords: [0.0, 0.0] },
            Vertex { position: [-s, s, -s], normal: [0.0, 0.0, -1.0], tex_coords: [0.0, 1.0] },
            Vertex { position: [s, s, -s], normal: [0.0, 0.0, -1.0], tex_coords: [1.0, 1.0] },
            Vertex { position: [s, -s, -s], normal: [0.0, 0.0, -1.0], tex_coords: [1.0, 0.0] },
            // Top face
            Vertex { position: [-s, s, -s], normal: [0.0, 1.0, 0.0], tex_coords: [0.0, 0.0] },
            Vertex { position: [-s, s, s], normal: [0.0, 1.0, 0.0], tex_coords: [0.0, 1.0] },
            Vertex { position: [s, s, s], normal: [0.0, 1.0, 0.0], tex_coords: [1.0, 1.0] },
            Vertex { position: [s, s, -s], normal: [0.0, 1.0, 0.0], tex_coords: [1.0, 0.0] },
            // Bottom face
            Vertex { position: [-s, -s, -s], normal: [0.0, -1.0, 0.0], tex_coords: [0.0, 0.0] },
            Vertex { position: [s, -s, -s], normal: [0.0, -1.0, 0.0], tex_coords: [1.0, 0.0] },
            Vertex { position: [s, -s, s], normal: [0.0, -1.0, 0.0], tex_coords: [1.0, 1.0] },
            Vertex { position: [-s, -s, s], normal: [0.0, -1.0, 0.0], tex_coords: [0.0, 1.0] },
            // Right face
            Vertex { position: [s, -s, -s], normal: [1.0, 0.0, 0.0], tex_coords: [0.0, 0.0] },
            Vertex { position: [s, s, -s], normal: [1.0, 0.0, 0.0], tex_coords: [0.0, 1.0] },
            Vertex { position: [s, s, s], normal: [1.0, 0.0, 0.0], tex_coords: [1.0, 1.0] },
            Vertex { position: [s, -s, s], normal: [1.0, 0.0, 0.0], tex_coords: [1.0, 0.0] },
            // Left face
            Vertex { position: [-s, -s, -s], normal: [-1.0, 0.0, 0.0], tex_coords: [0.0, 0.0] },
            Vertex { position: [-s, -s, s], normal: [-1.0, 0.0, 0.0], tex_coords: [1.0, 0.0] },
            Vertex { position: [-s, s, s], normal: [-1.0, 0.0, 0.0], tex_coords: [1.0, 1.0] },
            Vertex { position: [-s, s, -s], normal: [-1.0, 0.0, 0.0], tex_coords: [0.0, 1.0] },
        ];
        
        let indices = vec![
            0, 1, 2, 0, 2, 3,       // front
            4, 5, 6, 4, 6, 7,       // back
            8, 9, 10, 8, 10, 11,    // top
            12, 13, 14, 12, 14, 15, // bottom
            16, 17, 18, 16, 18, 19, // right
            20, 21, 22, 20, 22, 23, // left
        ];
        
        Mesh {
            name: "cube".to_string(),
            vertices,
            indices,
        }
    }

    /// Рендеринг кадра
    pub fn render(&self, _target_view: &TextureView) {
        // Здесь будет основной код рендеринга
        // В полной реализации используется wgpu command encoder
    }

    /// Получение камеры
    pub fn camera(&self) -> &Camera {
        &self.camera
    }

    /// Получение камеры (mutable)
    pub fn camera_mut(&mut self) -> &mut Camera {
        &mut self.camera
    }

    /// Количество мешей в сцене
    pub fn mesh_count(&self) -> usize {
        self.meshes.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_camera_creation() {
        let camera = Camera::new(
            Vec3::new(0.0, 5.0, 10.0),
            Vec3::ZERO,
            Vec3::Y,
        );
        
        assert_eq!(camera.position, Vec3::new(0.0, 5.0, 10.0));
        assert_eq!(camera.target, Vec3::ZERO);
        assert_eq!(camera.fov, 60.0);
    }

    #[test]
    fn test_camera_matrices() {
        let camera = Camera::new(
            Vec3::new(0.0, 0.0, 10.0),
            Vec3::ZERO,
            Vec3::Y,
        );
        
        let view = camera.view_matrix();
        let proj = camera.projection_matrix();
        let vp = camera.view_projection_matrix();
        
        // Проверяем что матрицы не нулевые
        assert!(view.abs_diff_ne(Mat4::ZERO, 0.001));
        assert!(proj.abs_diff_ne(Mat4::ZERO, 0.001));
        assert!(vp.abs_diff_ne(Mat4::ZERO, 0.001));
    }

    #[test]
    fn test_camera_movement() {
        let mut camera = Camera::new(
            Vec3::new(0.0, 0.0, 10.0),
            Vec3::ZERO,
            Vec3::Y,
        );
        
        camera.move_forward(5.0);
        assert!((camera.position.z - 5.0).abs() < 0.01);
        
        camera.move_right(3.0);
        assert!(camera.position.x > 0.0);
    }

    #[test]
    fn test_mesh_creation() {
        let cube = Renderer::create_cube(2.0);
        
        assert_eq!(cube.name, "cube");
        assert_eq!(cube.vertices.len(), 24); // 6 faces * 4 vertices
        assert_eq!(cube.indices.len(), 36);  // 6 faces * 2 triangles * 3 indices
    }

    #[test]
    fn test_vertex_layout() {
        let desc = Vertex::desc();
        assert_eq!(desc.array_stride, std::mem::size_of::<Vertex>() as u64);
        assert_eq!(desc.attributes.len(), 3); // position, normal, tex_coords
    }
}
