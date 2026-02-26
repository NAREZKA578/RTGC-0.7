#include "AmbientSystem.hpp"
#include "../core/Logger.hpp"
#include <random>

AmbientSystem::AmbientSystem(AudioSystem* sys) : audioSystem(sys) {
    LOG(L"Система амбиентного звука инициализирована");

    // Добавление слоёв
    AmbientLayer wind;
    wind.soundName = "wind";
    wind.minInterval = 10.0f;
    wind.maxInterval = 20.0f;
    AddLayer(wind);

    AmbientLayer birds;
    birds.soundName = "birds";
    birds.minInterval = 5.0f;
    birds.maxInterval = 15.0f;
    AddLayer(birds);
}

void AmbientSystem::AddLayer(const AmbientLayer& layer) {
    AmbientLayer l = layer;
    l.nextPlayTime = currentTime + (l.minInterval + (l.maxInterval - l.minInterval) * (rand() / (float)RAND_MAX));
    layers.push_back(l);
}

void AmbientSystem::Update(float dt) {
    currentTime += dt;
    for (auto& layer : layers) {
        if (currentTime >= layer.nextPlayTime) {
            audioSystem->PlayAt(layer.soundName, {0,0,0}, false);
            layer.nextPlayTime = currentTime + (layer.minInterval + (layer.maxInterval - layer.minInterval) * (rand() / (float)RAND_MAX));
        }
    }
}