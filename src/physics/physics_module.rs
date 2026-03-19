use nalgebra::{Vector3, Isometry3, Matrix3, UnitQuaternion, Point3};
use std::collections::HashMap;
use num_cpus;
mod spatial_hash;
mod thread_pool;
mod arena_allocator;
pub mod fracture_component;
pub mod deformable_terrain;
pub mod async_physics;
pub mod advanced_vehicle;
pub mod helicopter;
pub mod vehicle;
use spatial_hash::SpatialHash;
use thread_pool::ThreadPool;
use arena_allocator::ArenaAllocator;
pub use fracture_component::FractureComponent;
pub use deformable_terrain::{DeformableTerrainComponent, DeformableTerrainInterface, DeformationType};
pub use async_physics::AsyncPhysicsEngine;
pub use advanced_vehicle::{AdvancedVehicle, AdvancedWheel, AdvancedSuspension};
pub use helicopter::{Helicopter, HelicopterState, MainRotor, TailRotor, TurboshaftEngine, HelicopterControls};
pub use vehicle::{Vehicle, VehicleConfig, VehicleControls, WheelState};
use tracing;

#[derive(Debug, Clone)]
pub enum Shape {
    Sphere { radius: f32 },
    Box { half_extents: Vector3<f32> },
    Capsule { radius: f32, height: f32 },
    Terrain { height_map: Vec<Vec<f32>>, scale: Vector3<f32> },
    Mesh { vertices: Vec<Vector3<f32>>, indices: Vec<u32> },
}

#[derive(Debug, Clone)]
pub struct Ray {
    pub origin: Point3<f32>,
    pub direction: Vector3<f32>,
}

#[derive(Debug, Clone)]
pub struct RaycastHit {
    pub point: Point3<f32>,
    pub normal: Vector3<f32>,
    pub distance: f32,
    pub body_index: usize,
}

#[derive(Debug, Clone)]
pub struct RigidBody {
    pub position: Vector3<f32>,
    pub rotation: UnitQuaternion<f32>,
    pub velocity: Vector3<f32>,
    pub angular_velocity: Vector3<f32>,
    pub mass: f32,
    pub inverse_mass: f32,
    pub inertia_tensor: Matrix3<f32>,
    pub inverse_inertia_tensor: Matrix3<f32>,
    pub restitution: f32,
    pub friction: f32,
    pub linear_damping: f32,
    pub angular_damping: f32,
    pub shape: Shape,
    pub is_static: bool,
    pub bounds: Aabb,
    pub forces: Vector3<f32>,
    pub torques: Vector3<f32>,
    pub center_of_mass: Vector3<f32>,
    pub drag_coefficient: f32,
    pub lift_coefficient: f32,
    pub reference_area: f32,
}

impl RigidBody {
    pub fn new_sphere(position: Vector3<f32>, mass: f32, radius: f32) -> Self {
        let inverse_mass = if mass > 0.0 { 1.0 / mass } else { 0.0 };
        
        // Inertia tensor for a sphere: (2/5) * m * r^2
        let inertia_scalar = (2.0 / 5.0) * mass * radius * radius;
        let inertia_tensor = if mass > 0.0 {
            Matrix3::new(
                inertia_scalar, 0.0, 0.0,
                0.0, inertia_scalar, 0.0,
                0.0, 0.0, inertia_scalar
            )
        } else {
            Matrix3::zeros()
        };
        
        let inverse_inertia_tensor = if mass > 0.0 && inertia_scalar != 0.0 {
            inertia_tensor.try_inverse().unwrap_or(Matrix3::zeros())
        } else {
            Matrix3::zeros()
        };

        Self {
            position,
            rotation: UnitQuaternion::identity(),
            velocity: Vector3::new(0.0, 0.0, 0.0),
            angular_velocity: Vector3::new(0.0, 0.0, 0.0),
            mass,
            inverse_mass,
            inertia_tensor,
            inverse_inertia_tensor,
            restitution: 0.5,
            friction: 0.1,
            linear_damping: 0.99,
            angular_damping: 0.99,
            shape: Shape::Sphere { radius },
            is_static: mass <= 0.0,
            bounds: Aabb::from_shape_and_transform(&Shape::Sphere { radius }, &Isometry3::from_parts(position.into(), UnitQuaternion::identity())),
            forces: Vector3::zeros(),
            torques: Vector3::zeros(),
            center_of_mass: Vector3::zeros(),
            drag_coefficient: 0.47,
            lift_coefficient: 0.0,
            reference_area: std::f32::consts::PI * radius * radius,
        }
    }

    pub fn new_box(position: Vector3<f32>, mass: f32, half_extents: Vector3<f32>) -> Self {
        let inverse_mass = if mass > 0.0 { 1.0 / mass } else { 0.0 };
        
        // Inertia tensor for a box: 
        // Ixx = (1/12) * m * (h^2 + d^2)
        // Iyy = (1/12) * m * (w^2 + d^2)
        // Izz = (1/12) * m * (w^2 + h^2)
        let w = half_extents.x * 2.0;
        let h = half_extents.y * 2.0;
        let d = half_extents.z * 2.0;
        
        let inertia_tensor = if mass > 0.0 {
            Matrix3::new(
                (1.0 / 12.0) * mass * (h * h + d * d), 0.0, 0.0,
                0.0, (1.0 / 12.0) * mass * (w * w + d * d), 0.0,
                0.0, 0.0, (1.0 / 12.0) * mass * (w * w + h * h)
            )
        } else {
            Matrix3::zeros()
        };
        
        let inverse_inertia_tensor = if mass > 0.0 {
            inertia_tensor.try_inverse().unwrap_or(Matrix3::zeros())
        } else {
            Matrix3::zeros()
        };

        Self {
            position,
            rotation: UnitQuaternion::identity(),
            velocity: Vector3::new(0.0, 0.0, 0.0),
            angular_velocity: Vector3::new(0.0, 0.0, 0.0),
            mass,
            inverse_mass,
            inertia_tensor,
            inverse_inertia_tensor,
            restitution: 0.5,
            friction: 0.1,
            linear_damping: 0.99,
            angular_damping: 0.99,
            shape: Shape::Box { half_extents },
            is_static: mass <= 0.0,
            bounds: Aabb::from_shape_and_transform(&Shape::Box { half_extents }, &Isometry3::from_parts(position.into(), UnitQuaternion::identity())),
            forces: Vector3::zeros(),
            torques: Vector3::zeros(),
            center_of_mass: Vector3::zeros(),
            drag_coefficient: 1.05,
            lift_coefficient: 0.0,
            reference_area: 4.0 * half_extents.x * half_extents.y,
        }
    }

    pub fn new_capsule(position: Vector3<f32>, mass: f32, radius: f32, height: f32) -> Self {
        let inverse_mass = if mass > 0.0 { 1.0 / mass } else { 0.0 };
        
        // Approximate inertia tensor for a capsule
        // A capsule can be approximated as a cylinder with two hemispheres
        let cylinder_volume = std::f32::consts::PI * radius * radius * height;
        let hemisphere_volume = (2.0 / 3.0) * std::f32::consts::PI * radius * radius * radius;
        let total_volume = cylinder_volume + 2.0 * hemisphere_volume;
        let density = if total_volume > 0.0 { mass / total_volume } else { 0.0 };
        
        let cylinder_mass = density * cylinder_volume;
        let hemisphere_mass = density * hemisphere_volume;
        
        // Moment of inertia for cylinder around z-axis: (1/2) * m * r^2
        // Moment of inertia for cylinder around x,y-axis: (1/12) * m * h^2 + (1/4) * m * r^2
        // Moment of inertia for hemisphere around z-axis: (2/5) * m * r^2
        // Moment of inertia for hemisphere around x,y-axis: (2/5) * m * r^2 + (3/8)^2 * m * h^2 (parallel axis theorem)
        
        let cylinder_i_z = (1.0 / 2.0) * cylinder_mass * radius * radius;
        let cylinder_i_xy = (1.0 / 12.0) * cylinder_mass * height * height + (1.0 / 4.0) * cylinder_mass * radius * radius;
        
        let hemisphere_i_z = (2.0 / 5.0) * hemisphere_mass * radius * radius;
        let hemisphere_i_xy = (2.0 / 5.0) * hemisphere_mass * radius * radius;
        
        let total_i_z = cylinder_i_z + 2.0 * hemisphere_i_z;
        let total_i_xy = cylinder_i_xy + 2.0 * (hemisphere_i_xy + hemisphere_mass * (height / 2.0).powi(2));
        
        let inertia_tensor = if mass > 0.0 {
            Matrix3::new(
                total_i_xy, 0.0, 0.0,
                0.0, total_i_xy, 0.0,
                0.0, 0.0, total_i_z
            )
        } else {
            Matrix3::zeros()
        };
        
        let inverse_inertia_tensor = if mass > 0.0 {
            inertia_tensor.try_inverse().unwrap_or(Matrix3::zeros())
        } else {
            Matrix3::zeros()
        };

        Self {
            position,
            rotation: UnitQuaternion::identity(),
            velocity: Vector3::new(0.0, 0.0, 0.0),
            angular_velocity: Vector3::new(0.0, 0.0, 0.0),
            mass,
            inverse_mass,
            inertia_tensor,
            inverse_inertia_tensor,
            restitution: 0.5,
            friction: 0.1,
            linear_damping: 0.99,
            angular_damping: 0.99,
            shape: Shape::Capsule { radius, height },
            is_static: mass <= 0.0,
            bounds: Aabb::from_shape_and_transform(&Shape::Capsule { radius, height }, &Isometry3::from_parts(position.into(), UnitQuaternion::identity())),
            forces: Vector3::zeros(),
            torques: Vector3::zeros(),
            center_of_mass: Vector3::zeros(),
            drag_coefficient: 0.82,
            lift_coefficient: 0.0,
            reference_area: std::f32::consts::PI * radius * radius,
        }
    }

    pub fn new_terrain(position: Vector3<f32>, height_map: Vec<Vec<f32>>, scale: Vector3<f32>) -> Self {
        Self {
            position,
            rotation: UnitQuaternion::identity(),
            velocity: Vector3::new(0.0, 0.0, 0.0),
            angular_velocity: Vector3::new(0.0, 0.0, 0.0),
            mass: 0.0,
            inverse_mass: 0.0,
            inertia_tensor: Matrix3::zeros(),
            inverse_inertia_tensor: Matrix3::zeros(),
            restitution: 0.3,
            friction: 0.7,
            linear_damping: 1.0,
            angular_damping: 1.0,
            shape: Shape::Terrain { height_map, scale },
            is_static: true,
            bounds: Aabb::from_shape_and_transform(&Shape::Terrain { 
                height_map: vec![vec![0.0]], 
                scale 
            }, &Isometry3::from_parts(position.into(), UnitQuaternion::identity())),
            forces: Vector3::zeros(),
            torques: Vector3::zeros(),
            center_of_mass: Vector3::zeros(),
            drag_coefficient: 0.0,
            lift_coefficient: 0.0,
            reference_area: 0.0,
        }
    }

    pub fn new_mesh(position: Vector3<f32>, mass: f32, vertices: Vec<Vector3<f32>>, indices: Vec<u32>) -> Self {
        let inverse_mass = if mass > 0.0 { 1.0 / mass } else { 0.0 };
        
        // For mesh objects, we'll calculate a rough approximation of the inertia tensor
        // based on the bounding box of the mesh
        let mut min = Vector3::new(f32::MAX, f32::MAX, f32::MAX);
        let mut max = Vector3::new(f32::MIN, f32::MIN, f32::MIN);
        
        for vertex in &vertices {
            min.x = min.x.min(vertex.x);
            min.y = min.y.min(vertex.y);
            min.z = min.z.min(vertex.z);
            
            max.x = max.x.max(vertex.x);
            max.y = max.y.max(vertex.y);
            max.z = max.z.max(vertex.z);
        }
        
        let size = max - min;
        let half_extents = size / 2.0;
        
        // Calculate inertia tensor as if it were a box
        let w = half_extents.x * 2.0;
        let h = half_extents.y * 2.0;
        let d = half_extents.z * 2.0;
        
        let inertia_tensor = if mass > 0.0 {
            Matrix3::new(
                (1.0 / 12.0) * mass * (h * h + d * d), 0.0, 0.0,
                0.0, (1.0 / 12.0) * mass * (w * w + d * d), 0.0,
                0.0, 0.0, (1.0 / 12.0) * mass * (w * w + h * h)
            )
        } else {
            Matrix3::zeros()
        };
        
        let inverse_inertia_tensor = if mass > 0.0 {
            inertia_tensor.try_inverse().unwrap_or(Matrix3::zeros())
        } else {
            Matrix3::zeros()
        };

        Self {
            position,
            rotation: UnitQuaternion::identity(),
            velocity: Vector3::new(0.0, 0.0, 0.0),
            angular_velocity: Vector3::new(0.0, 0.0, 0.0),
            mass,
            inverse_mass,
            inertia_tensor,
            inverse_inertia_tensor,
            restitution: 0.4,
            friction: 0.6,
            linear_damping: 0.99,
            angular_damping: 0.99,
            shape: Shape::Mesh { vertices, indices },
            is_static: mass <= 0.0,
            bounds: Aabb::from_shape_and_transform(&Shape::Mesh { vertices: vec![], indices: vec![] }, &Isometry3::from_parts(position.into(), UnitQuaternion::identity())),
            forces: Vector3::zeros(),
            torques: Vector3::zeros(),
            center_of_mass: Vector3::zeros(),
            drag_coefficient: 1.2,
            lift_coefficient: 0.0,
            reference_area: size.x * size.y,
        }
    }

    pub fn apply_force(&mut self, force: Vector3<f32>) {
        if !self.is_static {
            self.forces += force;
        }
    }

    pub fn apply_force_at_point(&mut self, force: Vector3<f32>, point: Vector3<f32>) {
        if !self.is_static {
            self.forces += force;
            let r = point - self.position - self.center_of_mass;
            let torque = r.cross(&force);
            self.torques += torque;
        }
    }

    pub fn apply_impulse(&mut self, impulse: Vector3<f32>) {
        if !self.is_static {
            self.velocity += impulse * self.inverse_mass;
        }
    }

    pub fn apply_impulse_at_point(&mut self, impulse: Vector3<f32>, point: Vector3<f32>) {
        if !self.is_static {
            self.velocity += impulse * self.inverse_mass;
            let r = point - self.position - self.center_of_mass;
            let angular_impulse = r.cross(&impulse);
            self.angular_velocity += self.inverse_inertia_tensor * angular_impulse;
        }
    }

    pub fn apply_torque(&mut self, torque: Vector3<f32>) {
        if !self.is_static {
            self.torques += torque;
        }
    }

    pub fn apply_angular_impulse(&mut self, impulse: Vector3<f32>) {
        if !self.is_static {
            self.angular_velocity += self.inverse_inertia_tensor * impulse;
        }
    }

    pub fn set_center_of_mass(&mut self, com: Vector3<f32>) {
        self.center_of_mass = com;
    }

    pub fn get_center_of_mass_world(&self) -> Vector3<f32> {
        self.position + self.rotation.transform_vector(&self.center_of_mass)
    }

    pub fn clear_forces(&mut self) {
        self.forces = Vector3::zeros();
        self.torques = Vector3::zeros();
    }

    pub fn update(&mut self, dt: f32) {
        if !self.is_static {
            // Apply aerodynamic forces (drag and lift)
            let air_density = 1.225; // kg/m^3 at sea level
            let velocity_magnitude = self.velocity.magnitude();
            
            if velocity_magnitude > 0.001 {
                // Drag force: F_drag = -0.5 * rho * v^2 * Cd * A * direction
                let drag_magnitude = 0.5 * air_density * velocity_magnitude * velocity_magnitude 
                    * self.drag_coefficient * self.reference_area;
                let drag_direction = -self.velocity.normalize();
                let drag_force = drag_direction * drag_magnitude;
                self.forces += drag_force;
                
                // Lift force: F_lift = 0.5 * rho * v^2 * Cl * A * up_direction
                if self.lift_coefficient != 0.0 {
                    let lift_magnitude = 0.5 * air_density * velocity_magnitude * velocity_magnitude 
                        * self.lift_coefficient * self.reference_area;
                    let up_direction = self.rotation * Vector3::y();
                    let lift_force = up_direction * lift_magnitude;
                    self.forces += lift_force;
                }
            }
            
            // SYMPLECTIC EULER INTEGRATOR (more stable than explicit Euler)
            // Step 1: Update velocity using forces (semi-implicit)
            let linear_acceleration = self.forces * self.inverse_mass;
            self.velocity += linear_acceleration * dt;
            
            // Transform inertia tensor to world space for correct torque application
            let rotation_matrix = self.rotation.to_rotation_matrix();
            let world_inertia_tensor = rotation_matrix * self.inertia_tensor * rotation_matrix.transpose();
            let world_inverse_inertia_tensor = world_inertia_tensor.try_inverse().unwrap_or(self.inverse_inertia_tensor);
            
            // Step 2: Update angular velocity using torques
            let angular_acceleration = world_inverse_inertia_tensor * self.torques;
            self.angular_velocity += angular_acceleration * dt;
            
            // Step 3: Update position using NEW velocity (symplectic property)
            self.position += self.velocity * dt;
            
            // Step 4: Update orientation using NEW angular velocity
            let angular_velocity_quat = nalgebra::Quaternion::new(0.0, self.angular_velocity.x, self.angular_velocity.y, self.angular_velocity.z);
            let rotation_quat = nalgebra::Quaternion::from_parts(self.rotation.scalar(), self.rotation.vector().clone());
            let dq = 0.5 * angular_velocity_quat * rotation_quat;
            let new_rotation = UnitQuaternion::from_quaternion(nalgebra::Quaternion::new(
                rotation_quat.w + dq.w * dt,
                rotation_quat.i + dq.i * dt,
                rotation_quat.j + dq.j * dt,
                rotation_quat.k + dq.k * dt,
            ));
            self.rotation = new_rotation;
            self.rotation.renormalize();
            
            // Apply damping (after integration for stability)
            self.velocity *= self.linear_damping;
            self.angular_velocity *= self.angular_damping;
        }
        
        // Update bounding box
        self.bounds = Aabb::from_shape_and_transform(&self.shape, &self.get_world_transform());
    }
    
    pub fn get_world_transform(&self) -> Isometry3<f32> {
        Isometry3::from_parts(self.position.into(), self.rotation)
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Aabb {
    pub min: Vector3<f32>,
    pub max: Vector3<f32>,
}

impl Aabb {
    pub fn new(min: Vector3<f32>, max: Vector3<f32>) -> Self {
        Self { min, max }
    }

    pub fn from_shape_and_transform(shape: &Shape, transform: &Isometry3<f32>) -> Self {
        match shape {
            Shape::Sphere { radius } => {
                let center = transform.translation.vector;
                let half_size = Vector3::new(*radius, *radius, *radius);
                
                Aabb {
                    min: center - half_size,
                    max: center + half_size,
                }
            }
            Shape::Box { half_extents } => {
                let center = transform.translation.vector;
                
                Aabb {
                    min: center - half_extents,
                    max: center + half_extents,
                }
            }
            Shape::Capsule { radius, height } => {
                let center = transform.translation.vector;
                let half_height = height / 2.0;
                let half_size = Vector3::new(radius.max(half_height), radius.max(half_height), radius.max(half_height));
                
                Aabb {
                    min: center - half_size,
                    max: center + half_size,
                }
            }
            Shape::Terrain { height_map: _, scale } => {
                // For terrain, we'll create a large AABB that encompasses the entire terrain
                let center = transform.translation.vector;
                let half_size = Vector3::new(scale.x / 2.0, scale.y / 2.0, scale.z / 2.0);
                
                Aabb {
                    min: center - half_size,
                    max: center + half_size,
                }
            }
            Shape::Mesh { vertices, .. } => {
                let mut min = Vector3::new(f32::MAX, f32::MAX, f32::MAX);
                let mut max = Vector3::new(f32::MIN, f32::MIN, f32::MIN);
                
                for vertex in vertices {
                    let transformed_vertex = transform.transform_point(&Point3::from(*vertex));
                    let v = transformed_vertex.coords;
                    
                    min.x = min.x.min(v.x);
                    min.y = min.y.min(v.y);
                    min.z = min.z.min(v.z);
                    
                    max.x = max.x.max(v.x);
                    max.y = max.y.max(v.y);
                    max.z = max.z.max(v.z);
                }
                
                Aabb { min, max }
            }
        }
    }

    pub fn intersects(&self, other: &Aabb) -> bool {
        self.min.x <= other.max.x && self.max.x >= other.min.x &&
        self.min.y <= other.max.y && self.max.y >= other.min.y &&
        self.min.z <= other.max.z && self.max.z >= other.min.z
    }

    pub fn contains_point(&self, point: &Vector3<f32>) -> bool {
        point.x >= self.min.x && point.x <= self.max.x &&
        point.y >= self.min.y && point.y <= self.max.y &&
        point.z >= self.min.z && point.z <= self.max.z
    }

    pub fn center(&self) -> Vector3<f32> {
        (self.min + self.max) / 2.0
    }

    pub fn size(&self) -> Vector3<f32> {
        self.max - self.min
    }
}

pub struct PhysicsWorld {
    pub rigid_bodies: ArenaAllocator<RigidBody>,
    pub gravity: Vector3<f32>,
    pub time_step: f32,
    broadphase_pairs: Vec<(usize, usize)>,
    pub sub_steps: u32,
    spatial_hash: SpatialHash,
    thread_pool: ThreadPool,
    sleeping_threshold: f32,        // Velocity threshold for sleep activation
    deactivation_time: f32,         // Time before body goes to sleep
}

impl PhysicsWorld {
    pub fn new() -> Self {
        Self {
            rigid_bodies: ArenaAllocator::new(),
            gravity: Vector3::new(0.0, -9.81, 0.0),
            time_step: 1.0 / 60.0, // 60 FPS fixed timestep
            broadphase_pairs: Vec::new(),
            sub_steps: 4, // 4 substeps for stable collision resolution
            spatial_hash: SpatialHash::new(10.0), // Cell size of 10 units
            thread_pool: ThreadPool::new(num_cpus::get()).expect("Failed to create thread pool"),
            sleeping_threshold: 0.01, // Bodies with velocity < 0.01 m/s can sleep
            deactivation_time: 1.0,   // Sleep after 1 second of inactivity
        }
    }

    /// Create physics world with custom fixed timestep
    pub fn with_timestep(time_step: f32) -> Self {
        let mut world = Self::new();
        world.time_step = time_step;
        world
    }

    /// Enable/disable body sleeping for performance optimization
    pub fn set_sleeping_enabled(&mut self, enabled: bool) {
        if enabled {
            self.sleeping_threshold = 0.01;
            self.deactivation_time = 1.0;
        } else {
            self.sleeping_threshold = f32::MAX; // Never sleep
            self.deactivation_time = f32::MAX;
        }
    }

    pub fn add_body(&mut self, body: RigidBody) -> usize {
        self.rigid_bodies.allocate(body)
    }

    pub fn remove_body(&mut self, index: usize) -> Option<RigidBody> {
        if self.rigid_bodies.is_allocated(index) {
            let body = self.rigid_bodies.get(index).cloned()?;
            self.rigid_bodies.deallocate(index);
            Some(body)
        } else {
            None
        }
    }

    pub fn get_body(&self, index: usize) -> Option<&RigidBody> {
        self.rigid_bodies.get(index)
    }

    pub fn get_body_mut(&mut self, index: usize) -> Option<&mut RigidBody> {
        self.rigid_bodies.get_mut(index)
    }

    /// Main physics step with fixed timestep for deterministic simulation
    /// Uses Symplectic Euler integration for energy conservation
    pub fn step(&mut self, delta_time: f32) {
        tracing::trace!("Starting physics step with delta_time: {}", delta_time);
        let sub_dt = self.time_step / self.sub_steps as f32;
        
        for i in 0..self.sub_steps {
            tracing::trace!("Starting sub-step {}", i);
            
            // Step 1: Force integration - FIXED: sequential to avoid data races
            for body in self.rigid_bodies.iter_mut() {
                if !body.is_static {
                    // Apply gravity
                    body.apply_force(self.gravity * body.mass);
                }
            }
            
            // Step 2: Update positions and orientations using Symplectic Euler - FIXED: sequential
            for body in self.rigid_bodies.iter_mut() {
                if !body.is_static {
                    body.update(sub_dt);
                    body.clear_forces(); // Clear forces after integration
                }
            }
            
            // Step 3: Broad phase collision detection
            self.broadphase_collision_detection();
            
            // Step 4: Narrow phase and collision resolution with Sequential Impulse Solver
            self.handle_collisions_parallel();
            
            // Step 5: Solve constraints (contacts, joints) in parallel
            self.solve_constraints_parallel();
            
            // Step 6: Check for sleeping bodies (performance optimization)
            self.update_sleeping_bodies(sub_dt);
            
            tracing::trace!("Completed sub-step {}", i);
        }
        tracing::trace!("Completed physics step");
    }

    /// Update sleeping state of bodies for performance optimization
    fn update_sleeping_bodies(&mut self, dt: f32) {
        let threshold = self.sleeping_threshold;
        let deactivation_time = self.deactivation_time;
        
        // FIXED: Sequential version to avoid data races with raw pointers
        // TODO: Implement proper parallel sleeping body updates with thread-safe state
        for body in self.rigid_bodies.iter_mut() {
            if !body.is_static {
                let velocity_magnitude = body.velocity.magnitude();
                let angular_magnitude = body.angular_velocity.magnitude();
                
                if velocity_magnitude < threshold && angular_magnitude < threshold {
                    // Body is candidate for sleeping - increment idle timer
                    // In production: body.idle_timer += dt;
                    // if body.idle_timer > deactivation_time { body.sleep(); }
                } else {
                    // Reset idle timer if moving
                    // body.idle_timer = 0.0;
                }
            }
        }
    }

    fn broadphase_collision_detection(&mut self) {
        self.broadphase_pairs.clear();
        
        // Clear and rebuild spatial hash
        self.spatial_hash.clear();
        
        // Insert all bodies into the spatial hash
        for (index, body) in self.rigid_bodies.iter().enumerate() {
            if !body.is_static {
                self.spatial_hash.insert(index, &body.position);
            }
        }
        
        // Find potential collisions using spatial hash - FIXED: sequential to avoid borrow issues
        let mut pairs = Vec::new();
        
        tracing::trace!("Starting broadphase collision detection");
        let static_bodies: Vec<_> = self.rigid_bodies.iter()
            .enumerate()
            .filter(|(_, body)| !body.is_static)
            .collect();
        
        for (i, body_a) in &static_bodies {
            let candidates = self.spatial_hash.get_potential_collisions(&body_a.position);
            
            for j in candidates {
                // Ensure we don't test against the same object or duplicate pairs
                if i >= &j {
                    continue;
                }
                
                let body_b = &self.rigid_bodies[j];
                
                // Check if both are static (can happen if body_a was static but body_b was dynamic)
                if body_a.is_static && body_b.is_static {
                    continue;
                }
                
                // Double-check AABB intersection to reduce false positives
                if body_a.bounds.intersects(&body_b.bounds) {
                    pairs.push((*i, j));
                }
            }
        }
        
        self.broadphase_pairs = pairs;
        tracing::trace!("Broadphase collision detection found {} pairs", self.broadphase_pairs.len());
    }

    fn handle_collisions(&mut self) {
        let mut contacts = Vec::new();
        
        // Narrow phase collision detection using broadphase pairs
        for (i, j) in &self.broadphase_pairs {
            if let Some(contact) = self.detect_collision(*i, *j) {
                contacts.push(contact);
            }
        }
        
        // Sort contacts by depth to resolve deepest penetrations first
        contacts.sort_by(|a, b| {
            b.penetration_depth
                .partial_cmp(&a.penetration_depth)
                .unwrap_or(std::cmp::Ordering::Equal)
        });
        
        // Process contacts
        for contact in &contacts {
            self.resolve_contact(contact);
        }
    }

    fn handle_collisions_parallel(&mut self) {
        let mut contacts = Vec::new();
        
        // Narrow phase collision detection using broadphase pairs
        for (i, j) in &self.broadphase_pairs {
            if let Some(contact) = self.detect_collision(*i, *j) {
                contacts.push(contact);
            }
        }
        
        // Sort contacts by depth to resolve deepest penetrations first
        contacts.sort_by(|a, b| {
            b.penetration_depth
                .partial_cmp(&a.penetration_depth)
                .unwrap_or(std::cmp::Ordering::Equal)
        });
        
        // FIXED: Process contacts sequentially to avoid data races
        // TODO: Implement proper parallel contact resolution with disjoint sets
        tracing::trace!("Processing {} contacts sequentially (safe mode)", contacts.len());
        let bodies_slice = self.rigid_bodies.as_mut_slice();
        for contact in &contacts {
            Self::resolve_contact_sequential(contact, bodies_slice);
        }
        tracing::trace!("Completed processing contacts");
    }

    fn resolve_contact_sequential(contact: &Contact, bodies: &mut [Option<RigidBody>]) {
        // Get mutable references to both bodies involved in the contact
        if let (Some(ref mut body_a), Some(ref mut body_b)) = (
            bodies.get_mut(contact.body_a).and_then(|b| b.as_mut()),
            bodies.get_mut(contact.body_b).and_then(|b| b.as_mut())
        ) {
            // Calculate relative velocity at contact point
            let r_a = contact.contact_point - body_a.position;
            let r_b = contact.contact_point - body_b.position;
            
            let vel_a = body_a.velocity + body_a.angular_velocity.cross(&r_a);
            let vel_b = body_b.velocity + body_b.angular_velocity.cross(&r_b);
            let relative_vel = vel_a - vel_b;
            
            // Calculate impulse along normal
            let normal = contact.normal;
            let vel_along_normal = relative_vel.dot(&normal);
            
            // Don't resolve if velocities are separating
            if vel_along_normal > 0.0 {
                return;
            }
            
            // Calculate restitution
            let e = contact.restitution.min(1.0);
            
            // Calculate mass properties
            let inv_mass_a = if body_a.is_static { 0.0 } else { body_a.inverse_mass };
            let inv_mass_b = if body_b.is_static { 0.0 } else { body_b.inverse_mass };
            
            // Calculate moment arm cross products
            let r_a_cross_n = r_a.cross(&normal);
            let r_b_cross_n = r_b.cross(&normal);
            
            // Calculate inverse moments of inertia
            let inv_I_a = if body_a.is_static { Matrix3::zeros() } else { body_a.inverse_inertia_tensor };
            let inv_I_b = if body_b.is_static { Matrix3::zeros() } else { body_b.inverse_inertia_tensor };
            
            // Calculate impulse magnitude
            let term1 = inv_mass_a + inv_mass_b;
            let term2 = (inv_I_a * r_a_cross_n).cross(&r_a).dot(&normal);
            let term3 = (inv_I_b * r_b_cross_n).cross(&r_b).dot(&normal);
            let denominator = term1 + term2 + term3;
            
            if denominator == 0.0 {
                return;
            }
            
            let j = -(1.0 + e) * vel_along_normal / denominator;
            let impulse = j * normal;
            
            // Apply positional correction (Baumgarte stabilization)
            let percent = 0.2; // 20% correction
            let slop = 0.01; // Allow 1cm of penetration
            let correction = (contact.penetration_depth - slop).max(0.0) / (inv_mass_a + inv_mass_b) * percent * normal;
            
            // Apply impulses and corrections
            if !body_a.is_static {
                body_a.apply_impulse(impulse);
                body_a.position += correction * inv_mass_a;
            }
            
            if !body_b.is_static {
                body_b.apply_impulse(-impulse);
                body_b.position -= correction * inv_mass_b;
            }
            
            // Apply friction
            let tangent = relative_vel - vel_along_normal * normal;
            let tangent_length = tangent.magnitude();
            
            if tangent_length > 0.001 { // Don't apply friction if tangential velocity is very small
                let tangent = tangent.normalize();
                
                // Calculate tangent impulse magnitude
                let vel_along_tangent = relative_vel.dot(&tangent);
                let jt = -vel_along_tangent / denominator;
                
                // Clamp friction impulse
                let friction_coefficient = (body_a.friction + body_b.friction) / 2.0;
                if jt.abs() < j * friction_coefficient {
                    let friction_impulse = jt * tangent;
                    
                    if !body_a.is_static {
                        body_a.apply_impulse(friction_impulse);
                    }
                    
                    if !body_b.is_static {
                        body_b.apply_impulse(-friction_impulse);
                    }
                } else {
                    // Coulomb friction: friction force can't exceed normal force
                    let max_friction_impulse = friction_coefficient * j;
                    let friction_impulse = max_friction_impulse * tangent;
                    
                    if !body_a.is_static {
                        body_a.apply_impulse(friction_impulse);
                    }
                    
                    if !body_b.is_static {
                        body_b.apply_impulse(-friction_impulse);
                    }
                }
            }
        }
    }
    
    fn detect_collision(&self, i: usize, j: usize) -> Option<Contact> {
        let body_a = &self.rigid_bodies[i];
        let body_b = &self.rigid_bodies[j];
        
        // Skip if both are static
        if body_a.is_static && body_b.is_static {
            return None;
        }
        
        // Use different collision detection methods based on shape types
        match (&body_a.shape, &body_b.shape) {
            (Shape::Sphere { radius: rad_a }, Shape::Sphere { radius: rad_b }) => {
                let diff = body_b.position - body_a.position;
                let distance_sq = diff.magnitude_squared();
                let radius_sum = rad_a + rad_b;
                
                if distance_sq < radius_sum * radius_sum {
                    let distance = distance_sq.sqrt();
                    let normal = if distance > 0.0 { diff.normalize() } else { Vector3::y_axis() };
                    let penetration_depth = radius_sum - distance;
                    
                    Some(Contact {
                        body_a: i,
                        body_b: j,
                        contact_point: body_a.position + normal * *rad_a,
                        normal: normal,
                        penetration_depth: penetration_depth,
                        restitution: (body_a.restitution + body_b.restitution) / 2.0,
                        friction: (body_a.friction + body_b.friction) / 2.0,
                    })
                } else {
                    None
                }
            }
            (Shape::Box { half_extents: extents_a }, Shape::Box { half_extents: extents_b }) => {
                // SAT (Separating Axis Theorem) implementation for boxes
                self.detect_box_box_collision(i, j, body_a, body_b, extents_a, extents_b)
            }
            (Shape::Sphere { radius }, Shape::Box { half_extents }) => {
                // Sphere-box collision
                self.detect_sphere_box_collision(i, j, body_a, body_b, *radius, half_extents)
            }
            (Shape::Box { half_extents }, Shape::Sphere { radius }) => {
                // Sphere-box collision (reversed)
                if let Some(mut contact) = self.detect_sphere_box_collision(j, i, body_b, body_a, *radius, half_extents) {
                    // Swap body indices and invert normal
                    std::mem::swap(&mut contact.body_a, &mut contact.body_b);
                    contact.normal = -contact.normal;
                    Some(contact)
                } else {
                    None
                }
            }
            (Shape::Capsule { radius, height }, Shape::Sphere { .. }) => {
                // Capsule-sphere collision
                self.detect_capsule_sphere_collision(i, j, body_a, body_b, *radius, *height)
            }
            (Shape::Sphere { .. }, Shape::Capsule { radius, height }) => {
                // Capsule-sphere collision (reversed)
                if let Some(mut contact) = self.detect_capsule_sphere_collision(j, i, body_b, body_a, *radius, *height) {
                    // Swap body indices and invert normal
                    std::mem::swap(&mut contact.body_a, &mut contact.body_b);
                    contact.normal = -contact.normal;
                    Some(contact)
                } else {
                    None
                }
            }
            (Shape::Capsule { radius: r1, height: h1 }, Shape::Box { .. }) => {
                // Capsule-box collision
                self.detect_capsule_box_collision(i, j, body_a, body_b, *r1, *h1)
            }
            (Shape::Box { .. }, Shape::Capsule { radius: r2, height: h2 }) => {
                // Capsule-box collision (reversed)
                if let Some(mut contact) = self.detect_capsule_box_collision(j, i, body_b, body_a, *r2, *h2) {
                    // Swap body indices and invert normal
                    std::mem::swap(&mut contact.body_a, &mut contact.body_b);
                    contact.normal = -contact.normal;
                    Some(contact)
                } else {
                    None
                }
            }
            (Shape::Capsule { radius: r1, height: h1 }, Shape::Capsule { radius: r2, height: h2 }) => {
                // Capsule-capsule collision
                self.detect_capsule_capsule_collision(i, j, body_a, body_b, *r1, *h1, *r2, *h2)
            }
            _ => {
                // For now, only handle specific shape combinations
                // Complex shapes like meshes and terrains need special handling
                None
            }
        }
    }

    fn detect_box_box_collision(&self, i: usize, j: usize, body_a: &RigidBody, body_b: &RigidBody, extents_a: &Vector3<f32>, extents_b: &Vector3<f32>) -> Option<Contact> {
        // Simplified box-box collision using separating axis theorem
        // This is a basic implementation - a full SAT implementation would be more accurate
        
        let transform_a = body_a.get_world_transform();
        let transform_b = body_b.get_world_transform();
        
        // Transform positions to world space
        let pos_a = transform_a.translation.vector;
        let pos_b = transform_b.translation.vector;
        
        // Calculate the vector between the centers
        let center_diff = pos_b - pos_a;
        
        // Calculate axes to test (in world space)
        let axes = [
            // Axes from body A's orientation
            transform_a.rotation.transform_vector(&Vector3::x_axis()),
            transform_a.rotation.transform_vector(&Vector3::y_axis()),
            transform_a.rotation.transform_vector(&Vector3::z_axis()),
            // Axes from body B's orientation
            transform_b.rotation.transform_vector(&Vector3::x_axis()),
            transform_b.rotation.transform_vector(&Vector3::y_axis()),
            transform_b.rotation.transform_vector(&Vector3::z_axis()),
        ];
        
        let mut min_overlap = f32::MAX;
        let mut collision_normal = Vector3::y_axis();
        
        // Project both boxes onto each axis and check for overlap
        for axis in &axes {
            // Project box A onto axis
            let proj_a = extents_a.x * (transform_a.rotation.transform_vector(&Vector3::x_axis()).dot(axis)).abs() +
                        extents_a.y * (transform_a.rotation.transform_vector(&Vector3::y_axis()).dot(axis)).abs() +
                        extents_a.z * (transform_a.rotation.transform_vector(&Vector3::z_axis()).dot(axis)).abs();
            
            // Project box B onto axis
            let proj_b = extents_b.x * (transform_b.rotation.transform_vector(&Vector3::x_axis()).dot(axis)).abs() +
                        extents_b.y * (transform_b.rotation.transform_vector(&Vector3::y_axis()).dot(axis)).abs() +
                        extents_b.z * (transform_b.rotation.transform_vector(&Vector3::z_axis()).dot(axis)).abs();
            
            // Distance between projections
            let dist = center_diff.dot(axis).abs();
            
            // Check for separation
            if dist > proj_a + proj_b {
                return None; // No collision along this axis
            }
            
            // Calculate overlap
            let overlap = proj_a + proj_b - dist;
            if overlap < min_overlap {
                min_overlap = overlap;
                collision_normal = *axis;
            }
        }
        
        // Calculate contact point and other properties
        let contact_point = (pos_a + pos_b) / 2.0;
        let normal = collision_normal.normalize();
        
        Some(Contact {
            body_a: i,
            body_b: j,
            contact_point,
            normal,
            penetration_depth: min_overlap,
            restitution: (body_a.restitution + body_b.restitution) / 2.0,
            friction: (body_a.friction + body_b.friction) / 2.0,
        })
    }

    fn detect_sphere_box_collision(&self, sphere_idx: usize, box_idx: usize, sphere_body: &RigidBody, box_body: &RigidBody, sphere_radius: f32, box_half_extents: &Vector3<f32>) -> Option<Contact> {
        // Transform sphere center to box's local space
        let box_transform = box_body.get_world_transform();
        let inv_box_transform = box_transform.inverse();
        let sphere_center_local = inv_box_transform.transform_point(&Point3::from(sphere_body.position));
        
        // Find closest point on box to sphere center
        let closest_point = Vector3::new(
            sphere_center_local.x.clamp(-box_half_extents.x, box_half_extents.x),
            sphere_center_local.y.clamp(-box_half_extents.y, box_half_extents.y),
            sphere_center_local.z.clamp(-box_half_extents.z, box_half_extents.z),
        );
        
        // Transform closest point back to world space
        let closest_point_world = box_transform.transform_point(&Point3::from(closest_point)).coords;
        
        // Calculate distance between sphere center and closest point
        let diff = closest_point_world - sphere_body.position;
        let distance = diff.magnitude();
        
        if distance <= sphere_radius {
            let normal = if distance > 0.0 {
                diff.normalize()
            } else {
                Vector3::y_axis() // Default normal if sphere center is inside box
            };
            
            let penetration_depth = sphere_radius - distance;
            let contact_point = sphere_body.position + normal * sphere_radius;
            
            Some(Contact {
                body_a: sphere_idx,
                body_b: box_idx,
                contact_point,
                normal,
                penetration_depth,
                restitution: (sphere_body.restitution + box_body.restitution) / 2.0,
                friction: (sphere_body.friction + box_body.friction) / 2.0,
            })
        } else {
            None
        }
    }

    fn detect_capsule_sphere_collision(&self, capsule_idx: usize, sphere_idx: usize, capsule_body: &RigidBody, sphere_body: &RigidBody, capsule_radius: f32, capsule_height: f32) -> Option<Contact> {
        // Find the closest point on the capsule's line segment to the sphere center
        let capsule_transform = capsule_body.get_world_transform();
        let capsule_direction = capsule_transform.rotation.transform_vector(&Vector3::y_axis());
        let half_height = capsule_height / 2.0;
        
        // Capsule endpoints in world space
        let cap_start = capsule_body.position - capsule_direction * half_height;
        let cap_end = capsule_body.position + capsule_direction * half_height;
        
        // Vector from capsule start to sphere center
        let sphere_to_start = sphere_body.position - cap_start;
        let cap_segment = cap_end - cap_start;
        
        // Project sphere_to_start onto cap_segment to find the closest point
        let segment_length_sq = cap_segment.magnitude_squared();
        let t = if segment_length_sq > 0.0 {
            sphere_to_start.dot(&cap_segment) / segment_length_sq
        } else {
            0.0
        };
        
        // Clamp t to [0, 1] to stay within the capsule line segment
        let clamped_t = t.clamp(0.0, 1.0);
        let closest_point_on_segment = cap_start + cap_segment * clamped_t;
        
        // Calculate distance from sphere center to closest point on capsule segment
        let distance_vec = sphere_body.position - closest_point_on_segment;
        let distance = distance_vec.magnitude();
        
        // Total radius is the sum of capsule radius and sphere radius
        let total_radius = capsule_radius + sphere_body.shape.as_sphere_radius().unwrap_or(0.0);
        
        if distance <= total_radius {
            let normal = if distance > 0.0 {
                distance_vec.normalize()
            } else {
                // If the sphere center is exactly on the capsule segment,
                // use a perpendicular direction
                let perp_dir = if capsule_direction.x.abs() < 0.9 {
                    capsule_direction.cross(&Vector3::x_axis()).normalize()
                } else {
                    capsule_direction.cross(&Vector3::z_axis()).normalize()
                };
                perp_dir
            };
            
            let penetration_depth = total_radius - distance;
            let contact_point = closest_point_on_segment + normal * capsule_radius;
            
            Some(Contact {
                body_a: capsule_idx,
                body_b: sphere_idx,
                contact_point,
                normal,
                penetration_depth,
                restitution: (capsule_body.restitution + sphere_body.restitution) / 2.0,
                friction: (capsule_body.friction + sphere_body.friction) / 2.0,
            })
        } else {
            None
        }
    }

    fn detect_capsule_box_collision(&self, capsule_idx: usize, box_idx: usize, capsule_body: &RigidBody, box_body: &RigidBody, capsule_radius: f32, capsule_height: f32) -> Option<Contact> {
        // This is a complex collision detection problem that requires finding
        // the closest points between a capsule and a box
        // For simplicity, we'll use a simplified approach:
        // 1. Find the closest point on the capsule line segment to the box
        // 2. Then check if a sphere at that point with capsule radius collides with the box
        
        let capsule_transform = capsule_body.get_world_transform();
        let capsule_direction = capsule_transform.rotation.transform_vector(&Vector3::y_axis());
        let half_height = capsule_height / 2.0;
        
        // Capsule endpoints in world space
        let cap_start = capsule_body.position - capsule_direction * half_height;
        let cap_end = capsule_body.position + capsule_direction * half_height;
        
        // Transform box to capsule's local coordinate system for easier computation
        let box_transform = box_body.get_world_transform();
        let inv_capsule_transform = capsule_transform.inverse();
        let box_in_capsule_space = inv_capsule_transform * box_transform;
        
        // Find the closest point on the capsule segment to the box
        // This is a simplified approach - a full solution would involve more complex geometry
        let closest_point_on_segment = {
            // We'll sample points along the capsule segment and find the one closest to the box
            let num_samples = 10;
            let mut min_dist = f32::MAX;
            let mut closest_sample = cap_start;
            
            for i in 0..=num_samples {
                let t = i as f32 / num_samples as f32;
                let sample_point = cap_start.lerp(&cap_end, t);
                
                // Transform sample point to box's local space
                let sample_in_box_space = box_transform.inverse_transform_point(&Point3::from(sample_point));
                
                // Find closest point on box in its local space
                let closest_on_box = Vector3::new(
                    sample_in_box_space.x.clamp(-box_body.shape.as_box_half_extents().unwrap_or(Vector3::zeros()).x,
                                               box_body.shape.as_box_half_extents().unwrap_or(Vector3::zeros()).x),
                    sample_in_box_space.y.clamp(-box_body.shape.as_box_half_extents().unwrap_or(Vector3::zeros()).y,
                                               box_body.shape.as_box_half_extents().unwrap_or(Vector3::zeros()).y),
                    sample_in_box_space.z.clamp(-box_body.shape.as_box_half_extents().unwrap_or(Vector3::zeros()).z,
                                               box_body.shape.as_box_half_extents().unwrap_or(Vector3::zeros()).z),
                );
                
                // Transform closest point back to world space
                let closest_world = box_transform.transform_point(&Point3::from(closest_on_box)).coords;
                
                // Calculate distance
                let dist = (sample_point - closest_world).magnitude();
                
                if dist < min_dist {
                    min_dist = dist;
                    closest_sample = sample_point;
                }
            }
            
            closest_sample
        };
        
        // Now check if a sphere at the closest point with capsule radius collides with the box
        let temp_sphere_body = RigidBody::new_sphere(closest_point_on_segment, 0.0, capsule_radius);
        if let Some(contact) = self.detect_sphere_box_collision(capsule_idx, box_idx, &temp_sphere_body, box_body, capsule_radius, 
                                                               &box_body.shape.as_box_half_extents().unwrap_or(Vector3::new(1.0, 1.0, 1.0))) {
            Some(contact)
        } else {
            None
        }
    }

    fn detect_capsule_capsule_collision(&self, i: usize, j: usize, body_a: &RigidBody, body_b: &RigidBody, 
                                       radius_a: f32, height_a: f32, radius_b: f32, height_b: f32) -> Option<Contact> {
        // Find closest points between the two capsule line segments
        let transform_a = body_a.get_world_transform();
        let transform_b = body_b.get_world_transform();
        
        let dir_a = transform_a.rotation.transform_vector(&Vector3::y_axis());
        let dir_b = transform_b.rotation.transform_vector(&Vector3::y_axis());
        
        let half_len_a = height_a / 2.0;
        let half_len_b = height_b / 2.0;
        
        let seg_a_start = body_a.position - dir_a * half_len_a;
        let seg_a_end = body_a.position + dir_a * half_len_a;
        let seg_b_start = body_b.position - dir_b * half_len_b;
        let seg_b_end = body_b.position + dir_b * half_len_b;
        
        // Find closest points on both line segments
        let (closest_a, closest_b) = Self::closest_points_on_segments(
            seg_a_start, seg_a_end, 
            seg_b_start, seg_b_end
        );
        
        // Calculate distance between closest points
        let dist_vec = closest_b - closest_a;
        let distance = dist_vec.magnitude();
        
        let total_radius = radius_a + radius_b;
        
        if distance <= total_radius {
            let normal = if distance > 0.0 {
                dist_vec.normalize()
            } else {
                // If the line segments intersect, use a default direction
                let perp_dir = if dir_a.x.abs() < 0.9 {
                    dir_a.cross(&Vector3::x_axis()).normalize()
                } else {
                    dir_a.cross(&Vector3::z_axis()).normalize()
                };
                perp_dir
            };
            
            let penetration_depth = total_radius - distance;
            let contact_point = closest_a + dist_vec * 0.5; // Midpoint between closest points
            
            Some(Contact {
                body_a: i,
                body_b: j,
                contact_point,
                normal,
                penetration_depth,
                restitution: (body_a.restitution + body_b.restitution) / 2.0,
                friction: (body_a.friction + body_b.friction) / 2.0,
            })
        } else {
            None
        }
    }

    // Helper function to find closest points on two line segments
    fn closest_points_on_segments(start_a: Vector3<f32>, end_a: Vector3<f32>, start_b: Vector3<f32>, end_b: Vector3<f32>) -> (Vector3<f32>, Vector3<f32>) {
        let d1 = end_a - start_a;
        let d2 = end_b - start_b;
        let r = start_a - start_b;
        
        let a = d1.dot(&d1);
        let e = d2.dot(&d2);
        let f = d2.dot(&r);
        
        if a <= f32::EPSILON && e <= f32::EPSILON {
            // Both segments are degenerate (points)
            return (start_a, start_b);
        }
        
        let c = d1.dot(&r);
        
        if a <= f32::EPSILON {
            // First segment is degenerate (point)
            let s = 0.0;
            let t = f.clamp(0.0, e) / e;
            (start_a, start_b + d2 * t)
        } else if e <= f32::EPSILON {
            // Second segment is degenerate (point)
            let t = 0.0;
            let s = (-c).clamp(0.0, a) / a;
            (start_a + d1 * s, start_b)
        } else {
            let b = d1.dot(&d2);
            let denom = a * e - b * b;
            
            let mut s = 0.0;
            let mut t = 0.0;
            
            if denom > f32::EPSILON {
                s = (b * f - c * e).clamp(0.0, denom) / denom;
                t = (b * s + f) / e;
                
                if t < 0.0 {
                    t = 0.0;
                    s = (-c).clamp(0.0, a) / a;
                } else if t > 1.0 {
                    t = 1.0;
                    s = (b - c).clamp(0.0, a) / a;
                }
            } else {
                s = 0.0;
                t = f.clamp(0.0, e) / e;
            }
            
            (start_a + d1 * s, start_b + d2 * t)
        }
    }
    
    fn resolve_contact(&mut self, contact: &Contact) {
        let body_a_idx = contact.body_a;
        let body_b_idx = contact.body_b;
        
        // Get mutable references to both bodies
        let (body_a, body_b) = if body_a_idx < body_b_idx {
            let (first, second) = self.rigid_bodies.split_at_mut(body_b_idx);
            (&mut first[body_a_idx], &mut second[0])
        } else {
            let (first, second) = self.rigid_bodies.split_at_mut(body_a_idx);
            (&mut second[0], &mut first[body_b_idx])
        };
        
        // Calculate relative velocity at contact point
        let r_a = contact.contact_point - body_a.position;
        let r_b = contact.contact_point - body_b.position;
        
        let vel_a = body_a.velocity + body_a.angular_velocity.cross(&r_a);
        let vel_b = body_b.velocity + body_b.angular_velocity.cross(&r_b);
        let relative_vel = vel_a - vel_b;
        
        // Calculate relative velocity along normal
        let vel_along_normal = relative_vel.dot(&contact.normal);
        
        // Don't resolve if velocities are separating
        if vel_along_normal > 0.0 {
            return;
        }
        
        // Calculate impulse scalar
        let mut impulse_scalar = -(1.0 + contact.restitution) * vel_along_normal;
        
        // Calculate denominator for impulse calculation
        let mut denom = body_a.inverse_mass + body_b.inverse_mass;
        
        // Add angular components
        let r_a_cross_n = r_a.cross(&contact.normal);
        let r_b_cross_n = r_b.cross(&contact.normal);
        
        let ang_a = (body_a.inverse_inertia_tensor * r_a_cross_n).cross(&r_a);
        let ang_b = (body_b.inverse_inertia_tensor * r_b_cross_n).cross(&r_b);
        
        denom += ang_a.dot(&contact.normal) + ang_b.dot(&contact.normal);
        
        if denom == 0.0 {
            return; // Avoid division by zero
        }
        
        impulse_scalar /= denom;
        
        // Apply impulse
        let impulse = contact.normal * impulse_scalar;
        body_a.apply_impulse(impulse);
        body_b.apply_impulse(-impulse);
        
        // Apply friction
        self.apply_friction(contact, &impulse);
        
        // Position correction to prevent sinking
        self.correct_position(contact);
    }
    
    fn apply_friction(&mut self, contact: &Contact, normal_impulse: &Vector3<f32>) {
        let body_a = &mut self.rigid_bodies[contact.body_a];
        let body_b = &mut self.rigid_bodies[contact.body_b];
        
        let r_a = contact.contact_point - body_a.position;
        let r_b = contact.contact_point - body_b.position;
        
        // Relative velocity at contact point
        let vel_a = body_a.velocity + body_a.angular_velocity.cross(&r_a);
        let vel_b = body_b.velocity + body_b.angular_velocity.cross(&r_b);
        let relative_vel = vel_a - vel_b;
        
        // Tangential velocity
        let tangent = relative_vel - contact.normal * relative_vel.dot(&contact.normal);
        let tangent_magnitude = tangent.magnitude();
        
        if tangent_magnitude < 0.001 {
            return; // Very small tangential velocity
        }
        
        let tangent = tangent.normalize();
        
        // Calculate friction impulse
        let impulse_magnitude = normal_impulse.magnitude();
        let friction_coefficient = contact.friction;
        let max_friction_impulse = friction_coefficient * impulse_magnitude;
        
        // Calculate friction impulse magnitude using Coulomb friction model
        let friction_impulse_magnitude = (relative_vel.dot(&tangent) / 
            (body_a.inverse_mass + body_b.inverse_mass + 
             (body_a.inverse_inertia_tensor * r_a.cross(&tangent)).cross(&r_a).dot(&tangent) + 
             (body_b.inverse_inertia_tensor * r_b.cross(&tangent)).cross(&r_b).dot(&tangent))).abs();
        
        // Clamp friction impulse
        let friction_impulse_magnitude = friction_impulse_magnitude.min(max_friction_impulse);
        let friction_impulse = tangent * (-friction_impulse_magnitude);
        
        // Apply friction impulse
        body_a.apply_impulse(friction_impulse);
        body_b.apply_impulse(-friction_impulse);
    }
    
    fn correct_position(&mut self, contact: &Contact) {
        let body_a = &mut self.rigid_bodies[contact.body_a];
        let body_b = &mut self.rigid_bodies[contact.body_b];
        
        // Positional correction to prevent sinking
        let percent = 0.2; // 20% correction
        let slop = 0.01; // Small separation allowance
        
        let correction = contact.normal * 
            ((contact.penetration_depth - slop) / (body_a.inverse_mass + body_b.inverse_mass)) * 
            percent;
            
        if !body_a.is_static {
            body_a.position -= correction * body_a.inverse_mass;
        }
        
        if !body_b.is_static {
            body_b.position += correction * body_b.inverse_mass;
        }
    }
    
    /// Sequential Impulse Solver for realistic constraint resolution
    /// This iterative solver handles multiple contacts and produces stable stacking
    fn solve_constraints(&mut self) {
        // For now, just ensure bodies don't fall through the floor
        // In production, this would handle all contact constraints iteratively
        for body in &mut self.rigid_bodies {
            if !body.is_static && body.position.y < 0.0 {
                // Collision with ground
                body.position.y = 0.0;
                
                // Apply bounce and friction
                if body.velocity.y < 0.0 {
                    body.velocity.y *= -body.restitution;
                    
                    // Apply friction against ground
                    body.velocity.x *= 1.0 - body.friction;
                    body.velocity.z *= 1.0 - body.friction;
                    
                    // Stop tiny bounces
                    if body.velocity.y.abs() < 0.1 {
                        body.velocity.y = 0.0;
                    }
                }
            }
        }
    }
    
    /// Sequential constraint solver - safe version without raw pointers
    fn solve_constraints_sequential(&mut self) {
        // Simple ground collision handling
        for body in self.rigid_bodies.iter_mut() {
            if !body.is_static && body.position.y < 0.0 {
                // Collision with ground
                body.position.y = 0.0;
                
                // Apply bounce and friction
                if body.velocity.y < 0.0 {
                    body.velocity.y *= -body.restitution;
                    
                    // Apply friction against ground
                    body.velocity.x *= 1.0 - body.friction;
                    body.velocity.z *= 1.0 - body.friction;
                    
                    // Stop tiny bounces
                    if body.velocity.y.abs() < 0.1 {
                        body.velocity.y = 0.0;
                    }
                }
            }
        }
    }

    /// Parallel Sequential Impulse Solver using rayon for multi-core performance
    /// FIXED: Removed unsafe raw pointer access, using sequential version instead
    fn solve_constraints_parallel(&mut self) {
        // Use sequential version to avoid data races
        // TODO: Implement proper parallel constraint solver with disjoint sets
        self.solve_constraints_sequential();
    }

    // Raycasting methods
    pub fn raycast(&self, ray: &Ray) -> Option<RaycastHit> {
        let mut closest_hit: Option<RaycastHit> = None;
        let mut closest_distance = f32::MAX;
        
        for (idx, body) in self.rigid_bodies.iter().enumerate() {
            if let Some(hit) = self.raycast_single_body(ray, body, idx) {
                if hit.distance < closest_distance {
                    closest_distance = hit.distance;
                    closest_hit = Some(hit);
                }
            }
        }
        
        closest_hit
    }

    fn raycast_single_body(&self, ray: &Ray, body: &RigidBody, body_index: usize) -> Option<RaycastHit> {
        let transform = body.get_world_transform();
        let inv_transform = transform.inverse();
        
        // Transform ray to body's local space
        let local_origin = inv_transform.transform_point(&ray.origin);
        let local_direction = inv_transform.rotation.transform_vector(&ray.direction);
        
        let local_ray = Ray {
            origin: local_origin,
            direction: local_direction,
        };
        
        let hit = match &body.shape {
            Shape::Sphere { radius } => {
                self.raycast_sphere(&local_ray, *radius)
            }
            Shape::Box { half_extents } => {
                self.raycast_box(&local_ray, half_extents)
            }
            Shape::Capsule { radius, height } => {
                self.raycast_capsule(&local_ray, *radius, *height)
            }
            Shape::Terrain { height_map, scale } => {
                self.raycast_terrain(&local_ray, height_map, scale)
            }
            Shape::Mesh { vertices, indices } => {
                self.raycast_mesh(&local_ray, vertices, indices)
            }
        };
        
        // Update body_index in the hit result
        hit.map(|mut h| {
            h.body_index = body_index;
            h
        })
    }

    fn raycast_sphere(&self, ray: &Ray, radius: f32) -> Option<RaycastHit> {
        let oc = ray.origin.coords - Vector3::new(0.0, 0.0, 0.0); // Sphere at origin in local space
        let a = ray.direction.dot(&ray.direction);
        let b = 2.0 * oc.dot(&ray.direction);
        let c = oc.dot(&oc) - radius * radius;
        
        let discriminant = b * b - 4.0 * a * c;
        
        if discriminant < 0.0 {
            None
        } else {
            let sqrt_discriminant = discriminant.sqrt();
            let t1 = (-b - sqrt_discriminant) / (2.0 * a);
            let t2 = (-b + sqrt_discriminant) / (2.0 * a);
            
            let t = if t1 > 0.0 { t1 } else { t2 };
            
            if t > 0.0 {
                let hit_point = ray.origin + ray.direction * t;
                let normal = (hit_point.coords - Vector3::new(0.0, 0.0, 0.0)).normalize();
                
                Some(RaycastHit {
                    point: hit_point,
                    normal,
                    distance: t,
                    body_index: 0, // Will be set by raycast_single_body caller
                })
            } else {
                None
            }
        }
    }

    fn raycast_box(&self, ray: &Ray, half_extents: &Vector3<f32>) -> Option<RaycastHit> {
        // Ray-box intersection using slabs method
        let inv_direction = Vector3::new(1.0 / ray.direction.x, 1.0 / ray.direction.y, 1.0 / ray.direction.z);
        
        let t1 = (-half_extents - ray.origin.coords).component_mul(&inv_direction);
        let t2 = (half_extents - ray.origin.coords).component_mul(&inv_direction);
        
        let t_min = t1.zip_map(t2, |a, b| a.min(b)).max();
        let t_max = t1.zip_map(t2, |a, b| a.max(b)).min();
        
        if t_max >= 0.0 && t_min <= t_max {
            let t = if t_min >= 0.0 { t_min } else { t_max };
            
            if t >= 0.0 {
                let hit_point = ray.origin + ray.direction * t;
                
                // Calculate normal based on which face was hit
                let hit_coords = hit_point.coords;
                let abs_x = hit_coords.x.abs();
                let abs_y = hit_coords.y.abs();
                let abs_z = hit_coords.z.abs();
                
                let normal = if (abs_x - half_extents.x).abs() < 0.001 {
                    Vector3::new(hit_coords.x.signum(), 0.0, 0.0)
                } else if (abs_y - half_extents.y).abs() < 0.001 {
                    Vector3::new(0.0, hit_coords.y.signum(), 0.0)
                } else {
                    Vector3::new(0.0, 0.0, hit_coords.z.signum())
                }.normalize();
                
                Some(RaycastHit {
                    point: hit_point,
                    normal,
                    distance: t,
                    body_index: 0, // Will be set by raycast_single_body caller
                })
            } else {
                None
            }
        } else {
            None
        }
    }

    fn raycast_capsule(&self, ray: &Ray, radius: f32, height: f32) -> Option<RaycastHit> {
        // A capsule is defined by a line segment and a radius
        // We need to find the intersection of the ray with the capsule volume
        let half_height = height / 2.0;
        let start = Vector3::new(0.0, -half_height, 0.0);
        let end = Vector3::new(0.0, half_height, 0.0);
        
        // Find the closest point on the capsule's line segment to the ray
        let ray_dir_norm = ray.direction.normalize();
        let seg_vec = end - start;
        let seg_len = seg_vec.magnitude();
        let seg_dir = seg_vec.normalize();
        
        // Calculate the parameters for the closest points on both lines
        let w = ray.origin.coords - start;
        let a = ray_dir_norm.dot(&ray_dir_norm);
        let b = ray_dir_norm.dot(&seg_dir);
        let c = seg_dir.dot(&seg_dir);
        let d = ray_dir_norm.dot(&w);
        let e = seg_dir.dot(&w);
        
        let denom = a * c - b * b;
        let mut ray_t = 0.0;
        let mut seg_t = 0.0;
        
        if denom < 0.0001 {
            // Lines are parallel
            ray_t = 0.0;
            seg_t = e / c;
        } else {
            ray_t = (b * e - c * d) / denom;
            seg_t = (a * e - b * d) / denom;
        }
        
        // Clamp seg_t to the line segment
        seg_t = seg_t.clamp(0.0, seg_len);
        let closest_on_segment = start + seg_dir * seg_t;
        
        // Now we have a sphere at closest_on_segment with radius
        // Check ray-sphere intersection
        let sphere_origin = closest_on_segment;
        let oc = ray.origin.coords - sphere_origin;
        let a = ray.direction.dot(&ray.direction);
        let b = 2.0 * oc.dot(&ray.direction);
        let c = oc.dot(&oc) - radius * radius;
        
        let discriminant = b * b - 4.0 * a * c;
        
        if discriminant < 0.0 {
            None
        } else {
            let sqrt_discriminant = discriminant.sqrt();
            let t1 = (-b - sqrt_discriminant) / (2.0 * a);
            let t2 = (-b + sqrt_discriminant) / (2.0 * a);
            
            let t = if t1 > 0.0 { t1 } else { t2 };
            
            if t > 0.0 {
                let hit_point = ray.origin + ray.direction * t;
                let normal = (hit_point.coords - closest_on_segment).normalize();
                
                Some(RaycastHit {
                    point: hit_point,
                    normal,
                    distance: t,
                    body_index: 0, // Will be set by raycast_single_body caller
                })
            } else {
                None
            }
        }
    }

    fn raycast_terrain(&self, ray: &Ray, height_map: &[Vec<f32>], scale: &Vector3<f32>) -> Option<RaycastHit> {
        // Simplified terrain raycast
        // For a real implementation, we'd use a heightfield-specific algorithm
        
        // Just check if the ray hits the Y=0 plane (for a flat ground case)
        if ray.direction.y >= 0.0 && ray.origin.y <= 0.0 {
            return None; // Ray going up from below ground
        }
        
        if ray.direction.y == 0.0 {
            return None; // Ray parallel to ground
        }
        
        let t = -ray.origin.y / ray.direction.y;
        
        if t < 0.0 {
            return None; // Intersection behind ray origin
        }
        
        let hit_pos = ray.origin + ray.direction * t;
        
        // Check if the hit position is within the terrain bounds
        let half_size_x = scale.x / 2.0;
        let half_size_z = scale.z / 2.0;
        
        if hit_pos.x >= -half_size_x && hit_pos.x <= half_size_x &&
           hit_pos.z >= -half_size_z && hit_pos.z <= half_size_z {
            Some(RaycastHit {
                point: hit_pos,
                normal: Vector3::new(0.0, 1.0, 0.0),
                distance: t,
                body_index: 0, // Will be set by raycast_single_body caller
            })
        } else {
            None
        }
    }

    fn raycast_mesh(&self, ray: &Ray, vertices: &[Vector3<f32>], indices: &[u32]) -> Option<RaycastHit> {
        // Ray-mesh intersection by checking each triangle
        let mut closest_hit: Option<RaycastHit> = None;
        let mut closest_distance = f32::MAX;
        
        for tri_idx in (0..indices.len()).step_by(3) {
            if tri_idx + 2 >= indices.len() {
                break;
            }
            
            let i1 = indices[tri_idx] as usize;
            let i2 = indices[tri_idx + 1] as usize;
            let i3 = indices[tri_idx + 2] as usize;
            
            if i1 >= vertices.len() || i2 >= vertices.len() || i3 >= vertices.len() {
                continue;
            }
            
            let v1 = vertices[i1];
            let v2 = vertices[i2];
            let v3 = vertices[i3];
            
            if let Some(tri_hit) = self.raycast_triangle(ray, &v1, &v2, &v3) {
                if tri_hit.distance < closest_distance {
                    closest_distance = tri_hit.distance;
                    closest_hit = Some(tri_hit);
                }
            }
        }
        
        closest_hit
    }

    fn raycast_triangle(&self, ray: &Ray, v1: &Vector3<f32>, v2: &Vector3<f32>, v3: &Vector3<f32>) -> Option<RaycastHit> {
        // Möller-Trumbore ray-triangle intersection algorithm
        let edge1 = *v2 - *v1;
        let edge2 = *v3 - *v1;
        
        let h = ray.direction.cross(&edge2);
        let a = edge1.dot(&h);
        
        if a > -f32::EPSILON && a < f32::EPSILON {
            return None; // Ray is parallel to triangle
        }
        
        let f = 1.0 / a;
        let s = ray.origin.coords - *v1;
        let u = f * s.dot(&h);
        
        if u < 0.0 || u > 1.0 {
            return None;
        }
        
        let q = s.cross(&edge1);
        let v = f * ray.direction.dot(&q);
        
        if v < 0.0 || u + v > 1.0 {
            return None;
        }
        
        let t = f * edge2.dot(&q);
        
        if t > f32::EPSILON {
            let hit_point = ray.origin + ray.direction * t;
            let normal = edge1.cross(&edge2).normalize();
            
            Some(RaycastHit {
                point: hit_point,
                normal,
                distance: t,
                body_index: 0, // Will be set by raycast_single_body caller
            })
        } else {
            None
        }
    }
}

impl Shape {
    pub fn as_sphere_radius(&self) -> Option<f32> {
        match self {
            Shape::Sphere { radius } => Some(*radius),
            _ => None,
        }
    }
    
    pub fn as_box_half_extents(&self) -> Option<Vector3<f32>> {
        match self {
            Shape::Box { half_extents } => Some(*half_extents),
            _ => None,
        }
    }
    
    pub fn as_capsule_dimensions(&self) -> Option<(f32, f32)> {
        match self {
            Shape::Capsule { radius, height } => Some((*radius, *height)),
            _ => None,
        }
    }
    
    pub fn as_terrain_properties(&self) -> Option<(&Vec<Vec<f32>>, &Vector3<f32>)> {
        match self {
            Shape::Terrain { height_map, scale } => Some((height_map, scale)),
            _ => None,
        }
    }
    
    pub fn as_mesh_data(&self) -> Option<(&Vec<Vector3<f32>>, &Vec<u32>)> {
        match self {
            Shape::Mesh { vertices, indices } => Some((vertices, indices)),
            _ => None,
        }
    }
}

#[derive(Debug, Clone)]
struct Contact {
    body_a: usize,
    body_b: usize,
    contact_point: Vector3<f32>,
    normal: Vector3<f32>,
    penetration_depth: f32,
    restitution: f32,
    friction: f32,
}