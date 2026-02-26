#pragma once
#include "../physics/CharacterController.hpp"
#include <glm/glm.hpp>
#include <glm/gtc/quaternion.hpp> // Для glm::length

class AIController {
    CharacterController* character;
    glm::vec3 target;
public:
    void Update(float dt);
};