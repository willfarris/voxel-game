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

/*
#version 310 es

precision mediump float;

in vec3 position;
in vec3 normal;
in vec2 tex_coords;
in int vtype;

uniform mat4 model_matrix;
uniform mat4 view_matrix;
uniform mat4 perspective_matrix;
uniform vec3 transform_position;
uniform float time;


out vec3 v_normal;
out vec3 v_position;
out vec2 v_tex_coords;

#define WIND_SPEED 5.0

void main() {
    mat4 camera_matrix = perspective_matrix * view_matrix;

    vec4 pos4 = vec4(position, 1.0);
    if(vtype == 1) {
        pos4.xz += mod(position.y, 1.0) * 0.03 * sin(WIND_SPEED * (time + position.y));
    } else if(vtype == 2) {
        pos4.xz += 0.03 * sin(WIND_SPEED * (time + position.y + 0.1415));
    } else {
        //pos4.xz += position.y * time;
    }
    vec4 pos4_new = camera_matrix * model_matrix * pos4;
    
    
    v_position = pos4_new.xyz;
    v_normal = normal; //model_matrix * vec4(normal, 1.0).xyz;
    v_tex_coords = tex_coords;

    gl_Position = pos4_new;
}
*/