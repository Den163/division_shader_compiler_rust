#version 450 core

layout (location = 0) in vec3 pos;
layout (location = 1) in vec4 fColor;
layout (location = 2) in vec2 inUV;
layout (location = 3) in mat4 localToWorld;

layout (location = 0) out vec4 VertexColor;
layout (location = 1) out vec2 outUV;

void main() {
    VertexColor = fColor;
    outUV = inUV;

    gl_Position = vec4(pos, 1) * localToWorld;
}