#version 400 core

uniform float uBrightness;
uniform float uContrast;
uniform float uGrayscale;

in vec4 vCol;
in vec2 vUV;
in vec3 vNor;
in vec3 vPos;
in vec3 vNormalWorldSpace;

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

vec3 ambientReflection (float intesity, float factor, vec3 lightColor) {
    return intesity * factor * lightColor;
}

vec3 diffuseReflection(float intensity, float factor, vec3 lightColor, vec3 lightDirection, vec3 normal) {
    return clamp(dot(lightDirection, normal), 0.0f, 1.0f) * intensity * factor * lightColor;
}

vec3 specularReflection(float intensity, float factor, vec3 lightColor, float hardness, vec3 viewDirection, vec3 reflextionDirection) {
    return pow(clamp(dot(viewDirection, reflextionDirection), 0.0f, 1.0f), hardness) * intensity * factor * lightColor;
}

void main() {
    vec3 color = texture(tex, vUV).rgb;
    vec3 c = color * vCol.rgb;

    vec3 ambientLightColor = vec3(0.75f, 0.75f, 1.0f);
    vec3 lightColor = vec3(0.75f, 0.75f, 1.0f);
    vec3 lightPosition = vec3(0.0f, 1.0f, 1.0f);
    vec3 lightDirection = normalize(lightPosition - vPos);
    vec3 viewPosition = vec3(0.0f);


    float ambientIntensity = 0.5f;
    float diffuseIntensity = 0.5f;
    float specularIntensity = 1.0f;

    float hardness = 64.0f;

    float diffuseFactor = 1.0f;
    float ambientFactor = 1.0f;
    float specularFactor = 1.0f;

    vec3 normal = normalize(vNormalWorldSpace);
    vec3 viewDirection = normalize(viewPosition - vPos);
    vec3 reflectionDirection = reflect(-lightDirection, normal);

    vec3 ambient = ambientReflection(ambientIntensity, ambientFactor, ambientLightColor);
    vec3 diffuse = diffuseReflection(diffuseIntensity, diffuseFactor, lightColor, lightDirection, normal);
    vec3 specular = specularReflection(specularIntensity, specularFactor, lightColor, hardness, viewDirection, reflectionDirection);

    vec3 finalColor = (ambient + diffuse + specular) * c;

    fragColor = vec4(finalColor, 1.0f);
}