#version 310 es

precision mediump float;

in vec2 v_tex_coords;

layout (location = 0) uniform sampler2D ssao;
layout (location = 1) uniform sampler2D albedo;
layout (location = 2) uniform sampler2D position;
layout (location = 3) uniform sampler2D normal;

uniform vec2 resolution;
uniform float ssao_noise_size;

out vec4 color;

void main() {
    vec2 uv = v_tex_coords;//gl_FragCoord.xy / resolution;
    vec2 px = 1. / resolution;

    vec4 albedo = texture(albedo, uv);
    vec4 position = texture(position, uv);
    vec4 normal = texture(normal, uv);
    vec3 sky = vec3(0.4, 0.6, 1.0) * (1.0 - albedo.a);

    float vignette = 1.0 - 0.1 *length(uv - 0.5);

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

    vec3 light_dir = normalize(vec3(sqrt(2.)));
    float ambient = 0.5;
    float diffuse = max(dot(normal.xyz, light_dir), 0.0);

    vec3 out_color = albedo.rgb * (diffuse + ambient);

    color = vignette * (vec4(out_color.rgb, albedo.a) + vec4(sky, 1.0));
}