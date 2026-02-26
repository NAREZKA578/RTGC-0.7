#include <windows.h>

static HWND g_hwnd = nullptr;
static int g_width = 1280;
static int g_height = 720;
static bool g_running = false;
static int g_menuState = 0;
static int g_selectedCity = -1;

static LRESULT CALLBACK WindowProc(HWND hwnd, UINT uMsg, WPARAM wParam, LPARAM lParam) {
    if (uMsg == WM_DESTROY) {
        PostQuitMessage(0);
    }
    return DefWindowProc(hwnd, uMsg, wParam, lParam);
}

static void CreateMainWindow() {
    HINSTANCE hInstance = GetModuleHandle(nullptr);
    WNDCLASW wc = {};
    wc.lpfnWndProc = WindowProc;
    wc.hInstance = hInstance;
    wc.lpszClassName = "RTGCWindow";
    wc.hCursor = LoadCursor(nullptr, IDC_ARROW);
    wc.hbrBackground = (HBRUSH)(COLOR_WINDOW + 1);
    
    RegisterClassW(&wc);
    
    g_hwnd = CreateWindowEx(0, "RTGCWindow", "RTGC - Siberian Cities",
        WS_OVERLAPPEDWINDOW | WS_CAPTIONBAR | WS_SYSMENU,
        CW_USEDEFAULT, CW_USEDEFAULT,
        g_width, g_height,
        nullptr, nullptr, hInstance, nullptr);
    
    ShowWindow(g_hwnd, SW_SHOW);
}

static void DestroyMainWindow() {
    if (g_hwnd) {
        DestroyWindow(g_hwnd);
        g_hwnd = nullptr;
    }
}

static HDC GetDC() {
    return GetDC(g_hwnd);
}

static void ReleaseDC(HDC hdc) {
    ReleaseDC(g_hwnd, hdc);
}

static void SwapBuffers() {
}

static bool PollEvents() {
    MSG msg;
    while (PeekMessage(&msg, nullptr, 0, 0, PM_REMOVE)) {
        if (msg.message == WM_QUIT) {
            g_running = false;
            g_hwnd = nullptr;
        }
        TranslateMessage(&msg);
        DispatchMessage(&msg);
    }
}

static bool ShouldClose() {
    return !g_running;
}

static void Close() {
    PostMessage(g_hwnd, WM_CLOSE, 0, 0);
}

static void BeginFrame() {
    HDC hdc = GetDC();
    RECT rect;
    GetClientRect(g_hwnd, &rect);
    
    HBRUSH brush = CreateSolidBrush(RGB(20, 40, 60));
    FillRect(hdc, &rect, brush);
    DeleteObject(brush);
    
    ReleaseDC(hdc);
}

static void EndFrame() {
    SwapBuffers();
    PollEvents();
}

static void SetMenuState(int state) {
    g_menuState = state;
}

static int GetMenuState() {
    return g_menuState;
}

static void SetSelectedCity(int city) {
    g_selectedCity = city;
}

static int GetSelectedCity() {
    return g_selectedCity;
}