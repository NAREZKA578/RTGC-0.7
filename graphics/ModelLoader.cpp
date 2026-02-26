#include "ModelLoader.hpp"
#include <fstream>
#include <sstream>
#include <iostream>
#include <map>

std::vector<Mesh*> ModelLoader::LoadOBJ(const std::string& filepath) {
    std::vector<Mesh*> meshes;
    
    std::ifstream file(filepath);
    if (!file.is_open()) {
        std::cerr << "Could not open file: " << filepath << std::endl;
        return meshes;
    }

    std::vector<glm::vec3> positions;
    std::vector<glm::vec3> normals;
    std::vector<glm::vec2> texCoords;
    std::vector<Vertex> vertices;
    std::vector<unsigned int> indices;

    std::string line;
    while (std::getline(file, line)) {
        std::istringstream iss(line);
        std::string prefix;
        iss >> prefix;

        if (prefix == "v") {
            glm::vec3 pos;
            iss >> pos.x >> pos.y >> pos.z;
            positions.push_back(pos);
        }
        else if (prefix == "vn") {
            glm::vec3 norm;
            iss >> norm.x >> norm.y >> norm.z;
            normals.push_back(norm);
        }
        else if (prefix == "vt") {
            glm::vec2 uv;
            iss >> uv.x >> uv.y;
            texCoords.push_back(uv);
        }
        else if (prefix == "f") {
            std::string vertexStr;
            std::vector<std::string> faceVertices;
            
            while (iss >> vertexStr) {
                faceVertices.push_back(vertexStr);
            }
            
            // Process each vertex in the face
            for (const auto& vert : faceVertices) {
                std::replace(vert.begin(), vert.end(), '/', ' ');
                std::istringstream vertIss(vert);
                
                int posIdx, texIdx, normIdx;
                vertIss >> posIdx >> texIdx >> normIdx;
                
                // OBJ indices are 1-based, convert to 0-based
                posIdx--; normIdx--; texIdx--;
                
                Vertex v;
                if (posIdx >= 0 && posIdx < positions.size()) {
                    v.x = positions[posIdx].x;
                    v.y = positions[posIdx].y;
                    v.z = positions[posIdx].z;
                }
                
                if (normIdx >= 0 && normIdx < normals.size()) {
                    v.nx = normals[normIdx].x;
                    v.ny = normals[normIdx].y;
                    v.nz = normals[normIdx].z;
                }
                
                if (texIdx >= 0 && texIdx < texCoords.size()) {
                    v.u = texCoords[texIdx].x;
                    v.v = texCoords[texIdx].y;
                }
                
                vertices.push_back(v);
            }
        }
    }
    
    // Create a single mesh from all vertices
    if (!vertices.empty()) {
        std::vector<unsigned int> tempIndices;
        for (unsigned int i = 0; i < vertices.size(); ++i) {
            tempIndices.push_back(i);
        }
        
        Mesh* mesh = new Mesh(vertices, tempIndices);
        meshes.push_back(mesh);
    }

    return meshes;
}