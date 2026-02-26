#include "PhysicsObject.hpp"
#include "../core/Logger.hpp"

PhysicsObject::PhysicsObject(const std::string& name, const Vector3& pos, float m, physx::PxMaterial* mat)
    : GameObject(name, pos), mass(m), material(mat), rigidBody(nullptr) {

    // Создание физического тела
    rigidBody = PhysXInitializer::gPhysics->createRigidDynamic(
        physx::PxTransform(pos.ToPxVec3())
    );

    if (rigidBody) {
        rigidBody->setMass(mass.GetKilograms());

        // Пример: создание геометрии (куб)
        physx::PxBoxGeometry geom(1.0f, 1.0f, 1.0f);
        physx::PxShape* shape = PhysXInitializer::gPhysics->createShape(geom, *material);
        rigidBody->attachShape(*shape);
        shape->release();

        PhysXInitializer::gScene->addActor(*rigidBody);
        LOG(L"Создан PhysicsObject: " + std::wstring(name.begin(), name.end()));
    } else {
        ERROR(L"Не удалось создать PhysicsObject: " + std::wstring(name.begin(), name.end()));
    }
}

void PhysicsObject::Update(float dt) {
    GameObject::Update(dt);

    if (rigidBody) {
        // Синхронизация позиции из PhysX в GameObject
        physx::PxTransform t = rigidBody->getGlobalPose();
        position = Vector3(t.p.x, t.p.y, t.p.z);
        rotation = glm::quat(t.q.w, t.q.x, t.q.y, t.q.z);
    }
}

void PhysicsObject::ApplyForce(const Vector3& force) {
    if (rigidBody) {
        rigidBody->addForce(force.ToPxVec3(), physx::PxForceMode::eFORCE);
    }
}

void PhysicsObject::ApplyImpulse(const Vector3& impulse) {
    if (rigidBody) {
        rigidBody->addForce(impulse.ToPxVec3(), physx::PxForceMode::eIMPULSE);
    }
}

PhysicsObject::~PhysicsObject() {
    if (rigidBody) {
        PhysXInitializer::gScene->removeActor(*rigidBody);
        rigidBody->release();
    }
}