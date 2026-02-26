#pragma once
#include <glad/glad.h>
#include <glm/glm.hpp>

class ShadowMap {
    GLuint depthMapFBO = 0;
    GLuint depthMapTexture = 0;
    unsigned int SHADOW_WIDTH = 2048, SHADOW_HEIGHT = 2048;
    glm::mat4 lightSpaceMatrix{1.0f};

public:
    ShadowMap();
    ~ShadowMap();

    void RenderShadowMap(class Renderer& renderer, class Shader& depthShader);
    void BindForWriting();
    void BindForReading(GLuint unit);
    const glm::mat4& GetLightSpaceMatrix() const { return lightSpaceMatrix; }
};