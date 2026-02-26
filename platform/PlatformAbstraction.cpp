#include "PlatformAbstraction.hpp"
#include <thread>
#include <chrono>
#include <filesystem>

#ifdef _WIN32
    #include <windows.h>
    #include <io.h>
    #define access _access_s
    #define F_OK 0
#else
    #include <unistd.h>
#endif

std::string Platform::GetPlatformName() {
#ifdef _WIN32
    return "Windows";
#elif __linux__
    return "Linux";
#elif __APPLE__
    return "macOS";
#else
    return "Unknown";
#endif
}

void Platform::Sleep(unsigned int milliseconds) {
    std::this_thread::sleep_for(std::chrono::milliseconds(milliseconds));
}

bool Platform::FileExists(const std::string& path) {
    return std::filesystem::exists(path);
}

std::string Platform::GetExecutablePath() {
    return std::filesystem::current_path().string();
}