#version 400 core

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
layout (location = 5) in uint aSSBO;

// struct TextBlockPosition {
//     vec3 position;
//     uvec2 dimensions;
// 	   uvec2 step_size;
// }

vec3 block_position = vec3(-0.8, 0.8, -1);
vec2 block_step_size = vec2(0.05);
uvec2 block_dimensions = uvec2(32, 32);

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
	float fraction = (1.f + fract(aIndex) - 0.5) * block_step_size.y;
	float wholeIndex = aIndex - fract(aIndex);

	// Deltas.
	float deltaX = block_step_size.x * float(uint(wholeIndex) % block_dimensions.x); //// TESTING ////
	float deltaY = fraction + block_step_size.y * floor(wholeIndex / float(block_dimensions.x));// * 2.0; //// TESTING ////
	
	// Setting the position.
	gl_Position = vec4(aPos.x * block_step_size.x + block_position.x + deltaX, aPos.y * block_step_size.y + block_position.y - deltaY * 1.65f + fraction, -1.0, 1.f);

	// sending out the changes.
	color = aColor * ((gl_InstanceID == cursor_index && fract(time) < 0.5) ? 0.f : 1.f);
	letter1 = aLetter1;
	letter2 = aLetter2;
	quadCoord = vec2( -aPos.x,  aPos.y);
	step_size = block_step_size.x;
}