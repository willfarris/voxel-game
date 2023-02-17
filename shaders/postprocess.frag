#version 310 es

precision mediump float;

in vec2 v_tex_coords;

layout (location = 0) uniform sampler2D ssao;
layout (location = 1) uniform sampler2D albedo;
layout (location = 2) uniform sampler2D position;
layout (location = 3) uniform sampler2D normal;

uniform vec2 resolution;
uniform float ssao_noise_size;

out vec4 color;

void main() {
    vec2 uv = v_tex_coords;
    vec2 px = 1. / resolution;

    vec4 albedo = texture(albedo, uv);
    vec4 position = texture(position, uv);
    vec4 normal = texture(normal, uv);
    vec3 fog = vec3(0.4, 0.6, 1.0);

    float vignette = 1.0 - 0.1 *length(uv - 0.5);

    vec3 light_dir = normalize(vec3(sqrt(2.)));
    float ambient = 0.8;
    float diffuse = max(dot(normal.xyz, light_dir), 0.0);

    vec3 out_color = albedo.rgb * min(diffuse + ambient, 2.0);

    color = vec4(out_color.rgb, albedo.a);
}