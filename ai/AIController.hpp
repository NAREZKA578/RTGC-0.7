#pragma once
#include "../physics/CharacterController.hpp"
#include <glm/glm.hpp>
#include <glm/gtc/quaternion.hpp> // Для glm::length

class AIController {
    CharacterController* character;
    glm::vec3 target;
public:
    AIController(CharacterController* ctrl);
    void Update(float dt);
    void SetTarget(const glm::vec3& t) { target = t; }
};