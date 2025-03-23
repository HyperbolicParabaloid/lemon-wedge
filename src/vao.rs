use crate::vbo::VBO;
use std::os::raw::c_void;

pub struct VAO {
    id: u32
}

impl VAO {
    pub fn try_new() -> Self {
        let mut id = 0;

        unsafe {
            gl::GenVertexArrays(1, &mut id);
        }

        VAO { id }
    }

    pub fn link_attrib(&self, vbo: &VBO, layout: u32, num_components: i32, vbo_type: u32, stride: gl::types::GLsizei, offset: *const c_void) {
        vbo.bind();
        unsafe {
            gl::VertexAttribPointer(layout, num_components, vbo_type, gl::FALSE, stride, offset);
            gl::EnableVertexAttribArray(layout);
        }
        vbo.unbind();

    }

    pub fn link_attrib_i(&self, vbo: &VBO, layout: u32, num_components: i32, vbo_type: u32, stride: gl::types::GLsizei, offset: *const c_void) {
        vbo.bind();
        unsafe {
            gl::VertexAttribIPointer(layout, num_components, vbo_type, stride, offset);
            gl::EnableVertexAttribArray(layout);
        }
        vbo.unbind();
    }

    pub fn bind(&self) {
        unsafe {
            gl::BindVertexArray(self.id);
        }
    }

    pub fn unbind(&self) {
        unsafe {
            gl::BindVertexArray(0);
        }
    }

    pub fn delete(&self) {
        unsafe {
            gl::DeleteVertexArrays(1, &self.id);
        }
    }
}


