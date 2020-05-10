#version 330 core
out vec4 outColour;

in vec2 texCoords;
in vec3 surfaceNormal;
in vec3 toLight;
in vec3 toCamera;

uniform sampler2D texture_diffuse1;
uniform sampler2D texture_specular1;
uniform vec3 lightColour;
uniform float shineDamper;
uniform float reflectivity;
uniform vec3 attenuation;

void main() {

    float distance = length(toLight);
    float attFactor = attenuation.x + attenuation.y * distance + (attenuation.z * distance * distance);

    // Diffuse
    vec3 unitNormal = normalize(surfaceNormal);
    vec3 unitToLight = normalize(toLight);
    float brightness = dot(unitNormal, unitToLight);
    brightness = max(brightness, 0.0);
    vec3 diffuse = (brightness * lightColour)/attFactor;

    // Specular
    vec3 unitToCamera = normalize(toCamera);
    vec3 unitFromLight = -unitToLight;
    vec3 reflectedLight = reflect(unitFromLight, unitNormal);
    float specularFactor = dot(reflectedLight, unitToCamera);
    specularFactor = max(specularFactor, 0.0);
    float dampedFactor = pow(specularFactor, shineDamper);
    vec3 specularColour = (dampedFactor * reflectivity * lightColour)/attFactor;

    outColour = vec4(diffuse, 1.0) * texture(texture_diffuse1, texCoords) * texture(texture_specular1, texCoords) + vec4(specularColour, 1.0);
}
