#version 400 core

in vec4 vCol;
in vec2 vUV;

out vec4 fragColor;

uniform sampler2D tex;

void main()
{
    vec3 color = texture(tex, vUV).rgb;
    fragColor = vec4(color, 1) * vCol;
    // fragColor = vCol;
}