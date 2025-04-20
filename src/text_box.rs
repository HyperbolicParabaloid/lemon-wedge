use std::collections::HashMap;

use crate::shader::Shader;
use crate::ssbo::SSBO;
use crate::vao::VAO;
use crate::vbo::VBO;

use std::os::raw;
use glfw::ffi;

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
#[allow(unused)]
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
#[derive(Debug, Clone, Copy)]
pub struct CharVertex {
    letter: [glm::UVec4; 2],
    color: glm::Vec4, /*  */
    position: f32, /* X for vertical shift for letters like y and g that need to sink down,
                            Y for the index this character represents in the String, so we can offset it,
                            and Z, which will be the index into the TextBlockPositions SSBO. */
    ssbo_index: u32,
}
impl CharVertex {
    pub fn null_char() -> Self {
        let bit_array = CHAR_MAP[0].1;
        CharVertex {
            letter: [
                glm::UVec4::new(bit_array[0], bit_array[1], bit_array[2], bit_array[3]),
                glm::UVec4::new(bit_array[4], bit_array[5], bit_array[6], bit_array[7])],
            color: glm::Vec4::new(0.0, 0.0, 0.0, 0.0),
            position: 0.0,
            ssbo_index: 0
        }
    }

    pub fn new(text_char: char, text_block_index: usize, position: f32, color: &[f32; 4]) -> CharVertex {
        let (vertical_offset, bit_array) = CHAR_MAP[text_char as usize - 32];
        CharVertex {
            letter: [
                glm::UVec4::new(bit_array[0], bit_array[1], bit_array[2], bit_array[3]),
                glm::UVec4::new(bit_array[4], bit_array[5], bit_array[6], bit_array[7])],
            color: glm::Vec4::new(color[0], color[1], color[2], color[3]),
            position: position + vertical_offset,
            ssbo_index: text_block_index as u32
        }
    }
}

// This will be used in the SSBO.
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct BoxPosition {
    position: glm::Vec4,// This one should be obvious, just postion in 3D space.
    dimensions: u32,    // This is actually a "packed" value, two u16's representing the max X and Y chars that are drawn.
    step_size: u32,     // Another "packed" value representing the horizontal and vertical separation between chars.
                        // The u16's they break out into, represent the 100th of a % step difference, based on the size of the chars.
                        // That means, if we have the horizontal being 10000, we want the space between each char, to be (100 * 100) / 10000 * char size.
    char_size: u32,     // Packed val representing the font size of the chars width and height.
    padding: u32,       // Padding I guess.
}
impl BoxPosition {
    pub fn new(position: glm::Vec4, dimensions: [u16; 2], step_size: [u16; 2], char_size: [u16; 2]) -> Self {
        let dimensions = two_u16_to_u32(dimensions[0], dimensions[1]);
        let step_size = two_u16_to_u32(step_size[0], step_size[1]);
        let char_size = two_u16_to_u32(char_size[0], char_size[1]);

        BoxPosition { position, dimensions, step_size, char_size, padding: 0 }
    }
}
fn two_u16_to_u32(left_val: u16, right_val: u16) -> u32 {
    (!0 as u32 & (left_val as u32) << 16) + right_val as u32
}
#[allow(unused)]
fn u32_to_two_u16s(combined_val: u32) -> (u16, u16) {
    let max_u16_as_u32: u32 = (!0 as u16) as u32;
    let right_val: u16 = (max_u16_as_u32 & combined_val) as u16;
    let left_val: u16 = (((!0 as u32 - max_u16_as_u32) & combined_val) >> 16) as u16;
    (left_val, right_val)
}

/// The UV coordinates go from -1 -> 1, with (-1, 1) being the top-left corner of the window, and (1, -1) being the bottom-right.
#[derive(Clone, Copy)]
pub struct Dimensions {
    start_index: usize,
    length: usize,
    capacity: usize,
    top_left: glm::Vec2,
    bottom_right: glm::Vec2
}
impl Dimensions {
    pub fn empty() -> Self {
        Dimensions {
            start_index: 0,
            length: 0,
            capacity: TextWidget::CHARS_PER_BOX,
            top_left: glm::Vec2::new(0.0, 0.0),
            bottom_right: glm::Vec2::new(0.0, 0.0)
        }
    }
}

/// This struct is for holding all the Characters in the UI. It's job is essentially to render all the characters in just 2
/// draw calls so it can be done quickly, at the cost of modularity.
pub struct TextWidget<'a> {
    // Data items.
    text_boxes: Vec<Dimensions>,
    inactive_chars: Vec<CharVertex>,
    active_chars: Vec<CharVertex>,
    active_box: Option<usize>,
    positions: Vec<BoxPosition>,
    // Graphics specific items.
    shader_program: Option<Shader<'a>>,
    uniforms: std::collections::HashMap<&'a str, gl::types::GLint>,
    ssbo: Option<SSBO>,
    a_vao: Option<VAO>,
    ia_vao: Option<VAO>,
}

impl<'a> Drop for TextWidget<'a> {
   fn drop(&mut self) {
        if let Some(a_vao) = &self.a_vao { a_vao.delete(); }
        if let Some(ia_vao) = &self.ia_vao { ia_vao.delete(); }
        if let Some(ssbo) = &self.ssbo { ssbo.delete(); }
        if let Some(shader_program) = &self.shader_program { shader_program.delete(); }
   } 
}

impl<'a> TextWidget<'a> {
    const CHARS_PER_BOX: usize = 16;
    const FONT_SIZE: f32 = 2.0 / 64.0;
    const TAB_SIZE: u32 = 4;
    const TEXT_COLOR: [f32; 4] = [1.0, 1.0, 1.0, 1.0];

    pub fn new() -> Self {
        TextWidget {
            text_boxes: vec![Dimensions::empty()],
            inactive_chars: vec![],
            active_chars: vec![],
            positions: vec![BoxPosition::new(glm::Vec4::new(-0.8, 0.8, -1.0, 1.0), [32, 32], [1000, 1000], [0, 0])],
            active_box: None,
            shader_program: None,
            uniforms: HashMap::new(),
            ssbo: None,
            a_vao: None,
            ia_vao: None,
        }
    }


    // Debug info.
    pub fn debug_print(&self) {
        println!("Active_chars:  \t{:?}", &self.active_chars.iter().map(|c| c.position.floor()).collect::<Vec<_>>());
        println!("Inactive_chars:\t{:?}", &self.inactive_chars.iter().map(|c| c.position.floor()).collect::<Vec<_>>());
        println!("\n");
    }


    // Renders all the Characters to the window.
    pub fn draw(&self) {
        if let (Some(ia_vao), Some(shader_program)) = (&self.ia_vao, &self.shader_program) {
            shader_program.activate();
            unsafe {
                /* The Cursor's gl_InstanceID u32   */ gl::Uniform1ui(*self.uniforms.get("cursor_index").unwrap(), self.inactive_chars.len() as u32 - 1);
                /* The glfwGetTime                  */ gl::Uniform1f(*self.uniforms.get("time").unwrap(), ffi::glfwGetTime() as gl::types::GLfloat);
            }
            ia_vao.bind();
            unsafe { gl::DrawArraysInstanced(gl::TRIANGLE_STRIP, 0, 4, self.inactive_chars.len() as gl::types::GLsizei); }
            ia_vao.unbind();
        } else {
            eprintln!("You cannot draw an UI element that doesn't have a shader program.\nPerhaps you forgot to do: ui.init_shader(vert_file, frag_file)")
        }

        if let (Some(a_vao), Some(shader_program), Some(_)) = (&self.a_vao, &self.shader_program, &self.active_box) {
            shader_program.activate();
            a_vao.bind();
            unsafe { gl::DrawArraysInstanced(gl::TRIANGLE_STRIP, 0, 4, self.active_chars.len() as gl::types::GLsizei); }
            a_vao.unbind();
        }
    }


    //
    pub fn init_vaos(&mut self) {
        self.reset_vao(true);
        self.reset_vao(false);
    }


    // Self explanitory I suppose.
    pub fn reset_vao(&mut self, inactive_vao: bool) {
        let target_vao = if inactive_vao { &mut self.ia_vao } else { &mut self.a_vao };
        match target_vao {
            Some(vao) => {
                vao.delete();
            },
            None => {}
        }

        *target_vao = Some(VAO::try_new());
        self.set_vao(inactive_vao);
   }


    // Binds two newly made VBOs to the VAO.
    fn set_vao(&mut self, inactive_vao: bool) {
        if let (Some(vao), Some(shader_program)) = (if inactive_vao { &self.ia_vao } else { &self.a_vao }, &self.shader_program) {
            shader_program.activate();
            vao.bind();

            let quad_vbo = VBO::try_new(&QUAD.to_vec(), gl::STATIC_DRAW);
            let chars_vbo = VBO::try_new(if inactive_vao { &self.inactive_chars } else { &self.active_chars }, gl::STATIC_DRAW);

            /*      TEMPLATE    */
            /* aPos     : vec2  */ vao.link_attrib( &quad_vbo, 0, 2, gl::FLOAT, (2 * size_of::<f32>()) as i32, std::ptr::null() as *const raw::c_void);
            /*      INSTANCE    */
            /* aLetter1 : uvec4 */ vao.link_attrib_i( &chars_vbo, 1, 4, gl::UNSIGNED_INT, size_of::<CharVertex>() as i32, 0 as *const raw::c_void);
            /* aLetter2 : uvec4 */ vao.link_attrib_i( &chars_vbo, 2, 4, gl::UNSIGNED_INT, size_of::<CharVertex>() as i32, (4 * size_of::<u32>()) as *const raw::c_void);
            /* aColor   : vec4  */ vao.link_attrib( &chars_vbo, 3, 4, gl::FLOAT, size_of::<CharVertex>() as i32, (8 * size_of::<u32>()) as *const raw::c_void);
            /* aIndex   : float */ vao.link_attrib( &chars_vbo, 4, 1, gl::FLOAT, size_of::<CharVertex>() as i32, (4 * size_of::<f32>() + 8 * size_of::<u32>()) as *const raw::c_void);
            /* aSSBO    : uint  */ vao.link_attrib_i( &chars_vbo, 5, 1, gl::UNSIGNED_INT, size_of::<CharVertex>() as i32, (5 * size_of::<f32>() + 8 * size_of::<u32>()) as *const raw::c_void);

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


    // Creates new SSBO.
    pub fn gen_ssbo(&mut self) {
        // let test_vec: Vec<f32> = vec![0.1];
        // self.ssbo = Some(SSBO::try_new(&test_vec, gl::DYNAMIC_DRAW));
        if let Some(shader_program) = &self.shader_program {
            shader_program.activate();
            // println!("{:?}", self.positions);
            self.ssbo = Some(SSBO::try_new(&self.positions, gl::DYNAMIC_DRAW));
        }
    }


    // Creates a new shader program if one doesn't already exist.
    pub fn init_shader(&mut self, vert_file: &'a str, frag_file: &'a str) -> Result<(), Box<dyn std::error::Error>> {
        if let Some(shader_program) = &self.shader_program {
            shader_program.delete();
        }
        self.shader_program = Some(Shader::try_new(vert_file, frag_file, None)?);
        self.uniforms.clear();
        unsafe {
            self.uniforms.insert("cursor_index", gl::GetUniformLocation(self.shader_program.as_ref().unwrap().get_id().unwrap(), b"cursor_index\0".as_ptr() as *const _));
            self.uniforms.insert("time",  gl::GetUniformLocation(self.shader_program.as_ref().unwrap().get_id().unwrap(), b"time\0".as_ptr() as *const _));
        }

        Ok(())
    }


    // Self expalnitory I suppose.
    pub fn reload_shader(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        if let Some(shader_program) = &mut self.shader_program {
            println!("Reloading the shader.");
            shader_program.reload()?;
        }
        self.uniforms.clear();
        unsafe {
            self.uniforms.insert("cursor_index", gl::GetUniformLocation(self.shader_program.as_ref().unwrap().get_id().unwrap(), b"cursor_index\0".as_ptr() as *const _));
            self.uniforms.insert("time",  gl::GetUniformLocation(self.shader_program.as_ref().unwrap().get_id().unwrap(), b"time\0".as_ptr() as *const _));
        }
        Ok(())
    }


    // 
    pub fn add_text_box(&mut self, text: &String, top_left: glm::Vec2, bottom_right: glm::Vec2) {
        // We're using an "SSBO" to store the general positional data for all of the text-boxes. That means,
        // we need to to assign each char what "Text Box" it's suppoed to be apart of. That's why this is here.
        let ssbo_index = self.text_boxes.len();
        let dim_x: u32 = (Self::FONT_SIZE / (top_left.x - bottom_right.x)) as u32;

        // Grabbing the "space" where this text_box is going to live in the inactive_chars Vec.
        // Essentially: we start at "start_index", and we have "capacity" as our max chars before we need to resize,
        // and then we have "length", which is the actual real chars we're using right now. If we need to add more,
        // up until we over run "capacity", we can just add new CharVertices or remove them willy-nilly with no real
        // runtime cost.
        let start_index = self.inactive_chars.len();
        let length = text.len();
        // This looks weird but we're just "round up" for unsigned integer division.
        let capacity = 
            ( (length / Self::CHARS_PER_BOX)
            + if length % Self::CHARS_PER_BOX > 0 { 1 } else { 0 } )
            * Self::CHARS_PER_BOX;

        // Adding in the new Text Box dimension.
        self.text_boxes.push(Dimensions { start_index, length, capacity, top_left, bottom_right });
        self.positions.push(BoxPosition::new(glm::Vec4::new(top_left.x, top_left.y, -1.0, 1.0), [32, 32], [1000, 1000], [0, 0]));

        // Reserving at least enough space for the new chars.
        self.inactive_chars.reserve(
            self.inactive_chars.len() + capacity
        );

        // Adding all the new chars into the self.inactive_chars Vec.
        let mut count: f32 = 0.0;
        for c in text.chars() {
            match c {
                '\n' => { count += (dim_x - (count as u32 % dim_x)) as f32; }
                '\t' => {
                    let tab_change = (count as u32 % dim_x) % (Self::TAB_SIZE + 1);
                    count += if tab_change == Self::TAB_SIZE { Self::TAB_SIZE as f32 + 1.0 } else { Self::TAB_SIZE as f32 - tab_change as f32 }
                }
                ' '..='⌂' => {
                    self.inactive_chars.push(CharVertex::new(c, ssbo_index, count, &Self::TEXT_COLOR));
                    count += 1.0;
                }
                _ => {}
            }
        }

        // Adding in the "null character" buffer characters.
        for _ in text.len()..capacity {
            self.inactive_chars.push(CharVertex::null_char());
        }
    }


    // Sets a new text box as active, and updates the inactive/active_chars Vec's accordingly.
    pub fn set_active_text_box(&mut self, text_box_index: usize) {
        // If you enter an invalid text box index, let's just set it to zero.
        if text_box_index >= self.text_boxes.len() {
            self.set_active_text_box(0);
            println!("Invalid text box index, setting to 0.");
            return;
        }

        match text_box_index {
            0 => {
                match self.active_box {
                    None => {
                        println!("The active text box is already 0!");
                        return
                    },
                    Some(active_index) => {
                        println!("Setting the active text box from {} to 0.", active_index);
                        self.insert_active_into_inactive(active_index);
                        self.text_boxes[0] = Dimensions::empty();
                        self.active_box = None;
                        self.active_chars.clear();
                        self.init_vaos();
                    }
                }
            }
            
            text_box_index => {
                // If we have a previous, active text box, let's make sure to add it's contents back into the inactive_chars vec.
                if let Some(active_index) = self.active_box {
                    if active_index == text_box_index {
                        println!("The active text box is already set to {}!", active_index);
                        return;
                    }

                    println!("Setting the active text box from {} to {}.", active_index, text_box_index);
                    self.insert_active_into_inactive(active_index);
                } else {
                    println!("Setting the active text box from None to {}.", text_box_index);
                }

                // Updating the active_box.
                self.active_box = Some(text_box_index);
                self.text_boxes[0] = self.text_boxes[text_box_index];
                self.set_new_active(text_box_index);
                self.init_vaos();
            }
        }
    }


    //
    pub fn add_slice_to_active(&mut self, text: &str) {
        if let Some(_) = self.active_box {
            let dimensions = &self.text_boxes[0];
            let old_cap = dimensions.capacity;
            let old_len = dimensions.length;
            let text_len = text.len();

            let new_cap = self.capacity_from_length(old_len + text_len);

            let chars_slice = &self.add_chars(text, 0, text_len, self.active_chars[old_len - 1].position.floor() + 1.0);
            if old_cap < new_cap {
                self.active_chars.resize(new_cap, CharVertex::null_char());
            }
            self.active_chars[old_len..old_len + text_len].copy_from_slice(&chars_slice);
            
            let new_len = old_len + text_len;
            self.text_boxes[0].capacity = new_cap;
            self.text_boxes[0].length = new_len;

            self.reset_vao(false);
        }
    }


    // Replaces the active text box with a new string. This is useful for when you want to replace the entire text box with a new string.
    pub fn replace_active_with_slice(&mut self, text: &str) {
        if let Some(_) = self.active_box {
            let dimensions = &self.text_boxes[0];
            let old_cap = dimensions.capacity;
            let text_len = text.len();

            let new_cap = self.capacity_from_length(text_len);

            let chars_slice = &self.add_chars(text, 0, new_cap, 0.0);

            if old_cap != new_cap { self.active_chars.resize(new_cap, CharVertex::null_char()); }

            self.active_chars[..new_cap].copy_from_slice(&chars_slice);
            
            let new_len = text_len;
            self.text_boxes[0].capacity = new_cap;
            self.text_boxes[0].length = new_len;

            self.reset_vao(false);
        }
    }


    //
    fn get_dim_x(&self, ssbo_index: usize) -> u32 {
        let top_left = &self.text_boxes[ssbo_index].top_left;
        let bottom_right = &self.text_boxes[ssbo_index].bottom_right;
        
        (Self::FONT_SIZE / (top_left.x - bottom_right.x)) as u32
    }


    //
    fn add_chars(&self, text: &str, ssbo_index: usize, capacity: usize, mut count: f32) -> Vec<CharVertex> {
        println!("[TextWidget] Adding \"{}\" to the text box at index {}, starting at {}.", text, ssbo_index, count);

        let mut chars_vec = vec![];
        let dim_x = self.get_dim_x(ssbo_index);

        for c in text.chars() {
            match c {
                '\n' => { count += (dim_x - (count as u32 % dim_x)) as f32; }
                '\t' => {
                    let tab_change = (count as u32 % dim_x) % (Self::TAB_SIZE + 1);
                    count += if tab_change == Self::TAB_SIZE { Self::TAB_SIZE as f32 + 1.0 } else { Self::TAB_SIZE as f32 - tab_change as f32 }
                }
                ' '..='⌂' => {
                    chars_vec.push(CharVertex::new(c, ssbo_index, count, &Self::TEXT_COLOR));
                    count += 1.0;
                }
                _ => {}
            }
        }

        // Adding in the "null character" buffer characters.
        for _ in text.len()..capacity {
            chars_vec.push(CharVertex::null_char());
        }

        chars_vec
    }


    // Gets a new capacity.
    fn capacity_from_length(&self, length: usize) -> usize {
        ( (length / Self::CHARS_PER_BOX)
        + if length % Self::CHARS_PER_BOX > 0 { 1 } else { 0 } )
        * Self::CHARS_PER_BOX
    }


    // Sets the inactive/active_chars from a given range.
    fn set_new_active(&mut self, text_box_index: usize) {
        self.positions[0] = self.positions[text_box_index];

        let new_active_dimensions = &self.text_boxes[text_box_index];
        let start = new_active_dimensions.start_index;
        let end = start + new_active_dimensions.capacity;
        
        let capacity = self.capacity_from_length(new_active_dimensions.length);
        let new_active_slice = &self.inactive_chars[start..end];

        // Setting active_chars.
        self.active_chars.resize(capacity, CharVertex::null_char());
        self.active_chars.copy_from_slice(new_active_slice);

        // Nulling out the old rannge in the inactive_chars.
        let null_chars_range = vec![CharVertex::null_char(); new_active_dimensions.capacity];
        self.inactive_chars[start..end].copy_from_slice(&null_chars_range);
        
        self.gen_ssbo();
    }


    // This function sets the value of a given range with the data in active_chars. 
    fn insert_active_into_inactive(&mut self, active_index: usize) {
        let old_active_dimensions = &self.text_boxes[active_index];
        let active_len = self.active_chars.len();
        let old_active_cap = old_active_dimensions.capacity;

        let start = old_active_dimensions.start_index;
        let end = start + old_active_cap;
        
        if old_active_cap == active_len { // We don't need to resize the inactive_chars Vec! :D
        
            self.text_boxes[active_index].length = self.text_boxes[0].length;
            self.inactive_chars[start..end].copy_from_slice(&self.active_chars);
       
        } else { // We need to resize the inactive_chars Vec. :(
            self.fast_slice_replace(start, end);

            self.text_boxes[active_index].capacity = self.text_boxes[0].capacity;
            self.text_boxes[active_index].length = self.text_boxes[0].length;

            // Updating all the starting indices of the other items in the text_boxes in the Vec.
            if active_len > old_active_cap {
                for index in active_index + 1..self.text_boxes.len() {
                    self.text_boxes[index].start_index += active_len - old_active_cap;
                }
            } else {
                for index in active_index + 1..self.text_boxes.len() {
                    self.text_boxes[index].start_index -= old_active_cap - active_len;
                }
            }
        }        
    }


    // Decently performant slice insertion.
    fn fast_slice_insert(&mut self, index: usize) {
        self.inactive_chars.reserve(self.active_chars.len()); // optional, but increases the performance
        let mut v = self.inactive_chars.split_off(index);
        self.inactive_chars.extend_from_slice(&self.active_chars);
        self.inactive_chars.append(&mut v);
    }


    // Decently performant slice replacing.
    #[allow(unused)]
    fn fast_slice_replace(&mut self, start: usize, end: usize) {
        self.inactive_chars.drain(start..end);
        self.fast_slice_insert(start);
    }

}


#[cfg(test)]
mod tests {
    use super::*;
    // use super::super::graphics_engine;

    const PRINT_RESULTS: bool = false;

    // Generates the a CharVertex of the desired type, index and position.
    fn gen_char_vertex(text_char: char) -> CharVertex {
        let (_, bit_array) = CHAR_MAP[text_char as usize - 32];
        CharVertex{
            letter: [
                glm::UVec4::new(bit_array[0], bit_array[1], bit_array[2], bit_array[3]),
                glm::UVec4::new(bit_array[4], bit_array[5], bit_array[6], bit_array[7])],
            color: glm::Vec4::new(1.0, 1.0, 1.0, 1.0),
            position: 0.0,
            ssbo_index: 0 
        }
    }


    // Worlds slowest way to reverse engineer what the character I'm looking for is.
    fn char_from_charvertex(target_char: &CharVertex) -> char {
        for i in 32..127 as u8 {
            let ch = i as char;
            let char_vert = CharVertex::new(ch, 0, 0.0, &[0.0, 0.0, 0.0, 0.0]);
            if char_vert.letter == target_char.letter {
                return ch;
            }
        }

        ' '
    }


    // Print chars.
    fn print_chars(widget: &TextWidget) {
        if PRINT_RESULTS {
            println!("\n\n");
            let inactive_iter: Vec<char> = widget.inactive_chars.iter().map(|ch| char_from_charvertex(ch)).collect();
            let active_iter: Vec<char> = widget.active_chars.iter().map(|ch| char_from_charvertex(ch)).collect();

            println!("\n\t[ -- Inactive -- ]");
            let steps = inactive_iter.len() / TextWidget::CHARS_PER_BOX;
            for i in 0..steps {
                let start = i * TextWidget::CHARS_PER_BOX;
                let end = (i + 1) * TextWidget::CHARS_PER_BOX;
                println!("{:?}", &inactive_iter[start..end]);
            }

            println!("\n\t[ -- Active -- ]");
            let steps = active_iter.len() / TextWidget::CHARS_PER_BOX;
            for i in 0..steps {
                let start = i * TextWidget::CHARS_PER_BOX;
                let end = (i + 1) * TextWidget::CHARS_PER_BOX;
                println!("{:?}", &active_iter[start..end]);
            }
        }
    }


    // Test to make sure the size of the inactive_chars matches what is expected for some different scinerios.
    #[test]
    fn add_text_box_checking_length() {
        let uv_top_left = glm::Vec2::new(0.0, 0.0);
        let uv_bottom_right = glm::Vec2::new(0.0, 0.0);


        let mut twice_chars_per_box = String::new(); for _ in 0..(TextWidget::CHARS_PER_BOX * 2) { twice_chars_per_box.push('c'); }
        let mut twice_chars_per_box_plus_one = String::new(); for _ in 0..(TextWidget::CHARS_PER_BOX * 2 + 1) { twice_chars_per_box_plus_one.push('c'); }
        let mut thrice_chars_per_box_minus_one = String::new(); for _ in 0..(TextWidget::CHARS_PER_BOX * 3 - 1) { thrice_chars_per_box_minus_one.push('c'); }

        let mut widget = TextWidget::new(); widget.add_text_box(&twice_chars_per_box, uv_top_left, uv_bottom_right);
        assert_eq!(widget.inactive_chars.len(), TextWidget::CHARS_PER_BOX * 2);

        let mut widget = TextWidget::new(); widget.add_text_box(&twice_chars_per_box_plus_one, uv_top_left, uv_bottom_right);
        assert_eq!(widget.inactive_chars.len(), TextWidget::CHARS_PER_BOX * 3);

        let mut widget = TextWidget::new(); widget.add_text_box(&thrice_chars_per_box_minus_one, uv_top_left, uv_bottom_right);
        assert_eq!(widget.inactive_chars.len(), TextWidget::CHARS_PER_BOX * 3);
    }


    #[test]
    fn add_multiple_text_boxes_checking_length() {
        let uv_top_left = glm::Vec2::new(0.0, 0.0);
        let uv_bottom_right = glm::Vec2::new(0.0, 0.0);


        let mut twice_chars_per_box = String::new(); for _ in 0..(TextWidget::CHARS_PER_BOX * 2) { twice_chars_per_box.push('c'); }
        let mut twice_chars_per_box_plus_one = String::new(); for _ in 0..(TextWidget::CHARS_PER_BOX * 2 + 1) { twice_chars_per_box_plus_one.push('c'); }
        let mut thrice_chars_per_box_minus_one = String::new(); for _ in 0..(TextWidget::CHARS_PER_BOX * 3 - 1) { thrice_chars_per_box_minus_one.push('c'); }

        let mut widget = TextWidget::new();
        widget.add_text_box(&twice_chars_per_box, uv_top_left, uv_bottom_right);
        widget.add_text_box(&twice_chars_per_box, uv_top_left, uv_bottom_right);
        assert_eq!(widget.inactive_chars.len(), TextWidget::CHARS_PER_BOX * 4);

        let mut widget = TextWidget::new();
        widget.add_text_box(&twice_chars_per_box, uv_top_left, uv_bottom_right);
        widget.add_text_box(&twice_chars_per_box_plus_one, uv_top_left, uv_bottom_right);
        assert_eq!(widget.inactive_chars.len(), TextWidget::CHARS_PER_BOX * 5);

        let mut widget = TextWidget::new();
        widget.add_text_box(&thrice_chars_per_box_minus_one, uv_top_left, uv_bottom_right);
        widget.add_text_box(&twice_chars_per_box_plus_one, uv_top_left, uv_bottom_right);
        assert_eq!(widget.inactive_chars.len(), TextWidget::CHARS_PER_BOX * 6);
    }


    #[test]
    fn add_text_box_checking_contents() {
        let uv_top_left = glm::Vec2::new(0.0, 0.0);
        let uv_bottom_right = glm::Vec2::new(0.0, 0.0);

        let null_char = CharVertex::null_char().letter;
        let c_char = gen_char_vertex('c').letter;

        let mut twice_chars_per_box = String::new(); for _ in 0..(TextWidget::CHARS_PER_BOX * 2) { twice_chars_per_box.push('c'); }
        let mut twice_chars_per_box_plus_one = String::new(); for _ in 0..(TextWidget::CHARS_PER_BOX * 2 + 1) { twice_chars_per_box_plus_one.push('c'); }
        let mut thrice_chars_per_box_minus_one = String::new(); for _ in 0..(TextWidget::CHARS_PER_BOX * 3 - 1) { thrice_chars_per_box_minus_one.push('c'); }

        let mut widget = TextWidget::new(); widget.add_text_box(&twice_chars_per_box, uv_top_left, uv_bottom_right);
        assert!(widget.inactive_chars.iter().all(|&item| item.letter == c_char ));

        let mut widget = TextWidget::new(); widget.add_text_box(&twice_chars_per_box_plus_one, uv_top_left, uv_bottom_right);
        let length = widget.text_boxes[1].length;
        assert!(widget.inactive_chars[..length].iter().all(|&item| item.letter == c_char));
        assert!(widget.inactive_chars[length..].iter().all(|&item| item.letter == null_char ));

        let mut widget = TextWidget::new(); widget.add_text_box(&thrice_chars_per_box_minus_one, uv_top_left, uv_bottom_right);
        let length = widget.text_boxes[1].length;
        assert!(widget.inactive_chars[..length].iter().all(|&item| item.letter == c_char));
        assert!(widget.inactive_chars[length..].iter().all(|&item| item.letter == null_char ));
    }


    #[test]
    fn add_multiple_text_boxes_checking_contents() {
        let uv_top_left = glm::Vec2::new(0.0, 0.0);
        let uv_bottom_right = glm::Vec2::new(0.0, 0.0);

        let null_char = CharVertex::null_char().letter;
        let c_char = gen_char_vertex('c').letter;

        let mut twice_chars_per_box = String::new(); for _ in 0..(TextWidget::CHARS_PER_BOX * 2) { twice_chars_per_box.push('c'); }
        let mut twice_chars_per_box_plus_one = String::new(); for _ in 0..(TextWidget::CHARS_PER_BOX * 2 + 1) { twice_chars_per_box_plus_one.push('c'); }
        let mut thrice_chars_per_box_minus_one = String::new(); for _ in 0..(TextWidget::CHARS_PER_BOX * 3 - 1) { thrice_chars_per_box_minus_one.push('c'); }

        let mut widget = TextWidget::new();
        widget.add_text_box(&twice_chars_per_box, uv_top_left, uv_bottom_right);
        widget.add_text_box(&twice_chars_per_box, uv_top_left, uv_bottom_right);
        let first_capacity = widget.text_boxes[1].capacity;
        let first_length = widget.text_boxes[1].length;
        let second_length = widget.text_boxes[2].length;
        let second_start = widget.text_boxes[2].start_index;
        assert!(widget.inactive_chars[..first_length].iter().all(|&item| item.letter == c_char));
        assert!(widget.inactive_chars[first_length..first_capacity].iter().all(|&item| item.letter == null_char ));
        assert!(widget.inactive_chars[second_start..(second_start + second_length)].iter().all(|&item| item.letter == c_char));
        assert!(widget.inactive_chars[(second_start + second_length)..].iter().all(|&item| item.letter == null_char ));

        let mut widget = TextWidget::new();
        widget.add_text_box(&twice_chars_per_box, uv_top_left, uv_bottom_right);
        widget.add_text_box(&twice_chars_per_box_plus_one, uv_top_left, uv_bottom_right);
        let first_capacity = widget.text_boxes[1].capacity;
        let first_length = widget.text_boxes[1].length;
        let second_length = widget.text_boxes[2].length;
        let second_start = widget.text_boxes[2].start_index;
        assert!(widget.inactive_chars[..first_length].iter().all(|&item| item.letter == c_char));
        assert!(widget.inactive_chars[first_length..first_capacity].iter().all(|&item| item.letter == null_char ));
        assert!(widget.inactive_chars[second_start..(second_start + second_length)].iter().all(|&item| item.letter == c_char));
        assert!(widget.inactive_chars[(second_start + second_length)..].iter().all(|&item| item.letter == null_char ));


        let mut widget = TextWidget::new();
        widget.add_text_box(&thrice_chars_per_box_minus_one, uv_top_left, uv_bottom_right);
        widget.add_text_box(&twice_chars_per_box_plus_one, uv_top_left, uv_bottom_right);
        let first_capacity = widget.text_boxes[1].capacity;
        let first_length = widget.text_boxes[1].length;
        let second_length = widget.text_boxes[2].length;
        let second_start = widget.text_boxes[2].start_index;
        assert!(widget.inactive_chars[..first_length].iter().all(|&item| item.letter == c_char));
        assert!(widget.inactive_chars[first_length..first_capacity].iter().all(|&item| item.letter == null_char ));
        assert!(widget.inactive_chars[second_start..(second_start + second_length)].iter().all(|&item| item.letter == c_char));
        assert!(widget.inactive_chars[(second_start + second_length)..].iter().all(|&item| item.letter == null_char ));
    }


    #[test]
    fn setting_active_chars_from_none() {
        let uv_top_left = glm::Vec2::new(0.0, 0.0);
        let uv_bottom_right = glm::Vec2::new(0.0, 0.0);

        let null_char = CharVertex::null_char().letter;
        let c_char = gen_char_vertex('c').letter;


        let mut twice_chars_per_box_plus_one = String::new(); for _ in 0..(TextWidget::CHARS_PER_BOX * 2 + 1) { twice_chars_per_box_plus_one.push('c'); }
        let mut widget = TextWidget::new();
        widget.add_text_box(&twice_chars_per_box_plus_one, uv_top_left, uv_bottom_right);
        widget.add_text_box(&twice_chars_per_box_plus_one, uv_top_left, uv_bottom_right);
        widget.add_text_box(&twice_chars_per_box_plus_one, uv_top_left, uv_bottom_right);

        // First set of chars.
        let first_capacity = widget.text_boxes[1].capacity;
        let first_length = widget.text_boxes[1].length;
        // Second set of chars.
        let second_capacity = widget.text_boxes[2].capacity;
        let second_length = widget.text_boxes[2].length;
        let second_start = widget.text_boxes[2].start_index;
        // Third set of chars.
        let third_capacity = widget.text_boxes[3].capacity;
        let third_length = widget.text_boxes[3].length;
        let third_start = widget.text_boxes[3].start_index;

        // Testing first set of chars.
        assert!(widget.inactive_chars[..first_length].iter().all(|&item| item.letter == c_char));
        assert!(widget.inactive_chars[first_length..first_capacity].iter().all(|&item| item.letter == null_char ));

        // Testing second set of chars.
        assert!(widget.inactive_chars[second_start..(second_start + second_length)].iter().all(|&item| item.letter == c_char));
        assert!(widget.inactive_chars[(second_start + second_length)..(second_start + second_capacity)].iter().all(|&item| item.letter == null_char ));

        // Testing third set of chars.
        assert!(widget.inactive_chars[third_start..(third_start + third_length)].iter().all(|&item| item.letter == c_char));
        assert!(widget.inactive_chars[(third_start + third_length)..(third_start + third_capacity)].iter().all(|&item| item.letter == null_char ));

        // Setting the active text box, to 1. This should set the the:
        // start_index -> start_index + length
        // of the second chars set, to the null_char. So in essence, all the chars in the entire second
        // set should all be null_char now, and what WAS in the second char set, should now be in active_chars.
        let mut old_second_chars_set: Vec<CharVertex> = vec![CharVertex::null_char(); third_capacity];
        old_second_chars_set.copy_from_slice(&widget.inactive_chars[third_start..(third_start + third_capacity)]);
        widget.set_active_text_box(2);

        // inactive_chars got nulled correctly.
        assert!(widget.inactive_chars[second_start..(second_start + second_capacity)].iter().all(|&item| item.letter == null_char));

        // active chars got set correctly.
        assert!(widget.active_chars[..second_length].iter().all(|&item| item.letter == c_char));
        assert!(widget.active_chars[second_length..].iter().all(|&item| item.letter == null_char ));
    }


    #[test]
    fn setting_active_chars_from_some() {
        // This test is going to check, if we set active_chars from a None state, then set it again to something different,
        // if it will work properly.
        let uv_top_left = glm::Vec2::new(0.0, 0.0);
        let uv_bottom_right = glm::Vec2::new(0.0, 0.0);

        let null_char = CharVertex::null_char().letter;
        let c_char = gen_char_vertex('c').letter;


        let mut twice_chars_per_box = String::new(); for _ in 0..(TextWidget::CHARS_PER_BOX * 2) { twice_chars_per_box.push('c'); }
        let mut twice_chars_per_box_plus_one = String::new(); for _ in 0..(TextWidget::CHARS_PER_BOX * 2 + 1) { twice_chars_per_box_plus_one.push('c'); }
        let mut widget = TextWidget::new();
        widget.add_text_box(&twice_chars_per_box, uv_top_left, uv_bottom_right);
        widget.add_text_box(&twice_chars_per_box_plus_one, uv_top_left, uv_bottom_right);
        widget.add_text_box(&twice_chars_per_box_plus_one, uv_top_left, uv_bottom_right);

        // First set of chars.
        let first_capacity = widget.text_boxes[1].capacity;
        let first_length = widget.text_boxes[1].length;
        // Second set of chars.
        let second_capacity = widget.text_boxes[2].capacity;
        let second_length = widget.text_boxes[2].length;
        let second_start = widget.text_boxes[2].start_index;
        // Third set of chars.
        let third_capacity = widget.text_boxes[3].capacity;
        let third_length = widget.text_boxes[3].length;
        let third_start = widget.text_boxes[3].start_index;

        print_chars(&widget);

        // Setting the active text box, to 1. This should set the the:
        // start_index -> start_index + length
        // of the second chars set, to the null_char. So in essence, all the chars in the entire second
        // set should all be null_char now, and what WAS in the second char set, should now be in active_chars.
        widget.set_active_text_box(2);
        print_chars(&widget);
        
        widget.set_active_text_box(1);
        print_chars(&widget);


        // Testing first set of chars.
        assert!(widget.inactive_chars[..first_capacity].iter().all(|&item| item.letter == null_char));

        // Testing second set of chars.
        assert!(widget.inactive_chars[second_start..(second_start + second_length)].iter().all(|&item| item.letter == c_char));
        assert!(widget.inactive_chars[(second_start + second_length)..(second_start + second_capacity)].iter().all(|&item| item.letter == null_char ));

        // Testing third set of chars.
        assert!(widget.inactive_chars[third_start..(third_start + third_length)].iter().all(|&item| item.letter == c_char));
        assert!(widget.inactive_chars[(third_start + third_length)..(third_start + third_capacity)].iter().all(|&item| item.letter == null_char ));

        let mut old_second_chars_set: Vec<CharVertex> = vec![CharVertex::null_char(); third_capacity];
        old_second_chars_set.copy_from_slice(&widget.inactive_chars[third_start..(third_start + third_capacity)]);
        widget.set_active_text_box(1);

        // inactive_chars got nulled correctly.
        assert!(widget.inactive_chars[..first_capacity].iter().all(|&item| item.letter == null_char));

        // active chars got set correctly.
        assert!(widget.active_chars[..first_length].iter().all(|&item| item.letter == c_char));
        assert!(widget.active_chars[first_length..].iter().all(|&item| item.letter == null_char ));

    }


    #[test]
    fn adding_slice_to_active() {
        // This test is going to check, if we set active_chars from a None state, then set it again to something different,
        // if it will work properly.
        let uv_top_left = glm::Vec2::new(0.0, 0.0);
        let uv_bottom_right = glm::Vec2::new(0.0, 0.0);

        // let null_char = CharVertex::null_char().letter;
        // let c_char = gen_char_vertex('c').letter;


        let mut twice_chars_per_box = String::new(); for _ in 0..(TextWidget::CHARS_PER_BOX * 2) { twice_chars_per_box.push('c'); }
        let mut twice_chars_per_box_plus_one = String::new(); for _ in 0..(TextWidget::CHARS_PER_BOX * 2 + 1) { twice_chars_per_box_plus_one.push('c'); }
        let mut widget = TextWidget::new();
        widget.add_text_box(&twice_chars_per_box, uv_top_left, uv_bottom_right);
        widget.add_text_box(&twice_chars_per_box_plus_one, uv_top_left, uv_bottom_right);
        widget.add_text_box(&twice_chars_per_box_plus_one, uv_top_left, uv_bottom_right);

        // // First set of chars.
        // let first_capacity = widget.text_boxes[1].capacity;
        // let first_length = widget.text_boxes[1].length;
        // // Second set of chars.
        // let second_capacity = widget.text_boxes[2].capacity;
        // let second_length = widget.text_boxes[2].length;
        // let second_start = widget.text_boxes[2].start_index;
        // // Third set of chars.
        // let third_capacity = widget.text_boxes[3].capacity;
        // let third_length = widget.text_boxes[3].length;
        // let third_start = widget.text_boxes[3].start_index;

        print_chars(&widget);

        // Setting the active text box, to 1. This should set the the:
        // start_index -> start_index + length
        // of the second chars set, to the null_char. So in essence, all the chars in the entire second
        // set should all be null_char now, and what WAS in the second char set, should now be in active_chars.
        widget.set_active_text_box(1);
        print_chars(&widget);

        widget.add_slice_to_active("oh, hi man!");
        print_chars(&widget);

        widget.add_slice_to_active(" there's more text where that came from!");
        print_chars(&widget);

        widget.add_slice_to_active("12345678987654");
        print_chars(&widget);

        widget.set_active_text_box(2);
        print_chars(&widget);

        widget.set_active_text_box(0);
        print_chars(&widget);

        widget.set_active_text_box(0);
        print_chars(&widget);

        widget.set_active_text_box(100);
        print_chars(&widget);

    }
}