#version 450 core

in vec4 oColor;

out vec4 FragColor;

void main() {
    FragColor = oColor;
    // FragColor = vec4(0.64, 0.19, 0.19, 1.0);
}