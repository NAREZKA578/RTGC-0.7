#pragma once
#include <string>
#include <GL/gl.h>

class SimpleShader {
private:
    GLuint m_program;
    bool m_compiled;

public:
    SimpleShader();
    ~SimpleShader();

    bool LoadFromSource(const std::string& vertexSource, const std::string& fragmentSource);
    bool IsCompiled() const { return m_compiled; }

    void Use() const;
    GLuint GetProgram() const { return m_program; }

private:
    GLuint CompileShader(GLenum type, const std::string& source);
    bool LinkProgram(GLuint vertexShader, GLuint fragmentShader);
};