#include "AudioSystem.hpp"
#include "../core/Logger.hpp"

AudioSound::AudioSound(const std::string& file) : filename(file), loaded(false), volume(1.0f), looping(false) {
}

AudioSound::~AudioSound() {
    if (loaded) {
        Stop();
    }
}

bool AudioSound::Load() {
    // В реальной реализации здесь была бы загрузка аудиофайла
    loaded = true;
    Logger::Log("AudioSound загружен: ", filename);
    return true;
}

void AudioSound::Play() {
    if (!loaded) return;
    Logger::Log("Воспроизведение звука: ", filename);
}

void AudioSound::Stop() {
    if (!loaded) return;
    Logger::Log("Остановка звука: ", filename);
}

void AudioSound::Pause() {
    if (!loaded) return;
    Logger::Log("Пауза звука: ", filename);
}

void AudioSound::Resume() {
    if (!loaded) return;
    Logger::Log("Возобновление звука: ", filename);
}

AudioSystem::AudioSystem() : enabled(true), masterVolume(1.0f) {
}

AudioSystem::~AudioSystem() {
    Shutdown();
}

bool AudioSystem::Initialize() {
    Logger::Log("AudioSystem инициализирован");
    return true;
}

void AudioSystem::Shutdown() {
    StopAllSounds();
    for (auto sound : sounds) {
        delete sound;
    }
    sounds.clear();
    Logger::Log("AudioSystem выключен");
}

void AudioSystem::Update() {
    // Обновление 3D-позиций и других параметров
}

AudioSound* AudioSystem::LoadSound(const std::string& filename) {
    auto sound = new AudioSound(filename);
    if (sound->Load()) {
        sounds.push_back(sound);
        Logger::Log("Звук загружен в AudioSystem: ", filename);
        return sound;
    } else {
        delete sound;
        Logger::Error("Не удалось загрузить звук: ", filename);
        return nullptr;
    }
}

void AudioSystem::PlaySound(const std::string& filename) {
    for (auto sound : sounds) {
        // В реальной реализации здесь было бы сравнение по имени файла
        sound->Play();
        break;
    }
}

void AudioSystem::StopSound(const std::string& filename) {
    for (auto sound : sounds) {
        // В реальной реализации здесь было бы сравнение по имени файла
        sound->Stop();
        break;
    }
}

void AudioSystem::StopAllSounds() {
    for (auto sound : sounds) {
        sound->Stop();
    }
}