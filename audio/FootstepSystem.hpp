#pragma once
#include "AudioSystem.hpp"
#include <glm/glm.hpp>

class FootstepSystem {
    AudioSystem* audioSystem = nullptr;
    glm::vec3 lastPosition{0,0,0};
    float stepDistance = 0.5f;
    float stepTimer = 0.0f;

public:
    FootstepSystem(AudioSystem* sys);
    void Update(float dt, const glm::vec3& playerPosition);
};