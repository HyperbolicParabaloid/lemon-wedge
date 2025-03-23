use crate::{shader::Shader, vao::VAO, vbo::VBO};

pub const QUAD: [f32; 12] = [
    -1.0, -1.0, -1.0, 
     1.0, -1.0, -1.0,
    -1.0,  1.0, -1.0, 
     1.0,  1.0, -1.0,
];

pub struct Quad<'a> {
    shader_program: Option<Shader<'a>>,
    vao: Option<VAO>
}
impl<'a> Quad<'a> {
    pub fn new() -> Quad<'a> {
        Self {
            shader_program: None,
            vao: None,
        }
    }

    pub fn init_shader(&mut self, vert_file: &'a str, frag_file: &'a str) -> Result<(), Box<dyn std::error::Error>> {
        self.shader_program = Some(Shader::try_new(vert_file, frag_file)?);
        Ok(())
    }

    fn set_vao(&mut self) {
        if let (Some(vao), Some(shader_program)) = (&self.vao, &self.shader_program) {
            shader_program.activate();
            vao.bind();

            let quad_vbo = VBO::try_new(&QUAD.to_vec(), gl::STATIC_DRAW);
            let instanced_vbo = VBO::try_new(&(0..20).collect::<Vec<i32>>(), gl::STATIC_DRAW);

            // Position Vec2
            vao.link_attrib( &quad_vbo, 0, 3, gl::FLOAT, (3 * size_of::<f32>()) as gl::types::GLsizei, std::ptr::null() as *const std::os::raw::c_void);
            vao.link_attrib_i( &instanced_vbo, 1, 1, gl::INT, size_of::<i32>() as gl::types::GLsizei, std::ptr::null::<i32>() as *const std::os::raw::c_void);

            unsafe {
                gl::VertexAttribDivisor(1, 1);
            }

            vao.unbind();
            quad_vbo.unbind();
            instanced_vbo.unbind();
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
	        // unsafe { gl::DrawElementsInstanced(gl::TRIANGLE_STRIP, 4, gl::UNSIGNED_INT, std::ptr::null(), 10); }
	        unsafe { gl::DrawArraysInstanced(gl::TRIANGLE_STRIP, 0, 4, 20); }
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
