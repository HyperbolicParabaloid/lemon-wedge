#version 450 core

layout (points) in;
layout (triangle_strip, max_vertices = 4) out;

out vec4 oColor;

void main() {
    oColor = vec4(1.0);

    vec4 topLeft = vec4(gl_in[0].gl_Position.xy, -1.0, 1.0);
    vec4 bottomRight = vec4(gl_in[0].gl_Position.zw, -1.0, 1.0);
    vec4 bottomLeft = vec4(topLeft.x, bottomRight.yzw);
    vec4 topRight = vec4(bottomRight.x, topLeft.yzw);
    
    gl_Position = bottomLeft;
    EmitVertex();   
    gl_Position = bottomRight;
    EmitVertex();   
    gl_Position = topLeft;
    EmitVertex();   
    gl_Position = topRight;
    EmitVertex();

    EndPrimitive();

}