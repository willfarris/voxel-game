#version 310 es

precision mediump float;

in vec2 v_tex_coords;

// TODO: convert position + normal coords to view space
layout (location = 0) uniform sampler2D position;
layout (location = 1) uniform sampler2D normal;
layout (location = 2) uniform sampler2D albedo;
layout (location = 3) uniform sampler2D ssao_noise;

#define NUM_SAMPLES 32

uniform vec3 samples[NUM_SAMPLES];
uniform mat4 projection;
uniform vec2 resolution;
uniform float ssao_noise_size;

out float color;

void main() {

    vec2 uv = v_tex_coords;
    vec2 noise_scale = resolution / ssao_noise_size;
    
    // Sample textures from GBuffer
    vec3 f_position = texture(position, uv).xyz;
    vec3 f_normal = texture(normal, uv).xyz;
    vec4 f_albedo =  texture(albedo, uv).rgba;
    vec3 random_vec = texture(ssao_noise, uv * noise_scale).xyz;

    // Calculate SSAO
    vec3 tangent = normalize(random_vec - f_normal * dot(random_vec, f_normal));
    vec3 bitangent = cross(f_normal, tangent);
    mat3 TBN = mat3(tangent, bitangent, f_normal);

    float occlusion = 0.0;
    for(int i=0;i<NUM_SAMPLES;++i) {
        vec3 sample_pos = TBN * samples[i];
        sample_pos = f_position + sample_pos * 0.5;

        vec4 offset = vec4(sample_pos, 1.0);
        offset = projection * offset;
        offset.xyz /= offset.w;
        offset.xyz = offset.xyz * 0.5 + 0.5;
        
        float sample_depth = texture(position, offset.xy).z;
        float sample_alpha = texture(albedo, offset.xy).a;
        //float range_check = smoothstep(1.0, 0.0, 0.5 / abs(f_position.z - sample_depth));
        occlusion += (sample_depth >= sample_pos.z + 0.025 ? 1.0 : 0.0 + (1.0 - sample_alpha));
    }
    occlusion /= float(NUM_SAMPLES);
    
    color = occlusion;//vec4(occlusion * f_albedo.rgb, f_albedo.a);
}