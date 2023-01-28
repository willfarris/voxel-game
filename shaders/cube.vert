#version 310 es

precision mediump float;

layout (location = 0) in vec3 position;
layout (location = 1) in vec3 normal;
layout (location = 2) in vec2 tex_coords;
layout (location = 3) in float vtype;

uniform mat4 model_matrix;
uniform mat4 view_matrix;
uniform mat4 perspective_matrix;
uniform float time;

out vec3 v_position;
out vec3 v_normal;
out vec2 v_tex_coords;
flat out float v_type;

#define WIND_SPEED 2.0

void main() {

    vec4 position_worldspace = model_matrix * vec4(position, 1.0);
    if(vtype == 1.0) {
        position_worldspace.xz += mod(position.y, 1.0) * 0.03 * sin(WIND_SPEED * (time + position.y));
    }
    else if(vtype == 2.0) {
        position_worldspace.xz += 0.03 * sin(WIND_SPEED * (time + position.y + 0.1415));
    }

    vec4 position_viewspace = view_matrix * position_worldspace;
    v_position = position_viewspace.xyz;

    //mat3 normal_matrix = transpose(inverse(mat3(view_matrix * model_matrix)));
    mat3 normal_matrix = transpose(inverse(mat3(view_matrix)));
    v_normal = normal_matrix * normal;
    v_tex_coords = tex_coords;
    v_type = vtype;

    gl_Position = perspective_matrix * position_viewspace;
}