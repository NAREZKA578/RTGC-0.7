#pragma once
#include <glm/glm.hpp>
#include <array>

struct SpawnPoint {
    glm::vec3 position;
    bool isUsed = false;
};

class SpawnSystem {
    std::array<SpawnPoint, 4> points;
public:
    SpawnSystem();
    glm::vec3 GetSafeSpawn() const;
};