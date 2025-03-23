#version 400 core

// Outputs colors in RGBA
out vec4 FragColor;

// Inputs the color from the Vertex Shader
in vec4 color;
in vec2 quadCoord;

// Passing on the letter information.
flat in uvec4 letter1;
flat in uvec4 letter2;

flat in float step_size;

// Setting up the letter.
void main() {
	//ivec2 coord = ivec2(15 - int(quadCoord.x * 15.f), int(quadCoord.y * 15.f));
	//FragColor = vec4(float(coord.x) / 16.f, float(coord.y) / 16.f, 0.0f, 1.f);
	
	vec4 background_color = vec4(0.44, 0.0, 0.62, 0.35);
	
	// Outlining the text.
	int fragNum = int(quadCoord.x * 16.f) + int(quadCoord.y * 16.f)  * 16;
	int index = int(float(fragNum) / 32.f);
	uint num = 0;
	switch (index) {
		case 7: num = letter1.x; break;
		case 6: num = letter1.y; break;
		case 5: num = letter1.z; break;
		case 4: num = letter1.w; break;
		case 3: num = letter2.x; break;
		case 2: num = letter2.y; break;
		case 1: num = letter2.z; break;
		default: num = letter2.w; break;
	}
	// Extracting the bits.
	float bit = float(bitfieldExtract(num, fragNum % 32, 1));
	FragColor = color * bit + background_color * (1.0 - bit);

	// /*
	// Shading the bottom edge.
	int topFragNum = int(quadCoord.x * 16.f) + int((quadCoord.y) * 16.f - 1.f)  * 16;
	int topIndex = int(float(topFragNum) / 32.f);
	uint topNum = 0;
	switch (topIndex) {
		case 7: topNum = letter1.x; break;
		case 6: topNum = letter1.y; break;
		case 5: topNum = letter1.z; break;
		case 4: topNum = letter1.w; break;
		case 3: topNum = letter2.x; break;
		case 2: topNum = letter2.y; break;
		case 1: topNum = letter2.z; break;
		default: topNum = letter2.w; break;
	}
	// Extracting the bits.
	uint topBit = bitfieldExtract(topNum, topFragNum % 32, 1);

	// Shading the bottom edge.
	int btmFragNum = int(quadCoord.x * 16.f) + int((quadCoord.y) * 16.f + 1.f)  * 16;
	int btmIndex = int(float(btmFragNum) / 32.f);
	uint btmNum = 0;
	switch (btmIndex) {
		case 7: btmNum = letter1.x; break;
		case 6: btmNum = letter1.y; break;
		case 5: btmNum = letter1.z; break;
		case 4: btmNum = letter1.w; break;
		case 3: btmNum = letter2.x; break;
		case 2: btmNum = letter2.y; break;
		case 1: btmNum = letter2.z; break;
		default: btmNum = letter2.w; break;
	}
	// Extracting the bits.
	uint btmBit = bitfieldExtract(btmNum, btmFragNum % 32, 1);


	// Shading the right edge.
	int rgtFragNum = int(quadCoord.x * 16.f - 1.f) + int((quadCoord.y) * 16.f)  * 16;
	int rgtIndex = int(float(rgtFragNum) / 32.f);
	uint rgtNum = 0;
	switch (rgtIndex) {
		case 7: rgtNum = letter1.x; break;
		case 6: rgtNum = letter1.y; break;
		case 5: rgtNum = letter1.z; break;
		case 4: rgtNum = letter1.w; break;
		case 3: rgtNum = letter2.x; break;
		case 2: rgtNum = letter2.y; break;
		case 1: rgtNum = letter2.z; break;
		default: rgtNum = letter2.w; break;
	}
	// Extracting the bits.
	uint rgtBit = bitfieldExtract(rgtNum, rgtFragNum % 32, 1);

	// Shading the left edge.
	int lftFragNum = int(quadCoord.x * 16.f + 1.f) + int((quadCoord.y) * 16.f)  * 16;
	int lftIndex = int(float(lftFragNum) / 32.f);
	uint lftNum = 0;
	switch (lftIndex) {
		case 7: lftNum = letter1.x; break;
		case 6: lftNum = letter1.y; break;
		case 5: lftNum = letter1.z; break;
		case 4: lftNum = letter1.w; break;
		case 3: lftNum = letter2.x; break;
		case 2: lftNum = letter2.y; break;
		case 1: lftNum = letter2.z; break;
		default: lftNum = letter2.w; break;
	}
	// Extracting the bits.
	uint lftBit = bitfieldExtract(lftNum, lftFragNum % 32, 1);

	// This takes into account the step-size in order to apply more anti-aliasing depending on how small the characters are.
	FragColor = mix((FragColor * 0.25) * clamp(float(bit + btmBit + rgtBit + lftBit + topBit), 0.0, 4.0), FragColor, 10 * step_size);
	// */
}