#include "Terrain.hpp"
#include "../core/Logger.hpp"
#include <random>

Terrain::Terrain() {
    heights.resize(width * depth, 0.0f);
}

void Terrain::Initialize() {
    std::random_device rd;
    std::mt19937 gen(rd());
    std::uniform_real_distribution<> dis(0.0, 1.0);
    
    for (int z = 0; z < depth; ++z) {
        for (int x = 0; x < width; ++x) {
            int index = z * width + x;
            float baseHeight = 0.0f;
            
            if (x < width / 4 || x > 3 * width / 4 || z < depth / 4 || z > 3 * depth / 4) {
                baseHeight = 1.0f;
            }
            
            heights[index] = baseHeight + (dis(gen) - 0.5f) * 0.5f;
        }
    }
    
    Logger::Log("Terrain инициализирован: ", width, "x", depth);
}

void Terrain::Update(float dt) {
    // Static terrain doesn't need updates
}

void Terrain::GetVertices(std::vector<float>& vertices) const {
    vertices.clear();
    
    for (int z = 0; z < depth - 1; ++z) {
        for (int x = 0; x < width - 1; ++x) {
            float x1 = static_cast<float>(x) - width / 2.0f;
            float z1 = static_cast<float>(z) - depth / 2.0f;
            float y1 = heights[z * width + x] * 2.0f;
            
            float x2 = static_cast<float>(x + 1) - width / 2.0f;
            float z2 = static_cast<float>(z) - depth / 2.0f;
            float y2 = heights[z * width + (x + 1)] * 2.0f;
            
            float x3 = static_cast<float>(x) - width / 2.0f;
            float z3 = static_cast<float>(z + 1) - depth / 2.0f;
            float y3 = heights[(z + 1) * width + x] * 2.0f;
            
            float x4 = static_cast<float>(x + 1) - width / 2.0f;
            float z4 = static_cast<float>(z + 1) - depth / 2.0f;
            float y4 = heights[(z + 1) * width + (x + 1)] * 2.0f;
            
            // Triangle 1
            vertices.push_back(x1); vertices.push_back(y1); vertices.push_back(z1);
            vertices.push_back(x2); vertices.push_back(y2); vertices.push_back(z2);
            vertices.push_back(x3); vertices.push_back(y3); vertices.push_back(z3);
            
            // Triangle 2
            vertices.push_back(x2); vertices.push_back(y2); vertices.push_back(z2);
            vertices.push_back(x4); vertices.push_back(y4); vertices.push_back(z4);
            vertices.push_back(x3); vertices.push_back(y3); vertices.push_back(z3);
        }
    }
}

float Terrain::GetHeightAt(float x, float z) const {
    int gridX = static_cast<int>(x + width / 2.0f);
    int gridZ = static_cast<int>(z + depth / 2.0f);
    
    if (gridX < 0 || gridX >= width || gridZ < 0 || gridZ >= depth) {
        return 0.0f;
    }
    
    return heights[gridZ * width + gridX] * 2.0f;
}

Terrain::~Terrain() {
    Logger::Log("Terrain удален");
}
