#include "Window.hpp"
#include "GPUManager.hpp"
#include "../core/Logger.hpp"
#include <GL/gl.h>
#include <cstring>

typedef BOOL (WINAPI *PFNWGLSWAPINTERVALEXTPROC)(int interval);
typedef BOOL (WINAPI *PFNWGLCHOOSEPIXELFORMATARBPROC)(HDC hdc, const int* piAttribIList, const FLOAT* pfAttribFList, UINT nMaxFormats, int* piFormats, UINT* nNumFormats);

Window::Window(int width, int height, const std::string& title)
    : m_hwnd(nullptr)
    , m_hdc(nullptr)
    , m_hglrc(nullptr)
    , m_width(width)
    , m_height(height)
    , m_title(title)
    , m_initialized(false)
    , m_shouldClose(false)
    , m_vsyncMode(VSyncMode::ON)
    , m_msaaSamples(4)
    , m_useHighPerformanceGPU(true)
{
    memset(m_keys, 0, sizeof(m_keys));
    memset(m_keysPressed, 0, sizeof(m_keysPressed));

    GPUManager::InitializeGPUSelection();
    GPUManager::LogGPUInfo();
}

Window::~Window() {
    Shutdown();
}

LRESULT CALLBACK Window::WindowProc(HWND hwnd, UINT uMsg, WPARAM wParam, LPARAM lParam) {
    Window* window = nullptr;

    if (uMsg == WM_CREATE) {
        CREATESTRUCT* pCreate = (CREATESTRUCT*)lParam;
        window = (Window*)pCreate->lpCreateParams;
        SetWindowLongPtr(hwnd, GWLP_USERDATA, (LONG_PTR)window);
    } else {
        window = (Window*)GetWindowLongPtr(hwnd, GWLP_USERDATA);
    }

    switch (uMsg) {
        case WM_CLOSE:
            if (window) {
                Logger::Log("Window close requested by user");
                window->m_shouldClose = true;
            }
            return 0;
        case WM_DESTROY:
            PostQuitMessage(0);
            return 0;
        case WM_SIZE:
            if (window) {
                window->m_width = LOWORD(lParam);
                window->m_height = HIWORD(lParam);
            }
            return 0;
        case WM_KEYDOWN:
            if (window && wParam < 256) {
                window->m_keys[wParam] = true;
                window->m_keysPressed[wParam] = true;
            }
            return 0;
        case WM_KEYUP:
            if (window && wParam < 256) {
                window->m_keys[wParam] = false;
            }
            return 0;
        default:
            return DefWindowProc(hwnd, uMsg, wParam, lParam);
    }
}

bool Window::SetupPixelFormat() {
    PIXELFORMATDESCRIPTOR pfd = {
        sizeof(PIXELFORMATDESCRIPTOR),
        1,
        PFD_DRAW_TO_WINDOW | PFD_SUPPORT_OPENGL | PFD_DOUBLEBUFFER | PFD_STEREO_DONTCARE | PFD_GENERIC_ACCELERATED,
        PFD_TYPE_RGBA,
        32,
        8, 0, 8, 0, 8, 0,
        8,
        0,
        0, 0, 0, 0, 0,
        24,
        8,
        0,
        PFD_MAIN_PLANE,
        0,
        0, 0, 0
    };

    int pixelFormat = ChoosePixelFormat(m_hdc, &pfd);
    if (pixelFormat == 0) return false;

    if (!DescribePixelFormat(m_hdc, pixelFormat, sizeof(pfd), &pfd)) return false;

    if (!SetPixelFormat(m_hdc, pixelFormat, &pfd)) return false;

    return true;
}

bool Window::CreateOpenGLContext() {
    m_hglrc = wglCreateContext(m_hdc);
    if (!m_hglrc) return false;

    if (!wglMakeCurrent(m_hdc, m_hglrc)) {
        wglDeleteContext(m_hglrc);
        m_hglrc = nullptr;
        return false;
    }

    // Print OpenGL version info
    const char* version = (const char*)glGetString(GL_VERSION);
    const char* vendor = (const char*)glGetString(GL_VENDOR);
    const char* renderer = (const char*)glGetString(GL_RENDERER);

    // Log GPU information
    if (version && vendor && renderer) {
        Logger::Log("GPU Information:");
        Logger::Log("  Vendor: ", vendor);
        Logger::Log("  Renderer: ", renderer);
        Logger::Log("  OpenGL Version: ", version);
    }

    // Enable VSync
    PFNWGLSWAPINTERVALEXTPROC wglSwapIntervalEXT = (PFNWGLSWAPINTERVALEXTPROC)wglGetProcAddress("wglSwapIntervalEXT");
    if (wglSwapIntervalEXT) {
        int interval = (m_vsyncMode == VSyncMode::ON) ? 1 : ((m_vsyncMode == VSyncMode::ADAPTIVE) ? -1 : 0);
        wglSwapIntervalEXT(interval);
        Logger::Log("VSync enabled: ", (m_vsyncMode != VSyncMode::OFF));
    }

    return true;
}

bool Window::Initialize() {
    if (m_initialized) return true;

    // Register window class
    WNDCLASS wc = {};
    wc.lpfnWndProc = WindowProc;
    wc.hInstance = GetModuleHandle(nullptr);
    wc.lpszClassName = "RTGCWindowClass";
    wc.style = CS_HREDRAW | CS_VREDRAW | CS_OWNDC;
    wc.hbrBackground = (HBRUSH)GetStockObject(BLACK_BRUSH);
    wc.hCursor = LoadCursor(nullptr, IDC_ARROW);
    wc.hIcon = LoadIcon(nullptr, IDI_APPLICATION);

    if (!RegisterClass(&wc)) {
        return false;
    }

    // Create window with better style
    RECT rc = { 0, 0, m_width, m_height };
    DWORD style = WS_OVERLAPPEDWINDOW;

    AdjustWindowRect(&rc, style, FALSE);

    int x = (GetSystemMetrics(SM_CXSCREEN) - (rc.right - rc.left)) / 2;
    int y = (GetSystemMetrics(SM_CYSCREEN) - (rc.bottom - rc.top)) / 2;

    m_hwnd = CreateWindowExA(
        WS_EX_APPWINDOW,
        "RTGCWindowClass",
        m_title.c_str(),
        style,
        x, y,
        rc.right - rc.left, rc.bottom - rc.top,
        nullptr,
        nullptr,
        GetModuleHandle(nullptr),
        this
    );

    if (!m_hwnd) {
        return false;
    }

    // Get device context
    m_hdc = GetDC(m_hwnd);
    if (!m_hdc) {
        return false;
    }

    // Setup pixel format and OpenGL context
    if (!SetupPixelFormat() || !CreateOpenGLContext()) {
        return false;
    }

    // Show window
    ShowWindow(m_hwnd, SW_SHOW);
    UpdateWindow(m_hwnd);

    m_initialized = true;
    Logger::Log("Window initialized successfully");
    return true;
}

void Window::Shutdown() {
    if (!m_initialized) return;

    if (m_hglrc) {
        wglMakeCurrent(nullptr, nullptr);
        wglDeleteContext(m_hglrc);
        m_hglrc = nullptr;
    }

    if (m_hdc && m_hwnd) {
        ReleaseDC(m_hwnd, m_hdc);
        m_hdc = nullptr;
    }

    if (m_hwnd) {
        DestroyWindow(m_hwnd);
        m_hwnd = nullptr;
    }

    UnregisterClass("RTGCWindowClass", GetModuleHandle(nullptr));
    m_initialized = false;
}

void Window::PollEvents() {
    MSG msg;
    while (PeekMessage(&msg, nullptr, 0, 0, PM_REMOVE)) {
        if (msg.message == WM_QUIT) {
            m_shouldClose = true;
        }
        TranslateMessage(&msg);
        DispatchMessage(&msg);
    }
}

void Window::SwapBuffers() {
    if (m_hdc) {
        ::SwapBuffers(m_hdc);
    }
}

void Window::SetSize(int width, int height) {
    m_width = width;
    m_height = height;
    if (m_hwnd) {
        SetWindowPos(m_hwnd, nullptr, 0, 0, width, height, SWP_NOMOVE | SWP_NOZORDER);
    }
}

void Window::GetSize(int& width, int& height) const {
    width = m_width;
    height = m_height;
}

bool Window::IsKeyPressed(int key) const {
    if (key < 0 || key >= 256) return false;
    return m_keysPressed[key];
}

void Window::UpdateInput() {
    memcpy(m_keysPressed, m_keys, sizeof(m_keys));
}

void Window::SetVSync(VSyncMode mode) {
    m_vsyncMode = mode;

    PFNWGLSWAPINTERVALEXTPROC wglSwapIntervalEXT = (PFNWGLSWAPINTERVALEXTPROC)wglGetProcAddress("wglSwapIntervalEXT");
    if (wglSwapIntervalEXT && m_hglrc) {
        int interval = (mode == VSyncMode::ON) ? 1 : ((mode == VSyncMode::ADAPTIVE) ? -1 : 0);
        wglSwapIntervalEXT(interval);
        Logger::Log("VSync set to: ", (mode == VSyncMode::ON) ? "ON" : ((mode == VSyncMode::ADAPTIVE) ? "ADAPTIVE" : "OFF"));
    }
}

void Window::EnableAntiAliasing(int samples) {
    m_msaaSamples = samples;
    Logger::Log("Anti-aliasing enabled: ", samples, "x MSAA");
}

std::string Window::GetGPUInfo() const {
    const char* vendor = (const char*)glGetString(GL_VENDOR);
    const char* renderer = (const char*)glGetString(GL_RENDERER);
    const char* version = (const char*)glGetString(GL_VERSION);

    std::string info = "GPU: ";
    if (renderer) info += renderer;
    info += " | ";
    if (vendor) info += vendor;
    info += " | OpenGL: ";
    if (version) info += version;

    return info;
}