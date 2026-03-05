use glam::{Mat4, Vec3};

pub struct Camera {
    pub position: Vec3,
    pub target: Vec3,
    pub up: Vec3,
    pub fov: f32,
    pub near: f32,
    pub far: f32,
    pub aspect_ratio: f32,
    pub view_matrix: Mat4,
    pub projection_matrix: Mat4,
    pub view_projection_matrix: Mat4,
}

impl Camera {
    pub fn new(
        position: Vec3,
        target: Vec3,
        up: Vec3,
        fov: f32,
        near: f32,
        far: f32,
        aspect_ratio: f32,
    ) -> Self {
        let mut camera = Self {
            position,
            target,
            up,
            fov,
            near,
            far,
            aspect_ratio,
            view_matrix: Mat4::IDENTITY,
            projection_matrix: Mat4::IDENTITY,
            view_projection_matrix: Mat4::IDENTITY,
        };
        camera.update_matrices();
        camera
    }

    pub fn update_matrices(&mut self) {
        self.view_matrix = Mat4::look_at_rh(self.position, self.target, self.up);
        self.projection_matrix = Mat4::perspective_rh_gl(
            self.fov,
            self.aspect_ratio,
            self.near,
            self.far,
        );
        self.view_projection_matrix = self.projection_matrix * self.view_matrix;
    }

    pub fn set_aspect_ratio(&mut self, aspect_ratio: f32) {
        self.aspect_ratio = aspect_ratio;
        self.update_matrices();
    }

    pub fn look_at(&mut self, target: Vec3) {
        self.target = target;
        self.update_matrices();
    }

    pub fn translate(&mut self, offset: Vec3) {
        self.position += offset;
        self.target += offset;
        self.update_matrices();
    }

    pub fn rotate(&mut self, angle: f32, axis: Vec3) {
        // Rotate the camera around the target point
        let direction = self.position - self.target;
        let rotated_direction = Mat4::from_axis_angle(axis, angle) * direction.extend(0.0);
        self.position = self.target + rotated_direction.xyz();
        self.update_matrices();
    }
}

pub struct CameraUniform {
    pub view_matrix: [[f32; 4]; 4],
    pub projection_matrix: [[f32; 4]; 4],
    pub view_projection_matrix: [[f32; 4]; 4],
    pub camera_position: [f32; 4],
}

impl From<&Camera> for CameraUniform {
    fn from(camera: &Camera) -> Self {
        CameraUniform {
            view_matrix: camera.view_matrix.to_cols_array_2d(),
            projection_matrix: camera.projection_matrix.to_cols_array_2d(),
            view_projection_matrix: camera.view_projection_matrix.to_cols_array_2d(),
            camera_position: [camera.position.x, camera.position.y, camera.position.z, 1.0],
        }
    }
}