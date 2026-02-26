#pragma once
#ifndef WIN32_LEAN_AND_MEAN
#define WIN32_LEAN_AND_MEAN
#endif
#ifndef NOMINMAX
#define NOMINMAX
#endif
#include <string>
#include <windows.h>

class Window {
public:
    enum class VSyncMode {
        OFF,
        ON,
        ADAPTIVE
    };
    Window(int width = 1280, int height = 720, const std::string& title = "RTGC - Siberian Cities");
    ~Window();

    bool Initialize();
    void Shutdown();

    bool ShouldClose() const { return m_shouldClose; }
    void PollEvents();
    void SwapBuffers();

    void SetSize(int width, int height);
    void GetSize(int& width, int& height) const;

    // Input handling
    bool IsKeyPressed(int key) const;
    void UpdateInput();

    HWND GetHWND() const { return m_hwnd; }
    HDC GetHDC() const { return m_hdc; }
    bool IsInitialized() const { return m_initialized; }

    void SetVSync(VSyncMode mode);
    void EnableAntiAliasing(int samples);
    std::string GetGPUInfo() const;

    HGLRC GetHGLRC() const { return m_hglrc; }

private:
    static LRESULT CALLBACK WindowProc(HWND hwnd, UINT uMsg, WPARAM wParam, LPARAM lParam);
    bool SetupPixelFormat();
    bool CreateOpenGLContext();

    HWND m_hwnd;
    HDC m_hdc;
    HGLRC m_hglrc;
    int m_width;
    int m_height;
    std::string m_title;
    bool m_initialized;
    bool m_shouldClose;
    VSyncMode m_vsyncMode;
    int m_msaaSamples;
    bool m_useHighPerformanceGPU;

    bool m_keys[256] = {false};
    bool m_keysPressed[256] = {false};
};
