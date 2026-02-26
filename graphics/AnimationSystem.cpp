#include "AnimationSystem.hpp"
#include "Logger.hpp"
#include <glm/gtc/matrix_transform.hpp>

void AnimationClip::Update(float dt) {
    if (!isPlaying) return;
    currentTime += dt;
    if (currentTime >= duration) {
        currentTime = 0.0f; // Зацикливание
    }
}

void AnimationSystem::LoadAnimationClip(const std::string& name, const std::string& path) {
    AnimationClip clip;
    clip.name = name;
    clip.duration = 2.0f; // Пример
    clip.isPlaying = false;

    // Загрузка из .fbx через Assimp или другую библиотеку
    // clip.LoadFromFile(path);

    clips[name] = clip;
    LOG(L"Анимация загружена: " + std::wstring(name.begin(), name.end()));
}

void AnimationSystem::PlayAnimation(const std::string& clipName) {
    auto it = clips.find(clipName);
    if (it != clips.end()) {
        it->second.isPlaying = true;
        it->second.currentTime = 0.0f;
        activeAnimations.push_back(clipName);
    }
}

void AnimationSystem::Update(float dt) {
    for (const auto& clipName : activeAnimations) {
        auto it = clips.find(clipName);
        if (it != clips.end()) {
            it->second.Update(dt);
        }
    }
}

void AnimationSystem::ApplyTransforms(const std::string& clipName, class Mesh& mesh) {
    auto it = clips.find(clipName);
    if (it != clips.end()) {
        // Применение трансформаций к вершинам меша
        // mesh.ApplyBoneTransforms(it->second.bones);
    }
}