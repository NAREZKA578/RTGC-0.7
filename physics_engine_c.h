#ifndef PHYSICS_ENGINE_C_H
#define PHYSICS_ENGINE_C_H

#include "raylib.h"

#define MAX_PARTICLES 10000
#define GRAVITY -9.81f

typedef struct {
    Vector3 position;
    Vector3 velocity;
    Vector3 acceleration;
    Vector3 force;
    float mass;
    float inv_mass;  // 1/mass, 0 for static objects
    float radius;
    bool active;
    bool is_static;
} Particle;

// Physics engine functions
void init_physics_engine();
void update_physics_engine(Particle* particles, int count);
void set_gravity_enabled(bool enabled);
void apply_force(Particle* p, Vector3 force);
void add_impulse(Particle* p, Vector3 impulse);
void init_particle(Particle* p, Vector3 pos, float r);

#endif // PHYSICS_ENGINE_C_H