#include "CharacterController.hpp"
#include "../core/Logger.hpp"
#include <iostream>

CharacterController::CharacterController()
    : position(0.0f, 10.0f, 0.0f), velocity(0.0f, 0.0f, 0.0f), onGround(true)
{
}

CharacterController::~CharacterController() {
}

void CharacterController::Initialize() {
    std::cout << "CharacterController initialized" << std::endl;
}

void CharacterController::Update(float dt) {
    // Simple gravity
    if (!onGround) {
        velocity.y -= 9.81f * dt;
        position.y += velocity.y * dt;

        // Check ground collision
        if (position.y <= 0.0f) {
            position.y = 0.0f;
            velocity.y = 0.0f;
            onGround = true;
        }
    }
}

void CharacterController::Shutdown() {
    std::cout << "CharacterController shutdown" << std::endl;
}

void CharacterController::Move(const Vector3& direction, float dt) {
    if (dt <= 0.0f) return;

    // Simple movement without physics
    velocity.x = direction.x * 5.0f;
    velocity.z = direction.z * 5.0f;

    position.x += velocity.x * dt;
    position.z += velocity.z * dt;
}

void CharacterController::Jump() {
    if (onGround) {
        velocity.y = 8.0f; // Jump impulse
        onGround = false;
    }
}

Vector3 CharacterController::GetPosition() const {
    return position;
}

void CharacterController::SetPosition(const Vector3& pos) {
    position = pos;
    velocity = Vector3(0.0f, 0.0f, 0.0f);
    onGround = (position.y <= 0.01f);
}