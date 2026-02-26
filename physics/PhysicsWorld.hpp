#pragma once
#include "../math/Vector3.hpp"
#include <vector>
#include <memory>

class PhysicsBody {
private:
    Vector3 position;
    Vector3 velocity;
    Vector3 acceleration;
    float mass;
    bool isStatic;
    
public:
    PhysicsBody(const Vector3& pos = Vector3(0,0,0), float m = 1.0f, bool staticBody = false);
    
    void Update(float dt);
    void ApplyForce(const Vector3& force);
    void ApplyImpulse(const Vector3& impulse);
    
    // Getters/Setters
    Vector3 GetPosition() const { return position; }
    void SetPosition(const Vector3& pos) { position = pos; }
    Vector3 GetVelocity() const { return velocity; }
    void SetVelocity(const Vector3& vel) { velocity = vel; }
    float GetMass() const { return mass; }
    bool IsStatic() const { return isStatic; }
    
    // Physics queries
    float GetKineticEnergy() const;
    Vector3 GetMomentum() const;
};

class PhysicsWorld {
private:
    std::vector<std::unique_ptr<PhysicsBody>> bodies;
    Vector3 gravity;
    bool enabled;
    
public:
    PhysicsWorld(const Vector3& grav = Vector3(0, -9.81f, 0));
    
    void Update(float dt);
    PhysicsBody* CreateBody(const Vector3& pos, float mass = 1.0f, bool isStatic = false);
    void DestroyBody(PhysicsBody* body);
    
    void SetGravity(const Vector3& grav) { gravity = grav; }
    Vector3 GetGravity() const { return gravity; }
    
    void Clear();
    size_t GetBodyCount() const { return bodies.size(); }
};