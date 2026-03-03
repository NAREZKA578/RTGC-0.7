#include "PhysicsWorld.hpp"
#include "../core/Logger.hpp"
#include <algorithm>
#include <cmath>

PhysicsBody::PhysicsBody(const Vector3& pos, float m, bool staticBody)
    : position(pos), velocity(0, 0, 0), acceleration(0, 0, 0), force(0, 0, 0),
      mass(m), isStatic(staticBody), restitution(0.5f), friction(0.1f) {
    inv_mass = (mass > 0.0f) ? 1.0f / mass : 0.0f;
}

void PhysicsBody::Update(float dt) {
    if (isStatic) return;

    // Apply damping
    velocity *= 0.98f;

    // F = ma -> a = F/m
    acceleration = force * inv_mass;

    // Integrate
    velocity += acceleration * dt;
    position += velocity * dt;

    // Reset force for next frame
    force = Vector3(0, 0, 0);
}

void PhysicsBody::ApplyForce(const Vector3& f) {
    if (isStatic) return;
    force += f;
}

void PhysicsBody::ApplyImpulse(const Vector3& impulse) {
    if (isStatic) return;
    velocity += impulse * inv_mass;
}

PhysicsWorld::PhysicsWorld(const Vector3& grav) : gravity(grav), enabled(true) {
    Logger::Log("PhysicsWorld created with gravity: (", gravity.x, ", ", gravity.y, ", ", gravity.z, ")");
}

void PhysicsWorld::Update(float dt) {
    if (!enabled) return;

    // Update all bodies
    for (auto& body : bodies) {
        if (body && !body->IsStatic()) {
            // Apply gravity
            body->ApplyForce(gravity * body->GetMass());
            
            // Update physics
            body->Update(dt);
            
            // Simple ground collision
            if (body->GetPosition().y < 0.0f) {
                Vector3 pos = body->GetPosition();
                pos.y = 0.0f;
                body->SetPosition(pos);
                
                // Reflect velocity with damping
                Vector3 vel = body->GetVelocity();
                if (vel.y < 0) {
                    vel.y = -vel.y * (1.0f - body->restitution);
                    vel.x *= (1.0f - body->friction);
                    vel.z *= (1.0f - body->friction);
                    body->SetVelocity(vel);
                }
            }
        }
    }
    
    // Check collisions between bodies
    for (size_t i = 0; i < bodies.size(); ++i) {
        for (size_t j = i + 1; j < bodies.size(); ++j) {
            if (bodies[i] && bodies[j]) {
                CheckCollision(bodies[i].get(), bodies[j].get(), dt);
            }
        }
    }
}

void PhysicsWorld::CheckCollision(PhysicsBody* a, PhysicsBody* b, float dt) {
    if (a->IsStatic() && b->IsStatic()) return;
    
    Vector3 diff = b->GetPosition() - a->GetPosition();
    float dist_sq = diff.LengthSquared();
    float min_dist = 1.0f; // Assume radius of 1 for both bodies
    
    if (dist_sq < min_dist * min_dist) {
        float dist = sqrtf(dist_sq);
        if (dist == 0) return; // Bodies at same position
        
        Vector3 normal = diff / dist;
        
        // Penetration correction
        float penetration = min_dist - dist;
        Vector3 correction = normal * (penetration * 0.5f);
        
        if (!a->IsStatic()) {
            a->SetPosition(a->GetPosition() - correction);
        }
        if (!b->IsStatic()) {
            b->SetPosition(b->GetPosition() + correction);
        }
        
        // Collision response
        Vector3 rel_vel = b->GetVelocity() - a->GetVelocity();
        float vel_along_normal = rel_vel.Dot(normal);
        
        if (vel_along_normal > 0) return; // Already moving apart
        
        float e = std::min(a->restitution, b->restitution);
        float j = -(1 + e) * vel_along_normal;
        j /= a->GetInvMass() + b->GetInvMass();
        
        Vector3 impulse = normal * j;
        
        if (!a->IsStatic()) {
            a->ApplyImpulse(-impulse);
        }
        if (!b->IsStatic()) {
            b->ApplyImpulse(impulse);
        }
    }
}

PhysicsBody* PhysicsWorld::CreateBody(const Vector3& pos, float mass, bool isStatic) {
    auto body = std::make_unique<PhysicsBody>(pos, mass, isStatic);
    PhysicsBody* ptr = body.get();
    bodies.push_back(std::move(body));
    Logger::Log("PhysicsBody created: mass=", mass, ", static=", isStatic);
    return ptr;
}

void PhysicsWorld::AddBody(std::unique_ptr<PhysicsBody> body) {
    bodies.push_back(std::move(body));
}

void PhysicsWorld::DestroyBody(PhysicsBody* body) {
    bodies.erase(
        std::remove_if(bodies.begin(), bodies.end(),
            [body](const std::unique_ptr<PhysicsBody>& ptr) { return ptr.get() == body; }
        ),
        bodies.end()
    );
}

void PhysicsWorld::Clear() {
    bodies.clear();
    Logger::Log("PhysicsWorld cleared");
}