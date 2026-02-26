#include "InputManager.hpp"

std::unordered_map<int, bool> InputManager::keys;
std::unordered_map<std::string, int> InputManager::keyBindings;
std::unordered_map<int, bool> InputManager::keysPrevious;

void InputManager::Init() {
    // Простые привязки по умолчанию
    keyBindings["forward"] = 87;    // W
    keyBindings["backward"] = 83;   // S
    keyBindings["left"] = 65;       // A
    keyBindings["right"] = 68;      // D
    keyBindings["jump"] = 32;       // Space
    keyBindings["exit"] = 27;       // Escape
}

void InputManager::Update() {
    keysPrevious = keys;
}

void InputManager::BindKey(const std::string& action, int key) {
    keyBindings[action] = key;
}

bool InputManager::IsKeyDown(int key) {
    auto it = keys.find(key);
    return it != keys.end() && it->second;
}

bool InputManager::IsKeyDown(const std::string& action) {
    auto it = keyBindings.find(action);
    if (it != keyBindings.end()) {
        return IsKeyDown(it->second);
    }
    return false;
}

bool InputManager::IsKeyPressed(int key) {
    auto it = keys.find(key);
    auto prevIt = keysPrevious.find(key);
    return (it != keys.end() && it->second) && 
           (prevIt == keysPrevious.end() || !prevIt->second);
}

bool InputManager::IsKeyPressed(const std::string& action) {
    auto it = keyBindings.find(action);
    if (it != keyBindings.end()) {
        return IsKeyPressed(it->second);
    }
    return false;
}

void InputManager::SetKeyState(int key, bool pressed) {
    keys[key] = pressed;
}
