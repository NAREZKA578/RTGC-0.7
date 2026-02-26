#pragma once
#include <windows.h>
#include "../components/CharacterComponent.hpp"
#include <imgui.h>
#include <imgui_impl_win32.h>
#include <imgui_impl_opengl3.h>

class HudUI {
public:
    static void Render(CharacterComponent& character);
    static void Init(HWND hwnd);
};