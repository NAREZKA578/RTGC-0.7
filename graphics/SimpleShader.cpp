#include "SimpleShader.hpp"
#include <iostream>
#include <vector>

SimpleShader::SimpleShader()
    : m_program(0)
    , m_compiled(false)
{
}

SimpleShader::~SimpleShader() {
    if (m_program) {
        glDeleteProgram(m_program);
    }
}

GLuint SimpleShader::CompileShader(GLenum type, const std::string& source) {
    GLuint shader = glCreateShader(type);
    const char* src = source.c_str();
    glShaderSource(shader, 1, &src, nullptr);
    glCompileShader(shader);

    GLint success;
    glGetShaderiv(shader, GL_COMPILE_STATUS, &success);
    if (!success) {
        char infoLog[512];
        glGetShaderInfoLog(shader, 512, nullptr, infoLog);
        std::cout << "Shader compilation failed: " << infoLog << std::endl;
        glDeleteShader(shader);
        return 0;
    }

    return shader;
}

bool SimpleShader::LinkProgram(GLuint vertexShader, GLuint fragmentShader) {
    m_program = glCreateProgram();
    glAttachShader(m_program, vertexShader);
    glAttachShader(m_program, fragmentShader);
    glLinkProgram(m_program);

    GLint success;
    glGetProgramiv(m_program, GL_LINK_STATUS, &success);
    if (!success) {
        char infoLog[512];
        glGetProgramInfoLog(m_program, 512, nullptr, infoLog);
        std::cout << "Program linking failed: " << infoLog << std::endl;
        return false;
    }

    glDeleteShader(vertexShader);
    glDeleteShader(fragmentShader);
    return true;
}

bool SimpleShader::LoadFromSource(const std::string& vertexSource, const std::string& fragmentSource) {
    GLuint vertexShader = CompileShader(GL_VERTEX_SHADER, vertexSource);
    if (!vertexShader) return false;

    GLuint fragmentShader = CompileShader(GL_FRAGMENT_SHADER, fragmentSource);
    if (!fragmentShader) {
        glDeleteShader(vertexShader);
        return false;
    }

    if (!LinkProgram(vertexShader, fragmentShader)) {
        return false;
    }

    m_compiled = true;
    return true;
}

void SimpleShader::Use() const {
    if (m_compiled) {
        glUseProgram(m_program);
    }
}