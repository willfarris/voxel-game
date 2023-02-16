#version 310 es

precision mediump float;

in vec2 v_tex_coords;

uniform sampler2D skybox_texture;

out vec4 color;

void main() {
    color = texture2D(skybox_texture, v_tex_coords);
}