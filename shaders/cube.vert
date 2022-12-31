#version 310 es

precision mediump float;

layout (location = 0) in vec3 position;
layout (location = 1) in vec3 normal;
layout (location = 2) in vec2 tex_coords;
layout (location = 3) in int vtype;

uniform mat4 model_matrix;
uniform mat4 view_matrix;
uniform mat4 perspective_matrix;

out vec3 v_position;
out vec3 v_normal;
out vec2 v_tex_coords;
flat out int v_type;

void main() {
    mat4 camera_matrix = perspective_matrix * view_matrix;

    vec4 position_worldspace = camera_matrix * model_matrix * vec4(position, 1.0);

    /*if(vtype == 1) {
        position_worldspace.xz += mod(position.y, 1.0) * 0.03 * sin(WIND_SPEED * (time + position.y));
    }
    else if(vtype == 2) {
        position_worldspace.xz += 0.03 * sin(WIND_SPEED * (time + position.y + 0.1415));
    }*/

    v_position = position_worldspace.xyz;
    v_normal = normal;
    v_tex_coords = tex_coords;
    v_type = vtype;

    gl_Position = position_worldspace;
}