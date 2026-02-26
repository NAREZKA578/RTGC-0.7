#pragma once
#include "../math/Vector3.hpp"
#include <vector>

class Terrain {
private:
    std::vector<float> heights;
    int width = 64;
    int depth = 64;
    
public:
    Terrain();
    void Initialize();
    void Update(float dt);
    void GetVertices(std::vector<float>& vertices) const;
    float GetHeightAt(float x, float z) const;
    ~Terrain();
};
