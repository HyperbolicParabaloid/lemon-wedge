use std::error::Error;
use std::os::raw::c_void;
use glfw::ffi::glfwGetTime;
use glfw::Modifiers;

use crate::shader::Shader;
use crate::vao::VAO;
use crate::vbo::VBO;

pub mod text;

// Maping of a char as u8, to a [u32; 8] that represents the Char in memory for the GPU to draw.
pub const CHAR_MAP: [(f32, [u32; 8]); 100] = [
    /*   */ (0.5, [0, 0, 0, 0, 0, 0, 0, 0]),
    /* ! */ (0.5, [384, 25166208, 25166208, 25166208, 25166208, 25166208, 384, 25165824]),
    /* " */ (0.5, [0, 207621744, 242224752, 242222640, 0, 0, 0, 0]),
    /* # */ (0.5, [3120, 204475440, 2147385342, 204475440, 204475440, 2147385342, 204475440, 204472320]),
    /* $ */ (0.5, [384, 267395064, 966537612, 964698080, 133693852, 831273372, 536350704, 25165824]),
    /* % */ (0.5, [6148, 1007445532, 1714961520, 417333696, 58722072, 238820454, 946221116, 538443776]),
    /* & */ (0.5, [3968, 532691168, 811612396, 533598094, 499529966, 813445176, 943464446, 266731520]),
    /* ' */ (0.5, [0, 58721216, 62915008, 12583040, 0, 0, 0, 0]),
    /* ( */ (0.5, [448, 62916352, 234884096, 201329664, 201329664, 201330176, 117441472, 29360128]),
    /* ) */ (0.5, [896, 62914784, 7340080, 3145776, 3145776, 3145840, 14681024, 58720256]),
    /* * */ (0.5, [384, 229642224, 132121536, 132124656, 229638528, 0, 0, 0]),
    /* + */ (0.5, [0, 384, 25166208, 25174008, 536347008, 25166208, 25165824, 0]),
    /* , */ (0.85, [0, 0, 0, 0, 192, 31457760, 62916480, 0]),
    /* - */ (0.5, [0, 0, 0, 4080, 267386880, 0, 0, 0]),
    /* . */ (0.65, [0, 0, 0, 0, 0, 25166784, 62914944, 0]),
    /* / */ (0.5, [4, 786460, 3670128, 14680512, 58722048, 234888192, 939536384, 536870912]),
    /* 0 */ (0.5, [960, 132124272, 477632728, 416815512, 429398808, 454565432, 242223072, 62914560]),
    /* 1 */ (0.5, [384, 58722176, 25166208, 25166208, 25166208, 25166208, 25166784, 62914560]),
    /* 2 */ (0.5, [2016, 267394104, 404232216, 3670128, 14681024, 117444096, 469770232, 536346624]),
    /* 3 */ (0.5, [2016, 267394104, 404232216, 3670512, 31457328, 404232216, 473436144, 132120576]),
    /* 4 */ (0.5, [96, 14680544, 56624736, 207624288, 536354808, 6291552, 6291552, 6291456]),
    /* 5 */ (0.5, [8184, 536352768, 402659328, 534777840, 3670040, 1579032, 473436144, 132120576]),
    /* 6 */ (0.5, [2032, 267918392, 404232192, 534781936, 473438232, 404232216, 473436144, 132120576]),
    /* 7 */ (0.5, [8184, 536352792, 1572920, 3145824, 14680512, 25166720, 50332416, 50331648]),
    /* 8 */ (0.5, [2016, 267394104, 404232216, 204474336, 267394104, 404232216, 473436144, 132120576]),
    /* 9 */ (0.5, [2016, 267394104, 404232216, 473436152, 133693464, 1572888, 205000696, 133169152]),
    /* : */ (0.5, [0, 25166784, 62914944, 0, 0, 25166784, 62914944, 0]),
    /* ; */ (0.85, [0, 25166784, 62914944, 0, 384, 62915520, 62916480, 251658240]),
    /* < */ (0.5, [96, 14680512, 58722048, 234888192, 469765632, 117441408, 29360352, 6291456]),
    /* = */ (0.5, [0, 0, 8184, 536346624, 8184, 536346624, 0, 0]),
    /* > */ (0.5, [1536, 117441408, 29360352, 7340088, 3670128, 14680512, 58722048, 100663296]),
    /* ? */ (0.5, [2016, 267394104, 404232216, 1572920, 15729120, 25166208, 384, 25165824]),
    /* @ */ (0.65, [2016, 267394104, 941372396, 938227308, 909916088, 868235264, 470552572, 133693440]),
    /* A */ (0.5, [960, 132122592, 242224176, 204478488, 536354808, 941373468, 806121486, 1879965696]),
    /* B */ (0.5, [32752, 1073231932, 941373468, 943210480, 1073231932, 941373468, 943472632, 2146435072]),
    /* C */ (0.5, [2032, 267918876, 1007433732, 939538432, 939538432, 939801612, 505155576, 133169152]),
    /* D */ (0.5, [32736, 1073231996, 941373454, 940455950, 940455950, 940455964, 947666936, 2145386496]),
    /* E */ (0.5, [32764, 1073494044, 939538432, 941637600, 1071659040, 939538432, 941375484, 2147221504]),
    /* F */ (0.5, [16382, 536747534, 469769216, 470818800, 535829520, 469769216, 469777920, 1040187392]),
    /* G */ (0.5, [2032, 267919356, 1007564806, 939538432, 956184830, 940456974, 505155580, 133169152]),
    /* H */ (0.5, [15420, 1010571288, 404232216, 404234232, 536353848, 404232216, 404241468, 1010565120]),
    /* I */ (0.5, [4064, 266339200, 58721152, 58721152, 58721152, 58721152, 58724320, 266338304]),
    /* J */ (0.5, [2046, 134087736, 3670072, 3670072, 3682360, 808990776, 1014505456, 132120576]),
    /* K */ (0.5, [30744, 2016948336, 819999168, 1065369344, 1069562336, 812658736, 809007132, 2015100928]),
    /* L */ (0.5, [31744, 2080389120, 939538432, 939538432, 939538432, 939538432, 1008484348, 1073479680]),
    /* M */ (0.5, [28686, 2015262750, 2084469822, 1853255670, 1743152070, 1636196358, 1611030534, 1611005952]),
    /* N */ (0.5, [30750, 2082356236, 1040987916, 856437644, 835465420, 812396668, 809269278, 2015232000]),
    /* O */ (0.5, [2016, 267394104, 941371404, 806105100, 806105100, 806107164, 473436144, 132120576]),
    /* P */ (0.5, [16368, 536353820, 403445772, 403446812, 536354800, 402659328, 402669056, 1040187392]),
    /* Q */ (0.5, [2016, 267394104, 941371404, 806105100, 806105292, 820787324, 473436156, 133038080]),
    /* R */ (0.5, [16368, 536353820, 403445772, 404495416, 535830496, 418388024, 404503566, 1007550464]),
    /* S */ (0.5, [2032, 536362012, 940324864, 520097760, 16252956, 798732, 941367288, 267386880]),
    /* T */ (0.5, [16380, 1073492876, 58721152, 58721152, 58721152, 58721152, 58721152, 130023424]),
    /* U */ (0.5, [30750, 2015244300, 806105100, 806105100, 806105100, 806107164, 473436144, 132120576]),
    /* V */ (0.5, [28686, 1879977996, 806107164, 404232216, 473435184, 242222688, 132121536, 25165824]),
    /* W */ (0.5, [28686, 1879994382, 1879994766, 966539676, 966540732, 536354808, 511183920, 204472320]),
    /* X */ (0.5, [14364, 941366328, 242222688, 132121536, 62916576, 106958448, 473446428, 941359104]),
    /* Y */ (0.5, [28686, 1879980060, 404233272, 242223072, 62914944, 25166208, 25166784, 62914560]),
    /* Z */ (0.5, [16380, 1073491996, 3670128, 14680512, 58722048, 234888192, 940326908, 1073479680]),
    /* [ */ (0.5, [1984, 130024960, 100664832, 100664832, 100664832, 100664832, 100665280, 130023424]),
    /* \ */ (0.5, [8192, 805320704, 469765632, 117441408, 29360352, 7340088, 1835020, 262144]),
    /* ] */ (0.5, [992, 65011808, 6291552, 6291552, 6291552, 6291552, 6292448, 65011712]),
    /* ^ */ (0.5, [384, 62916576, 106957872, 404232216, 0, 0, 0, 0]),
    /* _ */ (0.85, [0, 0, 0, 0, 0, 0, 16382, 1073610752]),
    /* ` */ (0.5, [0, 100665088, 125830080, 29360320, 0, 0, 0, 0]),
    /* a */ (0.55, [0, 0, 4016, 535828720, 812658736, 808464504, 955785180, 261881856]),
    /* b */ (0.55, [6144, 402659328, 402660288, 534781488, 471341080, 404232216, 472911856, 132120576]),
    /* c */ (0.55, [0, 0, 2032, 267918360, 402659328, 402659328, 471339000, 133169152]),
    /* d */ (0.55, [24, 1572888, 1574872, 267918456, 404232216, 404232216, 473436156, 132907008]),
    /* e */ (0.55, [0, 0, 2016, 267394104, 404234232, 536352768, 471339000, 133169152]),
    /* f */ (0.55, [992, 133170736, 100664832, 100664832, 528490368, 100664832, 100664832, 100663296]),
    /* g */ (0.85, [0, 64489464, 238554136, 202903096, 133694424, 1575960, 238553072, 65011712]),
    /* h */ (0.55, [6144, 402659328, 402659328, 402660320, 535830064, 471341080, 404232216, 404226048]),
    /* i */ (0.55, [0, 58721152, 58720256, 896, 58721152, 58721152, 58721152, 58720256]),
    /* j */ (0.85, [48, 7864440, 3145728, 3145776, 3145776, 3151920, 477106144, 130023424]),
    /* k */ (0.55, [6144, 402659328, 405805168, 417339840, 528490432, 484448368, 405805112, 404226048]),
    /* l */ (0.55, [0, 29360576, 25166208, 25166208, 25166208, 25166208, 25166720, 58720256]),
    /* m */ (0.55, [0, 0, 12288, 1043873660, 1004286348, 831271308, 831271308, 831258624]),
    /* n */ (0.55, [0, 0, 0, 467673072, 506469400, 404232216, 404232216, 404226048]),
    /* o */ (0.65, [0, 0, 960, 132124272, 473438232, 404233272, 242223072, 62914560]),
    /* p */ (0.85, [992, 133172784, 202902552, 202903088, 267390432, 201329664, 201329664, 201326592]),
    /* q */ (0.85, [1984, 266341488, 405805104, 405802096, 267388848, 3145776, 3670076, 1835008]),
    /* r */ (0.55, [0, 0, 0, 232787952, 255331864, 201329664, 201329664, 201326592]),
    /* s */ (0.55, [0, 0, 65013744, 103810560, 125830112, 7340080, 103811056, 65011712]),
    /* t */ (0.55, [768, 50332416, 50335712, 532677376, 50332416, 50332416, 53478384, 31457280]),
    /* u */ (0.55, [0, 0, 0, 3120, 204475440, 204999736, 242747372, 63700992]),
    /* v */ (0.55, [0, 0, 6168, 404233272, 204476016, 106956768, 62915520, 25165824]),
    /* w */ (0.55, [0, 0, 24966, 1636197318, 1942894572, 913063548, 473438232, 404226048]),
    /* x */ (0.55, [0, 0, 403446812, 238552944, 65012160, 65013616, 238558236, 403439616]),
    /* y */ (0.85, [0, 6192, 405805104, 477106160, 128974896, 3151920, 477106144, 130023424]),
    /* z */ (0.55, [0, 0, 267915256, 3670128, 14680512, 58722048, 234885112, 267911168]),
    /* { */ (0.5, [992, 133172784, 234884096, 469776384, 402656256, 201330176, 238028784, 65011712]),
    /* | */ (0.5, [384, 25166208, 25166208, 25166208, 25166208, 25166208, 25166208, 25165824]),
    /* } */ (0.5, [1984, 266341488, 7340080, 3670044, 1572912, 3145840, 208670688, 130023424]),
    /* ~ */ (0.5, [0, 0, 0, 235282190, 966553848, 1617952768, 0, 0]),
    /* ☺ */ (0.5, [0, 0, 204475440, 0, 537145356, 404230128, 0, 0]),
    /* ◙ */ (0.5, [16380, 1073492028, 813445228, 818688460, 864826124, 906771980, 1007435772, 1073479680]),
    /* ◙ */ (0.5, [16380, 1073492028, 813445228, 818688460, 864826124, 906771980, 1007435772, 1073479680]),
    /* ◙ */ (0.5, [16380, 1073492028, 813445228, 818688460, 864826124, 906771980, 1007435772, 1073479680]),
    /* ◙ */ (0.5, [16380, 1073492028, 813445228, 818688460, 864826124, 906771980, 1007435772, 1073479680]),
];

// Maps GLFW Scancodes to their char representation.
const SCANCODE_MAP: [(char, char); 58] = [
    ('\0', '\0'),
    ('\0', '\0'),
    ('1', '!'),
    ('2', '@'),
    ('3', '#'),
    ('4', '$'),
    ('5', '%'),
    ('6', '^'),
    ('7', '&'),
    ('8', '*'),
    ('9', '('),
    ('0', ')'),
    ('-', '_'),
    ('=', '+'),
    ('\x08', '\x08'),
    ('\t', '\t'),
    ('q', 'Q'),
    ('w', 'W'),
    ('e', 'E'),
    ('r', 'R'),
    ('t', 'T'),
    ('y', 'Y'),
    ('u', 'U'),
    ('i', 'I'),
    ('o', 'O'),
    ('p', 'P'),
    ('[', '{'),
    (']', '}'),
    ('\n', '\n'), // ENTER
    ('\0', '\0'), // CTRL
    ('a', 'A'),
    ('s', 'S'),
    ('d', 'D'),
    ('f', 'F'),
    ('g', 'G'),
    ('h', 'H'),
    ('j', 'J'),
    ('k', 'K'),
    ('l', 'L'),
    (';', ':'),
    ('\'', '"'),
    ('`', '~'),
    ('\0', '\0'), // 42?
    ('\\', '|'),
    ('z', 'Z'),
    ('x', 'X'),
    ('c', 'C'),
    ('v', 'V'),
    ('b', 'B'),
    ('n', 'N'),
    ('m', 'M'),
    (',', '<'),
    ('.', '>'),
    ('/', '?'),
    ('\0', '\0'), // R_SHIFT
    ('*', '*'), // ?
    ('\0', '\0'), // L_ALT
    (' ', ' '), // SPACE
];

// Basic quad that all chars will be drawn on.
pub const QUAD: [f32; 8] = [
    -1.0, 0.0, 
     0.0, 0.0,
    -1.0, 1.0, 
     0.0, 1.0
];

#[allow(unused)]
#[derive(Clone)]
pub struct CharVertex {
    letter: [glm::UVec4; 2],
    color: glm::Vec4, /*  */
    position: f32, /* X for vertical shift for letters like y and g that need to sink down,
                            Y for the index this character represents in the String, so we can offset it,
                            and Z, which will be the index into the TextBlockPositions SSBO. */
    ssbo_index: u32,
}

// This will be used in the SSBO.
#[allow(unused)]
pub struct TextBlockPosition {
    position: glm::Vec3,
    dimensions: glm::UVec2,
    step_size: glm::Vec2,
}
impl TextBlockPosition {
    pub fn new(position: glm::Vec3, dimensions: glm::UVec2, step_size: glm::Vec2) -> Self {
        TextBlockPosition { position, dimensions, step_size }
    }
}

// Just adds meaningful names to the start->end indices that a certain TextBlock's contents
// relates to in the chars_vec array.
struct TextBlockRange {
    start: usize,
    end: usize
}

// Tells us where the cursor is on a given TextBlock so we can handle user
// inputs properly.
enum CursorPos {
    END(f32),
    BEGINING,
    CUSTOM((usize, f32))
}

// Manages multiple the TextBlocks, so they can all be updated/colored/drawn to the screen
// simultaneously for better performance.
pub struct UI<'a> {
    text_blocks: Vec<(text::TextBlock, TextBlockRange)>, // Holds all the TextBlocks, and their associated start->end indices in the chars_vec Vector.
    positions: Vec<TextBlockPosition>, // Data we're going to bind to the SSBO and send to the GPU.
    chars_vec: Vec<CharVertex>, // Buffer we're going to send to the GPU.
    shader_program: Option<Shader<'a>>,
    vao: Option<VAO>,
    uniforms: Vec<gl::types::GLint>,
    cursor_pos: CursorPos,
    cursor_char: char,
    dimensions: (u32, u32),
    tab_size: u32,
}

impl<'a> UI<'a> {
    /* * * * * * * * * * * * * * * * * * * * * * * * * * * *
     *                                                     * 
     *      ==========================================     *
     *                    Public Functions                 *
     *      ==========================================     *
     *                                                     *
     * * * * * * * * * * * * * * * * * * * * * * * * * * * */

    // Returns a new UI<'a> object.
    pub fn new() -> UI<'a> {
        let (vertical_offset, _) = CHAR_MAP['_' as usize - 32];
        let mut default_ui = Self {
            text_blocks: Vec::new(),
            positions: Vec::new(),
            chars_vec: Vec::new(),
            shader_program: None,
            vao: None,
            cursor_pos: CursorPos::END(vertical_offset),
            cursor_char: '_',
            uniforms: Vec::new(),
            dimensions: (32, 32),
            tab_size: 4,
        };

        default_ui.reset_vao();
        default_ui
    }

    // Creates a new shader program if one doesn't already exist.
    pub fn init_shader(&mut self, vert_file: &'a str, frag_file: &'a str) -> Result<(), Box<dyn std::error::Error>> {
        if let Some(shader_program) = &self.shader_program {
            shader_program.delete();
        }
        self.shader_program = Some(Shader::try_new(vert_file, frag_file)?);
        self.uniforms.clear();
        unsafe {
            self.uniforms.push(gl::GetUniformLocation(self.shader_program.as_ref().unwrap().get_id(), b"cursor_index\0".as_ptr() as *const _));
            self.uniforms.push(gl::GetUniformLocation(self.shader_program.as_ref().unwrap().get_id(), b"time\0".as_ptr() as *const _));
        }

        Ok(())
    }

    // Self expalnitory I suppose.
    pub fn reload_shader(&mut self) -> Result<(), Box<dyn Error>> {
        if let Some(shader_program) = &mut self.shader_program {
            println!("Reloading the shader.");
            shader_program.reload()?;
        }
        self.uniforms.clear();
        unsafe {
            self.uniforms.push(gl::GetUniformLocation(self.shader_program.as_ref().unwrap().get_id(), b"cursor_index\0".as_ptr() as *const _));
            self.uniforms.push(gl::GetUniformLocation(self.shader_program.as_ref().unwrap().get_id(), b"time\0".as_ptr() as *const _));
        }
        Ok(())
    }

    // Appends the given text_block with the specified character, and/or moves the cursor depending on what the input is.
    pub fn append_textblock(&mut self, text_block_index: usize, scancode_and_modifiers: (usize, Modifiers)) {
        let (scancode, _) = scancode_and_modifiers;
        match scancode {
            /* Arrows       */ 331 | 328 | 333 | 336  => { self.handle_arrow(scancode); }
            /* Home         */ 327  => { self.cursor_pos = CursorPos::BEGINING }
            /* End          */ 335  => { self.cursor_pos = CursorPos::END(1.0) } // Dumby Value btw.
            /* Backspace    */ 14   => { self.do_backspace(text_block_index); }    
            /* Invalid Chars*/ 0 | 1 | 29 | 42 | 54 | 56 => {}
            /* Valid Chars  */ 2..58=> { self.insert_char(text_block_index, scancode_and_modifiers); }
            /* Unhandled    */ _    => {}
        }

        // Resetting the Chars Vec, and the VAO.
        self.chars_vec.clear();
        self.set_char_vertex_vec(text_block_index);
        self.reset_vao();
    }

    // Pushes new TextBlock onto the Vector, and resets the chars_vec.
    pub fn push_textblock(&mut self, text_block: text::TextBlock, block_position: TextBlockPosition) {
        let start = self.text_blocks.len();
        let end = start + text_block.contents_len();
        self.text_blocks.push((text_block, TextBlockRange {start, end} ));
        self.positions.push(block_position);
        self.set_char_vertex_vec(start);
    }

    // Removes the a TextBlock object at the given location if the index was valid,
    // otherwise returns an error. Panics if index is out of bounds.
    pub fn remove_textblock(&mut self, index: usize) {
        self.positions.remove(index);
        let (_, indices) = self.text_blocks.remove(index);
        self.chars_vec.drain(indices.start..indices.end);
    }

    // Self explanitory I suppose.
    pub fn reset_vao(&mut self) {
        match &self.vao {
            Some(vao) => {
                vao.delete();
            },
            None => {}
        }
        self.vao = Some(VAO::try_new());
        self.set_vao();
    }

    // Renders all the Characters to the window.
    pub fn draw(&self) {
        if let (Some(vao), Some(shader_program)) = (&self.vao, &self.shader_program) {
            shader_program.activate();
            vao.bind();
            unsafe {
                /* The Cursor's gl_InstanceID u32   */ gl::Uniform1ui(self.uniforms[0], self.chars_vec.len() as u32 - 1);
                /* The glfwGetTime                  */ gl::Uniform1f(self.uniforms[1], glfwGetTime() as gl::types::GLfloat);
                gl::DrawArraysInstanced(gl::TRIANGLE_STRIP, 0, 4, self.chars_vec.len() as gl::types::GLsizei);
            }
            vao.unbind();
        } else {
            eprintln!("You cannot draw an UI element that doesn't have a shader program.\nPerhaps you forgot to do: ui.init_shader(vert_file, frag_file)")
        }
    }


    /* * * * * * * * * * * * * * * * * * * * * * * * * * * *
     *                                                     * 
     *      ==========================================     *
     *                    Non-Public                       *
     *                  Helper Fnctions                    *
     *      ==========================================     *
     *                                                     *
     * * * * * * * * * * * * * * * * * * * * * * * * * * * */

    // Function that takes in the current cursor position and a desired position. It returns the
    // valid position closest to the requested one.
    //
    // Example, where the underscore '_' represents the cursor.:
    //         >Hello, World!
    //          >    tabbed line!
    //          >hi_
    //
    // UP ARROW:    Here, we have to insert 2 ghost spaces, represented by a '-', before the '\t' in order to keep the spacing proper.
    //              This ghost spacing is not represented by an actual characters though, it's just to show the shift of the cursor.
    //              From now on, when we insert a new char, we'll need to account for the position the cursor is at so we can add the
    //              correct spacing in.
    //          >Hello, World!
    //          >--_ tabbed line!
    //          >hi_
    //
    // UP ARROW:    In a different example, where the isn't text above the target location of the cursor, we can just drop the cursor to
    //              the lowest line.
    //          >Hello, World!
    //          >_
    //          >hi_
    fn move_cursor_from_to(&mut self, from: (usize, f32), to: (usize, f32)) {

    }

    // Handles the cursor movement if the user presses an arrow key.
    fn handle_arrow(&mut self, arrow_key: usize) {
       let (vertical_shift, _) = CHAR_MAP[self.cursor_char as usize];
       match &self.cursor_pos {
           CursorPos::BEGINING => {
               match arrow_key {
                   /* LEFT  */ 331  => { self.cursor_pos = CursorPos::BEGINING }
                   /* UP    */ 328  => { self.cursor_pos = CursorPos::BEGINING }
                   /* RIGHT */ 333  => { self.cursor_pos = CursorPos::CUSTOM((std::cmp::min(1, self.chars_vec.len()), 1.0 + vertical_shift)) }
                   /* DOWN  */ 336  => { self.cursor_pos = CursorPos::CUSTOM((std::cmp::min(self.chars_vec.len(), 32), 32.0 + vertical_shift)) } // IDK YET :D
                   _ => {}
               }
           }
           CursorPos::CUSTOM((_, position)) => {
               let position = position.floor();
               match arrow_key {
                   /* LEFT  */ 331  => { self.cursor_pos = CursorPos::CUSTOM((0, position - 1.0)) }
                   /* UP    */ 328  => { self.cursor_pos = CursorPos::CUSTOM((0, position - 32.0)) }
                   /* RIGHT */ 333  => { self.cursor_pos = CursorPos::CUSTOM((0, position + 1.0)) }
                   /* DOWN  */ 336  => { self.cursor_pos = CursorPos::CUSTOM((0, position + 32.0)) } // IDK YET :D
                   _ => {}
               }
           }
           CursorPos::END(end_pos) => {
               match arrow_key {
                   /* LEFT  */ 331  => { self.cursor_pos = CursorPos::CUSTOM((
                       match self.chars_vec.len().checked_sub(2) {
                           None => 0,
                           Some(num) => num,
                       },
                       if end_pos - 2.0 >= 0.0 { end_pos - 2.0 } else { 0.0 })); }
                   /* UP    */ 328  => { self.cursor_pos = CursorPos::CUSTOM((
                       match self.chars_vec.len().checked_sub(33) {
                           None => 0,
                           Some(num) => num,
                       },
                       end_pos - 33.0)); } // THIS WILL NEED TO BE HANDLED
                   /* RIGHT */ 333  => { self.cursor_pos = CursorPos::END(*end_pos) }
                   /* DOWN  */ 336  => { self.cursor_pos = CursorPos::END(*end_pos) }
                   _ => {}
               }
           }
       };
    }

    // Binds two newly made VBOs to the VAO.
    fn set_vao(&mut self) {
        if let (Some(vao), Some(shader_program)) = (&self.vao, &self.shader_program) {
            shader_program.activate();
            vao.bind();
            
            let quad_vbo = VBO::try_new(&QUAD.to_vec(), gl::STATIC_DRAW);
            let chars_vbo = VBO::try_new(&self.chars_vec, gl::STATIC_DRAW);

            /*      TEMPLATE    */
            /* aPos     : vec2  */ vao.link_attrib( &quad_vbo, 0, 2, gl::FLOAT, (2 * size_of::<f32>()) as i32, std::ptr::null() as *const c_void);
            /*      INSTANCE    */
            /* aLetter1 : uvec4 */ vao.link_attrib_i( &chars_vbo, 1, 4, gl::UNSIGNED_INT, size_of::<CharVertex>() as i32, 0 as *const c_void);
            /* aLetter2 : uvec4 */ vao.link_attrib_i( &chars_vbo, 2, 4, gl::UNSIGNED_INT, size_of::<CharVertex>() as i32, (4 * size_of::<u32>()) as *const c_void);
            /* aColor   : vec4  */ vao.link_attrib( &chars_vbo, 3, 4, gl::FLOAT, size_of::<CharVertex>() as i32, (8 * size_of::<u32>()) as *const c_void);
            /* aIndex   : float */ vao.link_attrib( &chars_vbo, 4, 1, gl::FLOAT, size_of::<CharVertex>() as i32, (4 * size_of::<f32>() + 8 * size_of::<u32>()) as *const c_void);
            /* aSSBO    : uint  */ vao.link_attrib_i( &chars_vbo, 5, 1, gl::UNSIGNED_INT, size_of::<CharVertex>() as i32, (7 * size_of::<f32>() + 8 * size_of::<u32>()) as *const c_void);

            unsafe {
                /* Divisors */ gl::VertexAttribDivisor(1, 1);
                /* Divisors */ gl::VertexAttribDivisor(2, 1);
                /* Divisors */ gl::VertexAttribDivisor(3, 1);
                /* Divisors */ gl::VertexAttribDivisor(4, 1);
                /* Divisors */ gl::VertexAttribDivisor(5, 1);
            }
            vao.unbind();
            quad_vbo.unbind();
            chars_vbo.unbind();
        }
    }

    // Gets the correct Char and inserts it into the given TextBlock, as well as updating the position of the cursor.
    fn insert_char(&mut self, text_block_index: usize, scancode_and_modifiers: (usize, Modifiers))  {
        let (scancode, modifiers) = scancode_and_modifiers;

        let new_char = if modifiers.contains(Modifiers::Shift) {
            SCANCODE_MAP[scancode].1
        } else {
            SCANCODE_MAP[scancode].0
        };
        match self.cursor_pos {
            CursorPos::BEGINING => {
                self.text_blocks[text_block_index].0.insert_char(new_char, 0);
                self.cursor_pos = match new_char {
                    '\n' => CursorPos::CUSTOM((1, self.dimensions.0 as f32)),
                    '\t' => CursorPos::CUSTOM((1, self.tab_size as f32)),
                    _ => CursorPos::CUSTOM((1, 1.0))
                }
            }
            CursorPos::CUSTOM((index, position))  => {
                self.text_blocks[text_block_index].0.insert_char(new_char, index);
                let tab_change = (position as u32 % 32) % 5;
                self.cursor_pos = match new_char {
                    '\n' => CursorPos::CUSTOM((index + 1, position + (self.dimensions.0 - (position as u32 % self.dimensions.0)) as f32)),
                    '\t' => CursorPos::CUSTOM((index + 1, position + if tab_change == self.tab_size { self.tab_size as f32 + 1.0 } else { self.tab_size as f32 - tab_change as f32 })),
                    _ => CursorPos::CUSTOM((index + 1, position + 1.0))
                }
            }
            CursorPos::END(_) => { self.text_blocks[text_block_index].0.append_char(new_char); }
        }
    }

    // Returns the new CursorPos if the char "Backspaced" was a NEWLINE.
    fn backspace_newline(&self, new_index: usize, position: f32, text_block_index: usize) -> CursorPos {
        match new_index {
            0 => CursorPos::BEGINING,
            _ => {
                let start = match new_index.checked_sub(self.dimensions.0 as usize) { None => 0, Some(val) => val };
                let mut offset = position - self.dimensions.0 as f32;
                for c in self.text_blocks[text_block_index].0.get_contents()[start..new_index].chars().rev() {
                    match c {
                        '\n' => break,
                        '\t' => offset += 1.0,
                        _ => offset += 1.0
                    }
                }
                CursorPos::CUSTOM((new_index, offset))
            }
        }
    }

    // Returns the new CursorPos if the char "Backspaced" was a TAB.
    fn backspace_tab(&self, new_index: usize, position: f32) -> CursorPos {
        if new_index == 0 {
            CursorPos::BEGINING
        } else {
            CursorPos::CUSTOM((new_index, position - 1.0 - self.tab_size as f32))
        }
    }

    // Handles the "Backspace" operation of a user. "Handle" here meaning the removal of a char, TAB or NEWLINE that's in the way,
    // as well as updating the position of the cursor.
    fn do_backspace(&mut self, text_block_index: usize) {
        match self.cursor_pos {
            CursorPos::BEGINING => {}
            CursorPos::END(_) => { self.text_blocks[text_block_index].0.remove_char(); }
            CursorPos::CUSTOM((index, position))  => {
                let new_index = index - 1;
                if let Some(removed_char) = self.text_blocks[text_block_index].0.remove_char_at(new_index) {
                    // This match expression handles the logic for moving the Cursor to the correct position depending on whether
                    // the char deleted was a special character (i.e., TAB or NEWLINE) or a standard character.
                    self.cursor_pos = match removed_char {
                        '\n' => self.backspace_newline(new_index, position, text_block_index),
                        '\t' => self.backspace_tab(new_index, position),
                        _    => if new_index == 0 { CursorPos::BEGINING } else { CursorPos::CUSTOM((new_index, position - 1.0)) },
                    }
                }
            }
        }
    }

    // Generates the a CharVertex of the desired type, index and position.
    fn gen_char_vertex(&self, text_char: char, text_block_index: usize, position: f32) -> CharVertex {
        let (vertical_offset, bit_array) = CHAR_MAP[text_char as usize - 32];
        CharVertex{
            letter: [
                glm::UVec4::new(bit_array[0], bit_array[1], bit_array[2], bit_array[3]),
                glm::UVec4::new(bit_array[4], bit_array[5], bit_array[6], bit_array[7])],
            color: self.text_blocks[text_block_index].0.get_color(),
            position: position + vertical_offset,
            ssbo_index: text_block_index as u32
        }
    }

    // Takes in a mutable reference to self, to set the characters Vector.
    fn set_char_vertex_vec(&mut self, index: usize) {
        let text_block = &self.text_blocks[index];
        
        let mut count: f32 = 0.0;
        for text_char in text_block.0.get_contents().chars() {
            match text_char {
                '\n' => { count += (self.dimensions.0 - (count as u32 % self.dimensions.0)) as f32; }
                '\t' => {
                    let tab_change = (count as u32 % self.dimensions.0) % (self.tab_size + 1);
                    count += if tab_change == self.tab_size { self.tab_size as f32 + 1.0 } else { self.tab_size as f32 - tab_change as f32 }
                }
                ' '..='⌂' => {
                    self.chars_vec.push(self.gen_char_vertex(text_char, index, count));
                    count += 1.0;
                }
                _ => {}
            }
        }

        // Adding the Cursor to show the user where they will be typing.
        self.add_cursor(index, count);
    }

    // Adds the cursor into the correct position after all the chars are done being added into chars_vec.
    fn add_cursor(&mut self, index: usize, count: f32) {
        // Adding in the little _ to show where your cursor is.
        let (vertical_offset, _) = CHAR_MAP['_' as usize - 32];
        let mut cursor_char = self.gen_char_vertex('_', index, count + vertical_offset);
        cursor_char.position = match self.cursor_pos {
            CursorPos::BEGINING => { vertical_offset }
            CursorPos::CUSTOM((_, position)) => { position + vertical_offset }
            CursorPos::END(_) => {
                let new_end_position = count + vertical_offset;
                self.cursor_pos = CursorPos::END(new_end_position);
                new_end_position
            }
        };
        self.chars_vec.push(cursor_char)
    }

    // Decently performant slice insertion into vec.
    #[allow(unused)]
    fn fast_slice_insert<T: Clone>(&self, vec: &mut Vec<T>, slice: &[T], index: usize) {
        vec.reserve(slice.len()); // optional, but increases the performance
        let mut v = vec.split_off(index);
        vec.extend_from_slice(&slice);
        vec.append(&mut v);
    }

    // Decently performant slice replacing.
    #[allow(unused)]
    fn fast_replace_slice<T: Clone>(&self, vec: &mut Vec<T>, slice: &[T], start: usize, end: usize) {
        vec.drain(start..end);
        self.fast_slice_insert(vec, slice, start);
    }
}

// What I need to do now for the index of the cursor, is decide how I want to handle the motion.
// That boils down to a deciding how to increase the index, and what that index represents.