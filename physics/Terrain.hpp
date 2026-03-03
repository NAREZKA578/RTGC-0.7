#pragma once
#include "../math/Vector3.hpp"
#include <vector>
#include <random>

class Terrain {
private:
    std::vector<float> heights;
    int size;
    float scale;
    std::mt19937 rng;
    
    float Noise(float x, float z) const;
    float InterpolatedNoise(float x, float z) const;
    float PerlinNoise(float x, float z) const;

public:
    Terrain(int sz = 256, float sc = 0.02f);
    void Initialize();
    void Update(float dt);
    
    float GetHeightAt(float x, float z) const;
    Vector3 GetNormalAt(float x, float z) const;
    
    const std::vector<float>& GetHeights() const { return heights; }
    int GetSize() const { return size; }
};