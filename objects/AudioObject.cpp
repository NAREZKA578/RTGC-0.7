#include "AudioObject.hpp"
#include "../core/Logger.hpp"

AudioObject::AudioObject(const std::string& name, const Vector3& pos, const std::string& sound, AudioSystem* sys)
    : GameObject(name, pos), soundName(sound), audioSystem(sys), isPlaying(false) {
    LOG(L"Создан AudioObject: " + std::wstring(name.begin(), name.end()));
}

void AudioObject::Play(bool loop) {
    if (audioSystem) {
        audioSystem->PlayAt(soundName, position.ToGLM(), loop);
        isPlaying = true;
    }
}

void AudioObject::Stop() {
    // Реализация остановки звука
    isPlaying = false;
}

void AudioObject::Update(float dt) {
    GameObject::Update(dt);
    // Здесь можно обновлять позицию звука при движении объекта
    if (audioSystem && isPlaying) {
        audioSystem->PlayAt(soundName, position.ToGLM(), true);
    }
}

AudioObject::~AudioObject() {
    if (isPlaying) {
        Stop();
    }
}