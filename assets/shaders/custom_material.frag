#version 450
layout(location = 0) in vec2 v_Uv;

layout(location = 0) out vec4 o_Target;

layout(set = 1, binding = 0) uniform MyCustomMaterial {
    vec4  Color;
    float Time;
};

layout(set = 1, binding = 1) uniform texture2D MyCustomMaterial_texture;
layout(set = 1, binding = 2) uniform sampler MyCustomMaterial_sampler;


void main() {
    o_Target = Color * texture(sampler2D(MyCustomMaterial_texture,MyCustomMaterial_sampler), v_Uv + vec2(0.0, Time * 0.1));
}
