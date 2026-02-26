#pragma once
#include <unordered_map>
#include <string>

class InputManager {
private:
    static std::unordered_map<int, bool> keys;
    static std::unordered_map<std::string, int> keyBindings;
    static std::unordered_map<int, bool> keysPrevious;
    
public:
    static void Init();
    static void Update();
    static void BindKey(const std::string& action, int key);
    static bool IsKeyDown(int key);
    static bool IsKeyDown(const std::string& action);
    static bool IsKeyPressed(int key);
    static bool IsKeyPressed(const std::string& action);
    static void SetKeyState(int key, bool pressed);
};
