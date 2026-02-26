#pragma once
#include "GameLevel.hpp"
#include "WeatherSystem.hpp"
#include <string>
#include <mutex>
#include <array>
#include <enet/enet.h> // Для PlayerState

class SnapshotSystem {
    static std::mutex saveMutex;
public:
    static void SaveToFile(const GameLevel& level, const WeatherSystem& weather, const std::string& filename);
    static void LoadFromFile(const std::string& filename);
    static void StartAutoSave(const GameLevel& level, const WeatherSystem& weather);
};