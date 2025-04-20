# Lemon-Wedge

I want this to be a graph/tree-style, visual filesystem navigator.

Right now I'm side-tracked making a crummy "vi"-esque text editor.

## Progress

### 03/26/2025

So as of today I've flushed out the issues I was having with the SSBO. Turns out the internet lied to me, (kinda). Basic OpenGL std430 packing rules are
as follow:

    vec4 -> 4   *   4 bytes
    vec3 -> 4   *   4 bytes
    float/
    uint/
    int/ -> 1   *   4 bytes

This was the core of the issues.

But anyhow we got it working now so all is well. Now what I need to do is handle how I want to manage the "current" text-block. A.k.a., which text block the
user is currently interacting with, so we can place the cursor there and any edits will happen to that one.

Additionally, I'm thinking of having padding items added to the end of text_blocks, to round them up to something simple like multiples of 64 or something.
That way, small edits to something will only require that a portion of the chars_vec be updated, instead of having to set it back, or shift large chunks of
data when changing individual items one by one, at small block_position indices.

Okay so, practical example.

    text_blocks = [
        TextBlock0("Short"),
        TextBlock1("I am the\nfirst\t\ttext-block."),
        TextBlock2("I am the second\n\ntext-block, and I'm\nmuch longer than the first one.")
    ]

        ||
        ||
        \/

    chars_vec = [
        // Here, we have 5 characters with values, and we have 59 'Padding' CharVertex structs here. If we extend this string anymore, instead of having to shift the entire chars_vec Vector, we can safely add up to 59 more, and only need to update the data in this 0->63 index chunk.
        'S', 'h', 'o', 'r', 't', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ' 

        // This has 24 chars, and 40 padding CharVertices.
        'I', ' ', 'a', 'm', ' ', 't', 'h', 'e', 'f', 'i', 'r', 's', 't', 't', 'e', 'x', 't', '-', 'b', 'l', 'o', 'c', 'k', '.', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ' 

        // This is 65 actual characters, so we round up to the next multiple of 64: 128.        
        'I', ' ', 'a', 'm', ' ', 't', 'h', 'e', ' ', 's', 'e', 'c', 'o', 'n', 'd', 't', 'e', 'x', 't', '-', 'b', 'l', 'o', 'c', 'k', ',', ' ', 'a', 'n', 'd', ' ', 'I', ''', 'm', 'm', 'u', 'c', 'h', ' ', 'l', 'o', 'n', 'g', 'e', 'r', ' ', 't', 'h', 'a', 'n', ' ', 't', 'h', 'e', ' ', 'f', 'i', 'r', 's', 't', ' ', 'o', 'n', 'e', 
        // Second 64-CharVertex chunk. 
        '.', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ' 
    ]

### 04/20/2025

Decided to go with a refactor (which now, needs another refactor) but at least now, we have a good system for editing blocks of text and inserting them back into
the overall list.
