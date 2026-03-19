//! Physics - Vehicle physics simulation

use nalgebra::{Vector3, Quaternion, Matrix3, UnitQuaternion};
use crate::physics::physics_module::RigidBody;

/// Vehicle configuration
#[derive(Debug, Clone)]
pub struct VehicleConfig {
    pub mass: f32,
    pub wheel_count: u8,
    pub wheel_radius: f32,
    pub suspension_stiffness: f32,
    pub suspension_damping: f32,
    pub suspension_rest_length: f32,
    pub max_suspension_travel: f32,
    pub engine_force: f32,
    pub brake_force: f32,
    pub max_steering_angle: f32,
    pub lateral_friction: f32,
    pub longitudinal_friction: f32,
    pub drag_coefficient: f32,
    pub downforce_coefficient: f32,
}

impl Default for VehicleConfig {
    fn default() -> Self {
        Self {
            mass: 1500.0,
            wheel_count: 4,
            wheel_radius: 0.35,
            suspension_stiffness: 35000.0,
            suspension_damping: 4500.0,
            suspension_rest_length: 0.4,
            max_suspension_travel: 0.2,
            engine_force: 5000.0,
            brake_force: 10000.0,
            max_steering_angle: 0.6, // ~35 degrees
            lateral_friction: 1.0,
            longitudinal_friction: 1.0,
            drag_coefficient: 0.3,
            downforce_coefficient: 0.0,
        }
    }
}

/// Wheel state
#[derive(Debug, Clone)]
pub struct WheelState {
    /// Position relative to vehicle center
    pub local_position: Vector3<f32>,
    /// Current steering angle (radians)
    pub steering_angle: f32,
    /// Suspension compression (0 = rest, positive = compressed)
    pub suspension_compression: f32,
    /// Suspension velocity
    pub suspension_velocity: f32,
    /// Wheel rotation angle
    pub rotation_angle: f32,
    /// Wheel angular velocity
    pub angular_velocity: f32,
    /// Is wheel in contact with ground
    pub is_in_contact: bool,
    /// Contact point world position
    pub contact_point: Option<Vector3<f32>>,
    /// Contact normal
    pub contact_normal: Option<Vector3<f32>>,
}

impl WheelState {
    pub fn new(local_position: Vector3<f32>) -> Self {
        Self {
            local_position,
            steering_angle: 0.0,
            suspension_compression: 0.0,
            suspension_velocity: 0.0,
            rotation_angle: 0.0,
            angular_velocity: 0.0,
            is_in_contact: false,
            contact_point: None,
            contact_normal: None,
        }
    }

    pub fn front_left(config: &VehicleConfig) -> Self {
        Self::new(Vector3::new(1.0, -0.5, 0.8))
    }

    pub fn front_right(config: &VehicleConfig) -> Self {
        Self::new(Vector3::new(1.0, -0.5, -0.8))
    }

    pub fn rear_left(config: &VehicleConfig) -> Self {
        Self::new(Vector3::new(-1.0, -0.5, 0.8))
    }

    pub fn rear_right(config: &VehicleConfig) -> Self {
        Self::new(Vector3::new(-1.0, -0.5, -0.8))
    }
}

/// Vehicle control inputs
#[derive(Debug, Clone, Copy, Default)]
pub struct VehicleControls {
    /// Throttle input (-1.0 to 1.0, negative for reverse)
    pub throttle: f32,
    /// Brake input (0.0 to 1.0)
    pub brake: f32,
    /// Steering input (-1.0 to 1.0, left to right)
    pub steering: f32,
    /// Handbrake (0.0 to 1.0)
    pub handbrake: f32,
}

impl VehicleControls {
    pub fn new(throttle: f32, brake: f32, steering: f32, handbrake: f32) -> Self {
        Self {
            throttle: throttle.clamp(-1.0, 1.0),
            brake: brake.clamp(0.0, 1.0),
            steering: steering.clamp(-1.0, 1.0),
            handbrake: handbrake.clamp(0.0, 1.0),
        }
    }
}

/// Simple vehicle physics model
pub struct Vehicle {
    config: VehicleConfig,
    body: RigidBody,
    wheels: Vec<WheelState>,
    controls: VehicleControls,
    /// Center of gravity offset from body origin
    pub cog_offset: Vector3<f32>,
}

impl Vehicle {
    /// Creates a new vehicle with the given configuration
    pub fn new(config: VehicleConfig) -> Self {
        let mut body = RigidBody::new_box(Vector3::zeros(), config.mass, Vector3::new(0.9, 0.3, 2.25));
        
        let mut wheels = Vec::with_capacity(config.wheel_count as usize);
        
        // Set up default 4-wheel configuration
        if config.wheel_count >= 4 {
            wheels.push(WheelState::front_left(&config));
            wheels.push(WheelState::front_right(&config));
            wheels.push(WheelState::rear_left(&config));
            wheels.push(WheelState::rear_right(&config));
        }

        Self {
            config,
            body,
            wheels,
            controls: VehicleControls::default(),
            cog_offset: Vector3::new(0.0, 0.0, 0.0),
        }
    }

    /// Sets the vehicle controls
    pub fn set_controls(&mut self, controls: VehicleControls) {
        self.controls = controls;
    }

    /// Gets the current controls
    pub fn get_controls(&self) -> &VehicleControls {
        &self.controls
    }

    /// Updates the vehicle physics
    pub fn update(&mut self, dt: f32, ground_height: impl Fn(f32, f32) -> f32) {
        // Apply steering to front wheels
        let target_steering = self.controls.steering * self.config.max_steering_angle;
        
        if self.wheels.len() >= 2 {
            // Front wheel drive or 4WD
            self.wheels[0].steering_angle = target_steering;
            self.wheels[1].steering_angle = target_steering;
        }

        // Update suspension and wheel forces
        for (i, wheel) in self.wheels.iter_mut().enumerate() {
            self.update_wheel(i, wheel, dt, &ground_height);
        }

        // Apply aerodynamic drag
        self.apply_aerodynamics(dt);

        // Integrate rigid body motion (use update method which includes full integration)
        self.body.update(dt);
    }

    /// Updates a single wheel's physics
    fn update_wheel(
        &mut self,
        wheel_index: usize,
        wheel: &mut WheelState,
        dt: f32,
        ground_height: &impl Fn(f32, f32) -> f32,
    ) {
        // Get wheel world position
        let wheel_world_pos = self.body.position + self.body.rotation * wheel.local_position;
        
        // Raycast вниз для определения контакта с землёй
        let ray_origin = wheel_world_pos;
        let ray_direction = Vector3::new(0.0, -1.0, 0.0);
        let ray_length = self.config.wheel_radius + self.config.suspension_rest_length + self.config.max_suspension_travel;
        
        // Sample ground height at wheel position
        let ground_y = ground_height(wheel_world_pos.x, wheel_world_pos.z);
        
        // Calculate suspension compression
        let wheel_bottom_y = wheel_world_pos.y - self.config.wheel_radius;
        let suspension_deflection = ground_y - wheel_bottom_y;
        
        wheel.is_in_contact = suspension_deflection > 0.0;
        
        if wheel.is_in_contact {
            // Устанавливаем нормаль контакта (вверх, так как земля горизонтальная)
            wheel.contact_normal = Some(Vector3::new(0.0, 1.0, 0.0));
            wheel.contact_point = Some(Vector3::new(wheel_world_pos.x, ground_y, wheel_world_pos.z));
            
            wheel.suspension_compression = suspension_deflection.clamp(0.0, self.config.max_suspension_travel);
            
            // Calculate suspension force
            let spring_force = wheel.suspension_compression * self.config.suspension_stiffness;
            let damping_force = wheel.suspension_velocity * self.config.suspension_damping;
            let suspension_force = (spring_force + damping_force).max(0.0);
            
            // Apply suspension force to vehicle body
            let force_dir = self.body.rotation * Vector3::new(0.0, 1.0, 0.0);
            let force = force_dir * suspension_force;
            
            self.body.apply_force_at_point(force, wheel_world_pos);
            
            // Calculate tire forces based on slip
            self.apply_tire_forces(wheel, wheel_index, dt);
            
            // Update wheel rotation based on vehicle speed
            let linear_speed = self.body.velocity.norm();
            wheel.angular_velocity = linear_speed / self.config.wheel_radius;
        } else {
            wheel.suspension_compression = 0.0;
            wheel.suspension_velocity = 0.0;
        }
        
        // Update wheel rotation
        wheel.rotation_angle += wheel.angular_velocity * dt;
    }

    /// Applies tire forces based on slip angles
    fn apply_tire_forces(&mut self, wheel: &WheelState, wheel_index: usize, dt: f32) {
        if !wheel.is_in_contact {
            return;
        }

        let wheel_world_pos = self.body.position + self.body.rotation * wheel.local_position;
        let wheel_vel = self.body.get_velocity_at_point(wheel_world_pos);
        
        // Calculate slip angle (simplified)
        let forward = self.body.rotation * Vector3::new(0.0, 0.0, 1.0);
        let lateral = self.body.rotation * Vector3::new(1.0, 0.0, 0.0);
        
        let forward_vel = wheel_vel.dot(&forward);
        let lateral_vel = wheel_vel.dot(&lateral);
        
        // Apply driving/braking force
        let drive_force = if wheel_index < 2 {
            // Front wheel drive
            self.controls.throttle * self.config.engine_force
        } else {
            // Rear wheel drive (or change for 4WD)
            0.0
        };
        
        let braking_force = -self.controls.brake * self.config.brake_force 
                           - self.controls.handbrake * self.config.brake_force * 0.5;
        
        let longitudinal_force = drive_force + braking_force;
        
        // Apply forces
        let drive_dir = forward * longitudinal_force;
        self.body.apply_force(drive_dir);
    }

    /// Applies aerodynamic forces
    fn apply_aerodynamics(&mut self, dt: f32) {
        let speed_sq = self.body.velocity.norm_squared();
        let speed = self.body.velocity.norm();
        
        if speed < 0.01 {
            return;
        }
        
        // Air drag
        let drag_direction = -self.body.velocity.normalize();
        let drag_magnitude = 0.5 * 1.225 * self.config.drag_coefficient * 2.0 * speed_sq;
        let drag_force = drag_direction * drag_magnitude;
        
        self.body.apply_force(drag_force);
        
        // Downforce (if configured)
        if self.config.downforce_coefficient > 0.0 {
            let downforce = self.body.rotation * Vector3::new(0.0, -1.0, 0.0) 
                          * self.config.downforce_coefficient * speed_sq;
            self.body.apply_force(downforce);
        }
    }

    /// Gets the vehicle's rigid body
    pub fn body(&self) -> &RigidBody {
        &self.body
    }

    /// Gets the vehicle's rigid body (mutable)
    pub fn body_mut(&mut self) -> &mut RigidBody {
        &mut self.body
    }

    /// Gets all wheels
    pub fn wheels(&self) -> &[WheelState] {
        &self.wheels
    }

    /// Gets the vehicle speed
    pub fn speed(&self) -> f32 {
        self.body.velocity.norm()
    }

    /// Gets the vehicle position
    pub fn position(&self) -> Vector3<f32> {
        self.body.position
    }

    /// Sets the vehicle position
    pub fn set_position(&mut self, pos: Vector3<f32>) {
        self.body.position = pos;
    }

    /// Resets the vehicle state
    pub fn reset(&mut self) {
        self.body = RigidBody::new_box(Vector3::zeros(), self.config.mass, Vector3::new(0.9, 0.3, 2.25));
        
        for wheel in &mut self.wheels {
            wheel.suspension_compression = 0.0;
            wheel.suspension_velocity = 0.0;
            wheel.rotation_angle = 0.0;
            wheel.angular_velocity = 0.0;
            wheel.is_in_contact = false;
            wheel.contact_point = None;
            wheel.contact_normal = None;
        }
        
        self.controls = VehicleControls::default();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vehicle_creation() {
        let config = VehicleConfig::default();
        let vehicle = Vehicle::new(config.clone());
        
        assert_eq!(vehicle.body.mass, config.mass);
        assert_eq!(vehicle.wheels.len(), 4);
    }

    #[test]
    fn test_vehicle_controls() {
        let mut vehicle = Vehicle::new(VehicleConfig::default());
        
        let controls = VehicleControls::new(1.0, 0.5, 0.3, 0.0);
        vehicle.set_controls(controls.clone());
        
        assert_eq!(vehicle.get_controls().throttle, 1.0);
        assert_eq!(vehicle.get_controls().brake, 0.5);
        assert_eq!(vehicle.get_controls().steering, 0.3);
    }
}
