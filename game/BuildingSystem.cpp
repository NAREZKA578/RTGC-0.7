#include "BuildingSystem.hpp"
#include "../physics/PhysXInitializer.hpp"

void BuildingSystem::Place(const glm::vec3& pos, const std::string& type, uint32_t owner) {
    objects.push_back({pos, type, owner});
    auto* body = PhysXInitializer::gPhysics->createRigidDynamic(physx::PxTransform(physx::PxVec3(pos.x, pos.y, pos.z)));
    physx::PxShape* shape = PhysXInitializer::gPhysics->createShape(physx::PxBoxGeometry(2,2,2), *PhysXInitializer::gMaterial);
    body->attachShape(*shape);
    shape->release();
    PhysXInitializer::gScene->addActor(*body);
    physicsBodies.push_back(body);
    LOG(L"Объект построен и добавлен в физику");
}

void BuildingSystem::InteractWith(Buildable& obj) {
    if (obj.type == "container") {
        LOG(L"Открыт контейнер");
    }
}

void BuildingSystem::Clear() {
    for (auto* body : physicsBodies) {
        PhysXInitializer::gScene->removeActor(*body);
        body->release();
    }
    physicsBodies.clear();
    objects.clear();
}

const std::vector<Buildable>& BuildingSystem::GetObjects() const { return objects; }