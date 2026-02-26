#pragma once
#include <windows.h>

class SimpleWindow {
public:
    HWND hwnd;
    int width;
    int height;
    
    SimpleWindow() : hwnd(nullptr), width(1280), height(720) {}
    
    bool Create(const wchar_t* title) {
        HINSTANCE hInstance = GetModuleHandle(nullptr);
        
        WNDCLASSW wc = {};
        wc.lpfnWndProc = [](HWND h, UINT u, WPARAM w, LPARAM l) -> LRESULT {
            return DefWindowProc(h, u, w, l);
        };
        wc.hInstance = hInstance;
        wc.hCursor = LoadCursor(nullptr, IDC_ARROW);
        wc.lpszClassName = L"SimpleWindow";
        
        RegisterClass(&wc);
        
        hwnd = CreateWindowEx(0, L"SimpleWindow", title,
            WS_OVERLAPPEDWINDOW | WS_CAPTIONBAR | WS_SYSMENU,
            CW_USEDEFAULT, CW_USEDEFAULT,
            width, height, nullptr, nullptr, hInstance, nullptr);
            
        if (!hwnd) return false;
        
        ShowWindow(hwnd, SW_SHOW);
        return true;
    }
    
    void Destroy() {
        if (hwnd) {
            DestroyWindow(hwnd);
            hwnd = nullptr;
        }
    }
    
    bool ShouldClose() const {
        return !hwnd;
    }
    
    void SwapBuffers() {}
    void PollEvents() {}
    
    int GetWidth() const { return width; }
    int GetHeight() const { return height; }
    HWND GetHandle() const { return hwnd; }
};