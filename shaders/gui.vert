#version 310 es

precision mediump float;

in vec2 position;
in vec2 tex_coords;

uniform mat3 model_matrix;

out vec2 v_position;
out vec2 v_tex_coords;

void main() {

    vec3 position_transformed = model_matrix * vec3(position, 1.0);

    v_position = position_transformed.xy;
    v_tex_coords = tex_coords;

    gl_Position = vec4(position_transformed);
}