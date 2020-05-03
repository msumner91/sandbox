#version 330 core
out vec4 FragColor;

in vec2 TexCoords;

uniform sampler2D texture_normal1;

void main()
{
    FragColor = texture(texture_normal1, TexCoords);
}
