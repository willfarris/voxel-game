#version 310 es

precision mediump float;

in vec3 v_position;
in vec3 v_normal;
in vec2 v_tex_coords;

uniform sampler2D texture_map;

out vec4 color;

void main() {

    vec4 tex_color = texture(texture_map, v_tex_coords).rgba;
    if (tex_color.a < 0.5) { discard; }

    color = vec4(tex_color.rgb, 1.0);
}