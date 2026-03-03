// Game module for RTGC - Siberian Cities
use crate::physics::{PhysicsWorld, RigidBody, Shape};
use nalgebra::Vector3;
use std::time::Instant;

pub struct Game {
    pub physics_world: PhysicsWorld,
    vehicle: usize,  // Index of the vehicle in the physics world
    last_update: Instant,
    accumulated_time: f32,
}

impl Game {
    pub fn new() -> Self {
        let mut physics_world = PhysicsWorld::new();
        
        // Create a terrain
        let height_map = vec![vec![0.0; 100]; 100]; // Simplified flat terrain
        let terrain = RigidBody::new_terrain(
            Vector3::new(0.0, 0.0, 0.0),
            height_map,
            Vector3::new(1000.0, 100.0, 1000.0)
        );
        physics_world.add_body(terrain);
        
        // Create a vehicle (using capsule for better rolling)
        let vehicle = RigidBody::new_capsule(
            Vector3::new(0.0, 5.0, 0.0),
            1000.0,  // Mass ~1 tonne for a vehicle
            1.5,     // Radius
            4.0      // Height
        );
        let vehicle_index = physics_world.rigid_bodies.len();
        physics_world.add_body(vehicle);
        
        // Add some obstacles
        for i in 0..10 {
            let obstacle = RigidBody::new_box(
                Vector3::new(
                    (i as f32) * 10.0 - 50.0, 
                    5.0, 
                    20.0
                ),
                100.0,  // Mass
                Vector3::new(2.0, 2.0, 2.0)  // Half extents
            );
            physics_world.add_body(obstacle);
        }
        
        Self {
            physics_world,
            vehicle: vehicle_index,
            last_update: Instant::now(),
            accumulated_time: 0.0,
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
            self.physics_world.step();
            self.accumulated_time -= fixed_dt;
        }
    }
    
    pub fn apply_vehicle_force(&mut self, force: Vector3<f32>) {
        if let Some(vehicle_body) = self.physics_world.rigid_bodies.get_mut(self.vehicle) {
            vehicle_body.apply_force(force);
        }
    }
    
    pub fn get_vehicle_position(&self) -> Vector3<f32> {
        if let Some(vehicle_body) = self.physics_world.rigid_bodies.get(self.vehicle) {
            vehicle_body.position
        } else {
            Vector3::new(0.0, 0.0, 0.0)
        }
    }
    
    pub fn get_vehicle_rotation(&self) -> nalgebra::UnitQuaternion<f32> {
        if let Some(vehicle_body) = self.physics_world.rigid_bodies.get(self.vehicle) {
            vehicle_body.rotation
        } else {
            nalgebra::UnitQuaternion::identity()
        }
    }
    
    pub fn get_vehicle_velocity(&self) -> Vector3<f32> {
        if let Some(vehicle_body) = self.physics_world.rigid_bodies.get(self.vehicle) {
            vehicle_body.velocity
        } else {
            Vector3::new(0.0, 0.0, 0.0)
        }
    }
    
    pub fn get_all_bodies(&self) -> &Vec<RigidBody> {
        &self.physics_world.rigid_bodies
    }
}