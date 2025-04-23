#version 450 core
#extension GL_ARB_separate_shader_objects : enable

// TEMPLATE:
// Position.
layout (location = 0) in vec2 aPos;

// PER INSTANCE:
// Letter.
layout (location = 1) in uvec4 aLetter1;
// Letter.
layout (location = 2) in uvec4 aLetter2;
// Color.
layout (location = 3) in vec4 aColor;
// Letter offset / index.
layout (location = 4) in float aIndex;
// SSBO index.
layout (location = 5) in uint aSSBOIndex;

uvec2 u32_to_two_u16s(uint combined_val) {
    uint right_val = 65535 & combined_val;
    uint left_val = (4294901760 & combined_val) >> 16;
	return uvec2(left_val, right_val);
}

// For each character:
struct TextBlock {
    vec4 position;		// This one should be obvious, just postion in 3D space.
    uint dimensions;    // This is actually a "packed" value, two u16's representing the max X and Y chars that are drawn.
    uint step_size;     // Another "packed" value representing the horizontal and vertical separation between chars.
                        // The u16's they break out into, represent the 100th of a % step difference, based on the size of the chars.
                        // That means, if we have the horizontal being 10000, we want the space between each char, to be (100 * 100) / 10000 * char size.
    uint char_size;     // Packed val representing the font size of the chars width and height.
	uint padding;		// Padding I suppose.
};

// SSBO containing character positions and data
layout(std430, binding = 0) buffer TextBlocksBuffer {
	TextBlock positions[];
	// float test_position[];
};

// vec3 block_position = vec3(-0.8, 0.8, -1.0);
// uvec2 block_dimensions = uvec2(32, 32);
// vec2 block_step_size = vec2(0.05);

// Outputs the color for the Fragment Shader
out vec4 color;
// Sending out the position.
out vec2 quadCoord;
// Passing on the letter information.
flat out uvec4 letter1;
flat out uvec4 letter2;

flat out float step_size;

uniform float time;
uniform uint cursor_index;

// Calculating the position of each of the characters.
void main() {
	// This is for sinking stuff like lowercase g's and y's so they look right.


	// vec3 block_position = vec3(-0.8, 0.8, -1.0);
	// uvec2 block_dimensions = uvec2(32, 32);
	vec2 block_step_size = vec2(4.0 / 64.0);

	vec3 block_position = positions[aSSBOIndex].position.xyz;
	uvec2 block_dimensions = u32_to_two_u16s(positions[aSSBOIndex].dimensions);
	uvec2 uvec_steps = u32_to_two_u16s(positions[aSSBOIndex].step_size);
	// vec2 block_step_size = vec2(uvec_steps) / 1000.0 * 0.05;
	// uvec2 chars = u32_to_two_u16s(positions[aSSBOIndex].char_size);

	float fraction = (1.f + fract(aIndex) - 0.5) * block_step_size.y;
	float wholeIndex = aIndex - fract(aIndex);

	if (aColor.w == 0.0 || (block_dimensions.y != 0.0 && wholeIndex >=  block_dimensions.x * block_dimensions.y)) {
		gl_Position = vec4(1.0, 1.0, 1.0, 0.0);
		return;
	}

	// Deltas.
	float deltaX = block_step_size.x * float(uint(wholeIndex) % block_dimensions.x); //// TESTING ////
	float deltaY = fraction + block_step_size.y * floor(wholeIndex / float(block_dimensions.x));// * 2.0; //// TESTING ////
	
	// Setting the position.
	gl_Position = vec4(aPos.x * block_step_size.x + block_position.x + deltaX + block_step_size.x, aPos.y * block_step_size.y + block_position.y - deltaY * 1.65f + fraction - block_step_size.y, -1.0, 1.f);

	// sending out the changes.
	// color = aColor * ((gl_InstanceID == cursor_index && fract(time) < 0.5) ? 0.f : 1.f);
	color = aColor * ((gl_InstanceID == cursor_index && fract(time) < 0.5) ? 0.f : 1.f);

	letter1 = aLetter1;
	letter2 = aLetter2;
	quadCoord = vec2( -aPos.x,  aPos.y);
	step_size = block_step_size.x;
	// */
}