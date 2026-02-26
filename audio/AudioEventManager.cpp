#include "AudioEventManager.hpp"
#include "../core/Logger.hpp"

AudioEventManager::AudioEventManager(AudioSystem* sys) : audioSystem(sys) {
    // Регистрация стандартных событий
    RegisterEvent("Footstep", [](AudioSystem& sys) {
        sys.PlayAt("footstep", {0,0,0});
    });
    RegisterEvent("Shoot", [](AudioSystem& sys) {
        sys.PlayAt("shoot", {0,0,0});
    });
    RegisterEvent("Explosion", [](AudioSystem& sys) {
        sys.PlayAt("explosion", {0,0,0});
    });
    LOG(L"Система аудио-событий инициализирована");
}

void AudioEventManager::RegisterEvent(const std::string& eventName, std::function<void(AudioSystem&)> callback) {
    eventMap[eventName] = callback;
}

void AudioEventManager::TriggerEvent(const std::string& eventName) {
    auto it = eventMap.find(eventName);
    if (it != eventMap.end()) {
        it->second(*audioSystem);
    }
}