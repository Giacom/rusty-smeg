#version 330 core

layout (location = 0) in vec3 position;
layout (location = 1) in vec3 color;
layout (location = 2) in vec2 texCoord;

out vec3 ourColor;
out vec2 TexCoord;

uniform mat4 perspective;
uniform mat4 view;
uniform mat4 model;

void main()
{
	gl_Position = perspective * view * model * vec4(position.x, position.y, position.z, 1.0);
	TexCoord = texCoord;
	ourColor = color;
}