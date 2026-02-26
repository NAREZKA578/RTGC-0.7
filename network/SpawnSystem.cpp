#include "SpawnSystem.hpp"

SpawnSystem::SpawnSystem() {
    points = {{{0,10,0}}, {{100,10,0}}, {{-100,10,0}}, {{0,10,100}}};
}

glm::vec3 SpawnSystem::GetSafeSpawn() const {
    for (const auto& p : points) {
        if (!p.isUsed) return p.position;
    }
    return points[0].position; // fallback
}