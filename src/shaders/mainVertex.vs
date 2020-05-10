#version 330 core
layout (location = 0) in vec3 aPos;
layout (location = 1) in vec3 aNormal;
layout (location = 2) in vec2 aTexCoords;

out vec2 texCoords;
out vec3 surfaceNormal;
out vec3 toLight;
out vec3 toCamera;

uniform mat4 model;
uniform mat4 view;
uniform mat4 projection;
uniform vec3 lightPosition;

void main() {
    vec4 worldPosition = model * vec4(aPos, 1.0);
    surfaceNormal = (model * vec4(aNormal, 0.0)).xyz;
    toLight = lightPosition - worldPosition.xyz;
    toCamera = (inverse(view) * vec4(0.0,0.0,0.0,1.0)).xyz - worldPosition.xyz;

    texCoords = aTexCoords;
    gl_Position = projection * view * worldPosition;
}
