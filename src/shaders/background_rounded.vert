#version 450 core
layout (location = 0) in vec4 aPos;

void main() {
	gl_Position = aPos;
}