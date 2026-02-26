#include "CharacterController.hpp"
#include "../core/Logger.hpp"

CharacterController::CharacterController() : position(0.0f, 10.0f, 0.0f), velocity(0.0f, 0.0f, 0.0f) {}

void CharacterController::Initialize() {
    Logger::Log("CharacterController инициализирован");
}

void CharacterController::Update(float dt) {
    position += velocity * dt;
    
    // Простая гравитация
    if (position.y > 0.0f) {
        velocity.y -= 9.81f * dt;
    } else {
        position.y = 0.0f;
        velocity.y = 0.0f;
    }
}

void CharacterController::Move(const Vector3& direction, float dt) {
    velocity.x = direction.x * speed;
    velocity.z = direction.z * speed;
}

void CharacterController::Shutdown() {
    Logger::Log("CharacterController выключен");
}

Vector3 CharacterController::GetPosition() const {
    return position;
}

void CharacterController::SetPosition(const Vector3& pos) {
    position = pos;
    velocity = Vector3(0.0f, 0.0f, 0.0f);
}
