#version 330 core

layout (location = 0) in vec3 a_position;
layout (location = 1) in vec3 a_normal;
layout (location = 2) in float a_moisture;
layout (location = 3) in float a_slope;
layout (location = 4) in vec2 a_texcoord;

uniform mat4 u_model;
uniform mat4 u_view;
uniform mat4 u_projection;

out vec3 frag_position;
out vec3 frag_normal;
out float frag_moisture;
out float frag_slope;
out vec2 frag_texcoord;

void main() {
    vec4 world_pos = u_model * vec4(a_position, 1.0);
    gl_Position = u_projection * u_view * world_pos;
    
    frag_position = world_pos.xyz;
    frag_normal = mat3(transpose(inverse(u_model))) * a_normal;
    frag_moisture = a_moisture;
    frag_slope = a_slope;
    frag_texcoord = a_texcoord;
}
