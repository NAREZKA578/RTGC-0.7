#pragma once
#include "../math/Vector3.hpp"
#include <vector>
#include <memory>

class PhysicsBody {
private:
    Vector3 position;
    Vector3 velocity;
    Vector3 acceleration;
    Vector3 force;
    float mass;
    float inv_mass;
    bool isStatic;
    float restitution;
    float friction;

public:
    PhysicsBody(const Vector3& pos = Vector3(0,0,0), float m = 1.0f, bool staticBody = false);

    void Update(float dt);
    void ApplyForce(const Vector3& f);
    void ApplyImpulse(const Vector3& impulse);
    
    // Getters/Setters
    Vector3 GetPosition() const { return position; }
    void SetPosition(const Vector3& pos) { position = pos; }
    Vector3 GetVelocity() const { return velocity; }
    void SetVelocity(const Vector3& vel) { velocity = vel; }
    float GetMass() const { return mass; }
    float GetInvMass() const { return inv_mass; }
    bool IsStatic() const { return isStatic; }
    void SetStatic(bool s) { isStatic = s; }
    
    void SetRestitution(float rest) { restitution = rest; }
    void SetFriction(float fric) { friction = fric; }
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
    void AddBody(std::unique_ptr<PhysicsBody> body);
    
    void SetGravity(const Vector3& grav) { gravity = grav; }
    Vector3 GetGravity() const { return gravity; }
    
    void Clear();
    size_t GetBodyCount() const { return bodies.size(); }
    
    const std::vector<std::unique_ptr<PhysicsBody>>& GetBodies() const { return bodies; }
};