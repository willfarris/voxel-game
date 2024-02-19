#version 310 es

precision mediump float;


layout (location = 0) in vec3 position;
layout (location = 1) in vec3 normal;
layout (location = 2) in vec2 tex_coords;
layout (location = 3) in float vtype;

out vec2 v_tex_coords;

#define WIND_SPEED 2.0

void main() {
    v_tex_coords = tex_coords;
    gl_Position = vec4(position, 1.0);
}