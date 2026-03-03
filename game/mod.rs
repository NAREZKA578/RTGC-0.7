// Game module for Off-Road Truck Simulator
use crate::physics::{PhysicsWorld, RigidBody, Shape};
use nalgebra::Vector3;
use std::time::Instant;

pub struct Game {
    pub physics_world: PhysicsWorld,
    truck: usize,  // Index of the truck in the physics world
    last_update: Instant,
    accumulated_time: f32,
    // Vehicle specific properties
    throttle: f32,
    steering: f32,
    brake: f32,
    front_wheel_angle: f32,
    max_speed: f32,
    max_steering_angle: f32,
}

impl Game {
    pub fn new() -> Self {
        let mut physics_world = PhysicsWorld::new();
        
        // Create a more complex terrain with hills and valleys
        let mut height_map = vec![vec![0.0; 100]; 100];
        for x in 0..100 {
            for z in 0..100 {
                // Create a hilly terrain with some obstacles
                let x_norm = (x as f32 - 50.0) / 50.0;
                let z_norm = (z as f32 - 50.0) / 50.0;
                
                // Multiple hills and valleys
                let hill1 = 5.0 * ((x_norm * 2.0).powi(2) + (z_norm * 2.0).powi(2)).sin();
                let hill2 = 3.0 * ((x_norm * 3.0 - 1.0).powi(2) + (z_norm * 3.0 - 1.0).powi(2)).cos();
                let valley = -2.0 * ((x_norm * 1.5).powi(2) + (z_norm * 1.5).powi(2)).cos();
                
                height_map[x][z] = hill1 + hill2 + valley + 10.0; // Adding base height to ensure positive values
            }
        }
        
        let terrain = RigidBody::new_terrain(
            Vector3::new(0.0, 0.0, 0.0),
            height_map,
            Vector3::new(1000.0, 100.0, 1000.0)
        );
        physics_world.add_body(terrain);
        
        // Create a truck (using box for more realistic shape)
        let truck = RigidBody::new_box(
            Vector3::new(0.0, 5.0, 0.0),
            3000.0,  // Heavier mass for a truck (~3 tonnes)
            Vector3::new(2.0, 1.0, 4.0)  // Dimensions: width, height, length
        );
        let truck_index = physics_world.rigid_bodies.len();
        physics_world.add_body(truck);
        
        // Add some environmental objects like rocks and logs
        for i in 0..20 {
            let angle = (i as f32) * 0.5;
            let distance = 15.0 + (i as f32) * 2.0;
            
            // Randomly place rocks around the starting area
            let rock_type = i % 3;
            let position = Vector3::new(
                distance * angle.cos(),
                2.0,
                distance * angle.sin()
            );
            
            let rock = match rock_type {
                0 => RigidBody::new_sphere(position, 100.0, 1.0), // Small sphere
                1 => RigidBody::new_box(position, 150.0, Vector3::new(1.0, 0.5, 1.0)), // Flat box
                _ => RigidBody::new_capsule(position, 200.0, 0.8, 2.0), // Capsule/log
            };
            physics_world.add_body(rock);
        }
        
        // Add some ramps and obstacles
        let ramp = RigidBody::new_box(
            Vector3::new(-20.0, 2.0, -10.0),
            500.0,  // Heavy so it doesn't move
            Vector3::new(5.0, 0.5, 3.0)  // Wide, thin ramp
        );
        physics_world.add_body(ramp);
        
        Self {
            physics_world,
            truck: truck_index,
            last_update: Instant::now(),
            accumulated_time: 0.0,
            throttle: 0.0,
            steering: 0.0,
            brake: 0.0,
            front_wheel_angle: 0.0,
            max_speed: 30.0,  // Max speed in m/s
            max_steering_angle: 0.5,  // Max steering angle in radians
        }
    }
    
    pub fn update(&mut self) {
        let delta_time = self.last_update.elapsed().as_secs_f32();
        self.last_update = Instant::now();
        
        // Accumulate time for physics updates
        self.accumulated_time += delta_time;
        
        // Fixed timestep for physics
        let fixed_dt = 1.0 / 60.0; // 60 FPS physics
        
        while self.accumulated_time >= fixed_dt {
            self.update_truck_physics();
            self.physics_world.step();
            self.accumulated_time -= fixed_dt;
        }
    }
    
    fn update_truck_physics(&mut self) {
        if let Some(truck_body) = self.physics_world.rigid_bodies.get_mut(self.truck) {
            // Get current velocity
            let current_speed = truck_body.velocity.magnitude();
            
            // Calculate forward direction
            let forward_dir = truck_body.rotation.transform_vector(&Vector3::new(0.0, 0.0, 1.0));
            let right_dir = truck_body.rotation.transform_vector(&Vector3::new(1.0, 0.0, 0.0));
            
            // Apply throttle (forward/reverse force)
            let throttle_force = forward_dir * self.throttle * 5000.0;
            
            // Apply steering (lateral force for turning)
            let steering_force = right_dir * self.steering * 2000.0;
            
            // Apply braking force
            let brake_force = -truck_body.velocity.normalize() * self.brake * 3000.0;
            
            // Apply forces to the truck
            truck_body.apply_force(throttle_force);
            truck_body.apply_force(steering_force);
            if self.brake > 0.0 {
                truck_body.apply_force(brake_force);
            }
            
            // Apply damping to simulate friction and air resistance
            let damping_factor = 0.95;
            truck_body.velocity = truck_body.velocity * damping_factor;
        }
    }
    
    // Vehicle control methods
    pub fn set_throttle(&mut self, value: f32) {
        self.throttle = value.clamp(-1.0, 1.0);  // Range from -1 (reverse) to 1 (forward)
    }
    
    pub fn set_steering(&mut self, value: f32) {
        self.steering = value.clamp(-1.0, 1.0);  // Range from -1 (left) to 1 (right)
    }
    
    pub fn set_brake(&mut self, value: f32) {
        self.brake = value.clamp(0.0, 1.0);  // Range from 0 (no brake) to 1 (full brake)
    }
    
    pub fn get_truck_position(&self) -> Vector3<f32> {
        if let Some(truck_body) = self.physics_world.rigid_bodies.get(self.truck) {
            truck_body.position
        } else {
            Vector3::new(0.0, 0.0, 0.0)
        }
    }
    
    pub fn get_truck_rotation(&self) -> nalgebra::UnitQuaternion<f32> {
        if let Some(truck_body) = self.physics_world.rigid_bodies.get(self.truck) {
            truck_body.rotation
        } else {
            nalgebra::UnitQuaternion::identity()
        }
    }
    
    pub fn get_truck_velocity(&self) -> Vector3<f32> {
        if let Some(truck_body) = self.physics_world.rigid_bodies.get(self.truck) {
            truck_body.velocity
        } else {
            Vector3::new(0.0, 0.0, 0.0)
        }
    }
    
    pub fn get_truck_speed_kmh(&self) -> f32 {
        self.get_truck_velocity().magnitude() * 3.6  // Convert m/s to km/h
    }
    
    pub fn get_all_bodies(&self) -> &Vec<RigidBody> {
        &self.physics_world.rigid_bodies
    }
    
    // Additional methods for rendering and game state
    pub fn reset_truck(&mut self) {
        if let Some(truck_body) = self.physics_world.rigid_bodies.get_mut(self.truck) {
            truck_body.position = Vector3::new(0.0, 5.0, 0.0);
            truck_body.velocity = Vector3::new(0.0, 0.0, 0.0);
            truck_body.rotation = nalgebra::UnitQuaternion::identity();
        }
        self.throttle = 0.0;
        self.steering = 0.0;
        self.brake = 0.0;
    }
    
    pub fn get_truck_forward_direction(&self) -> Vector3<f32> {
        if let Some(truck_body) = self.physics_world.rigid_bodies.get(self.truck) {
            truck_body.rotation.transform_vector(&Vector3::new(0.0, 0.0, 1.0))
        } else {
            Vector3::new(0.0, 0.0, 1.0)
        }
    }
}