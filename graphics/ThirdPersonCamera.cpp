#include "ThirdPersonCamera.hpp"
#include <glm/gtc/matrix_transform.hpp>

ThirdPersonCamera::ThirdPersonCamera(glm::vec3 offset) : Camera(), distance(offset.z), height(offset.y) {}

void ThirdPersonCamera::Follow(const glm::vec3& target, const glm::quat& rotation, float dt) {
    glm::vec3 targetPos = target + rotation * glm::vec3(0, height, distance); // Отступ за объектом
    Position = glm::lerp(Position, targetPos, dt * 5.0f); // Плавное движение
    updateCameraVectors();
}