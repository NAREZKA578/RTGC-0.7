#pragma once
#include <glm/glm.hpp>
#include <array>

struct PlayerState {
    uint32_t playerId = 0;
    glm::vec3 position{0,0,0};
    glm::vec3 rotation{0,0,0};
    float health = 100.0f;
    bool isValid = false;
    uint32_t teamId = 0;
};