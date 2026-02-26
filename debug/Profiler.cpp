#include "Profiler.hpp"
#include "../core/Logger.hpp"
#include <iostream>

void Profiler::BeginSample(const std::string& name) {
    data[name].start = std::chrono::high_resolution_clock::now();
}

void Profiler::EndSample(const std::string& name) {
    auto end = std::chrono::high_resolution_clock::now();
    auto duration = std::chrono::duration_cast<std::chrono::microseconds>(end - data[name].start).count();
    data[name].totalTime += duration;
    data[name].callCount++;
}

void Profiler::PrintResults() {
    for (const auto& [name, data] : data) {
        if (data.callCount > 0) {
            long long avgTime = data.totalTime / data.callCount;
            LOG(L"Профиль [" + std::wstring(name.begin(), name.end()) + L"]: " +
                std::to_wstring(avgTime) + L" мкс (среднее), " +
                std::to_wstring(data.callCount) + L" вызовов");
        }
    }
}