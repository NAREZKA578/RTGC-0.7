#pragma once
#include <vector>
#include <glad/glad.h>
#include "Logger.hpp"
#include <cstddef> // для offsetof

struct Vertex {
    float x, y, z, nx, ny, nz, u, v;
};

class Mesh {
public:
    std::vector<Vertex> vertices;
    std::vector<unsigned int> indices;
    unsigned int VAO, VBO, EBO;

    Mesh(const std::vector<Vertex>& v, const std::vector<unsigned int>& i);
    void Render() const;
    ~Mesh();
};