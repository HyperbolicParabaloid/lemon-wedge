use glm;

/// # Definition of the CHAR_MAP constant.
/// 
/// This mapping starts at the ASCII value "32", and goes all the way to value 132.
/// This mapping defines a unique [u32; 8] for each char from 32-132, where each bit
/// of the of the corresponding [u32; 8] array represents a pixel either a being visible
/// '1', or being transparent, '0'.
/// 
/// In this way, we can cheaply render 16x16 bit-wise text.
/*
As you can see, pressently we have 3 smaller structs, and one main struct called TextBlock. What I want
is the follow:

#1: Individual characters will be sent to the GPU, basically just holding the [UVec4; 2] data that describes
    how to draw the text, a Vec4 that describes their color, and an index. I'll explain the index in a moment.
#2: The individual characters will all be drawn on quads, hence why we're going to instance the quads.
#3: The only other matter of interest, is that we really need a good way of realting the position of the a
    given character to one another. For instance: if a String is all in one paragraph, we don't need to store
    the individual positions of all the characters separately. In fact, that'd really suck if we tried to move
    stuff around, the character positions would all need to be moved separately. What we need is a way to say,
    "I'm a part of TextBlock #3", "I'm a part of TextBlock #10", etc. If we can handle this, that means that all
    we need to figure out, and there change, the positions of a block of text is what the position of their parent
    text block is. For this, we can simply store the index of the text block in the characters, and send all the
    parent TextBlock positions to the GPU in a large SSBO. We can use the index to get the SSBO that char belongs
    to, and use it's position/color/offset/etc information to determine the characteristics that char should have.
    This will make it easy to do things like change the size, position, color, etc of text. This should be cool.
*/

pub struct TextBlock {
    contents: String,
    color: glm::Vec4,
}
 
impl TextBlock {
    pub fn new() -> TextBlock {
        Default::default()
    }

    pub fn from(contents: String, color: glm::Vec4) -> TextBlock {
        TextBlock{
            contents,
            color,
        }
    }

    pub fn contents_len(&self) -> usize {
        self.contents.len()
    }

    pub fn get_contents(&self) -> &String {
        &self.contents
    }

    pub fn get_color(&self) -> glm::Vec4 {
        self.color
    }

    pub fn set_contents(&mut self, contents: String) {
        self.contents = contents;
    }

    pub fn append_str(&mut self, additional_contents: &str) {
        self.contents.push_str(additional_contents);
    }

    pub fn append_char(&mut self, new_char: char) {
        self.contents.push(new_char);
    }

    pub fn insert_char(&mut self, new_char: char, index: usize) {
        self.contents.insert(index, new_char);
    }

    pub fn remove_char(&mut self) {
        self.contents.pop();
    }

    pub fn remove_char_at(&mut self, index: usize) -> Option<char> {
        if index < self.contents_len() {
            return self.contents.drain(index..index + 1).next()
        }
        None
    }
}

impl Default for TextBlock {
    fn default() -> Self {
        Self {
            contents: String::new(),
            color: glm::vec4(1.0, 1.0, 1.0, 1.0),
        }
    }
}
