#version 310 es

precision mediump float;

in vec2 v_tex_coords;

uniform sampler2D lighting_output;

out vec4 color;

void main() {
    color = texture(lighting_output, v_tex_coords);
}