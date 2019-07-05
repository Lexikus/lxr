#version 400 core

uniform float uFloat;

in vec4 vCol;
out vec4 FragColor;

void main() {
	FragColor = vec4(uFloat, vCol);
}