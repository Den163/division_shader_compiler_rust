#version 450 core

layout (location = 0) in vec4 VertexColor;
layout (location = 1) in vec2 inUV;

layout (location = 0) out vec4 FragColor;

layout (std140, binding = 1) uniform Uniforms {
    vec4 TestColor;
};

layout (binding = 0) uniform sampler2D _tex;

void main() {
    vec4 tex = vec4(texture(_tex, inUV));
    FragColor = VertexColor * tex;
    FragColor.rgb += TestColor.rgb;
}