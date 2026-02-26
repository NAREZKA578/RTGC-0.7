#include "GPUManager.hpp"
#include "../core/Logger.hpp"
#include <Windows.h>

extern "C" {
    __declspec(dllexport) DWORD NvOptimusEnablement = 0x00000001;
    __declspec(dllexport) int AmdPowerXpressRequestHighPerformance = 1;
}

namespace GPUManager {

    void InitializeGPUSelection() {
        Logger::Log("=== GPU Selection Initialized ===");
        Logger::Log("NVIDIA Optimus: ENABLED (High Performance)");
        Logger::Log("AMD PowerXpress: ENABLED (High Performance)");
    }

    void LogGPUInfo() {
        Logger::Log("=== GPU Information ===");
        Logger::Log("Attempting to use High Performance GPU...");
        Logger::Log("NVIDIA/AMD discrete GPU preferred over integrated graphics");
    }

    bool UseHighPerformanceGPU() {
        return true;
    }
}
