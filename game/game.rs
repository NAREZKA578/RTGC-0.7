use crate::physics::{PhysicsWorld, RigidBody, Shape};
use nalgebra::{Vector3, UnitQuaternion, Isometry3};
use std::time::Instant;

// Suspension system for the truck
pub struct Suspension {
    pub spring_stiffness: f32,
    pub damping_coefficient: f32,
    pub rest_length: f32,
    pub current_length: f32,
    pub max_compression: f32,
    pub max_extension: f32,
}

impl Suspension {
    pub fn new(spring_stiffness: f32, damping_coefficient: f32, rest_length: f32, max_compression: f32, max_extension: f32) -> Self {
        Self {
            spring_stiffness,
            damping_coefficient,
            rest_length,
            current_length: rest_length,
            max_compression,
            max_extension,
        }
    }

    pub fn update_suspension(&mut self, compression: f32) -> f32 {
        let displacement = self.rest_length - compression;
        
        // Calculate spring force
        let spring_force = self.spring_stiffness * displacement.max(-self.max_compression).min(self.max_extension);
        
        // Calculate damping force based on velocity of compression
        let damping_force = self.damping_coefficient * 0.0; // Placeholder - would need velocity information
        
        spring_force + damping_force
    }
}

// Wheel structure for the truck
pub struct Wheel {
    pub position: Vector3<f32>,
    pub radius: f32,
    pub suspension: Suspension,
    pub rotation_angle: f32,
    pub steering_angle: f32,
}

impl Wheel {
    pub fn new(position: Vector3<f32>, radius: f32, suspension: Suspension) -> Self {
        Self {
            position,
            radius,
            suspension,
            rotation_angle: 0.0,
            steering_angle: 0.0,
        }
    }

    pub fn update_rotation(&mut self, delta_time: f32, linear_velocity: f32) {
        // Update wheel rotation based on truck's linear velocity
        self.rotation_angle += linear_velocity * delta_time / self.radius;
    }

    pub fn apply_steering(&mut self, steering_input: f32) {
        // Limit steering angle to realistic values
        self.steering_angle = steering_input * 0.5; // Max 30 degrees in radians
    }
}

// Animation system for various game elements
pub struct Animation {
    pub start_time: Instant,
    pub duration: f32,
    pub active: bool,
    pub animation_type: AnimationType,
}

#[derive(Clone)]
pub enum AnimationType {
    CargoLoad,
    CargoUnload,
    SuspensionBounce,
    WheelRotation,
}

impl Animation {
    pub fn new(animation_type: AnimationType, duration: f32) -> Self {
        Self {
            start_time: Instant::now(),
            duration,
            active: true,
            animation_type,
        }
    }

    pub fn update(&mut self) -> bool {
        if !self.active {
            return false;
        }

        let elapsed = self.start_time.elapsed().as_secs_f32();
        if elapsed >= self.duration {
            self.active = false;
            return false;
        }
        true
    }

    pub fn progress(&self) -> f32 {
        let elapsed = self.start_time.elapsed().as_secs_f32();
        (elapsed / self.duration).min(1.0)
    }

    pub fn reset(&mut self) {
        self.start_time = Instant::now();
        self.active = true;
    }
}

// Cargo attachment system
pub struct CargoAttachment {
    pub attached: bool,
    pub cargo_body_id: Option<usize>,
    pub attachment_point: Vector3<f32>,
    pub max_distance: f32,
}

impl CargoAttachment {
    pub fn new(attachment_point: Vector3<f32>) -> Self {
        Self {
            attached: false,
            cargo_body_id: None,
            attachment_point,
            max_distance: 2.0,
        }
    }

    pub fn attach_cargo(&mut self, cargo_body_id: usize) -> bool {
        if !self.attached {
            self.attached = true;
            self.cargo_body_id = Some(cargo_body_id);
            true
        } else {
            false
        }
    }

    pub fn detach_cargo(&mut self) -> bool {
        if self.attached {
            self.attached = false;
            self.cargo_body_id = None;
            true
        } else {
            false
        }
    }
}

// Main game structure with enhanced features
pub struct Game {
    pub physics_world: PhysicsWorld,
    truck: usize,  // Index of the truck in the physics world
    wheels: Vec<Wheel>,
    cargo_attachment: CargoAttachment,
    animations: Vec<Animation>,
    last_update: Instant,
    accumulated_time: f32,
    // Vehicle specific properties
    throttle: f32,
    steering: f32,
    brake: f32,
    front_wheel_angle: f32,
    max_speed: f32,
    max_steering_angle: f32,
    score: u32,
}

impl Game {
    pub fn new() -> Self {
        let mut physics_world = PhysicsWorld::new();

        // Create a more complex terrain with hills and valleys
        let mut height_map = vec![vec![0.0; 200]; 200];
        for x in 0..200 {
            for z in 0..200 {
                // Create a hilly terrain with some obstacles
                let x_norm = (x as f32 - 100.0) / 100.0;
                let z_norm = (z as f32 - 100.0) / 100.0;

                // Multiple hills and valleys
                let hill1 = 5.0 * ((x_norm * 2.0).powi(2) + (z_norm * 2.0).powi(2)).sin();
                let hill2 = 3.0 * ((x_norm * 3.0 - 1.0).powi(2) + (z_norm * 3.0 - 1.0).powi(2)).cos();
                let valley = -2.0 * ((x_norm * 1.5).powi(2) + (z_norm * 1.5).powi(2)).cos();

                // Add some rivers/valleys
                let river = -3.0 * (z_norm - 0.5*x_norm).abs().min(0.3);

                height_map[x][z] = hill1 + hill2 + valley + river + 10.0; // Adding base height to ensure positive values
            }
        }

        let terrain = RigidBody::new_terrain(
            Vector3::new(-100.0, 0.0, -100.0), // Center the terrain around origin
            height_map,
            Vector3::new(2000.0, 100.0, 2000.0) // Larger terrain size
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

        // Create wheels for the truck
        let mut wheels = Vec::new();
        
        // Front left wheel
        let fl_suspension = Suspension::new(5000.0, 1000.0, 1.0, 0.3, 0.5);
        wheels.push(Wheel::new(Vector3::new(-1.0, 0.5, 2.0), 0.6, fl_suspension));
        
        // Front right wheel
        let fr_suspension = Suspension::new(5000.0, 1000.0, 1.0, 0.3, 0.5);
        wheels.push(Wheel::new(Vector3::new(1.0, 0.5, 2.0), 0.6, fr_suspension));
        
        // Rear left wheel
        let rl_suspension = Suspension::new(6000.0, 1200.0, 1.0, 0.3, 0.5);
        wheels.push(Wheel::new(Vector3::new(-1.0, 0.5, -2.0), 0.6, rl_suspension));
        
        // Rear right wheel
        let rr_suspension = Suspension::new(6000.0, 1200.0, 1.0, 0.3, 0.5);
        wheels.push(Wheel::new(Vector3::new(1.0, 0.5, -2.0), 0.6, rr_suspension));

        // Add some environmental objects like rocks and logs in a larger area
        for i in 0..100 {
            let angle = (i as f32) * 0.3;
            let distance = 10.0 + (i as f32) * 3.0;
            let height_offset = (angle.sin() * 2.0) + 2.0; // Vary height

            // Randomly place rocks around the starting area
            let rock_type = i % 4;
            let position = Vector3::new(
                distance * angle.cos(),
                height_offset,
                distance * angle.sin()
            );

            let rock = match rock_type {
                0 => RigidBody::new_sphere(position, 100.0, 1.0), // Small sphere
                1 => RigidBody::new_box(position, 150.0, Vector3::new(1.0, 0.5, 1.0)), // Flat box
                2 => RigidBody::new_capsule(position, 200.0, 0.8, 2.0), // Capsule/log
                _ => RigidBody::new_cone(position, 120.0, 0.7, 1.5), // Cone shaped object
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

        // Add a bridge
        let bridge = RigidBody::new_box(
            Vector3::new(0.0, 5.0, 30.0),
            1000.0,  // Heavy so it doesn't move
            Vector3::new(10.0, 0.5, 2.0)  // Long, narrow bridge
        );
        physics_world.add_body(bridge);

        // Add some trees (represented as cylinders or cones)
        for i in 0..50 {
            let angle = (i as f32) * 0.5;
            let distance = 30.0 + (i as f32) * 4.0;

            let tree_position = Vector3::new(
                distance * angle.cos(),
                2.0,
                distance * angle.sin()
            );

            // Trees as tall boxes or capsules
            let tree = if i % 3 == 0 {
                RigidBody::new_capsule(tree_position, 150.0, 0.5, 4.0) // Tall capsule
            } else {
                RigidBody::new_box(tree_position, 200.0, Vector3::new(0.5, 4.0, 0.5)) // Tall box
            };

            physics_world.add_body(tree);
        }

        // Add water areas (flat surfaces at lower height)
        let water_area = RigidBody::new_box(
            Vector3::new(50.0, -1.0, 50.0),
            10000.0, // Very heavy to stay still
            Vector3::new(30.0, 0.1, 30.0) // Large flat surface
        );
        physics_world.add_body(water_area);

        // Add cargo objects
        for i in 0..10 {
            let angle = (i as f32) * 0.628; // 2*pi/10
            let distance = 25.0;
            
            let cargo_position = Vector3::new(
                distance * angle.cos(),
                2.0,
                distance * angle.sin()
            );
            
            // Different types of cargo
            let cargo = match i % 3 {
                0 => RigidBody::new_box(cargo_position, 500.0, Vector3::new(1.5, 1.0, 1.5)), // Container
                1 => RigidBody::new_capsule(cargo_position, 400.0, 0.7, 3.0), // Log
                _ => RigidBody::new_sphere(cargo_position, 600.0, 1.2), // Boulder
            };
            
            physics_world.add_body(cargo);
        }

        // Add delivery points
        for i in 0..5 {
            let angle = (i as f32) * 1.256; // 2*pi/5
            let distance = 40.0;
            
            let delivery_position = Vector3::new(
                distance * angle.cos(),
                1.0,
                distance * angle.sin()
            );
            
            // Delivery zones (visual indicators that don't collide)
            let delivery_zone = RigidBody::new_box(delivery_position, 0.0, Vector3::new(3.0, 0.1, 3.0));
            physics_world.add_body(delivery_zone);
        }

        Self {
            physics_world,
            truck: truck_index,
            wheels,
            cargo_attachment: CargoAttachment::new(Vector3::new(0.0, 1.0, 0.0)),
            animations: Vec::new(),
            last_update: Instant::now(),
            accumulated_time: 0.0,
            throttle: 0.0,
            steering: 0.0,
            brake: 0.0,
            front_wheel_angle: 0.0,
            max_speed: 30.0,  // Max speed in m/s
            max_steering_angle: 0.5,  // Max steering angle in radians
            score: 0,
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
            self.update_truck_physics(delta_time);
            self.update_wheels(delta_time);
            self.update_animations();
            self.check_cargo_delivery();
            self.physics_world.step();
            self.accumulated_time -= fixed_dt;
        }
    }

    fn update_truck_physics(&mut self, delta_time: f32) {
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

    fn update_wheels(&mut self, delta_time: f32) {
        if let Some(truck_body) = self.physics_world.rigid_bodies.get(self.truck) {
            // Update wheel rotations based on truck's velocity
            for wheel in self.wheels.iter_mut() {
                wheel.update_rotation(delta_time, truck_body.velocity.magnitude());
                
                // Apply steering to front wheels
                if wheel.position.z > 0.0 { // Front wheels
                    wheel.apply_steering(self.steering);
                }
            }
        }
    }

    fn update_animations(&mut self) {
        self.animations.retain_mut(|anim| anim.update());
    }

    fn check_cargo_delivery(&mut self) {
        if let Some(truck_body) = self.physics_world.rigid_bodies.get(self.truck) {
            // Check if we're near a delivery point with cargo
            if self.cargo_attachment.attached {
                for (i, body) in self.physics_world.rigid_bodies.iter().enumerate() {
                    // Find delivery zones (very low mass boxes)
                    if body.shape == Shape::Box && body.mass < 1.0 && body.position.distance(&truck_body.position) < 5.0 {
                        // Deliver cargo
                        if self.cargo_attachment.detach_cargo() {
                            self.score += 100;
                            
                            // Add delivery animation
                            self.animations.push(Animation::new(AnimationType::CargoUnload, 1.0));
                        }
                        break;
                    }
                }
            }
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

    pub fn activate_cargo_action(&mut self) {
        if let Some(truck_body) = self.physics_world.rigid_bodies.get(self.truck) {
            // Look for nearby cargo to pick up or deliver
            if !self.cargo_attachment.attached {
                // Try to pick up cargo
                for (i, body) in self.physics_world.rigid_bodies.iter().enumerate() {
                    if body.mass > 100.0 && body.mass < 1000.0 && 
                       body.position.distance(&truck_body.position) < 5.0 {
                        // Attach this cargo
                        if self.cargo_attachment.attach_cargo(i) {
                            self.animations.push(Animation::new(AnimationType::CargoLoad, 1.0));
                            break;
                        }
                    }
                }
            }
        }
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

    pub fn get_wheels(&self) -> &Vec<Wheel> {
        &self.wheels
    }

    pub fn get_score(&self) -> u32 {
        self.score
    }

    pub fn get_cargo_attached(&self) -> bool {
        self.cargo_attachment.attached
    }
}