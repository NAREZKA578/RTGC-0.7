#pragma once
#include "GameObject.hpp"
#include <string>
#include "../audio/AudioSystem.hpp"

class AudioObject : public GameObject {
    std::string soundName;
    AudioSystem* audioSystem;
    bool isPlaying;

public:
    AudioObject(const std::string& name, const Vector3& pos, const std::string& sound, AudioSystem* sys);
    ~AudioObject() override;

    void Play(bool loop = false);
    void Stop();
    void SetSound(const std::string& sound) { soundName = sound; }
    void SetAudioSystem(AudioSystem* sys) { audioSystem = sys; }

    void Update(float dt) override;
};