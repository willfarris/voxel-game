#version 310 es

precision mediump float;

in vec2 v_tex_coords;

uniform sampler2D ssao;
uniform sampler2D albedo;
uniform vec2 resolution;

out vec4 color;

void main() {
    vec2 uv = v_tex_coords;//gl_FragCoord.xy / resolution;
    vec2 px = 1. / resolution;

    vec4 albedo = texture(albedo, uv);

    float ssao_avg = 0.0;//texture(ssao, uv).r;
    int radius = 2;
    for(int i=-radius;i<=radius;++i) {
        for(int j=-radius;j<=radius;++j) {
            vec2 offset = px * vec2(float(i), float(j));
            ssao_avg += texture(ssao, uv + offset).r;
        }
    }
    ssao_avg /= 25.0;

    ssao_avg = smoothstep(0.0, 1.0, ssao_avg);

    color = vec4(ssao_avg * albedo.rgb, albedo.a);
}