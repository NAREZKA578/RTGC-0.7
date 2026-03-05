use nalgebra::{Vector3, UnitQuaternion, Matrix3};
use std::f32::consts::PI;

#[derive(Debug, Clone)]
pub struct AdvancedSuspension {
    pub spring_stiffness: f32,           // N/m
    pub compression_damping: f32,        // N*s/m for compression
    pub rebound_damping: f32,            // N*s/m for rebound
    pub rest_length: f32,                // meters
    pub current_length: f32,             // meters
    pub max_compression: f32,            // meters
    pub max_extension: f32,              // meters
    pub tire_radius: f32,                // meters
    pub friction_coefficient: f32,       // static friction coefficient
    pub slip_ratio: f32,                 // dimensionless
    pub slip_angle: f32,                 // radians
    pub camber_angle: f32,               // radians
}

impl AdvancedSuspension {
    pub fn new(
        spring_stiffness: f32,
        compression_damping: f32,
        rebound_damping: f32,
        rest_length: f32,
        max_compression: f32,
        max_extension: f32,
        tire_radius: f32,
    ) -> Self {
        Self {
            spring_stiffness,
            compression_damping,
            rebound_damping,
            rest_length,
            current_length: rest_length,
            max_compression,
            max_extension,
            tire_radius,
            friction_coefficient: 0.8, // Default grip
            slip_ratio: 0.0,
            slip_angle: 0.0,
            camber_angle: 0.0,
        }
    }

    pub fn update_suspension(
        &mut self,
        wheel_velocity: Vector3<f32>,
        contact_normal: Vector3<f32>,
        vehicle_velocity: Vector3<f32>,
        dt: f32,
    ) -> (Vector3<f32>, f32) {
        // Calculate current suspension compression
        let compression = self.rest_length - (self.current_length - self.tire_radius);
        
        // Calculate spring force (non-linear progressive stiffness could be added here)
        let spring_force = self.calculate_spring_force(compression);
        
        // Calculate damping force based on suspension velocity
        let damping_force = self.calculate_damping_force(wheel_velocity, contact_normal);
        
        // Total suspension force
        let suspension_force = spring_force + damping_force;
        
        // Calculate tire contact patch forces using Pacejka/Magic Formula approach
        let (longitudinal_force, lateral_force) = self.calculate_tire_forces(
            vehicle_velocity,
            wheel_velocity,
            contact_normal,
            suspension_force,
            dt,
        );
        
        // Combine all forces
        let total_force = longitudinal_force + lateral_force + 
                         contact_normal * suspension_force;
        
        (total_force, suspension_force)
    }

    fn calculate_spring_force(&self, compression: f32) -> f32 {
        // Non-linear spring with progressive rate could be implemented here
        // For now, simple linear spring with limits
        let effective_compression = compression.max(-self.max_compression).min(self.max_extension);
        self.spring_stiffness * effective_compression
    }

    fn calculate_damping_force(&self, wheel_velocity: Vector3<f32>, contact_normal: Vector3<f32>) -> f32 {
        let velocity_along_normal = wheel_velocity.dot(&contact_normal);
        
        // Different damping coefficients for compression vs rebound
        let damping_coefficient = if velocity_along_normal > 0.0 {
            // Compression (wheel moving upward)
            self.compression_damping
        } else {
            // Rebound (wheel moving downward)
            self.rebound_damping
        };
        
        damping_coefficient * velocity_along_normal
    }

    fn calculate_tire_forces(
        &mut self,
        vehicle_velocity: Vector3<f32>,
        wheel_velocity: Vector3<f32>,
        contact_normal: Vector3<f32>,
        normal_force: f32,
        dt: f32,
    ) -> (Vector3<f32>, Vector3<f32>) {
        // Calculate wheel's forward and lateral directions relative to vehicle
        let forward = Vector3::new(0.0, 0.0, 1.0); // Assuming wheel's forward direction
        let lateral = Vector3::new(1.0, 0.0, 0.0); // Assuming wheel's lateral direction
        
        // Calculate slip ratio and slip angle (simplified)
        let wheel_forward_vel = wheel_velocity.dot(&forward);
        let vehicle_forward_vel = vehicle_velocity.dot(&forward);
        
        // Slip ratio: (wheel speed - vehicle speed) / |vehicle speed|
        if vehicle_forward_vel.abs() > 0.1 {
            self.slip_ratio = (wheel_forward_vel - vehicle_forward_vel) / vehicle_forward_vel.abs();
        } else {
            self.slip_ratio = 0.0;
        }
        
        // Slip angle: angle between wheel heading and actual travel direction
        let side_slip_vel = vehicle_velocity.dot(&lateral);
        if vehicle_forward_vel.abs() > 0.1 {
            self.slip_angle = (side_slip_vel / vehicle_forward_vel.abs()).atan();
        } else {
            self.slip_angle = 0.0;
        }
        
        // Using a simplified version of Pacejka's magic formula
        let max_grip = self.friction_coefficient * normal_force;
        
        // Calculate longitudinal and lateral force factors based on slip
        let longitudinal_factor = self.calculate_pacejka_value(self.slip_ratio, 10.0, 1.0, 1.0);
        let lateral_factor = self.calculate_pacejka_value(self.slip_angle.to_degrees(), 10.0, 1.0, 1.0);
        
        let longitudinal_force_magnitude = max_grip * longitudinal_factor;
        let lateral_force_magnitude = max_grip * lateral_factor;
        
        // Apply forces in the correct directions
        let longitudinal_force = forward * longitudinal_force_magnitude;
        let lateral_force = lateral * lateral_force_magnitude;
        
        (longitudinal_force, lateral_force)
    }

    fn calculate_pacejka_value(&self, slip: f32, peak_slip: f32, peak_value: f32, curvature: f32) -> f32 {
        // Simplified Pacejka-like function
        let b = curvature;
        let c = peak_value;
        let d = peak_value;
        
        let bx = b * slip;
        d * (c * (bx - (bx * bx * 0.0174533).sin()).tan())
    }
}

#[derive(Debug, Clone)]
pub struct AdvancedWheel {
    pub position: Vector3<f32>,          // Position relative to vehicle center
    pub radius: f32,                     // Tire radius in meters
    pub suspension: AdvancedSuspension,
    pub rotation_angle: f32,             // Current rotation in radians
    pub steering_angle: f32,             // Steering angle in radians
    pub angular_velocity: f32,           // Angular velocity in rad/s
    pub drive_torque: f32,               // Applied drive/brake torque in N*m
    pub brake_torque: f32,               // Brake torque in N*m
    pub rolling_resistance: f32,         // Rolling resistance coefficient
}

impl AdvancedWheel {
    pub fn new(
        position: Vector3<f32>,
        radius: f32,
        suspension: AdvancedSuspension,
    ) -> Self {
        Self {
            position,
            radius,
            suspension,
            rotation_angle: 0.0,
            steering_angle: 0.0,
            angular_velocity: 0.0,
            drive_torque: 0.0,
            brake_torque: 0.0,
            rolling_resistance: 0.015, // Typical value for car tires
        }
    }

    pub fn update(
        &mut self,
        vehicle_velocity: Vector3<f32>,
        vehicle_angular_velocity: Vector3<f32>,
        wheel_linear_velocity: Vector3<f32>,
        contact_normal: Vector3<f32>,
        dt: f32,
    ) -> (Vector3<f32>, f32) {
        // Calculate forces from suspension model
        let (force, normal_force) = self.suspension.update_suspension(
            wheel_linear_velocity,
            contact_normal,
            vehicle_velocity,
            dt,
        );
        
        // Apply drive/brake torques
        let net_torque = self.drive_torque - self.brake_torque.signum() * self.brake_torque.abs().min(normal_force * self.suspension.friction_coefficient * self.radius);
        
        // Update wheel angular velocity based on torques and forces
        self.update_angular_velocity(net_torque, force, normal_force, dt);
        
        // Update rotation angle
        self.rotation_angle += self.angular_velocity * dt;
        
        (force, normal_force)
    }

    fn update_angular_velocity(&mut self, torque: f32, lateral_force: Vector3<f32>, normal_force: f32, dt: f32) {
        // Moment of inertia for a wheel (approximated as a solid disk)
        let inertia = 0.5 * 30.0 * self.radius * self.radius; // Assuming 30kg wheel mass
        
        // Calculate angular acceleration from torques
        let angular_acceleration = torque / inertia;
        
        // Consider forces affecting rotation (like rolling resistance)
        let rolling_resistance_torque = -self.rolling_resistance * normal_force * self.radius * self.angular_velocity.signum();
        
        // Update angular velocity
        self.angular_velocity += (angular_acceleration + (rolling_resistance_torque / inertia)) * dt;
    }

    pub fn apply_drive_torque(&mut self, torque: f32) {
        self.drive_torque = torque;
    }

    pub fn apply_brake_torque(&mut self, torque: f32) {
        self.brake_torque = torque;
    }

    pub fn set_steering_angle(&mut self, angle: f32) {
        self.steering_angle = angle.clamp(-PI/3.0, PI/3.0); // Limit to ~60 degrees
    }
}

#[derive(Debug, Clone)]
pub struct AdvancedVehicle {
    pub chassis_body_index: usize,       // Reference to chassis rigid body
    pub wheels: Vec<AdvancedWheel>,      // Vehicle wheels
    pub mass: f32,                       // Total vehicle mass in kg
    pub engine_torque: f32,              // Current engine torque in N*m
    pub engine_rpm: f32,                 // Current engine RPM
    pub gear_ratio: f32,                 // Current gear ratio
    pub final_drive_ratio: f32,          // Final drive ratio
    pub steering_angle: f32,             // Current steering angle in radians
    pub max_steering_angle: f32,         // Maximum steering angle in radians
    pub max_engine_torque: f32,          // Maximum engine torque in N*m
    pub max_engine_rpm: f32,             // Maximum engine RPM
    pub brake_torque: f32,               // Current brake torque per wheel in N*m
    pub aero_drag_coefficient: f32,      // Aerodynamic drag coefficient
    pub frontal_area: f32,               // Frontal area in m^2
    pub air_density: f32,                // Air density in kg/m^3
    pub center_of_gravity: Vector3<f32>, // CoG offset from chassis center
}

impl AdvancedVehicle {
    pub fn new(chassis_body_index: usize, mass: f32) -> Self {
        Self {
            chassis_body_index,
            wheels: Vec::new(),
            mass,
            engine_torque: 0.0,
            engine_rpm: 0.0,
            gear_ratio: 1.0,
            final_drive_ratio: 3.5,
            steering_angle: 0.0,
            max_steering_angle: PI / 3.0, // ~60 degrees
            max_engine_torque: 400.0,     // 400 N*m typical for a car
            max_engine_rpm: 6000.0,
            brake_torque: 0.0,
            aero_drag_coefficient: 0.35,  // Typical for a sedan
            frontal_area: 2.2,            // Typical for a sedan in m^2
            air_density: 1.225,           // At sea level
            center_of_gravity: Vector3::new(0.0, -0.5, 0.0), // Below chassis center
        }
    }

    pub fn add_wheel(&mut self, wheel: AdvancedWheel) {
        self.wheels.push(wheel);
    }

    pub fn update_vehicle_physics(&mut self, chassis_body: &mut crate::physics::RigidBody, dt: f32) {
        // Get chassis state
        let chassis_velocity = chassis_body.velocity;
        let chassis_angular_velocity = chassis_body.angular_velocity;
        let chassis_transform = chassis_body.get_world_transform();

        // Calculate aerodynamic drag force
        let velocity_mag_sq = chassis_velocity.magnitude_squared();
        let drag_force_magnitude = 0.5 * self.air_density * self.aero_drag_coefficient * self.frontal_area * velocity_mag_sq;
        let drag_direction = -chassis_velocity.normalize();
        let drag_force = drag_direction * drag_force_magnitude;

        // Apply drag force at center of gravity
        let cog_world_pos = chassis_transform.transform_point(&nalgebra::Point3::from(self.center_of_gravity));
        chassis_body.apply_force(drag_force);

        // Update each wheel and apply forces to chassis
        for (i, wheel) in self.wheels.iter_mut().enumerate() {
            // Calculate wheel world position and velocity
            let wheel_local_pos = wheel.position;
            let wheel_world_pos = chassis_transform.transform_point(&nalgebra::Point3::from(wheel_local_pos));
            let wheel_linear_velocity = chassis_velocity + 
                                       chassis_angular_velocity.cross(&(wheel_world_pos.coords - chassis_body.position));

            // For simplicity, assume contact normal is up (would require raycast in real implementation)
            let contact_normal = Vector3::new(0.0, 1.0, 0.0);

            // Apply steering angle to front wheels
            if i == 0 || i == 1 { // Assuming first two wheels are front wheels
                wheel.set_steering_angle(self.steering_angle);
            }

            // Update wheel physics
            let (wheel_force, normal_force) = wheel.update(
                chassis_velocity,
                chassis_angular_velocity,
                wheel_linear_velocity,
                contact_normal,
                dt,
            );

            // Apply forces to chassis body
            chassis_body.apply_force(wheel_force);
            
            // Apply torque from lateral forces (simplified)
            let moment_arm = wheel_world_pos.coords - cog_world_pos.coords;
            let torque = moment_arm.cross(&wheel_force);
            chassis_body.apply_torque(torque);
        }

        // Update engine RPM based on wheel speeds (simplified)
        self.update_engine_rpm(dt);
    }

    fn update_engine_rpm(&mut self, dt: f32) {
        // Simplified calculation based on average wheel angular velocity
        if !self.wheels.is_empty() {
            let avg_wheel_angular_velocity: f32 = self.wheels.iter()
                .map(|w| w.angular_velocity.abs())
                .sum::<f32>() / self.wheels.len() as f32;
            
            // Convert wheel angular velocity to engine RPM considering gear ratios
            self.engine_rpm = avg_wheel_angular_velocity * self.final_drive_ratio * self.gear_ratio * 60.0 / (2.0 * PI);
            self.engine_rpm = self.engine_rpm.clamp(0.0, self.max_engine_rpm);
        }
    }

    pub fn apply_throttle(&mut self, throttle: f32) {
        // Calculate engine torque based on throttle input and engine characteristics
        // Simplified engine curve
        let normalized_rpm = self.engine_rpm / self.max_engine_rpm;
        let efficiency_factor = 1.0 - (normalized_rpm - 0.5).powi(2) * 0.2; // Simplified power curve
        self.engine_torque = throttle * self.max_engine_torque * efficiency_factor;
    }

    pub fn apply_brakes(&mut self, brake_intensity: f32) {
        self.brake_torque = brake_intensity * 2000.0; // Max 2000 N*m per wheel
    }

    pub fn set_steering(&mut self, steering_input: f32) {
        self.steering_angle = steering_input * self.max_steering_angle;
    }

    pub fn shift_gear(&mut self, gear_ratio: f32) {
        self.gear_ratio = gear_ratio;
    }
}