#version 400 core

// Instance ID.
flat in int instance;

flat in float count;

// Outputs colors in RGBA
out vec4 FragColor;

void main() {
    float scale = float(instance) / count;
    FragColor = vec4(0.0, 1.0 - scale, scale, 1.0);
    // FragColor = vec4(1.0);
}