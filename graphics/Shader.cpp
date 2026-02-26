#include "Shader.hpp"
#include <iostream>
#include <fstream>
#include <sstream>
#include <vector>

Shader::Shader(const char* vertexPath, const char* fragmentPath) : id(0) {
    std::string vCode = ReadFile(vertexPath);
    std::string fCode = ReadFile(fragmentPath);
    if (vCode.empty() || fCode.empty()) {
        ERROR(L"Не удалось прочитать файлы шейдера");
        return;
    }
    const char* vSrc = vCode.c_str();
    const char* fSrc = fCode.c_str();

    GLuint vShader = glCreateShader(GL_VERTEX_SHADER);
    glShaderSource(vShader, 1, &vSrc, nullptr);
    glCompileShader(vShader);
    CheckCompileErrors(vShader, "VERTEX");

    GLuint fShader = glCreateShader(GL_FRAGMENT_SHADER);
    glShaderSource(fShader, 1, &fSrc, nullptr);
    glCompileShader(fShader);
    CheckCompileErrors(fShader, "FRAGMENT");

    id = glCreateProgram();
    glAttachShader(id, vShader);
    glAttachShader(id, fShader);
    glLinkProgram(id);
    CheckCompileErrors(id, "PROGRAM");

    glDeleteShader(vShader);
    glDeleteShader(fShader);
}

void Shader::Use() const {
    if (id != 0) {
        glUseProgram(id);
    }
}

std::string Shader::ReadFile(const std::string& path) {
    std::ifstream file(path);
    if (!file.is_open()) {
        ERROR(L"Не удалось открыть шейдер: " + std::wstring(path.begin(), path.end()));
        return "";
    }
    std::stringstream buffer;
    buffer << file.rdbuf();
    return buffer.str();
}

void Shader::CheckCompileErrors(unsigned int shader, const std::string& type) {
    GLint success;
    GLchar infoLog[1024];
    if (type != "PROGRAM") {
        glGetShaderiv(shader, GL_COMPILE_STATUS, &success);
        if (!success) {
            glGetShaderInfoLog(shader, 1024, NULL, infoLog);
            ERROR(L"Ошибка компиляции шейдера (" + std::wstring(type.begin(), type.end()) + L"): " + std::wstring(infoLog, infoLog + strlen(infoLog)));
        }
    } else {
        glGetProgramiv(shader, GL_LINK_STATUS, &success);
        if (!success) {
            glGetProgramInfoLog(shader, 1024, NULL, infoLog);
            ERROR(L"Ошибка линковки программы: " + std::wstring(infoLog, infoLog + strlen(infoLog)));
        }
    }
}