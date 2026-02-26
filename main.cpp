#pragma comment(linker, "/SUBSYSTEM:WINDOWS")
#include "EngineMinimal.hpp"
#include <Windows.h>

int WINAPI WinMain(HINSTANCE hInstance, HINSTANCE hPrevInstance, LPSTR lpCmdLine, int nCmdShow) {
    Engine engine;

    if (!engine.Initialize()) {
        MessageBoxA(NULL, "Не удалось инициализировать движок RTGC", "Ошибка", MB_OK | MB_ICONERROR);
        return -1;
    }

    engine.Run();

    return 0;
}
