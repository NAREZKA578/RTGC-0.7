#include "Renderer.hpp"
#include "../world/Terrain.hpp"
#include "../physics/CharacterController.hpp"
#include "../core/Logger.hpp"
#include <GL/gl.h>
#include <GL/glu.h>
#include <windows.h>
#include <iostream>
#include <cmath>
#include <cstring>

Renderer::Renderer()
    : m_window(nullptr)
    , m_initialized(false)
    , m_currentState(MenuState::LOADING_STATE)
{
    InitializeSiberianCities();
}

Renderer::~Renderer() {
    Shutdown();
}

void Renderer::InitializeSiberianCities() {
    const char* cities[] = {
        "Novosibirsk", "Krasnoyarsk", "Irkutsk", "Tomsk", "Omsk",
        "Barnaul", "Kemerovo", "Norilsk", "Chita", "Abakan",
        "Dudinka", "Nazarovo", "Tayshet", "Ulan-Ude"
    };

    const float cityPos[][3] = {
        {100, 100, 0}, {80, 80, 0}, {60, 120, 0}, {40, 140, 0}, {20, 160, 0},
        {20, 200, 0}, {40, 220, 0}, {60, 240, 0}, {80, 260, 0}, {100, 280, 0},
        {120, 300, 0}, {140, 320, 0}, {160, 340, 0}, {180, 360, 0}
    };

    for (size_t i = 0; i < sizeof(cities)/sizeof(cities[0]); ++i) {
        m_cities.push_back({cities[i], cityPos[i][0], cityPos[i][1], cityPos[i][2]});
    }
}

bool Renderer::Initialize() {
    if (m_initialized) return true;

    m_window = new Window(1920, 1080, "RTGC - Siberian Cities");
    if (!m_window->Initialize()) {
        return false;
    }

    // Enable graphics features
    m_window->EnableAntiAliasing(8);
    m_window->SetVSync(Window::VSyncMode::ADAPTIVE);

    m_initialized = true;
    Logger::Log("Renderer initialized successfully");
    return true;
}



void Renderer::Shutdown() {
    if (!m_initialized) return;

    if (m_window) {
        m_window->Shutdown();
        delete m_window;
        m_window = nullptr;
    }

    m_initialized = false;
}

void Renderer::BeginFrame() {
    if (!m_window) return;

    // Make OpenGL context current
    wglMakeCurrent(m_window->GetHDC(), m_window->GetHGLRC());

    // Setup viewport
    int width, height;
    m_window->GetSize(width, height);
    glViewport(0, 0, width, height);

    // Clear screen with cinematic gradient for AAA feel
    glClearColor(0.015f, 0.015f, 0.025f, 1.0f);
    glClear(GL_COLOR_BUFFER_BIT | GL_DEPTH_BUFFER_BIT);
}

void Renderer::EndFrame() {
    if (m_window) {
        m_window->SwapBuffers();
    }
}

void Renderer::Clear() {
    glClear(GL_COLOR_BUFFER_BIT | GL_DEPTH_BUFFER_BIT);
}

void Renderer::RenderMenu() {
    if (!m_initialized || !m_window) return;

    int width, height;
    m_window->GetSize(width, height);

    // Draw solid dark background
    glClearColor(0.1f, 0.1f, 0.15f, 1.0f);
    glClear(GL_COLOR_BUFFER_BIT | GL_DEPTH_BUFFER_BIT);

    // Draw text using GDI for clean text rendering (like in Snow Runner menus)
    HDC hdc = m_window->GetHDC();
    if (hdc) {
        // Title font
        HFONT titleFont = CreateFontA(48, 0, 0, 0, FW_BOLD, FALSE, FALSE, FALSE,
                                      DEFAULT_CHARSET, OUT_DEFAULT_PRECIS,
                                      CLIP_DEFAULT_PRECIS, CLEARTYPE_QUALITY,
                                      DEFAULT_PITCH | FF_DONTCARE, "Arial");
        HFONT oldFont = (HFONT)SelectObject(hdc, titleFont);
        SetBkMode(hdc, TRANSPARENT);
        SetTextColor(hdc, RGB(255, 255, 255));
        SetTextAlign(hdc, TA_CENTER | TA_BASELINE);

        // Title
        TextOutA(hdc, width / 2, height / 2 - 150, "RTGC - Siberian Cities", 23);

        // Menu options font
        HFONT menuFont = CreateFontA(24, 0, 0, 0, FW_NORMAL, FALSE, FALSE, FALSE,
                                     DEFAULT_CHARSET, OUT_DEFAULT_PRECIS,
                                     CLIP_DEFAULT_PRECIS, CLEARTYPE_QUALITY,
                                     DEFAULT_PITCH | FF_DONTCARE, "Arial");
        SelectObject(hdc, menuFont);
        SetTextColor(hdc, RGB(200, 200, 200));

        // Menu options
        TextOutA(hdc, width / 2, height / 2 - 50, "1. Generate New City", 19);
        TextOutA(hdc, width / 2, height / 2 - 20, "2. Select Existing City", 22);
        TextOutA(hdc, width / 2, height / 2 + 10, "3. Settings", 10);
        TextOutA(hdc, width / 2, height / 2 + 40, "4. Exit", 6);

        // Controls info (like in Snow Runner)
        HFONT controlsFont = CreateFontA(18, 0, 0, 0, FW_NORMAL, FALSE, FALSE, FALSE,
                                         DEFAULT_CHARSET, OUT_DEFAULT_PRECIS,
                                         CLIP_DEFAULT_PRECIS, CLEARTYPE_QUALITY,
                                         DEFAULT_PITCH | FF_DONTCARE, "Arial");
        SelectObject(hdc, controlsFont);
        SetTextColor(hdc, RGB(150, 150, 150));
        SetTextAlign(hdc, TA_LEFT | TA_BASELINE);

        TextOutA(hdc, 50, height - 100, "Controls: WASD - Move, SPACE - Jump, ESC - Back", 49);

        SelectObject(hdc, oldFont);
        DeleteObject(titleFont);
        DeleteObject(menuFont);
        DeleteObject(controlsFont);
    }
}

void Renderer::RenderWorldCreation(const WorldConfig::WorldSettings& settings) {
    if (!m_initialized || !m_window) return;

    int width, height;
    m_window->GetSize(width, height);

    // Draw solid dark gray background (no gradient)
    glBegin(GL_QUADS);
    glColor3f(0.08f, 0.08f, 0.1f);
    glVertex2f(0, 0);
    glVertex2f(width, 0);
    glVertex2f(width, height);
    glVertex2f(0, height);
    glEnd();

    // Draw world creation panel
    float panelX = width / 2 - 400;
    float panelY = height / 2 - 350;
    float panelWidth = 800;
    float panelHeight = 700;

    // Main panel (dark gray)
    glColor3f(0.12f, 0.12f, 0.15f);
    glBegin(GL_QUADS);
    glVertex2f(panelX, panelY);
    glVertex2f(panelX + panelWidth, panelY);
    glVertex2f(panelX + panelWidth, panelY + panelHeight);
    glVertex2f(panelX, panelY + panelHeight);
    glEnd();

    // Panel border (gray)
    glColor3f(0.35f, 0.35f, 0.4f);
    glLineWidth(2.0f);
    glBegin(GL_LINE_LOOP);
    glVertex2f(panelX, panelY);
    glVertex2f(panelX + panelWidth, panelY);
    glVertex2f(panelX + panelWidth, panelY + panelHeight);
    glVertex2f(panelX, panelY + panelHeight);
    glEnd();
    glLineWidth(1.0f);

    // Title bar (darker gray)
    glColor3f(0.15f, 0.15f, 0.18f);
    glBegin(GL_QUADS);
    glVertex2f(panelX, panelY);
    glVertex2f(panelX + panelWidth, panelY);
    glVertex2f(panelX + panelWidth, panelY + 60);
    glVertex2f(panelX, panelY + 60);
    glEnd();

    // Title bar accent (gray)
    glColor3f(0.3f, 0.3f, 0.4f);
    glBegin(GL_QUADS);
    glVertex2f(panelX + 50, panelY + 15);
    glVertex2f(panelX + 350, panelY + 15);
    glVertex2f(panelX + 350, panelY + 35);
    glVertex2f(panelX + 50, panelY + 35);
    glEnd();

    // World settings sections
    float startY = panelY + 100;
    float sectionHeight = 80;

    // World name section (gray)
    glColor3f(0.15f, 0.15f, 0.18f);
    glBegin(GL_QUADS);
    glVertex2f(panelX + 20, startY);
    glVertex2f(panelX + 380, startY);
    glVertex2f(panelX + 380, startY + sectionHeight);
    glVertex2f(panelX + 20, startY + sectionHeight);
    glEnd();

    // Map size section (gray)
    glColor3f(0.15f, 0.15f, 0.18f);
    glBegin(GL_QUADS);
    glVertex2f(panelX + 420, startY);
    glVertex2f(panelX + 780, startY);
    glVertex2f(panelX + 780, startY + sectionHeight);
    glVertex2f(panelX + 420, startY + sectionHeight);
    glEnd();

    // Terrain type section (gray)
    glColor3f(0.15f, 0.15f, 0.18f);
    glBegin(GL_QUADS);
    glVertex2f(panelX + 20, startY + sectionHeight + 20);
    glVertex2f(panelX + 380, startY + sectionHeight + 20);
    glVertex2f(panelX + 380, startY + sectionHeight * 2 + 20);
    glVertex2f(panelX + 20, startY + sectionHeight * 2 + 20);
    glEnd();

    // Climate type section (gray)
    glColor3f(0.15f, 0.15f, 0.18f);
    glBegin(GL_QUADS);
    glVertex2f(panelX + 420, startY + sectionHeight + 20);
    glVertex2f(panelX + 780, startY + sectionHeight + 20);
    glVertex2f(panelX + 780, startY + sectionHeight * 2 + 20);
    glVertex2f(panelX + 420, startY + sectionHeight * 2 + 20);
    glEnd();

    // Season section (gray)
    glColor3f(0.15f, 0.15f, 0.18f);
    glBegin(GL_QUADS);
    glVertex2f(panelX + 20, startY + sectionHeight * 2 + 40);
    glVertex2f(panelX + 380, startY + sectionHeight * 2 + 40);
    glVertex2f(panelX + 380, startY + sectionHeight * 3 + 40);
    glVertex2f(panelX + 20, startY + sectionHeight * 3 + 40);
    glEnd();

    // Density section (gray)
    glColor3f(0.15f, 0.15f, 0.18f);
    glBegin(GL_QUADS);
    glVertex2f(panelX + 420, startY + sectionHeight * 2 + 40);
    glVertex2f(panelX + 780, startY + sectionHeight * 2 + 40);
    glVertex2f(panelX + 780, startY + sectionHeight * 3 + 40);
    glVertex2f(panelX + 420, startY + sectionHeight * 3 + 40);
    glEnd();

    // Buttons
    float buttonY = panelY + panelHeight - 80;

    // Create World button (gray)
    glColor3f(0.18f, 0.18f, 0.22f);
    glBegin(GL_QUADS);
    glVertex2f(panelX + 150, buttonY);
    glVertex2f(panelX + 380, buttonY);
    glVertex2f(panelX + 380, buttonY + 50);
    glVertex2f(panelX + 150, buttonY + 50);
    glEnd();

    // Back button (gray)
    glColor3f(0.18f, 0.18f, 0.22f);
    glBegin(GL_QUADS);
    glVertex2f(panelX + 420, buttonY);
    glVertex2f(panelX + 650, buttonY);
    glVertex2f(panelX + 650, buttonY + 50);
    glVertex2f(panelX + 420, buttonY + 50);
    glEnd();

    // Button borders (gray)
    glColor3f(0.4f, 0.4f, 0.45f);
    glLineWidth(2.0f);
    glBegin(GL_LINE_LOOP);
    glVertex2f(panelX + 150, buttonY);
    glVertex2f(panelX + 380, buttonY);
    glVertex2f(panelX + 380, buttonY + 50);
    glVertex2f(panelX + 150, buttonY + 50);
    glEnd();

    glBegin(GL_LINE_LOOP);
    glVertex2f(panelX + 420, buttonY);
    glVertex2f(panelX + 650, buttonY);
    glVertex2f(panelX + 650, buttonY + 50);
    glVertex2f(panelX + 420, buttonY + 50);
    glEnd();
    glLineWidth(1.0f);

    // Add GDI text
    HDC hdc = m_window->GetHDC();
    if (hdc) {
        HFONT titleFont = CreateFontA(28, 0, 0, 0, FW_BOLD, FALSE, FALSE, FALSE,
                                      DEFAULT_CHARSET, OUT_DEFAULT_PRECIS,
                                      CLIP_DEFAULT_PRECIS, CLEARTYPE_QUALITY,
                                      DEFAULT_PITCH | FF_DONTCARE, "Arial");
        HFONT oldFont = (HFONT)SelectObject(hdc, titleFont);
        SetBkMode(hdc, TRANSPARENT);
        SetTextColor(hdc, RGB(255, 255, 255));
        SetTextAlign(hdc, TA_CENTER | TA_BASELINE);

        // Panel title
        TextOutA(hdc, width / 2, height / 2 - 320, "World Creation", 13);

        HFONT labelFont = CreateFontA(18, 0, 0, 0, FW_NORMAL, FALSE, FALSE, FALSE,
                                     DEFAULT_CHARSET, OUT_DEFAULT_PRECIS,
                                     CLIP_DEFAULT_PRECIS, CLEARTYPE_QUALITY,
                                     DEFAULT_PITCH | FF_DONTCARE, "Arial");
        SelectObject(hdc, labelFont);
        SetTextColor(hdc, RGB(200, 200, 200));

        // Section labels
        TextOutA(hdc, panelX + 190, startY + 70, "World Name", 10);
        TextOutA(hdc, panelX + 590, startY + 70, "Map Size", 8);
        TextOutA(hdc, panelX + 190, startY + sectionHeight + 90, "Terrain Type", 12);
        TextOutA(hdc, panelX + 590, startY + sectionHeight + 90, "Climate Type", 12);
        TextOutA(hdc, panelX + 190, startY + sectionHeight * 2 + 110, "Season", 6);
        TextOutA(hdc, panelX + 590, startY + sectionHeight * 2 + 110, "Density", 7);

        // Values from settings
        SetTextColor(hdc, RGB(255, 255, 255));
        TextOutA(hdc, panelX + 190, startY + 30, settings.worldName.c_str(), settings.worldName.length());

        std::string mapSizeStr = (settings.mapSize == WorldConfig::MapSize::SMALL) ? "Small" :
                                (settings.mapSize == WorldConfig::MapSize::MEDIUM) ? "Medium" :
                                (settings.mapSize == WorldConfig::MapSize::LARGE) ? "Large" : "Huge";
        TextOutA(hdc, panelX + 590, startY + 30, mapSizeStr.c_str(), mapSizeStr.length());

        std::string terrainStr = settings.GetTerrainName();
        TextOutA(hdc, panelX + 190, startY + sectionHeight + 50, terrainStr.c_str(), terrainStr.length());

        std::string climateStr = settings.GetClimateName();
        TextOutA(hdc, panelX + 590, startY + sectionHeight + 50, climateStr.c_str(), climateStr.length());

        std::string seasonStr = settings.GetSeasonName();
        TextOutA(hdc, panelX + 190, startY + sectionHeight * 2 + 70, seasonStr.c_str(), seasonStr.length());

        std::string densityStr = (settings.cityDensity < 0.3f) ? "Low" : (settings.cityDensity < 0.7f) ? "Medium" : "High";
        TextOutA(hdc, panelX + 590, startY + sectionHeight * 2 + 70, densityStr.c_str(), densityStr.length());

        // Buttons
        HFONT buttonFont = CreateFontA(20, 0, 0, 0, FW_BOLD, FALSE, FALSE, FALSE,
                                       DEFAULT_CHARSET, OUT_DEFAULT_PRECIS,
                                       CLIP_DEFAULT_PRECIS, CLEARTYPE_QUALITY,
                                       DEFAULT_PITCH | FF_DONTCARE, "Arial");
        SelectObject(hdc, buttonFont);
        SetTextColor(hdc, RGB(255, 255, 255));
        TextOutA(hdc, panelX + 265, buttonY + 30, "Create World", 12);
        TextOutA(hdc, panelX + 535, buttonY + 30, "Back", 4);

        // Controls
        HFONT controlsFont = CreateFontA(16, 0, 0, 0, FW_NORMAL, FALSE, FALSE, FALSE,
                                         DEFAULT_CHARSET, OUT_DEFAULT_PRECIS,
                                         CLIP_DEFAULT_PRECIS, CLEARTYPE_QUALITY,
                                         DEFAULT_PITCH | FF_DONTCARE, "Arial");
        SelectObject(hdc, controlsFont);
        SetTextColor(hdc, RGB(150, 150, 150));
        SetTextAlign(hdc, TA_LEFT | TA_BASELINE);
        TextOutA(hdc, 50, height - 100, "ENTER - Create World, ESC - Back to Menu", 38);

        SelectObject(hdc, oldFont);
        DeleteObject(titleFont);
        DeleteObject(labelFont);
        DeleteObject(buttonFont);
        DeleteObject(controlsFont);
    }
}

void Renderer::RenderCitySelection(int selectedIndex) {
    if (!m_initialized || !m_window) return;

    int width, height;
    m_window->GetSize(width, height);

    // Draw solid dark gray background (no gradient)
    glBegin(GL_QUADS);
    glColor3f(0.08f, 0.08f, 0.1f);
    glVertex2f(0, 0);
    glVertex2f(width, 0);
    glVertex2f(width, height);
    glVertex2f(0, height);
    glEnd();

    // Draw decorative border
    glColor3f(0.3f, 0.5f, 0.3f);
    glLineWidth(3.0f);
    glBegin(GL_LINE_LOOP);
    glVertex2f(50, 50);
    glVertex2f(width - 50, 50);
    glVertex2f(width - 50, height - 50);
    glVertex2f(50, height - 50);
    glEnd();
    glLineWidth(1.0f);

    // Draw selection panel
    // Outer glow
    glColor4f(0.1f, 0.2f, 0.1f, 0.3f);
    glBegin(GL_QUADS);
    glVertex2f(width/2 - 420, height/2 - 220);
    glVertex2f(width/2 + 420, height/2 - 220);
    glVertex2f(width/2 + 420, height/2 + 220);
    glVertex2f(width/2 - 420, height/2 + 220);
    glEnd();

    // Main panel
    glColor3f(0.08f, 0.12f, 0.08f);
    glBegin(GL_QUADS);
    glVertex2f(width/2 - 400, height/2 - 200);
    glVertex2f(width/2 + 400, height/2 - 200);
    glVertex2f(width/2 + 400, height/2 + 200);
    glVertex2f(width/2 - 400, height/2 + 200);
    glEnd();

    // Panel border
    glColor3f(0.4f, 0.6f, 0.4f);
    glLineWidth(2.0f);
    glBegin(GL_LINE_LOOP);
    glVertex2f(width/2 - 400, height/2 - 200);
    glVertex2f(width/2 + 400, height/2 - 200);
    glVertex2f(width/2 + 400, height/2 + 200);
    glVertex2f(width/2 - 400, height/2 + 200);
    glEnd();
    glLineWidth(1.0f);

    // Draw title
    glColor3f(0.15f, 0.25f, 0.15f);
    glBegin(GL_QUADS);
    glVertex2f(width/2 - 120, height/2 - 180);
    glVertex2f(width/2 + 120, height/2 - 180);
    glVertex2f(width/2 + 120, height/2 - 140);
    glVertex2f(width/2 - 120, height/2 - 140);
    glEnd();

    // Title gold accent
    glColor3f(1.0f, 0.85f, 0.2f);
    glBegin(GL_QUADS);
    glVertex2f(width/2 - 100, height/2 - 170);
    glVertex2f(width/2 + 100, height/2 - 170);
    glVertex2f(width/2 + 100, height/2 - 150);
    glVertex2f(width/2 - 100, height/2 - 150);
    glEnd();

    // Draw city options with better colors
    float itemHeight = 30;
    float itemSpacing = 35;
    float startY = height/2 + 150;

    for (size_t i = 0; i < m_cities.size() && i < 12; ++i) {
        float yPos = startY - (int)i * itemSpacing;

        // Item background - brighter if selected
        float brightness = (i == (size_t)selectedIndex) ? 1.5f : 1.0f;
        if (i % 4 == 0) {
            glColor3f(0.15f * brightness, 0.25f * brightness, 0.15f * brightness);
        } else if (i % 4 == 1) {
            glColor3f(0.12f * brightness, 0.2f * brightness, 0.25f * brightness);
        } else if (i % 4 == 2) {
            glColor3f(0.25f * brightness, 0.15f * brightness, 0.15f * brightness);
        } else {
            glColor3f(0.15f * brightness, 0.15f * brightness, 0.25f * brightness);
        }

        glBegin(GL_QUADS);
        glVertex2f(width/2 - 350, yPos - itemHeight/2);
        glVertex2f(width/2 + 350, yPos - itemHeight/2);
        glVertex2f(width/2 + 350, yPos + itemHeight/2);
        glVertex2f(width/2 - 350, yPos + itemHeight/2);
        glEnd();

        // Item border
        glColor3f(0.5f, 0.7f, 0.5f);
        glLineWidth(1.0f);
        glBegin(GL_LINE_LOOP);
        glVertex2f(width/2 - 350, yPos - itemHeight/2);
        glVertex2f(width/2 + 350, yPos - itemHeight/2);
        glVertex2f(width/2 + 350, yPos + itemHeight/2);
        glVertex2f(width/2 - 350, yPos + itemHeight/2);
        glEnd();

        // Number indicator
        if (i % 4 == 0) {
            glColor3f(0.6f * brightness, 0.9f * brightness, 0.6f * brightness);
        } else if (i % 4 == 1) {
            glColor3f(0.6f * brightness, 0.6f * brightness, 0.9f * brightness);
        } else if (i % 4 == 2) {
            glColor3f(0.9f * brightness, 0.6f * brightness, 0.6f * brightness);
        } else {
            glColor3f(0.7f * brightness, 0.7f * brightness, 0.9f * brightness);
        }

        glBegin(GL_QUADS);
        glVertex2f(width/2 - 330, yPos - itemHeight/2 + 2);
        glVertex2f(width/2 - 290, yPos - itemHeight/2 + 2);
        glVertex2f(width/2 - 290, yPos + itemHeight/2 - 2);
        glVertex2f(width/2 - 330, yPos + itemHeight/2 - 2);
        glEnd();
    }

    // Scroll indicators (since we show only 12 cities)
    glColor3f(0.5f, 0.6f, 0.5f);
    glBegin(GL_TRIANGLES);
    glVertex2f(width/2 - 30, height/2 + 170);
    glVertex2f(width/2 - 15, height/2 + 180);
    glVertex2f(width/2 - 45, height/2 + 180);
    glEnd();

    glBegin(GL_TRIANGLES);
    glVertex2f(width/2 + 30, height/2 - 240);
    glVertex2f(width/2 + 45, height/2 - 230);
    glVertex2f(width/2 + 15, height/2 - 230);
    glEnd();

    // Add GDI text for title and city names
    HDC hdc = m_window->GetHDC();
    if (hdc) {
        HFONT titleFont = CreateFontA(32, 0, 0, 0, FW_BOLD, FALSE, FALSE, FALSE,
                                      DEFAULT_CHARSET, OUT_DEFAULT_PRECIS,
                                      CLIP_DEFAULT_PRECIS, CLEARTYPE_QUALITY,
                                      DEFAULT_PITCH | FF_DONTCARE, "Arial");
        HFONT oldFont = (HFONT)SelectObject(hdc, titleFont);
        SetBkMode(hdc, TRANSPARENT);
        SetTextColor(hdc, RGB(255, 255, 255));
        SetTextAlign(hdc, TA_CENTER | TA_BASELINE);

        // Title
        TextOutA(hdc, width / 2, height / 2 - 160, "Select City", 10);

        HFONT cityFont = CreateFontA(20, 0, 0, 0, FW_NORMAL, FALSE, FALSE, FALSE,
                                     DEFAULT_CHARSET, OUT_DEFAULT_PRECIS,
                                     CLIP_DEFAULT_PRECIS, CLEARTYPE_QUALITY,
                                     DEFAULT_PITCH | FF_DONTCARE, "Arial");
        SelectObject(hdc, cityFont);

        for (size_t i = 0; i < m_cities.size() && i < 12; ++i) {
            float yPos = startY - (int)i * itemSpacing;
            int textY = (int)(yPos + itemHeight / 4); // center vertically

            if (i == (size_t)selectedIndex) {
                SetTextColor(hdc, RGB(255, 255, 0)); // yellow for selected
            } else {
                SetTextColor(hdc, RGB(200, 200, 200)); // gray for others
            }

            SetTextAlign(hdc, TA_LEFT | TA_BASELINE);
            TextOutA(hdc, width / 2 - 280, textY, m_cities[i].name, strlen(m_cities[i].name));
        }

        // Controls
        HFONT controlsFont = CreateFontA(16, 0, 0, 0, FW_NORMAL, FALSE, FALSE, FALSE,
                                         DEFAULT_CHARSET, OUT_DEFAULT_PRECIS,
                                         CLIP_DEFAULT_PRECIS, CLEARTYPE_QUALITY,
                                         DEFAULT_PITCH | FF_DONTCARE, "Arial");
        SelectObject(hdc, controlsFont);
        SetTextColor(hdc, RGB(150, 150, 150));
        SetTextAlign(hdc, TA_LEFT | TA_BASELINE);
        TextOutA(hdc, 50, height - 100, "WASD or Arrow Keys - Select, ENTER - Confirm, ESC - Back", 58);

        SelectObject(hdc, oldFont);
        DeleteObject(titleFont);
        DeleteObject(cityFont);
        DeleteObject(controlsFont);
    }
}

void Renderer::RenderGame(const Vector3& cameraPos) {
    if (!m_initialized || !m_window) return;

    int width, height;
    m_window->GetSize(width, height);

    // Setup 3D projection with better FOV
    glMatrixMode(GL_PROJECTION);
    glLoadIdentity();
    gluPerspective(60.0f, (float)width / (float)height, 0.1f, 2000.0f);

    glMatrixMode(GL_MODELVIEW);
    glLoadIdentity();

    // Setup camera following the character
    gluLookAt(cameraPos.x, cameraPos.y + 15.0f, cameraPos.z + 30.0f,
              cameraPos.x, cameraPos.y, cameraPos.z,
              0.0f, 1.0f, 0.0f);

    // Translate world so character is at origin
    glTranslatef(-cameraPos.x, -cameraPos.y, -cameraPos.z);

    // Enable lighting for AAA quality
    glEnable(GL_LIGHTING);
    glEnable(GL_LIGHT0);

    // Ambient light
    float ambientLight[] = {0.15f, 0.15f, 0.18f, 1.0f};
    glLightfv(GL_LIGHT0, GL_AMBIENT, ambientLight);

    // Diffuse light (sunlight)
    float diffuseLight[] = {0.9f, 0.85f, 0.75f, 1.0f};
    glLightfv(GL_LIGHT0, GL_DIFFUSE, diffuseLight);

    // Specular light
    float specularLight[] = {0.6f, 0.6f, 0.7f, 1.0f};
    glLightfv(GL_LIGHT0, GL_SPECULAR, specularLight);

    // Light position
    float lightPosition[] = {50.0f, 100.0f, 50.0f, 1.0f};
    glLightfv(GL_LIGHT0, GL_POSITION, lightPosition);

    glEnable(GL_COLOR_MATERIAL);
    glColorMaterial(GL_FRONT, GL_AMBIENT_AND_DIFFUSE);

    // Draw sky gradient (background) with AAA atmosphere
    glBegin(GL_QUADS);
    glColor3f(0.08f, 0.12f, 0.22f);
    glVertex3f(-1000.0f, 0.0f, -1000.0f);
    glColor3f(0.04f, 0.06f, 0.12f);
    glVertex3f(1000.0f, 0.0f, -1000.0f);
    glColor3f(0.04f, 0.06f, 0.12f);
    glVertex3f(1000.0f, 0.0f, 1000.0f);
    glColor3f(0.08f, 0.12f, 0.22f);
    glVertex3f(-1000.0f, 0.0f, 1000.0f);
    glEnd();

    // Draw ground plane with realistic snow colors
    glDisable(GL_LIGHTING);
    glBegin(GL_QUADS);
    glColor3f(0.92f, 0.95f, 1.0f);
    glVertex3f(-500.0f, 0.0f, -500.0f);
    glColor3f(0.85f, 0.88f, 0.95f);
    glVertex3f(500.0f, 0.0f, -500.0f);
    glColor3f(0.85f, 0.88f, 0.95f);
    glVertex3f(500.0f, 0.0f, 500.0f);
    glColor3f(0.92f, 0.95f, 1.0f);
    glVertex3f(-500.0f, 0.0f, 500.0f);
    glEnd();

    // Draw grid lines with subtle snow shadows
    glColor3f(0.7f, 0.75f, 0.8f);
    glLineWidth(1.5f);
    glBegin(GL_LINES);
    for (int i = -500; i <= 500; i += 50) {
        glVertex3f((float)i, 0.01f, -500.0f);
        glVertex3f((float)i, 0.01f, 500.0f);
        glVertex3f(-500.0f, 0.01f, (float)i);
        glVertex3f(500.0f, 0.01f, (float)i);
    }
    glEnd();
    glLineWidth(1.0f);

    glEnable(GL_LIGHTING);

    // Draw buildings/cities with AAA lighting
    glDisable(GL_LIGHTING);
    for (size_t i = 0; i < m_cities.size(); ++i) {
        const City& city = m_cities[i];

        glPushMatrix();
        glTranslatef(city.x * 0.01f, 1.5f, city.z * 0.01f);

        // AAA building materials with realistic colors
        float baseR = 0.6f, baseG = 0.65f, baseB = 0.7f;
        if (i % 4 == 1) { baseR = 0.7f; baseG = 0.6f; baseB = 0.55f; }
        else if (i % 4 == 2) { baseR = 0.55f; baseG = 0.65f; baseB = 0.5f; }
        else if (i % 4 == 3) { baseR = 0.75f; baseG = 0.6f; baseB = 0.6f; }

        // Draw building with realistic lighting
        glBegin(GL_QUADS);

        // Front face (brighter)
        glColor3f(baseR * 1.2f, baseG * 1.2f, baseB * 1.2f);
        glVertex3f(-1.0f, -1.0f, 1.0f);
        glVertex3f(1.0f, -1.0f, 1.0f);
        glVertex3f(1.0f, 1.0f, 1.0f);
        glVertex3f(-1.0f, 1.0f, 1.0f);

        // Back face (darker - in shadow)
        glColor3f(baseR * 0.6f, baseG * 0.6f, baseB * 0.6f);
        glVertex3f(-1.0f, -1.0f, -1.0f);
        glVertex3f(-1.0f, 1.0f, -1.0f);
        glVertex3f(1.0f, 1.0f, -1.0f);
        glVertex3f(1.0f, -1.0f, -1.0f);

        // Left face (medium)
        glColor3f(baseR * 0.8f, baseG * 0.8f, baseB * 0.8f);
        glVertex3f(-1.0f, -1.0f, -1.0f);
        glVertex3f(-1.0f, -1.0f, 1.0f);
        glVertex3f(1.0f, 1.0f, 1.0f);
        glVertex3f(-1.0f, 1.0f, -1.0f);

        // Right face (medium-dark)
        glColor3f(baseR * 0.7f, baseG * 0.7f, baseB * 0.7f);
        glVertex3f(1.0f, -1.0f, -1.0f);
        glVertex3f(1.0f, 1.0f, -1.0f);
        glVertex3f(1.0f, 1.0f, 1.0f);
        glVertex3f(1.0f, -1.0f, 1.0f);

        // Top face (brightest - lit from above)
        glColor3f(baseR * 1.3f, baseG * 1.3f, baseB * 1.3f);
        glVertex3f(-1.0f, 1.0f, -1.0f);
        glVertex3f(1.0f, 1.0f, -1.0f);
        glVertex3f(1.0f, 1.0f, 1.0f);
        glVertex3f(-1.0f, 1.0f, 1.0f);

        // Bottom face (darkest)
        glColor3f(baseR * 0.4f, baseG * 0.4f, baseB * 0.4f);
        glVertex3f(-1.0f, -1.0f, -1.0f);
        glVertex3f(1.0f, -1.0f, -1.0f);
        glVertex3f(1.0f, -1.0f, 1.0f);
        glVertex3f(-1.0f, -1.0f, 1.0f);
        glEnd();

        // Add realistic roof with specular highlights
        glColor3f(0.8f, 0.7f, 0.5f);
        glBegin(GL_TRIANGLES);
        glVertex3f(0.0f, 2.5f, 0.0f);
        glColor3f(0.6f, 0.5f, 0.4f);
        glVertex3f(-1.2f, 1.0f, -1.2f);
        glVertex3f(1.2f, 1.0f, -1.2f);

        glVertex3f(0.0f, 2.5f, 0.0f);
        glColor3f(0.7f, 0.6f, 0.5f);
        glVertex3f(1.2f, 1.0f, -1.2f);
        glColor3f(0.5f, 0.45f, 0.4f);
        glVertex3f(1.2f, 1.0f, 1.2f);

        glVertex3f(0.0f, 2.5f, 0.0f);
        glColor3f(0.5f, 0.45f, 0.4f);
        glVertex3f(1.2f, 1.0f, 1.2f);
        glColor3f(0.6f, 0.5f, 0.4f);
        glVertex3f(-1.2f, 1.0f, 1.2f);

        glVertex3f(0.0f, 2.5f, 0.0f);
        glColor3f(0.6f, 0.5f, 0.4f);
        glVertex3f(-1.2f, 1.0f, 1.2f);
        glColor3f(0.6f, 0.5f, 0.4f);
        glVertex3f(-1.2f, 1.0f, -1.2f);
        glEnd();

        glPopMatrix();
    }

    // Draw character with AAA quality
    glPushMatrix();
    glTranslatef(0.0f, 0.5f, 0.0f);

    // Character with realistic lighting
    float charR = 0.75f, charG = 0.5f, charB = 0.25f;

    glBegin(GL_QUADS);

    // Front face
    glColor3f(charR * 1.3f, charG * 1.3f, charB * 1.3f);
    glVertex3f(-0.5f, -0.5f, 0.5f);
    glVertex3f(0.5f, -0.5f, 0.5f);
    glVertex3f(0.5f, 0.5f, 0.5f);
    glVertex3f(-0.5f, 0.5f, 0.5f);

    // Back face
    glColor3f(charR * 0.5f, charG * 0.5f, charB * 0.5f);
    glVertex3f(-0.5f, -0.5f, -0.5f);
    glVertex3f(-0.5f, 0.5f, -0.5f);
    glVertex3f(0.5f, 0.5f, -0.5f);
    glVertex3f(0.5f, -0.5f, -0.5f);

    // Left face
    glColor3f(charR * 0.8f, charG * 0.8f, charB * 0.8f);
    glVertex3f(-0.5f, -0.5f, -0.5f);
    glVertex3f(-0.5f, -0.5f, 0.5f);
    glVertex3f(-0.5f, 0.5f, 0.5f);
    glVertex3f(-0.5f, 0.5f, -0.5f);

    // Right face
    glColor3f(charR * 0.9f, charG * 0.9f, charB * 0.9f);
    glVertex3f(0.5f, -0.5f, -0.5f);
    glVertex3f(0.5f, 0.5f, -0.5f);
    glVertex3f(0.5f, 0.5f, 0.5f);
    glVertex3f(0.5f, -0.5f, 0.5f);

    // Top face
    glColor3f(charR * 1.1f, charG * 1.1f, charB * 1.1f);
    glVertex3f(-0.5f, 0.5f, -0.5f);
    glVertex3f(0.5f, 0.5f, -0.5f);
    glVertex3f(0.5f, 0.5f, 0.5f);
    glVertex3f(-0.5f, 0.5f, 0.5f);

    // Bottom face
    glColor3f(charR * 0.4f, charG * 0.4f, charB * 0.4f);
    glVertex3f(-0.5f, -0.5f, -0.5f);
    glVertex3f(0.5f, -0.5f, -0.5f);
    glVertex3f(0.5f, -0.5f, 0.5f);
    glVertex3f(-0.5f, -0.5f, 0.5f);
    glEnd();

    glPopMatrix();

    glDisable(GL_LIGHTING);
    glDisable(GL_FOG);
    glDisable(GL_CULL_FACE);
}

void Renderer::RenderLoading(float progress) {
    if (!m_initialized || !m_window) return;

    int width, height;
    m_window->GetSize(width, height);

    // Draw gradient background (dark blue to black)
    glBegin(GL_QUADS);
    glColor3f(0.02f, 0.02f, 0.05f);
    glVertex2f(0, 0);
    glColor3f(0.01f, 0.01f, 0.02f);
    glVertex2f(width, 0);
    glColor3f(0.01f, 0.01f, 0.02f);
    glVertex2f(width, height);
    glColor3f(0.02f, 0.02f, 0.05f);
    glVertex2f(0, height);
    glEnd();

    // Draw loading box
    glColor3f(0.1f, 0.1f, 0.2f);
    glBegin(GL_QUADS);
    glVertex2f(width/2 - 200, height/2 - 80);
    glVertex2f(width/2 + 200, height/2 - 80);
    glVertex2f(width/2 + 200, height/2 + 80);
    glVertex2f(width/2 - 200, height/2 + 80);
    glEnd();

    // Draw loading box border
    glColor3f(0.5f, 0.6f, 0.5f);
    glLineWidth(2.0f);
    glBegin(GL_LINE_LOOP);
    glVertex2f(width/2 - 200, height/2 - 80);
    glVertex2f(width/2 + 200, height/2 - 80);
    glVertex2f(width/2 + 200, height/2 + 80);
    glVertex2f(width/2 - 200, height/2 + 80);
    glEnd();
    glLineWidth(1.0f);

    // Draw title bar
    glColor3f(0.15f, 0.15f, 0.3f);
    glBegin(GL_QUADS);
    glVertex2f(width/2 - 200, height/2 - 80);
    glVertex2f(width/2 + 200, height/2 - 80);
    glVertex2f(width/2 + 200, height/2 - 60);
    glVertex2f(width/2 - 200, height/2 - 60);
    glEnd();

    // Draw title text (gold bar)
    glColor3f(1.0f, 0.85f, 0.2f);
    glBegin(GL_QUADS);
    glVertex2f(width/2 - 150, height/2 - 70);
    glVertex2f(width/2 + 150, height/2 - 70);
    glVertex2f(width/2 + 150, height/2 - 50);
    glVertex2f(width/2 - 150, height/2 - 50);
    glEnd();

    // Draw loading progress bar
    float progressWidth = 400;
    float progressHeight = 10;
    float barX = width/2 - progressWidth/2;
    float barY = height/2 + 10;

    // Progress background
    glColor3f(0.15f, 0.15f, 0.15f);
    glBegin(GL_QUADS);
    glVertex2f(barX, barY - progressHeight/2);
    glVertex2f(barX + progressWidth, barY - progressHeight/2);
    glVertex2f(barX + progressWidth, barY + progressHeight/2);
    glVertex2f(barX, barY + progressHeight/2);
    glEnd();

    // Progress fill (actual progress from engine)
    float fillWidth = progressWidth * progress;
    glColor3f(0.3f, 0.6f, 0.3f);
    glBegin(GL_QUADS);
    glVertex2f(barX, barY - progressHeight/2);
    glVertex2f(barX + fillWidth, barY - progressHeight/2);
    glVertex2f(barX + fillWidth, barY + progressHeight/2);
    glVertex2f(barX, barY + progressHeight/2);
    glEnd();

    // Draw loading text indicator
    glColor3f(0.7f, 0.7f, 0.9f);
    glBegin(GL_QUADS);
    glVertex2f(width/2 - 80, barY + 25);
    glVertex2f(width/2 + 80, barY + 25);
    glVertex2f(width/2 + 80, barY + 45);
    glVertex2f(width/2 - 80, barY + 45);
    glEnd();

    glColor3f(0.5f, 0.7f, 0.5f);
    glBegin(GL_QUADS);
    glVertex2f(width/2 - 60, barY + 30);
    glVertex2f(width/2 + 60, barY + 30);
    glVertex2f(width/2 + 60, barY + 40);
    glVertex2f(width/2 - 60, barY + 40);
    glEnd();
}

void Renderer::RenderError() {
    if (!m_initialized || !m_window) return;

    int width, height;
    m_window->GetSize(width, height);

    // Dark red background
    glBegin(GL_QUADS);
    glColor3f(0.2f, 0.05f, 0.05f);
    glVertex2f(0, 0);
    glColor3f(0.1f, 0.02f, 0.02f);
    glVertex2f(width, 0);
    glColor3f(0.1f, 0.02f, 0.02f);
    glVertex2f(width, height);
    glColor3f(0.2f, 0.05f, 0.05f);
    glVertex2f(0, height);
    glEnd();

    // Error box
    glColor3f(0.2f, 0.1f, 0.1f);
    glBegin(GL_QUADS);
    glVertex2f(width/2 - 150, height/2 - 50);
    glVertex2f(width/2 + 150, height/2 - 50);
    glVertex2f(width/2 + 150, height/2 + 50);
    glVertex2f(width/2 - 150, height/2 + 50);
    glEnd();

    // Error border
    glColor3f(0.8f, 0.3f, 0.3f);
    glLineWidth(3.0f);
    glBegin(GL_LINE_LOOP);
    glVertex2f(width/2 - 150, height/2 - 50);
    glVertex2f(width/2 + 150, height/2 - 50);
    glVertex2f(width/2 + 150, height/2 + 50);
    glVertex2f(width/2 - 150, height/2 + 50);
    glEnd();
    glLineWidth(1.0f);

    // Error title bar
    glColor3f(0.6f, 0.1f, 0.1f);
    glBegin(GL_QUADS);
    glVertex2f(width/2 - 120, height/2 - 50);
    glVertex2f(width/2 + 120, height/2 - 50);
    glVertex2f(width/2 + 120, height/2 - 40);
    glVertex2f(width/2 - 120, height/2 - 40);
    glEnd();

    // Error title text
    glColor3f(1.0f, 0.8f, 0.8f);
    glBegin(GL_QUADS);
    glVertex2f(width/2 - 80, height/2 - 45);
    glVertex2f(width/2 + 80, height/2 - 45);
    glVertex2f(width/2 + 80, height/2 - 35);
    glVertex2f(width/2 - 80, height/2 - 35);
    glEnd();

    // Error message
    glColor3f(0.9f, 0.6f, 0.6f);
    glBegin(GL_QUADS);
    glVertex2f(width/2 - 130, height/2 + 10);
    glVertex2f(width/2 + 130, height/2 + 10);
    glVertex2f(width/2 + 130, height/2 + 40);
    glVertex2f(width/2 - 130, height/2 + 40);
    glEnd();

    glColor3f(0.7f, 0.5f, 0.5f);
    glBegin(GL_QUADS);
    glVertex2f(width/2 - 110, height/2 + 15);
    glVertex2f(width/2 + 110, height/2 + 15);
    glVertex2f(width/2 + 110, height/2 + 25);
    glVertex2f(width/2 - 110, height/2 + 25);
    glEnd();
}

void Renderer::RenderTerrain(const Terrain& terrain) {
    // Placeholder - terrain is now rendered in RenderGame()
}

void Renderer::RenderCharacter(const CharacterController& character) {
    // Placeholder - character is now rendered in RenderGame()
}