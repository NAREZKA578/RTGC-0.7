#pragma once
#include "Camera.hpp"
#include <glm/gtc/quaternion.hpp>

class ThirdPersonCamera : public Camera {
    float distance = 10.0f;
    float height = 5.0f;
public:
    ThirdPersonCamera(glm::vec3 offset = glm::vec3(0, 5, -10));
    void Follow(const glm::vec3& target, const glm::quat& rotation, float dt);
};