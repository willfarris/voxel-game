#version 310 es

precision mediump float;

in vec2 v_tex_coords;

uniform sampler2D composite_output;

out vec4 color;

void main() {
    color = texture(composite_output, v_tex_coords);
}