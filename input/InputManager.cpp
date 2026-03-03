#include "InputManager.hpp"

bool InputManager::keys[256] = {false};
bool InputManager::prevKeys[256] = {false};

void InputManager::Init() {
    // Initialization code if needed
}

void InputManager::Update() {
    // Copy current state to previous state
    for (int i = 0; i < 256; ++i) {
        prevKeys[i] = keys[i];
    }
}

bool InputManager::IsKeyDown(int key) {
    if (key < 0 || key > 255) return false;
    return keys[key];
}

bool InputManager::IsKeyPressed(int key) {
    if (key < 0 || key > 255) return false;
    return keys[key] && !prevKeys[key];
}

bool InputManager::IsKeyReleased(int key) {
    if (key < 0 || key > 255) return false;
    return !keys[key] && prevKeys[key];
}