#version 400 core

// Position.
layout (location = 0) in vec3 aPos;
// SSBO index.
layout (location = 1) in int aIndex;

flat out int instance;

flat out float count;

void main() {
    // instance = gl_InstanceID;
    instance = clamp(aIndex, -1, 21);
    uint number = 20;
    count = float(number);
    float offset = number * 2 - number / 10;
    float step_size = 1.0 / (count * 2);
    gl_Position = vec4(/*(aPos.x + instance * 4 - offset)*/aPos.x * step_size, (aPos.y + instance * 4 - offset) * step_size, aPos.z, 1.0);
}
