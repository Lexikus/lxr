#version 400 core

uniform float uBrightness;
uniform float uContrast;
uniform float uGrayscale;

layout (location = 0) in vec3 aPos;
layout (location = 1) in vec3 aNor;
layout (location = 2) in vec2 aUV;
layout (location = 3) in vec4 aCol;

uniform mat4 model;
uniform mat4 view;
uniform mat4 projection;

out vec4 vCol;
out vec2 vUV;
out vec3 vNor;
out vec3 vPos;
out vec3 vNormalWorldSpace;

void main() {
    vCol = aCol;
    vUV = aUV;
    vNor = aNor;
    vPos = (model * vec4(aPos, 1.0f)).xyz;
    vNormalWorldSpace = (transpose(inverse(model)) * vec4(aNor, 0.0f)).xyz;
    gl_Position = projection * view * model * vec4(aPos, 1.0f);
}