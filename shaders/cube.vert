#version 310 es

precision mediump float;

in vec3 position;
in vec3 normal;
in vec2 tex_coords;
in int vtype;

uniform mat4 model_matrix;
uniform mat4 view_matrix;
uniform mat4 perspective_matrix;

out vec3 v_position;
out vec3 v_normal;
out vec2 v_tex_coords;

void main() {
    mat4 camera_matrix = perspective_matrix * view_matrix;

    vec4 position_worldspace = camera_matrix * model_matrix * vec4(position, 1.0);

    v_position = position_worldspace.xyz;
    v_normal = normal;
    v_tex_coords = tex_coords;

    gl_Position = position_worldspace;
}