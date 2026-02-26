#include "SnapshotSystem.hpp"
#include <fstream>
#include <thread>
#include <mutex>
#include <chrono>

std::mutex SnapshotSystem::saveMutex;

void SnapshotSystem::SaveToFile(const GameLevel& level, const WeatherSystem& weather, const std::string& filename) {
    std::lock_guard<std::mutex> lock(saveMutex);
    std::ofstream file(filename);
    if (file.is_open()) {
        file << "Snapshot saved at: " << std::chrono::duration_cast<std::chrono::seconds>(
            std::chrono::system_clock::now().time_since_epoch()).count() << "\n";
        file << "Weather: " << weather.timeOfDay << "h\n";
        file << "Entities: " << level.ecs.registry.size() << "\n";
        LOG(L"Снапшот сохранён: " + std::wstring(filename.begin(), filename.end()));
    } else {
        ERROR(L"Не удалось сохранить снапшот: " + std::wstring(filename.begin(), filename.end()));
    }
}

void SnapshotSystem::LoadFromFile(const std::string& filename) {
    LOG(L"Загрузка снапшота: " + std::wstring(filename.begin(), filename.end()));
}

void SnapshotSystem::StartAutoSave(const GameLevel& level, const WeatherSystem& weather) {
    std::thread([level, weather]() mutable {
        while (true) {
            std::this_thread::sleep_for(std::chrono::minutes(5));
            SaveToFile(level, weather, "save_" + std::to_string(
                std::chrono::duration_cast<std::chrono::seconds>(
                    std::chrono::system_clock::now().time_since_epoch()).count()) + ".dat");
        }
    }).detach();
}