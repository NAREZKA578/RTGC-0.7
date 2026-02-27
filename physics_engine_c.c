#include "physics_engine_c.h"
#include <math.h>
#include <stdbool.h>
#include <stdlib.h>

static bool gravity_enabled = true;

void init_physics_engine() {
    gravity_enabled = true;
}

void set_gravity_enabled(bool enabled) {
    gravity_enabled = enabled;
}

void init_particle(Particle* p, Vector3 pos, float r) {
    p->position = pos;
    p->velocity = (Vector3){0};
    p->acceleration = (Vector3){0};
    p->force = (Vector3){0};
    p->radius = r;
    p->mass = r * r * r * 4.0f * 3.14159f * 1000.0f / 3.0f;  // Volume * density
    p->inv_mass = (p->mass > 0.0f) ? 1.0f / p->mass : 0.0f;
    p->active = true;
    p->is_static = false;
}

void apply_force(Particle* p, Vector3 force) {
    if (!p->is_static) {
        p->force.x += force.x;
        p->force.y += force.y;
        p->force.z += force.z;
    }
}

void add_impulse(Particle* p, Vector3 impulse) {
    if (!p->is_static) {
        p->velocity.x += impulse.x * p->inv_mass;
        p->velocity.y += impulse.y * p->inv_mass;
        p->velocity.z += impulse.z * p->inv_mass;
    }
}

void update_physics_engine(Particle* particles, int count) {
    const float dt = 1.0f/60.0f;  // Fixed timestep
    const float sub_steps = 4.0f;
    const float sub_dt = dt/sub_steps;
    
    for (int s = 0; s < sub_steps; s++) {
        // Clear forces and apply gravity
        for (int i = 0; i < count; i++) {
            Particle* p = &particles[i];
            
            if (p->active && !p->is_static) {
                // Clear forces
                p->force = (Vector3){0, 0, 0};
                
                // Apply gravity if enabled
                if (gravity_enabled) {
                    p->force.y += p->mass * GRAVITY;
                }
                
                // Apply damping
                p->velocity.x *= 0.99f;
                p->velocity.y *= 0.99f;
                p->velocity.z *= 0.99f;
                
                // Integrate
                p->acceleration.x = p->force.x * p->inv_mass;
                p->acceleration.y = p->force.y * p->inv_mass;
                p->acceleration.z = p->force.z * p->inv_mass;
                
                p->velocity.x += p->acceleration.x * sub_dt;
                p->velocity.y += p->acceleration.y * sub_dt;
                p->velocity.z += p->acceleration.z * sub_dt;
                
                p->position.x += p->velocity.x * sub_dt;
                p->position.y += p->velocity.y * sub_dt;
                p->position.z += p->velocity.z * sub_dt;
                
                // Ground collision
                if (p->position.y - p->radius < -2.0f) {
                    p->position.y = -2.0f + p->radius;
                    
                    // Reflect velocity with energy loss
                    p->velocity.y = -p->velocity.y * 0.6f;  // Bounce with damping
                    
                    // Apply friction
                    p->velocity.x *= 0.8f;
                    p->velocity.z *= 0.8f;
                }
                
                // Boundary collision
                if (p->position.x - p->radius < -10.0f || p->position.x + p->radius > 10.0f) {
                    p->position.x = p->position.x < 0 ? -10.0f + p->radius : 10.0f - p->radius;
                    p->velocity.x = -p->velocity.x * 0.6f;
                }
                
                if (p->position.z - p->radius < -10.0f || p->position.z + p->radius > 10.0f) {
                    p->position.z = p->position.z < 0 ? -10.0f + p->radius : 10.0f - p->radius;
                    p->velocity.z = -p->velocity.z * 0.6f;
                }
            }
        }
        
        // Particle-particle collisions
        for (int i = 0; i < count; i++) {
            for (int j = i + 1; j < count; j++) {
                Particle* p1 = &particles[i];
                Particle* p2 = &particles[j];
                
                if (!p1->active || !p2->active) continue;
                if (p1->is_static && p2->is_static) continue;
                
                Vector3 diff = (Vector3){
                    p2->position.x - p1->position.x,
                    p2->position.y - p1->position.y,
                    p2->position.z - p1->position.z
                };
                
                float distance_sq = diff.x*diff.x + diff.y*diff.y + diff.z*diff.z;
                float min_dist = p1->radius + p2->radius;
                
                if (distance_sq < min_dist * min_dist) {
                    float distance = sqrtf(distance_sq);
                    
                    if (distance == 0) continue;  // Particles at same position
                    
                    // Normal vector
                    Vector3 normal = (Vector3){
                        diff.x/distance,
                        diff.y/distance,
                        diff.z/distance
                    };
                    
                    // Penetration depth
                    float penetration = min_dist - distance;
                    
                    // Separate particles
                    if (!p1->is_static && !p2->is_static) {
                        float total_inv_mass = p1->inv_mass + p2->inv_mass;
                        float ratio1 = p1->inv_mass / total_inv_mass;
                        float ratio2 = p2->inv_mass / total_inv_mass;
                        
                        p1->position.x -= normal.x * penetration * ratio2;
                        p1->position.y -= normal.y * penetration * ratio2;
                        p1->position.z -= normal.z * penetration * ratio2;
                        
                        p2->position.x += normal.x * penetration * ratio1;
                        p2->position.y += normal.y * penetration * ratio1;
                        p2->position.z += normal.z * penetration * ratio1;
                    } else if (p1->is_static) {
                        p2->position.x += normal.x * penetration;
                        p2->position.y += normal.y * penetration;
                        p2->position.z += normal.z * penetration;
                    } else {
                        p1->position.x -= normal.x * penetration;
                        p1->position.y -= normal.y * penetration;
                        p1->position.z -= normal.z * penetration;
                    }
                    
                    // Calculate relative velocity
                    Vector3 rel_velocity = (Vector3){
                        p2->velocity.x - p1->velocity.x,
                        p2->velocity.y - p1->velocity.y,
                        p2->velocity.z - p1->velocity.z
                    };
                    
                    // Velocity along normal
                    float vel_along_normal = rel_velocity.x*normal.x + rel_velocity.y*normal.y + rel_velocity.z*normal.z;
                    
                    // Don't resolve if velocities are separating
                    if (vel_along_normal > 0) continue;
                    
                    // Calculate impulse scalar
                    float restitution = 0.6f;  // Bounciness
                    float impulse_scalar = -(1 + restitution) * vel_along_normal;
                    impulse_scalar /= p1->inv_mass + p2->inv_mass;
                    
                    // Apply impulse
                    Vector3 impulse = (Vector3){
                        impulse_scalar * normal.x,
                        impulse_scalar * normal.y,
                        impulse_scalar * normal.z
                    };
                    
                    if (!p1->is_static) {
                        p1->velocity.x -= impulse.x * p1->inv_mass;
                        p1->velocity.y -= impulse.y * p1->inv_mass;
                        p1->velocity.z -= impulse.z * p1->inv_mass;
                    }
                    
                    if (!p2->is_static) {
                        p2->velocity.x += impulse.x * p2->inv_mass;
                        p2->velocity.y += impulse.y * p2->inv_mass;
                        p2->velocity.z += impulse.z * p2->inv_mass;
                    }
                }
            }
        }
    }
}