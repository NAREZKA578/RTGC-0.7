#pragma once
#include <string>

namespace Platform {
    std::string GetPlatformName();
    void Sleep(unsigned int milliseconds);
    bool FileExists(const std::string& path);
    std::string GetExecutablePath();
}