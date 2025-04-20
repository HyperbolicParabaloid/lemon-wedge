#version 450 core

layout (points) in;
layout (triangle_strip, max_vertices = 4) out;

in DATA
{
    vec2 topLeft;
    vec2 bottomRight;
    vec4 color;
} data_in[];

out vec4 oColor;

void main() {
    vec2 bottomLeft = data_in[0].topLeft;
    vec2 topRight = data_in[0].topRight;
    
    gl_Position = vec4(bottomLeft, -1.0, 1.0);    // 1:bottom-left
    EmitVertex();   
    gl_Position = vec4(data_in[0].bottomRight, -1.0, 1.0);    // 1:bottom-left
    EmitVertex();   
    gl_Position = vec4(data_in[0].topLeft, -1.0, 1.0);    // 1:bottom-left
    EmitVertex();   
    gl_Position = vec4(topRight, -1.0, 1.0);    // 1:bottom-left
    EmitVertex();   

    EndPrimitive();

    oColor = data_in[0].color;
}