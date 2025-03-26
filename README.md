# Lemon-Wedge

I want this to be a graph/tree-style, visual filesystem navigator.

Right now I'm side-tracked making a crummy "vi"-esque text editor.

## Progress

### 03/26/2025

So ass of today I've flushed out the issues I was having with the SSBO. Turns out the internet lied to me, (kinda). Basic OpenGL std430 packing rules are
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