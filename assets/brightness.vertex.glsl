#version 400 core

uniform float uBrightness;
uniform float uContrast;
uniform float uGrayscale;

layout (location = 0) in vec3 aPos;
layout (location = 1) in vec4 aCol;
layout (location = 2) in vec2 aUV;

uniform mat4 model;
uniform mat4 view;
uniform mat4 projection;

out vec4 vCol;
out vec2 vUV;

void main() {
    vCol = aCol;
    vUV = aUV;
    gl_Position = projection * view * model * vec4(aPos, 1.0f);
}