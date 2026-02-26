#pragma once
#include <vector>
#include <string>
#include <unordered_map>
#include <glm/glm.hpp>
#include <glm/gtc/quaternion.hpp>

struct Bone {
    std::string name;
    glm::mat4 offsetMatrix{1.0f};
    glm::mat4 finalTransform{1.0f};
};

struct AnimationClip {
    std::string name;
    float duration = 0.0f;
    float currentTime = 0.0f;
    std::vector<Bone> bones;
    bool isPlaying = false;

    void Update(float dt);
    void ApplyToMesh(class Mesh& mesh);
};

class AnimationSystem {
    std::unordered_map<std::string, AnimationClip> clips;
    std::vector<std::string> activeAnimations;

public:
    void LoadAnimationClip(const std::string& name, const std::string& path);
    void PlayAnimation(const std::string& clipName);
    void Update(float dt);
    void ApplyTransforms(const std::string& clipName, class Mesh& mesh);
};