#version 310 es

precision mediump float;

in vec3 v_position;
in vec3 v_normal;
in vec2 v_tex_coords;
flat in float v_type;

uniform float time;
uniform sampler2D texture_map;

layout (location = 0) out vec4 position;
layout (location = 1) out vec4 normal;
layout (location = 2) out vec4 albedo;

void main() {
    vec4 color = texture(texture_map, v_tex_coords).rgba;
    if (color.a < 0.5) { discard; }

    position = vec4(v_position, 1.0);
    normal = vec4(v_normal, 1.0);
    albedo = color;
}
