#pragma once
#include <string>
#include <functional>
#include <unordered_map>
#include "AudioSystem.hpp"

class AudioEventManager {
    std::unordered_map<std::string, std::function<void(AudioSystem&)>> eventMap;
    AudioSystem* audioSystem = nullptr;

public:
    AudioEventManager(AudioSystem* sys);
    void RegisterEvent(const std::string& eventName, std::function<void(AudioSystem&)> callback);
    void TriggerEvent(const std::string& eventName);
};