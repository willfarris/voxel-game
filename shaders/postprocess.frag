#version 310 es

precision mediump float;

in vec2 v_tex_coords;

uniform sampler2D ssao;
uniform sampler2D albedo;
uniform vec2 resolution;
uniform float ssao_noise_size;

out vec4 color;

void main() {
    vec2 uv = v_tex_coords;//gl_FragCoord.xy / resolution;
    vec2 px = 1. / resolution;

    vec4 albedo = texture(albedo, uv);

    //float vignette = 1.0 - 0.5 *length(uv - 0.5);

    //float ssao_avg = texture(ssao, uv).r;
    
    /*float ssao_avg = 0.0;
    for(int i=0;i<int(ssao_noise_size);++i) {
        for(int j=0;j<int(ssao_noise_size);++j) {
            vec2 offset = px * vec2(float(i), float(j));
            ssao_avg += texture(ssao, uv + offset).r;
        }
    }
    ssao_avg /= ssao_noise_size * ssao_noise_size;*/

    //ssao_avg = smoothstep(0.0, 1.0, ssao_avg);

    color = vec4(albedo.rgb, albedo.a);
}