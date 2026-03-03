#pragma once

class InputManager {
private:
    static bool keys[256];
    static bool prevKeys[256];
    
public:
    static void Init();
    static void Update();
    static bool IsKeyDown(int key);
    static bool IsKeyPressed(int key);
    static bool IsKeyReleased(int key);
};