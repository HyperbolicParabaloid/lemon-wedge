#version 450 core

layout (location = 0) in vec2 iTopLeft;
layout (location = 1) in vec2 iBottomRight;

out DATA
{
    vec2 oTopLeft;
    vec2 oBottomRight;
} data_out;

void main() {
    data_out.oTopLeft = iTopLeft;
    data_out.oBottomRight = iBottomRight;
	gl_Position = vec4(iTopLeft + iBottomRight, -1.0f, 1.0f);
}