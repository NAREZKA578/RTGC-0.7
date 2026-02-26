#pragma once
#include "AudioSystem.hpp"
#include <string>
#include <vector>

struct AmbientLayer {
    std::string soundName;
    float volume = 1.0f;
    float minInterval = 5.0f;
    float maxInterval = 15.0f;
    float nextPlayTime = 0.0f;
};

class AmbientSystem {
    AudioSystem* audioSystem = nullptr;
    std::vector<AmbientLayer> layers;
    float currentTime = 0.0f;

public:
    AmbientSystem(AudioSystem* sys);
    void AddLayer(const AmbientLayer& layer);
    void Update(float dt);
};