#pragma once
#include "Mesh.hpp"
#include <vector>
#include <string>

class ModelLoader {
public:
    static std::vector<Mesh*> LoadOBJ(const std::string& filepath);
};