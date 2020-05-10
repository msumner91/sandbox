#version 330 core
out vec4 outColour;

in vec2 texCoords;
in vec3 surfaceNormal;
in vec3 toLight;

uniform sampler2D textureSampler1;
uniform sampler2D textureSampler2;
uniform vec3 lightColour;
uniform vec3 attenuation;

void main() {

    // Mutli texture sampling
    vec4 grassTexColour = texture(textureSampler1, texCoords) * 0.8;
    vec4 rockTexColour = texture(textureSampler2, texCoords) * 0.2;

    // Attenuation
    float distance = length(toLight);
    float attFactor = attenuation.x + attenuation.y * distance + (attenuation.z * distance * distance);

    // Diffuse
    vec3 unitNormal = normalize(surfaceNormal);
    vec3 unitToLight = normalize(toLight);
    float brightness = dot(unitNormal, unitToLight);
    brightness = max(brightness, 0.0);
    vec3 diffuse = (brightness * lightColour)/attFactor;

    outColour = vec4(diffuse, 1.0) * (grassTexColour + rockTexColour);
}
