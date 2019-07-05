#version 400 core

uniform float uFloat;

layout (location = 0) in vec3 aPos;
layout (location = 1) in vec4 aCol;

out vec4 vCol;

void main() {
    vCol = aCol;
	gl_Position = vec4(aPos.x, aPos.y, aPos.z, 1.0);
}