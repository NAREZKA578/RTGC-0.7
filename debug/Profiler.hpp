#pragma once
#include <string>
#include <chrono>
#include <unordered_map>

class Profiler {
    struct ProfileData {
        std::chrono::high_resolution_clock::time_point start;
        long long totalTime = 0;
        int callCount = 0;
    };
    std::unordered_map<std::string, ProfileData> data;

public:
    void BeginSample(const std::string& name);
    void EndSample(const std::string& name);
    void PrintResults();
};