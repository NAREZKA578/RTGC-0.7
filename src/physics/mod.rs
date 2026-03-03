use nalgebra::{Vector3, Isometry3};
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct RigidBody {
    pub position: Vector3<f32>,
    pub rotation: nalgebra::UnitQuaternion<f32>,
    pub velocity: Vector3<f32>,
    pub angular_velocity: Vector3<f32>,
    pub mass: f32,
    pub inverse_mass: f32,
    pub inertia_tensor: nalgebra::Matrix3<f32>,
    pub inverse_inertia_tensor: nalgebra::Matrix3<f32>,
    pub restitution: f32,
    pub friction: f32,
}

impl RigidBody {
    pub fn new(position: Vector3<f32>, mass: f32) -> Self {
        let inverse_mass = if mass > 0.0 { 1.0 / mass } else { 0.0 };
        
        // Simple inertia tensor for a sphere
        let inertia_tensor = if mass > 0.0 {
            let r_squared = 2.0 / 5.0 * mass; // Assuming radius = 1 for simplicity
            nalgebra::Matrix3::new(
                r_squared, 0.0, 0.0,
                0.0, r_squared, 0.0,
                0.0, 0.0, r_squared
            )
        } else {
            nalgebra::Matrix3::zeros()
        };
        
        let inverse_inertia_tensor = if mass > 0.0 {
            inertia_tensor.try_inverse().unwrap_or(nalgebra::Matrix3::zeros())
        } else {
            nalgebra::Matrix3::zeros()
        };

        Self {
            position,
            rotation: nalgebra::UnitQuaternion::identity(),
            velocity: Vector3::new(0.0, 0.0, 0.0),
            angular_velocity: Vector3::new(0.0, 0.0, 0.0),
            mass,
            inverse_mass,
            inertia_tensor,
            inverse_inertia_tensor,
            restitution: 0.5,
            friction: 0.1,
        }
    }

    pub fn apply_force(&mut self, force: Vector3<f32>) {
        if self.inverse_mass > 0.0 {
            self.velocity += force * self.inverse_mass;
        }
    }

    pub fn apply_impulse(&mut self, impulse: Vector3<f32>) {
        if self.inverse_mass > 0.0 {
            self.velocity += impulse * self.inverse_mass;
        }
    }

    pub fn update(&mut self, dt: f32) {
        // Update position based on velocity
        self.position += self.velocity * dt;
        
        // Apply gravity (assuming -9.81 m/s^2 in y direction)
        if self.inverse_mass > 0.0 {
            self.velocity.y -= 9.81 * dt;
        }
        
        // Simple damping to prevent infinite acceleration
        self.velocity *= 0.999;
        self.angular_velocity *= 0.99;
    }
}

pub struct PhysicsWorld {
    pub rigid_bodies: Vec<RigidBody>,
    pub gravity: Vector3<f32>,
    pub time_step: f32,
}

impl PhysicsWorld {
    pub fn new() -> Self {
        Self {
            rigid_bodies: Vec::new(),
            gravity: Vector3::new(0.0, -9.81, 0.0),
            time_step: 1.0 / 60.0, // 60 FPS
        }
    }

    pub fn add_body(&mut self, body: RigidBody) {
        self.rigid_bodies.push(body);
    }

    pub fn step(&mut self) {
        for body in &mut self.rigid_bodies {
            body.update(self.time_step);
        }
        
        // Simple collision detection and response could go here
        self.handle_collisions();
    }

    fn handle_collisions(&mut self) {
        // Placeholder for collision detection logic
        // This would involve checking for intersections between bodies
        // and applying appropriate responses
    }
}