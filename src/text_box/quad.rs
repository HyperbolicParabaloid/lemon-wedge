use crate::{shader::Shader, vao::VAO, vbo::VBO};

pub struct Quad<'a> {
    positions: Vec<glm::Vec4>,
    shader_program: Option<Shader<'a>>,
    vao: Option<VAO>
}
impl<'a> Quad<'a> {
    pub fn new() -> Quad<'a> {
        Self {
            positions: vec![],
            shader_program: None,
            vao: None,
        }
    }

    pub fn add_bg(&mut self, top_left: glm::Vec2, bottom_right: glm::Vec2) {
        self.positions.push(glm::vec4(top_left.x, top_left.y, bottom_right.x, bottom_right.y));
        self.reset_vao();
    }

    pub fn init_shader(&mut self, vert_file: &'a str, frag_file: &'a str, geom_file: &'a str) -> Result<(), Box<dyn std::error::Error>> {
        self.shader_program = Some(Shader::try_new(vert_file, frag_file, Some(geom_file))?);
        Ok(())
    }

    pub fn reload_shader(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        if let Some(shader_program) = &mut self.shader_program {
            shader_program.reload()?;
        }
        Ok(())
    }

    fn set_vao(&mut self) {
        if let (Some(vao), Some(shader_program)) = (&self.vao, &self.shader_program) {
            shader_program.activate();
            vao.bind();

            let vbo = VBO::try_new(&self.positions, gl::STATIC_DRAW);
            /* topLeft      : vec2 */ vao.link_attrib( &vbo, 0, 4, gl::FLOAT, size_of::<glm::Vec4>() as gl::types::GLsizei, 0 as *const std::os::raw::c_void);

            vao.unbind();
            vbo.unbind();
        }
    }

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

    pub fn draw(&self) {
        if let (Some(vao), Some(shader_program)) = (&self.vao, &self.shader_program) {
            shader_program.activate();
            vao.bind();
	        unsafe { gl::DrawArrays(gl::POINTS, 0, self.positions.len() as i32); }
            vao.unbind();
        } else {
            eprintln!("You cannot draw an Quad element that doesn't have a shader program.\nPerhaps you forgot to do: ui.init_shader(vert_file, frag_file)")
        }
    }
    // Add a cleanup method
    pub fn cleanup(&mut self) {
        if let Some(vao) = &self.vao {
            vao.delete();
            self.vao = None;
        }
    }
}

// Implement Drop to ensure cleanup
impl<'a> Drop for Quad<'a> {
    fn drop(&mut self) {
        self.cleanup();
    }
}
