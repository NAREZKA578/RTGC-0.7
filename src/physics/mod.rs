use nalgebra::{Vector3, Isometry3, Matrix3, UnitQuaternion};
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub enum Shape {
    Sphere { radius: f32 },
    Box { half_extents: Vector3<f32> },
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
    pub shape: Shape,
    pub is_static: bool,
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
            shape: Shape::Sphere { radius },
            is_static: mass <= 0.0,
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
            shape: Shape::Box { half_extents },
            is_static: mass <= 0.0,
        }
    }

    pub fn apply_force(&mut self, force: Vector3<f32>) {
        if !self.is_static {
            self.velocity += force * self.inverse_mass;
        }
    }

    pub fn apply_impulse(&mut self, impulse: Vector3<f32>) {
        if !self.is_static {
            self.velocity += impulse * self.inverse_mass;
        }
    }

    pub fn apply_torque(&mut self, torque: Vector3<f32>) {
        if !self.is_static {
            // Transform torque to local space, apply, then back to world space
            let local_torque = self.rotation.inverse_transform_vector(&torque);
            let local_angular_velocity = self.inverse_inertia_tensor * local_torque;
            self.angular_velocity += self.rotation.transform_vector(&local_angular_velocity);
        }
    }

    pub fn apply_angular_impulse(&mut self, impulse: Vector3<f32>) {
        if !self.is_static {
            self.angular_velocity += self.inverse_inertia_tensor * impulse;
        }
    }

    pub fn update(&mut self, dt: f32) {
        if !self.is_static {
            // Update position based on velocity
            self.position += self.velocity * dt;
            
            // Update orientation based on angular velocity
            let angular_speed = self.angular_velocity.magnitude();
            if angular_speed > 0.0001 {
                let axis = self.angular_velocity.normalize();
                let rotation_change = UnitQuaternion::from_axis_angle(&axis, angular_speed * dt);
                self.rotation = rotation_change * self.rotation;
                self.rotation.renormalize();
            }
            
            // Apply gravity (assuming -9.81 m/s^2 in y direction)
            self.velocity.y -= 9.81 * dt;
            
            // Simple damping to prevent infinite acceleration
            self.velocity *= 0.999;
            self.angular_velocity *= 0.99;
        }
    }
    
    pub fn get_world_transform(&self) -> Isometry3<f32> {
        Isometry3::from_parts(self.position.into(), self.rotation)
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
        // Integrate velocities
        for body in &mut self.rigid_bodies {
            if !body.is_static {
                // Apply gravity
                body.apply_force(self.gravity * body.mass);
                
                // Update positions and orientations
                body.update(self.time_step);
            }
        }
        
        // Handle collisions
        self.handle_collisions();
        
        // Solve constraints (like contacts)
        self.solve_constraints();
    }

    fn handle_collisions(&mut self) {
        let mut contacts = Vec::new();
        
        // Broad phase: simple all-vs-all check
        for i in 0..self.rigid_bodies.len() {
            for j in (i + 1)..self.rigid_bodies.len() {
                if let Some(contact) = self.detect_collision(i, j) {
                    contacts.push(contact);
                }
            }
        }
        
        // Process contacts
        for contact in &contacts {
            self.resolve_contact(contact);
        }
    }
    
    fn detect_collision(&self, i: usize, j: usize) -> Option<Contact> {
        let body_a = &self.rigid_bodies[i];
        let body_b = &self.rigid_bodies[j];
        
        // Skip if both are static
        if body_a.is_static && body_b.is_static {
            return None;
        }
        
        // Sphere-sphere collision detection
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
            _ => {
                // For now, only handle sphere-sphere collisions
                // More complex shapes would require additional algorithms
                None
            }
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
        let friction_impulse_scalar = relative_vel.dot(&tangent);
        let friction_impulse_scalar = friction_impulse_scalar / 
            (body_a.inverse_mass + body_b.inverse_mass); // Simplified
        
        // Clamp friction impulse
        let max_friction_impulse = contact.friction * impulse_magnitude;
        let friction_impulse_scalar = friction_impulse_scalar.clamp(
            -max_friction_impulse, 
            max_friction_impulse
        );
        
        let friction_impulse = tangent * (-friction_impulse_scalar);
        
        // Apply friction impulse
        body_a.apply_impulse(friction_impulse);
        body_b.apply_impulse(-friction_impulse);
    }
    
    fn correct_position(&mut self, contact: &Contact) {
        let body_a = &mut self.rigid_bodies[contact.body_a];
        let body_b = &mut self.rigid_bodies[contact.body_b];
        
        // Simple positional correction to prevent sinking
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
    
    fn solve_constraints(&mut self) {
        // For now, just ensure bodies don't fall through the floor
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