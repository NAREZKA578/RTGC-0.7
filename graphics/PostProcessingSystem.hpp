#pragma once
#include <glad/glad.h>
#include <glm/glm.hpp>
#include "Shader.hpp"

class PostProcessingSystem {
    GLuint framebuffer = 0;
    GLuint textureColorbuffer = 0;
    GLuint renderbuffer = 0;
    Shader* postProcessShader = nullptr;
    bool bloomEnabled = true;
    bool motionBlurEnabled = false;
    GLuint quadVAO = 0, quadVBO = 0;

public:
    PostProcessingSystem(Shader* shader);
    ~PostProcessingSystem();

    void Setup(unsigned int width, unsigned int height);
    void BeginRender();
    void EndRender();
    void Render();
    void SetBloom(bool enabled) { bloomEnabled = enabled; }
    void SetMotionBlur(bool enabled) { motionBlurEnabled = enabled; }
};