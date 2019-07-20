#version 400 core

uniform float uBrightness;
uniform float uContrast;
uniform float uGrayscale;

in vec4 vCol;
in vec2 vUV;

out vec4 fragColor;

uniform sampler2D tex;

void setBrightness(inout vec3 c ) {
    c = clamp(c + uBrightness, 0.0f, 1.0f);
}

void setContrast(inout vec3 c) {
    float f = (uContrast + 1.0f) / (1.0f - uContrast);
    c = f * (c - 0.5f) + 0.5f;
}

void setGrayscale(inout vec3 c) {
    float f = (c.x + c.y + c.z) / 3;
    c = mix(vec3(f), c, uGrayscale);
}

void main() {
    vec3 color = texture(tex, vUV).rgb;
    vec3 c = color * vCol.rgb;

    // setBrightness(c);
    // setContrast(c);
    setGrayscale(c);

    fragColor = vec4(c, 1.0f);
    // fragColor = vCol;
}