#version 330 core

in vec3 frag_position;
in vec3 frag_normal;
in float frag_moisture;
in float frag_slope;
in vec2 frag_texcoord;

out vec4 FragColor;

uniform vec3 u_light_pos;
uniform vec3 u_view_pos;
uniform vec3 u_light_color;
uniform float u_ambient_intensity;

// Исп-7: Поддержка solid color для HUD
uniform vec4 u_color;
uniform bool u_use_solid_color;

// Задача 10: Fog (туман дистанции)
uniform float u_fog_start;
uniform float u_fog_end;
uniform vec3 u_fog_color;

void main() {
    // Исп-7: Если режим solid color - просто вернуть цвет
    if (u_use_solid_color) {
        FragColor = u_color;
        return;
    }

    // Determine color based on height and biome
    vec3 color;
    
    // Simple height-based coloring (will be replaced with splatmap texturing later)
    float height_norm = frag_position.y / 100.0;  // Normalize to ~100m max
    
    if (height_norm < -0.3) {
        // Deep ocean
        color = vec3(0.1, 0.2, 0.4);
    } else if (height_norm < 0.0) {
        // Ocean
        color = vec3(0.2, 0.3, 0.5);
    } else if (height_norm < 0.05) {
        // Beach
        color = vec3(0.76, 0.7, 0.5);
    } else if (height_norm < 0.3) {
        // Plains/Forest - blend based on moisture
        float grass_factor = smoothstep(0.0, 0.5, frag_moisture);
        color = mix(vec3(0.4, 0.7, 0.2), vec3(0.2, 0.5, 0.1), grass_factor);
    } else if (height_norm < 0.6) {
        // Hills
        color = vec3(0.5, 0.5, 0.4);
    } else if (height_norm < 0.8) {
        // Mountains - blend with snow based on slope
        float snow_factor = smoothstep(0.3, 0.7, frag_slope);
        color = mix(vec3(0.6, 0.6, 0.6), vec3(0.9, 0.9, 0.95), snow_factor);
    } else {
        // Snow
        color = vec3(0.9, 0.9, 0.95);
    }
    
    // Ambient lighting
    float ambient_strength = u_ambient_intensity * 0.3;
    vec3 ambient = ambient_strength * u_light_color;
    
    // Diffuse lighting
    vec3 norm = normalize(frag_normal);
    vec3 light_dir = normalize(u_light_pos - frag_position);
    float diff = max(dot(norm, light_dir), 0.0);
    vec3 diffuse = diff * u_light_color * u_ambient_intensity;
    
    // Specular lighting
    float specular_strength = 0.3;
    vec3 view_dir = normalize(u_view_pos - frag_position);
    vec3 reflect_dir = reflect(-light_dir, norm);
    float spec = pow(max(dot(view_dir, reflect_dir), 0.0), 32.0);
    vec3 specular = specular_strength * spec * u_light_color;
    
    vec3 result = (ambient + diffuse + specular) * color;
    
    // Задача 10: Туман по расстоянию от камеры
    float dist = length(frag_position - u_view_pos);
    float fog_factor = clamp((dist - u_fog_start) / (u_fog_end - u_fog_start), 0.0, 1.0);
    result = mix(result, u_fog_color, fog_factor);
    
    FragColor = vec4(result, 1.0);
}
