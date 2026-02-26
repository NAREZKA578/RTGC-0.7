#include <windows.h>

static HWND g_hwnd = nullptr;
static int g_width = 1280;
static int g_height = 720;
static bool g_running = false;
static int g_menuState = 0;
static int g_selectedCity = -1;

static const char* g_cities[] = {
    "Novosibirsk",
    "Krasnoyarsk",
    "Irkutsk",
    "Tomsk",
    "Omsk",
    "Barnaull",
    "Kemerovo",
    "Norilsk",
    "Chita",
    "Abakan",
    "Udinka",
    "Nazaraevosibirsk",
    "Tayshet",
    "Norilsk"
};

static const float g_cityPositions[][3] = {
    {-100, 100, 0},
    {-80, 80, 0},
    {-60, 120, 0},
    {-40, 140, 0},
    {-20, 160, 0},
    {0, 180, 0},
    {20, 200, 0},
    {40, 220, 0},
    {60, 240, 0},
    {80, 260, 0},
    {100, 280, 0},
    {120, 300, 0},
    {140, 320, 0},
    {160, 340, 0},
    {180, 360, 0}
};

static LRESULT CALLBACK WindowProc(HWND hwnd, UINT uMsg, WPARAM wParam, LPARAM lParam) {
    if (uMsg == WM_DESTROY) {
        PostQuitMessage(0);
    }
    return DefWindowProc(hwnd, uMsg, wParam, lParam);
}

void CreateWindow() {
    WNDCLAS wc = {};
    wc.lpfnWndProc = WindowProc;
    wc.hInstance = GetModuleHandle(nullptr);
    wc.lpszClassName = "RTGCWindow";
    wc.hCursor = LoadCursor(nullptr, IDC_ARROW);
    wc.hbrBackground = (HBRUSH)GetStockObject(WHITE_BRUSH);
    
    RegisterClassA(&wc);
    
    g_hwnd = CreateWindowEx(0, "RTGCWindow", "RTGC - Siberian Cities",
        WS_OVERLAPPEDWINDOW | WS_CAPTIONBAR | WS_SYSMENU,
        CW_USEDEFAULT, CW_USEDEFAULT,
        g_width, g_height,
        nullptr, nullptr, GetModuleHandle(nullptr), nullptr);
    
    if (!g_hwnd) {
        char buf[256];
        sprintf_s(buf, "Failed to create window: %lu", GetLastError());
        MessageBoxA(nullptr, buf, "Error", MB_ICONERROR);
        return;
    }
    
    ShowWindow(g_hwnd, SW_SHOW);
    UpdateWindow(g_hwnd);
}

void DestroyWindow() {
    if (g_hwnd) {
        DestroyWindow(g_hwnd);
        g_hwnd = nullptr;
    g_running = false;
    }
}

bool PollEvents() {
    MSG msg;
    while (PeekMessageA(&msg, nullptr, 0, 0, PM_REMOVE)) {
        TranslateMessage(&msg);
        DispatchMessageA(&msg);
        
        if (msg.message == WM_QUIT) {
            g_running = false;
        }
        
        if (msg.message == WM_KEYDOWN) {
            if (wParam == VK_ESCAPE) {
                g_running = false;
            } else if (wParam == VK_RETURN) {
                if (g_menuState == 0) {
                    g_menuState = 1;
                } else if (g_menuState == 1) {
                    g_selectedCity = (g_selectedCity + 1) % 15;
                    g_menuState = 2;
                } else {
                    g_menuState = 0;
                    g_selectedCity = -1;
                }
            }
        }
    }
    
    return g_running;
}

void BeginFrame() {
    HDC hdc = GetDC(g_hwnd);
    
    RECT rect;
    GetClientRect(g_hwnd, &rect);
    
    HBRUSH brush = CreateSolidBrush(RGB(30, 50, 70));
    FillRect(hdc, &rect, brush);
    DeleteObject(brush);
    
    ReleaseDC(g_hwnd, hdc);
}

void EndFrame() {
    SwapBuffers(g_hwnd);
}

void DrawText(int x, int y, const char* text, int color) {
    HDC hdc = GetDC(g_hwnd);
    
    RECT rect;
    GetClientRect(g_hwnd, &rect);
    
    HFONT font = CreateFontA(24, 0, 0, 0, FW_BOLD, DEFAULT_CHARSET, 
                             OUT_DEFAULT_PRECIS, CLIP_DEFAULT_PRECIS,
                             DEFAULT_QUALITY, DEFAULT_PITCH | FF_SWISS, "Arial");
    HFONT oldFont = (HFONT)SelectObject(hdc, font);
    
    SetBkMode(hdc, TRANSPARENT);
    SetTextColor(hdc, color);
    
    TextOutA(hdc, x, y, text, (int)strlen(text));
    
    SelectObject(hdc, oldFont);
    DeleteObject(font);
    ReleaseDC(g_hwnd, hdc);
}

void DrawMenu() {
    HDC hdc = GetDC(g_hwnd);
    
    RECT rect;
    GetClientRect(g_hwnd, &rect);
    
    int centerX = rect.right / 2;
    int centerY = rect.bottom / 2;
    
    SetTextColor(hdc, RGB(255, 255, 255));
    
    DrawText(centerX - 200, centerY - 150, "RTGC - Siberian Cities", RGB(0, 0, 0));
    DrawText(centerX - 120, centerY - 50, "[1] Generate New City", RGB(200, 200, 0));
    DrawText(centerX - 120, centerY, "  [ESC] Exit", RGB(200, 200, 0));
}

void DrawCitySelection() {
    HDC hdc = GetDC(g_hwnd);
    
    RECT rect;
    GetClientRect(g_hwnd, &rect);
    
    int centerX = rect.right / 2;
    int centerY = rect.bottom / 2;
    
    SetTextColor(hdc, RGB(255, 255, 255));
    
    DrawText(centerX - 150, centerY - 150, "Select Your City:", RGB(0, 255, 0));
    
    int y = centerY - 80;
    for (int i = 0; i < 15; i++) {
        char text[100];
        sprintf_s(text, "[%d] %s", i + 1, g_cities[i]);
        DrawText(centerX - 120, centerY + y, text, RGB(0, 200, 0));
        y += 30;
    }
    
    DrawText(centerX - 100, centerY + y + 30, "  [ESC] Back to Main Menu", RGB(200, 200, 0));
}

void DrawGame() {
    HDC hdc = GetDC(g_hwnd);
    
    RECT rect;
    GetClientRect(g_hwnd, &rect);
    
    SetTextColor(hdc, RGB(255, 255, 255));
    
    DrawText(20, 30, "Game Running", RGB(0, 255, 0));
    DrawText(20, 60, "Controls: WASD - Move, Space - Jump", RGB(200, 200, 0));
    DrawText(20, 90, "Current City: Novosibirsk (0, 0, 0)", RGB(0, 255, 0));
    DrawText(20, 120, "Position: (0, 10, 0)", RGB(200, 200, 0));
    DrawText(20, 150, "FPS: 60", RGB(0, 255, 0));
    
    DrawText(20, 180, "  [ESC] Main Menu", RGB(255, 255, 0));
}

void DrawLoading() {
    HDC hdc = GetDC(g_hwnd);
    
    RECT rect;
    GetClientRect(g_hwnd, &rect);
    
    SetTextColor(hdc, RGB(255, 255, 255));
    
    int centerX = rect.right / 2;
    int centerY = rect.bottom / 2;
    
    DrawText(centerX - 100, centerY - 50, "Loading city...", RGB(0, 255, 0));
}

void SetWindowSize(int w, int h) {
    g_width = w;
    g_height = h;
    
    RECT rect = {0, 0, g_width, g_height};
    AdjustWindowRect(&rect, WS_OVERLAPPEDWINDOW, FALSE);
    SetWindowPos(g_hwnd, nullptr, 0, 
               rect.right - rect.left, rect.bottom - rect.top, 
               SWP_NOMOVE | SWP_NOZORDER);
}

int GetWidth() {
    return g_width;
}

int GetHeight() {
    return g_height;
}

HWND GetHwnd() {
    return g_hwnd;
}

void SetRunning(bool running) {
    g_running = running;
}

bool IsRunning() {
    return g_running;
}

int GetMenuState() {
    return g_menuState;
}

void SetMenuState(int state) {
    g_menuState = state;
}

int GetSelectedCity() {
    return g_selectedCity;
}

void SetSelectedCity(int city) {
    g_selectedCity = city;
}

const char* GetCityName(int index) {
    if (index >= 0 && index < 15) {
        return g_cities[index];
    }
    return "Unknown";
}