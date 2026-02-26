#include <windows.h>
#include <cstdio>

namespace RTGC {
    constexpr int WINDOW_WIDTH = 1280;
    constexpr int WINDOW_HEIGHT = 720;
    
    HWND g_hwnd = nullptr;
    bool g_running = true;
    int g_menuState = 0;  // 0 = main menu, 1 = city selection, 2 = game
    
    struct City {
        const char* name;
        float x;
        float y;
        float z;
    };
    
    constexpr int CITY_COUNT = 15;
    City g_cities[CITY_COUNT] = {
        {"Novosibirsk", 100, 100, 0},
        {"Krasnoyarsk", 80, 80, 0},
        {"Irkutsk", 60, 120, 0},
        {"Tomsk", 40, 140, 0},
        {"Omsk", 20, 160, 0},
        {"Barnaull", 20, 200, 0},
        {"Kemerovo", 40, 220, 0},
        {"Novokuznetsk", 60, 240, 0},
        {"Krasnoyarsk (Krai)", 80, 280, 0},
        {"Chita", 100, 260, 0},
        {"Abakan", 80, 300, 0},
        {"Dudinka", 140, 320, 0},
        {"Nazarovosibirsk", 120, 340, 0},
        {"Tayshet", 160, 360, 0}
    };
    
    LRESULT CALLBACK WndProc(HWND hwnd, UINT uMsg, WPARAM wParam, LPARAM lParam) {
        if (uMsg == WM_QUIT) {
            g_running = false;
            PostQuitMessage(0);
            return 0;
        }
        return DefWindowProc(hwnd, uMsg, wParam, lParam);
    }
    
    bool CreateWindow() {
        WNDCLAS wc = {};
        wc.lpfnWndProc = WndProc;
        wc.hInstance = GetModuleHandle(nullptr);
        wc.lpszClassName = "RTGCWindow";
        wc.hCursor = LoadCursor(nullptr, IDC_ARROW);
        
        RegisterClassA(&wc);
        
        g_hwnd = CreateWindowExA(
            0, "RTGCWindow",
            "RTGC - Siberian Cities",
            WS_OVERLAPPEDWINDOW | WS_CAPTIONBOX | WS_MINIMIZEBOX | WS_SYSMENU,
            CW_USEDEFAULT, CW_USEDEFAULT,
            WINDOW_WIDTH, WINDOW_HEIGHT,
            nullptr, nullptr, GetModuleHandle(nullptr), nullptr
        );
        
        if (!g_hwnd) {
            char buf[256];
            sprintf_s(buf, "CreateWindow failed: %lu", GetLastError());
            MessageBoxA(nullptr, "Error", buf, MB_OK);
            return false;
        }
        
        ShowWindow(g_hwnd, SW_SHOW);
        return true;
    }
    
    void Run() {
        while (g_running) {
            MSG msg;
            while (PeekMessageA(&msg, nullptr, 0, 0, PM_REMOVE)) {
                TranslateMessage(&msg);
                DispatchMessageA(&msg);
            }
            
            HDC hdc = GetDC(g_hwnd);
            
            RECT clientRect;
            GetClientRect(g_hwnd, &clientRect);
            
            if (g_menuState == 0) {
                DrawMainMenu(hdc, clientRect);
            } else if (g_menuState == 1) {
                DrawCitySelection(hdc, clientRect);
            } else if (g_menuState == 2) {
                DrawGame(hdc, clientRect);
            }
            
            ReleaseDC(g_hwnd, hdc);
        }
    }
    }
    
    void DrawMainMenu(HDC hdc, RECT& rect) {
        HBRUSH brush = CreateSolidBrush(RGB(30, 50, 70));
        FillRect(hdc, &rect, brush);
        DeleteObject(brush);
        
        SetBkMode(hdc, TRANSPARENT);
        SetTextColor(hdc, RGB(255, 255, 255));
        
        SetTextAlign(hdc, TA_CENTER);
        
        TextOutA(hdc, 0, 0, "RTGC - Siberian Cities", 32);
        TextOutA(hdc, 0, 40, "Press [1] Generate New City", 20);
        TextOutA(hdc, 0, 80, "[2] Select Existing City", 20);
        TextOutA(hdc, 0, 120, "[ESC] Exit", 20);
    }
    
    void DrawCitySelection(HDC hdc, RECT& rect) {
        HBRUSH brush = CreateSolidBrush(RGB(40, 60, 80));
        FillRect(hdc, &rect, brush);
        DeleteObject(brush);
        
        SetBkMode(hdc, TRANSPARENT);
        SetTextColor(hdc, RGB(200, 255, 200));
        SetTextAlign(hdc, TA_CENTER);
        
        TextOutA(hdc, 0, 0, "Select a city:", 28);
        
        int y = 100;
        for (int i = 0; i < CITY_COUNT; i++) {
            char buf[128];
            sprintf_s(buf, "%d. %s", i + 1, g_cities[i].name);
            TextOutA(hdc, 100, y, buf, 20);
            y += 30;
        }
        
        TextOutA(hdc, 0, y + 20, "[ESC] Back to menu", 20);
        TextOutA(hdc, 0, y + 50, "[ENTER] Start in selected city", 20);
    }
    
    void DrawGame(HDC hdc, RECT& rect) {
        HBRUSH brush = CreateSolidBrush(RGB(100, 150, 100));
        FillRect(hdc, &rect, brush);
        DeleteObject(brush);
        
        SetBkMode(hdc, TRANSPARENT);
        SetTextColor(hdc, RGB(255, 255, 255));
        SetTextAlign(hdc, TA_LEFT);
        
        TextOutA(hdc, 20, 50, "Game running...", 16);
        TextOutA(hdc, 20, 80, "Controls: WASD to move", 18);
        TextOutA(hdc, 20, 120, "[ESC] Return to menu", 18);
        
        if (g_menuState == 3) {
            DrawSelectedCity(hdc, rect);
        }
    }
    
    void DrawSelectedCity(HDC hdc, RECT& rect) {
        if (g_menuState < 3) return;
        
        City& city = g_cities[g_menuState - 3];
        
        HBRUSH brush = CreateSolidBrush(RGB(200, 150, 50));
        Ellipse(hdc, 
                rect.left + city.x * 2 + 50, rect.top - city.y * 2 - 50,
                rect.left + city.x * 2 + 150, rect.top - city.y * 2 + 50,
                RGB(100, 200, 100)
        );
        DeleteObject(brush);
        
        SetBkMode(hdc, TRANSPARENT);
        SetTextColor(hdc, RGB(255, 200, 200));
        SetTextAlign(hdc, TA_LEFT);
        
        char title[128];
        sprintf_s(title, "City: %s", city.name);
        TextOutA(hdc, 20, 30, title, 18);
        
        TextOutA(hdc, 20, 70, "Controls: WASD - Move", 16);
        TextOutA(hdc, 20, 90, "Space - Jump", 16);
        TextOutA(hdc, 20, 110, "[ESC] Back to menu", 18);
        TextOutA(hdc, 20, 130, "[M] Open menu", 18);
    }
}