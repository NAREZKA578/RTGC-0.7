#pragma once
#include <glm/glm.hpp>
#include <vector>
#include <string>
#include <PhysX/PxPhysicsAPI.h>
#include "../core/Logger.hpp"
using namespace physx;

struct Buildable {
    glm::vec3 position;
    std::string type;
    uint32_t owner = 0;
};

class BuildingSystem {
    std::vector<Buildable> objects;
    std::vector<PxRigidDynamic*> physicsBodies;
public:
    void Place(const glm::vec3& pos, const std::string& type, uint32_t owner = 0);
    void InteractWith(Buildable& obj);
    void Clear();
    const std::vector<Buildable>& GetObjects() const;
};