pub struct VBO {
    id: u32
}

impl VBO {
    pub fn try_new<T>(vertices: &Vec<T>, draw_type: u32) -> Self {
        let mut id = 0;
        unsafe {
            gl::GenBuffers(1, &mut id);
            gl::BindBuffer(gl::ARRAY_BUFFER, id);
            gl::BufferData(
                gl::ARRAY_BUFFER,
                (vertices.len() * size_of::<T>()) as gl::types::GLsizeiptr,
                vertices.as_ptr() as *const _,
                draw_type);
        }
        
        VBO { id }
    }

    pub fn bind(&self) {
        unsafe {
            gl::BindBuffer(gl::ARRAY_BUFFER, self.id);
        }
    }

    pub fn unbind(&self) {
        unsafe {
            gl::BindBuffer(gl::ARRAY_BUFFER, 0);
        }
    }

    pub fn delete(&self) {
        unsafe {
            gl::DeleteBuffers(1, &self.id);
        }
    }
}

impl Drop for VBO {
    fn drop(&mut self) {
        self.delete();
    }
}