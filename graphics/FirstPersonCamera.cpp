#include "FirstPersonCamera.hpp"

FirstPersonCamera::FirstPersonCamera(glm::vec3 position) : Camera(position) {}

void FirstPersonCamera::Update(float dt) {
    // Обновление позиции/вращения в зависимости от ввода
    // Реализация зависит от InputManager
}