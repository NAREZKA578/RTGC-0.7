#pragma once
#include <glad/glad.h>
#include <string>
#include "Logger.hpp"

class Shader {
public:
    unsigned int id;

    Shader(const char* vertexPath, const char* fragmentPath);
    void Use() const;

private:
    std::string ReadFile(const std::string& path);
    void CheckCompileErrors(unsigned int shader, const std::string& type);
};