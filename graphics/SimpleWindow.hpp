#pragma once
#include <windows.h>

class SimpleWindow {
public:
    HWND hwnd;
    
    SimpleWindow() : hwnd(nullptr) {}
    ~SimpleWindow();
    
    bool Create(int width, int height, const char* title);
    void Destroy();
    void Show();
    void Hide();
    
    bool Poll();
    HDC GetDC();
    void ReleaseDC(HDC hdc);
    void SwapBuffers();
    bool ShouldClose();
    
    int GetWidth() { return 1280; }
    int GetHeight() { return 720; }
};