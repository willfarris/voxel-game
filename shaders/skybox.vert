#version 310 es

precision mediump float;

layout (location = 0) in vec3 position;
layout (location = 1) in vec3 normal;
layout (location = 2) in vec2 tex_coords;
layout (location = 3) in float vtype;

uniform mat4 model_matrix;
uniform mat4 view_matrix;
uniform mat4 perspective_matrix;

out vec2 v_tex_coords;

void main() {
    vec4 position_worldspace = model_matrix * vec4(position, 1.0);
    vec4 position_viewspace = view_matrix * position_worldspace;

    v_tex_coords = tex_coords;

    gl_Position = perspective_matrix * position_viewspace;
}