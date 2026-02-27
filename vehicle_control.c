#include "vehicle_control.h"
#include "raylib.h"
#include <stdlib.h>
#include <math.h>

// Vehicle-specific data structures
typedef struct {
    float wheel_base;      // Distance between front and rear axles
    float track_width;     // Distance between left and right wheels
    float max_steer_angle; // Maximum steering angle in radians
    float max_speed;       // Maximum speed in m/s
    float acceleration;    // Acceleration rate
    float braking;         // Braking rate
    float friction;        // Tire friction coefficient
} CarData;

typedef struct {
    float wheel_base;
    float track_width;
    float max_steer_angle;
    float max_speed;
    float acceleration;
    float braking;
    float friction;
} TruckData;

typedef struct {
    float wheel_base;
    float max_steer_angle;
    float max_speed;
    float acceleration;
    float braking;
    float lean_angle;      // Current lean angle for motorcycle
} MotorcycleData;

typedef struct {
    float max_thrust;
    float max_lift;
    float wing_area;
    float drag_coefficient;
} AirplaneData;

typedef struct {
    float rotor_radius;
    float max_lift;
    float tail_rotor_force;
} HelicopterData;

typedef struct {
    float track_width;
    float max_speed;
    float acceleration;
    float braking;
    float turn_rate;
} TankData;

void init_car(Vehicle* v, Vector3 pos) {
    v->position = pos;
    v->velocity = (Vector3){0};
    v->rotation = (Vector3){0};
    v->mass = 1200.0f;  // 1200kg
    v->type = VEHICLE_CAR;
    v->throttle = 0.0f;
    v->brake = 0.0f;
    v->steering = 0.0f;
    v->handbrake = false;
    
    // Allocate and initialize car-specific data
    CarData* data = malloc(sizeof(CarData));
    data->wheel_base = 2.7f;           // meters
    data->track_width = 1.6f;          // meters
    data->max_steer_angle = 0.6f;      // radians (~34 degrees)
    data->max_speed = 50.0f;           // m/s (~180 km/h)
    data->acceleration = 3.0f;         // m/s^2
    data->braking = 8.0f;              // m/s^2
    data->friction = 0.8f;             // Coefficient of friction
    
    v->data = data;
}

void init_truck(Vehicle* v, Vector3 pos) {
    v->position = pos;
    v->velocity = (Vector3){0};
    v->rotation = (Vector3){0};
    v->mass = 3500.0f;  // 3500kg
    v->type = VEHICLE_TRUCK;
    v->throttle = 0.0f;
    v->brake = 0.0f;
    v->steering = 0.0f;
    v->handbrake = false;
    
    // Allocate and initialize truck-specific data
    TruckData* data = malloc(sizeof(TruckData));
    data->wheel_base = 4.5f;
    data->track_width = 2.0f;
    data->max_steer_angle = 0.4f;      // Smaller steering angle for stability
    data->max_speed = 27.0f;           // ~97 km/h
    data->acceleration = 1.5f;         // Slower acceleration
    data->braking = 5.0f;              // Slower braking
    data->friction = 0.9f;             // Higher friction for grip
    
    v->data = data;
}

void init_motorcycle(Vehicle* v, Vector3 pos) {
    v->position = pos;
    v->velocity = (Vector3){0};
    v->rotation = (Vector3){0};
    v->mass = 200.0f;  // 200kg
    v->type = VEHICLE_MOTORCYCLE;
    v->throttle = 0.0f;
    v->brake = 0.0f;
    v->steering = 0.0f;
    v->handbrake = false;
    
    // Allocate and initialize motorcycle-specific data
    MotorcycleData* data = malloc(sizeof(MotorcycleData));
    data->wheel_base = 1.4f;
    data->max_steer_angle = 0.8f;      // Can lean more
    data->max_speed = 60.0f;           // ~216 km/h
    data->acceleration = 5.0f;
    data->braking = 10.0f;
    data->lean_angle = 0.0f;
    
    v->data = data;
}

void init_airplane(Vehicle* v, Vector3 pos) {
    v->position = pos;
    v->velocity = (Vector3){0};
    v->rotation = (Vector3){0};
    v->mass = 5000.0f;  // 5000kg
    v->type = VEHICLE_AIRPLANE;
    v->throttle = 0.0f;
    v->brake = 0.0f;
    v->steering = 0.0f;
    v->handbrake = false;
    
    // Allocate and initialize airplane-specific data
    AirplaneData* data = malloc(sizeof(AirplaneData));
    data->max_thrust = 100000.0f;      // Newtons
    data->max_lift = 200000.0f;        // Newtons
    data->wing_area = 30.0f;           // m^2
    data->drag_coefficient = 0.02f;
    
    v->data = data;
}

void init_helicopter(Vehicle* v, Vector3 pos) {
    v->position = pos;
    v->velocity = (Vector3){0};
    v->rotation = (Vector3){0};
    v->mass = 5000.0f;  // 5000kg
    v->type = VEHICLE_HELICOPTER;
    v->throttle = 0.0f;
    v->brake = 0.0f;
    v->steering = 0.0f;
    v->handbrake = false;
    
    // Allocate and initialize helicopter-specific data
    HelicopterData* data = malloc(sizeof(HelicopterData));
    data->rotor_radius = 6.0f;
    data->max_lift = 50000.0f;         // Newtons
    data->tail_rotor_force = 5000.0f;
    
    v->data = data;
}

void init_tank(Vehicle* v, Vector3 pos) {
    v->position = pos;
    v->velocity = (Vector3){0};
    v->rotation = (Vector3){0};
    v->mass = 60000.0f;  // 60 tons
    v->type = VEHICLE_TANK;
    v->throttle = 0.0f;
    v->brake = 0.0f;
    v->steering = 0.0f;
    v->handbrake = false;
    
    // Allocate and initialize tank-specific data
    TankData* data = malloc(sizeof(TankData));
    data->track_width = 3.0f;
    data->max_speed = 15.0f;           // ~54 km/h
    data->acceleration = 0.8f;
    data->braking = 2.0f;
    data->turn_rate = 0.3f;            // Radians per second
    
    v->data = data;
}

void handle_vehicle_controls(Vehicle* v) {
    switch(v->type) {
        case VEHICLE_CAR:
        case VEHICLE_TRUCK: {
            CarData* data = (CarData*)v->data;
            
            // Throttle and brake
            if (IsKeyDown(KEY_W) || IsKeyDown(KEY_UP)) {
                v->throttle = fminf(1.0f, v->throttle + 0.05f);
            } else if (IsKeyDown(KEY_S) || IsKeyDown(KEY_DOWN)) {
                v->throttle = fmaxf(-1.0f, v->throttle - 0.05f);
            } else {
                // Natural deceleration
                if (v->throttle > 0) v->throttle = fmaxf(0, v->throttle - 0.02f);
                else if (v->throttle < 0) v->throttle = fminf(0, v->throttle + 0.02f);
            }
            
            // Steering
            if (IsKeyDown(KEY_A) || IsKeyDown(KEY_LEFT)) {
                v->steering = fmaxf(-data->max_steer_angle, v->steering - 0.03f);
            } else if (IsKeyDown(KEY_D) || IsKeyDown(KEY_RIGHT)) {
                v->steering = fminf(data->max_steer_angle, v->steering + 0.03f);
            } else {
                // Return steering to center
                if (v->steering > 0) v->steering = fmaxf(0, v->steering - 0.03f);
                else if (v->steering < 0) v->steering = fminf(0, v->steering + 0.03f);
            }
            
            // Handbrake
            v->handbrake = IsKeyDown(KEY_SPACE);
            break;
        }
        
        case VEHICLE_MOTORCYCLE: {
            MotorcycleData* data = (MotorcycleData*)v->data;
            
            // Throttle and brake
            if (IsKeyDown(KEY_W) || IsKeyDown(KEY_UP)) {
                v->throttle = fminf(1.0f, v->throttle + 0.05f);
            } else if (IsKeyDown(KEY_S) || IsKeyDown(KEY_DOWN)) {
                v->throttle = fmaxf(-1.0f, v->throttle - 0.05f);
            } else {
                if (v->throttle > 0) v->throttle = fmaxf(0, v->throttle - 0.02f);
                else if (v->throttle < 0) v->throttle = fminf(0, v->throttle + 0.02f);
            }
            
            // Steering (and leaning)
            if (IsKeyDown(KEY_A) || IsKeyDown(KEY_LEFT)) {
                v->steering = fmaxf(-data->max_steer_angle, v->steering - 0.05f);
                data->lean_angle = fmaxf(-0.5f, data->lean_angle - 0.02f);
            } else if (IsKeyDown(KEY_D) || IsKeyDown(KEY_RIGHT)) {
                v->steering = fminf(data->max_steer_angle, v->steering + 0.05f);
                data->lean_angle = fminf(0.5f, data->lean_angle + 0.02f);
            } else {
                // Return to center
                if (v->steering > 0) v->steering = fmaxf(0, v->steering - 0.05f);
                else if (v->steering < 0) v->steering = fminf(0, v->steering + 0.05f);
                
                if (data->lean_angle > 0) data->lean_angle = fmaxf(0, data->lean_angle - 0.02f);
                else if (data->lean_angle < 0) data->lean_angle = fminf(0, data->lean_angle + 0.02f);
            }
            break;
        }
        
        case VEHICLE_AIRPLANE: {
            AirplaneData* data = (AirplaneData*)v->data;
            
            // Throttle
            if (IsKeyDown(KEY_W) || IsKeyDown(KEY_UP)) {
                v->throttle = fminf(1.0f, v->throttle + 0.02f);
            } else if (IsKeyDown(KEY_S) || IsKeyDown(KEY_DOWN)) {
                v->throttle = fmaxf(0.0f, v->throttle - 0.02f);
            }
            
            // Control surfaces (simplified)
            if (IsKeyDown(KEY_A) || IsKeyDown(KEY_LEFT)) {
                v->rotation.z += 0.02f;  // Roll left
            } else if (IsKeyDown(KEY_D) || IsKeyDown(KEY_RIGHT)) {
                v->rotation.z -= 0.02f;  // Roll right
            }
            
            if (IsKeyDown(KEY_Q)) {
                v->rotation.x -= 0.02f;  // Pitch up
            } else if (IsKeyDown(KEY_E)) {
                v->rotation.x += 0.02f;  // Pitch down
            }
            
            if (IsKeyDown(KEY_Z)) {
                v->rotation.y -= 0.02f;  // Yaw left
            } else if (IsKeyDown(KEY_C)) {
                v->rotation.y += 0.02f;  // Yaw right
            }
            break;
        }
        
        case VEHICLE_HELICOPTER: {
            HelicopterData* data = (HelicopterData*)v->data;
            
            // Collective (up/down)
            if (IsKeyDown(KEY_W) || IsKeyDown(KEY_UP)) {
                v->throttle = fminf(1.0f, v->throttle + 0.02f);
            } else if (IsKeyDown(KEY_S) || IsKeyDown(KEY_DOWN)) {
                v->throttle = fmaxf(0.0f, v->throttle - 0.02f);
            }
            
            // Cyclic control (forward/back, left/right)
            if (IsKeyDown(KEY_Q)) {
                v->rotation.x -= 0.01f;  // Tilt forward
            } else if (IsKeyDown(KEY_E)) {
                v->rotation.x += 0.01f;  // Tilt back
            }
            
            if (IsKeyDown(KEY_A) || IsKeyDown(KEY_LEFT)) {
                v->rotation.z -= 0.01f;  // Roll left
            } else if (IsKeyDown(KEY_D) || IsKeyDown(KEY_RIGHT)) {
                v->rotation.z += 0.01f;  // Roll right
            }
            
            // Pedals (yaw)
            if (IsKeyDown(KEY_Z)) {
                v->rotation.y -= 0.02f;  // Yaw left
            } else if (IsKeyDown(KEY_C)) {
                v->rotation.y += 0.02f;  // Yaw right
            }
            break;
        }
        
        case VEHICLE_TANK: {
            TankData* data = (TankData*)v->data;
            
            // Differential steering
            float left_thrust = 0.0f, right_thrust = 0.0f;
            
            if (IsKeyDown(KEY_W) || IsKeyDown(KEY_UP)) {
                left_thrust = 1.0f;
                right_thrust = 1.0f;
            } else if (IsKeyDown(KEY_S) || IsKeyDown(KEY_DOWN)) {
                left_thrust = -1.0f;
                right_thrust = -1.0f;
            }
            
            if (IsKeyDown(KEY_A) || IsKeyDown(KEY_LEFT)) {
                left_thrust = -0.5f;
                right_thrust = 0.5f;
            } else if (IsKeyDown(KEY_D) || IsKeyDown(KEY_RIGHT)) {
                left_thrust = 0.5f;
                right_thrust = -0.5f;
            }
            
            // Apply thrust to tank
            Vector3 forward = (Vector3){
                sinf(v->rotation.y),
                0,
                cosf(v->rotation.y)
            };
            
            Vector3 thrust = (Vector3){
                forward.x * (left_thrust + right_thrust) * 0.5f,
                0,
                forward.z * (left_thrust + right_thrust) * 0.5f
            };
            
            // Turning creates rotation
            float turn_torque = (right_thrust - left_thrust) * 0.1f;
            v->rotation.y += turn_torque;
            
            // Apply thrust to velocity
            v->velocity.x += thrust.x * data->acceleration * 0.1f;
            v->velocity.z += thrust.z * data->acceleration * 0.1f;
            
            // Apply braking/drag
            v->velocity.x *= 0.95f;
            v->velocity.z *= 0.95f;
            
            break;
        }
    }
}

void update_vehicle(Vehicle* v, float delta_time) {
    // Update position based on velocity
    v->position.x += v->velocity.x * delta_time;
    v->position.y += v->velocity.y * delta_time;
    v->position.z += v->velocity.z * delta_time;
    
    // Apply gravity unless flying
    if (v->type != VEHICLE_AIRPLANE && v->type != VEHICLE_HELICOPTER) {
        v->velocity.y += -9.81f * delta_time;  // Gravity
        
        // Ground collision
        if (v->position.y < 0.5f) {  // Assuming vehicle height of 0.5f
            v->position.y = 0.5f;
            v->velocity.y = fmaxf(0, v->velocity.y);  // Stop downward velocity
            
            // Apply ground friction if not airborne
            v->velocity.x *= 0.9f;
            v->velocity.z *= 0.9f;
        }
    }
    
    switch(v->type) {
        case VEHICLE_CAR:
        case VEHICLE_TRUCK: {
            CarData* data = (CarData*)v->data;
            
            // Calculate speed-dependent steering effect
            float speed_factor = fminf(1.0f, sqrtf(v->velocity.x*v->velocity.x + v->velocity.z*v->velocity.z) / data->max_speed);
            float effective_steering = v->steering * (1.0f - speed_factor * 0.7f);  // Less responsive at high speed
            
            // Apply steering to rotation
            v->rotation.y += effective_steering * delta_time * 2.0f;
            
            // Calculate forward direction
            Vector3 forward = (Vector3){
                sinf(v->rotation.y),
                0,
                cosf(v->rotation.y)
            };
            
            // Apply thrust/braking in forward direction
            float traction = (v->throttle - (v->brake * (v->throttle >= 0 ? 1.0f : -1.0f))) * data->acceleration;
            v->velocity.x += forward.x * traction * delta_time;
            v->velocity.z += forward.z * traction * delta_time;
            
            // Apply handbrake effect
            if (v->handbrake) {
                v->velocity.x *= 0.9f;
                v->velocity.z *= 0.9f;
            }
            
            // Apply ground friction
            float friction_coeff = data->friction;
            if (v->handbrake) friction_coeff *= 2.0f;  // Extra friction when handbrake is on
            
            // Calculate slip factor based on steering and speed
            float slip_factor = fabsf(v->steering) * speed_factor;
            friction_coeff *= (1.0f - slip_factor * 0.5f);
            
            v->velocity.x *= (1.0f - (1.0f - friction_coeff) * delta_time * 5.0f);
            v->velocity.z *= (1.0f - (1.0f - friction_coeff) * delta_time * 5.0f);
            
            break;
        }
        
        case VEHICLE_MOTORCYCLE: {
            MotorcycleData* data = (MotorcycleData*)v->data;
            
            // Similar to car but with lean mechanics
            v->rotation.y += v->steering * delta_time * 3.0f;
            
            Vector3 forward = (Vector3){
                sinf(v->rotation.y),
                0,
                cosf(v->rotation.y)
            };
            
            float traction = v->throttle * data->acceleration;
            v->velocity.x += forward.x * traction * delta_time;
            v->velocity.z += forward.z * traction * delta_time;
            
            // Apply braking
            if (v->brake > 0) {
                float braking_force = v->brake * data->braking;
                float speed = sqrtf(v->velocity.x*v->velocity.x + v->velocity.z*v->velocity.z);
                if (speed > 0.1f) {
                    float brake_x = (v->velocity.x / speed) * braking_force * delta_time;
                    float brake_z = (v->velocity.z / speed) * braking_force * delta_time;
                    v->velocity.x -= brake_x;
                    v->velocity.z -= brake_z;
                    
                    // Prevent negative speed from braking
                    if ((v->velocity.x * forward.x + v->velocity.z * forward.z) < 0) {
                        v->velocity.x = 0;
                        v->velocity.z = 0;
                    }
                }
            }
            
            // Ground friction
            v->velocity.x *= 0.95f;
            v->velocity.z *= 0.95f;
            
            // Lean affects turning
            v->rotation.z = data->lean_angle;
            
            break;
        }
        
        case VEHICLE_AIRPLANE: {
            AirplaneData* data = (AirplaneData*)v->data;
            
            // Calculate lift based on speed and angle of attack
            float speed = sqrtf(v->velocity.x*v->velocity.x + v->velocity.y*v->velocity.y + v->velocity.z*v->velocity.z);
            float lift = fminf(data->max_lift, speed * 10.0f);  // Simplified lift formula
            
            // Apply lift opposite to gravity
            v->velocity.y += (lift / v->mass) * delta_time;
            
            // Apply thrust in forward direction
            Vector3 forward = (Vector3){
                sinf(v->rotation.y) * cosf(v->rotation.x),
                -sinf(v->rotation.x),
                cosf(v->rotation.y) * cosf(v->rotation.x)
            };
            
            float thrust = v->throttle * data->max_thrust / v->mass;
            v->velocity.x += forward.x * thrust * delta_time;
            v->velocity.y += forward.y * thrust * delta_time;
            v->velocity.z += forward.z * thrust * delta_time;
            
            // Apply drag
            float drag = data->drag_coefficient * speed * speed;
            v->velocity.x -= (v->velocity.x / speed) * drag * delta_time / v->mass;
            v->velocity.y -= (v->velocity.y / speed) * drag * delta_time / v->mass;
            v->velocity.z -= (v->velocity.z / speed) * drag * delta_time / v->mass;
            
            break;
        }
        
        case VEHICLE_HELICOPTER: {
            HelicopterData* data = (HelicopterData*)v->data;
            
            // Apply collective thrust upward
            float lift = v->throttle * data->max_lift / v->mass;
            v->velocity.y += lift * delta_time;
            
            // Apply cyclic control
            v->velocity.x += -sinf(v->rotation.z) * delta_time * 5.0f;  // Roll affects X
            v->velocity.z += -sinf(v->rotation.x) * delta_time * 5.0f;  // Pitch affects Z
            
            // Apply yaw control
            v->rotation.y += (v->rotation.y > 0 ? 1 : -1) * delta_time * 2.0f;  // Simplified
            
            // Apply drag
            v->velocity.x *= 0.98f;
            v->velocity.y *= 0.98f;
            v->velocity.z *= 0.98f;
            
            break;
        }
        
        case VEHICLE_TANK: {
            // Tank updates handled in control function due to differential steering
            break;
        }
    }
}

void draw_vehicle(Vehicle* v) {
    // Draw a simple representation based on vehicle type
    switch(v->type) {
        case VEHICLE_CAR:
            DrawCube(v->position, 2.0f, 0.8f, 4.0f, RED);
            DrawCube((Vector3){v->position.x, v->position.y - 0.4f, v->position.z + 1.5f}, 0.4f, 0.4f, 0.4f, BLACK);
            DrawCube((Vector3){v->position.x, v->position.y - 0.4f, v->position.z - 1.5f}, 0.4f, 0.4f, 0.4f, BLACK);
            DrawCube((Vector3){v->position.x + 0.8f, v->position.y - 0.4f, v->position.z + 1.5f}, 0.4f, 0.4f, 0.4f, BLACK);
            DrawCube((Vector3){v->position.x + 0.8f, v->position.y - 0.4f, v->position.z - 1.5f}, 0.4f, 0.4f, 0.4f, BLACK);
            break;
            
        case VEHICLE_TRUCK:
            DrawCube(v->position, 2.5f, 1.2f, 6.0f, ORANGE);
            DrawCube((Vector3){v->position.x, v->position.y - 0.6f, v->position.z + 2.0f}, 0.5f, 0.5f, 0.5f, BLACK);
            DrawCube((Vector3){v->position.x, v->position.y - 0.6f, v->position.z - 2.0f}, 0.5f, 0.5f, 0.5f, BLACK);
            DrawCube((Vector3){v->position.x + 1.0f, v->position.y - 0.6f, v->position.z + 2.0f}, 0.5f, 0.5f, 0.5f, BLACK);
            DrawCube((Vector3){v->position.x + 1.0f, v->position.y - 0.6f, v->position.z - 2.0f}, 0.5f, 0.5f, 0.5f, BLACK);
            DrawCube((Vector3){v->position.x + 1.8f, v->position.y - 0.6f, v->position.z + 2.0f}, 0.5f, 0.5f, 0.5f, BLACK);
            DrawCube((Vector3){v->position.x + 1.8f, v->position.y - 0.6f, v->position.z - 2.0f}, 0.5f, 0.5f, 0.5f, BLACK);
            break;
            
        case VEHICLE_MOTORCYCLE:
            DrawCube(v->position, 0.3f, 0.6f, 1.5f, PURPLE);
            DrawSphere((Vector3){v->position.x, v->position.y - 0.4f, v->position.z + 0.6f}, 0.25f, BLACK);
            DrawSphere((Vector3){v->position.x, v->position.y - 0.4f, v->position.z - 0.6f}, 0.25f, BLACK);
            break;
            
        case VEHICLE_AIRPLANE:
            DrawCube(v->position, 1.0f, 0.5f, 4.0f, BLUE);
            DrawCube((Vector3){v->position.x, v->position.y, v->position.z - 2.0f}, 4.0f, 0.2f, 1.0f, BLUE);
            DrawCube((Vector3){v->position.x, v->position.y + 1.0f, v->position.z}, 0.2f, 1.0f, 3.0f, BLUE);
            break;
            
        case VEHICLE_HELICOPTER:
            DrawCube(v->position, 1.5f, 1.0f, 2.0f, YELLOW);
            DrawCube((Vector3){v->position.x, v->position.y + 1.0f, v->position.z}, 5.0f, 0.1f, 0.2f, GRAY);
            DrawCube((Vector3){v->position.x + 2.0f, v->position.y + 0.5f, v->position.z}, 0.1f, 0.1f, 1.0f, GRAY);
            break;
            
        case VEHICLE_TANK:
            DrawCube(v->position, 3.0f, 1.0f, 5.0f, BROWN);
            DrawCube((Vector3){v->position.x, v->position.y - 0.5f, v->position.z}, 3.5f, 0.3f, 5.5f, DARKBROWN);
            DrawCube((Vector3){v->position.x, v->position.y - 0.7f, v->position.z + 1.5f}, 0.4f, 0.4f, 0.4f, BLACK);
            DrawCube((Vector3){v->position.x, v->position.y - 0.7f, v->position.z - 1.5f}, 0.4f, 0.4f, 0.4f, BLACK);
            DrawCube((Vector3){v->position.x + 1.0f, v->position.y - 0.7f, v->position.z + 1.5f}, 0.4f, 0.4f, 0.4f, BLACK);
            DrawCube((Vector3){v->position.x + 1.0f, v->position.y - 0.7f, v->position.z - 1.5f}, 0.4f, 0.4f, 0.4f, BLACK);
            DrawCube((Vector3){v->position.x - 1.0f, v->position.y - 0.7f, v->position.z + 1.5f}, 0.4f, 0.4f, 0.4f, BLACK);
            DrawCube((Vector3){v->position.x - 1.0f, v->position.y - 0.7f, v->position.z - 1.5f}, 0.4f, 0.4f, 0.4f, BLACK);
            break;
    }
}