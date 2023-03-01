#version 450
layout(location = 0) in vec2 v_Uv;

layout(location = 0) out vec4 o_Target;

layout(set = 1, binding = 0) uniform MyCustomMaterial {
    vec4  Color;
    float Time;
};

layout(set = 1, binding = 1) uniform texture2D base_color_texture;
layout(set = 1, binding = 2) uniform sampler   base_color_sampler;

layout(set = 1, binding = 3) uniform texture2D noise_texture;
layout(set = 1, binding = 4) uniform sampler   noise_sampler;


void main() {
    float anim_t = sin(Time * 0.4);
    float noise_x = texture(sampler2D(noise_texture, noise_sampler), v_Uv + vec2(0.0, anim_t)).r;
    float noise_y = texture(sampler2D(noise_texture, noise_sampler), v_Uv + vec2(0.0, -anim_t)).r;
    o_Target = Color * texture(sampler2D(base_color_texture, base_color_sampler), v_Uv + vec2(noise_x, noise_y) * 0.1);
    o_Target += vec4(noise_x, noise_y, 0., 1.) * .2;
}
