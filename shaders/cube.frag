#version 310 es

precision mediump float;

in vec3 v_position;
in vec3 v_normal;
in vec2 v_tex_coords;
flat in int v_type;

uniform float time;
uniform sampler2D texture_map;

out vec4 color;

void main() {

    vec3 sunlight_dir = vec3(sqrt(2.0));

    vec4 albedo = texture(texture_map, v_tex_coords).rgba;
    if (albedo.a < 0.5) { discard; }

    vec3 diffuse_color = albedo.rgb * min(1.0, max(dot(sunlight_dir, v_normal), 0.0) + 0.4);
    color = vec4(diffuse_color, 1.0);
}