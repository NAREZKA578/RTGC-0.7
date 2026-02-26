#include "PhysicsWorld.hpp"
#include "../core/Logger.hpp"
#include <algorithm>

PhysicsBody::PhysicsBody(const Vector3& pos, float m, bool staticBody) 
    : position(pos), mass(m), isStatic(staticBody) {
    velocity = Vector3(0, 0, 0);
    acceleration = Vector3(0, 0, 0);
}

void PhysicsBody::Update(float dt) {
    if (isStatic) return;
    
    // F = ma, a = F/m
    velocity += acceleration * dt;
    position += velocity * dt;
    
    // Сбрасываем ускорение для следующего кадра
    acceleration = Vector3(0, 0, 0);
}

void PhysicsBody::ApplyForce(const Vector3& force) {
    if (isStatic || mass <= 0.0f) return;
    acceleration += force / mass;
}

void PhysicsBody::ApplyImpulse(const Vector3& impulse) {
    if (isStatic || mass <= 0.0f) return;
    velocity += impulse / mass;
}

float PhysicsBody::GetKineticEnergy() const {
    return 0.5f * mass * velocity.LengthSquared();
}

Vector3 PhysicsBody::GetMomentum() const {
    return velocity * mass;
}

PhysicsWorld::PhysicsWorld(const Vector3& grav) : gravity(grav), enabled(true) {
    Logger::Log("PhysicsWorld создан с гравитацией: (", gravity.x, ", ", gravity.y, ", ", gravity.z, ")");
}

void PhysicsWorld::Update(float dt) {
    if (!enabled) return;
    
    for (auto& body : bodies) {
        if (body && !body->IsStatic()) {
            // Применяем гравитацию
            body->ApplyForce(gravity * body->GetMass());
            body->Update(dt);
            
            // Простая проверка земли
            Vector3 pos = body->GetPosition();
            if (pos.y < 0.0f) {
                pos.y = 0.0f;
                body->SetPosition(pos);
                
                // Гасим вертикальную скорость при столкновении с землей
                Vector3 vel = body->GetVelocity();
                if (vel.y < 0) {
                    vel.y = -vel.y * 0.3f; // Отскок с затуханием
                    body->SetVelocity(vel);
                }
            }
        }
    }
}

PhysicsBody* PhysicsWorld::CreateBody(const Vector3& pos, float mass, bool isStatic) {
    auto body = std::make_unique<PhysicsBody>(pos, mass, isStatic);
    PhysicsBody* ptr = body.get();
    bodies.push_back(std::move(body));
    Logger::Log("PhysicsBody создан: масса=", mass, ", статический=", isStatic);
    return ptr;
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
    Logger::Log("PhysicsWorld очищен");
}