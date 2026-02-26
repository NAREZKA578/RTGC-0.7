#pragma once
#include "GameObject.hpp"
#include <PhysX/PxRigidDynamic.h>
#include "../math/Mass.hpp"
#include "../physics/PhysXInitializer.hpp"

class PhysicsObject : public GameObject {
    physx::PxRigidDynamic* rigidBody;
    Mass mass;
    physx::PxMaterial* material;

public:
    PhysicsObject(const std::string& name, const Vector3& pos, float m, physx::PxMaterial* mat);
    ~PhysicsObject() override;

    physx::PxRigidDynamic* GetRigidBody() const { return rigidBody; }
    void SetMass(float m) { mass = Mass(m); }
    Mass GetMass() const { return mass; }

    void Update(float dt) override;
    void ApplyForce(const Vector3& force);
    void ApplyImpulse(const Vector3& impulse);
};