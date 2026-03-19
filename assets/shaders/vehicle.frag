#version 330 core

in vec3 frag_position;
in vec3 frag_normal;
in vec2 frag_texcoord;

out vec4 FragColor;

uniform vec3 u_light_pos;
uniform vec3 u_view_pos;
uniform vec3 u_light_color;
uniform vec3 u_vehicle_color;
uniform float u_ambient_intensity;

void main() {
    // Vehicle color from uniform
    vec3 color = u_vehicle_color;
    
    // Ambient lighting
    float ambient_strength = u_ambient_intensity * 0.3;
    vec3 ambient = ambient_strength * u_light_color;
    
    // Diffuse lighting
    vec3 norm = normalize(frag_normal);
    vec3 light_dir = normalize(u_light_pos - frag_position);
    float diff = max(dot(norm, light_dir), 0.0);
    vec3 diffuse = diff * u_light_color * u_ambient_intensity;
    
    // Specular lighting (more shiny for vehicle)
    float specular_strength = 0.5;
    vec3 view_dir = normalize(u_view_pos - frag_position);
    vec3 reflect_dir = reflect(-light_dir, norm);
    float spec = pow(max(dot(view_dir, reflect_dir), 0.0), 64.0);
    vec3 specular = specular_strength * spec * u_light_color;
    
    vec3 result = (ambient + diffuse + specular) * color;
    FragColor = vec4(result, 1.0);
}
