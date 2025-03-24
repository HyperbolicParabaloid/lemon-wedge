pub struct SSBO {
    id: u32
}

impl SSBO {
    pub fn try_new<T>(vertices: &Vec<T>, draw_type: u32) -> Self {
        let mut id = 0;

        unsafe {
            /*
        // Check OpenGL version first
        let version = gl::GetString(gl::VERSION);
        let version_str = std::ffi::CStr::from_ptr(version as *const i8)
            .to_str()
            .unwrap_or("Unknown");
        println!("OpenGL Version: {}", version_str);
        
        // Check if SSBOs are supported
        let mut num_extensions = 0;
        gl::GetIntegerv(gl::NUM_EXTENSIONS, &mut num_extensions);
        let mut has_ssbo_support = false;
        
        for i in 0..num_extensions {
            let ext = gl::GetStringi(gl::EXTENSIONS, i as u32);
            let ext_str = std::ffi::CStr::from_ptr(ext as *const i8)
                .to_str()
                .unwrap_or("Unknown");
            
            if ext_str.contains("GL_ARB_shader_storage_buffer_object") {
                has_ssbo_support = true;
                println!("SSBO extension supported!");
                break;
            }
        }
        
        if !has_ssbo_support {
            panic!("Shader Storage Buffer Objects not supported");
        }
        
        // Try creating the SSBO
        gl::GenBuffers(1, &mut id);
        let error = gl::GetError();
        if error != gl::NO_ERROR {
            panic!("OpenGL error after GenBuffers: {}", error);
        }
        
        gl::BindBuffer(gl::SHADER_STORAGE_BUFFER, id);
        let error = gl::GetError();
        if error != gl::NO_ERROR {
            panic!("OpenGL error after BindBuffer: {}", error);
        }
        
        gl::BufferData(
            gl::SHADER_STORAGE_BUFFER,
            (vertices.len() * size_of::<T>()) as gl::types::GLsizeiptr,
            vertices.as_ptr() as *const _,
            draw_type,
        );
        let error = gl::GetError();
        if error != gl::NO_ERROR {
            panic!("OpenGL error after BufferData: {}", error);
        }
        
        gl::BindBufferBase(gl::SHADER_STORAGE_BUFFER, 0, id);
        let error = gl::GetError();
        if error != gl::NO_ERROR {
            panic!("OpenGL error after BindBufferBase: {}", error);
        }
        */




            gl::GenBuffers(1, &mut id);
            gl::BindBuffer(gl::SHADER_STORAGE_BUFFER, id);
            gl::BufferData(
                gl::SHADER_STORAGE_BUFFER,
                (vertices.len() * size_of::<T>()) as gl::types::GLsizeiptr,
                vertices.as_ptr() as *const _,
                draw_type);
            gl::BindBufferBase(gl::SHADER_STORAGE_BUFFER, 0, id);
        }
        
        SSBO { id }
    }

    pub fn update_data<T>(&self, vertices: Vec<T>, start_index: usize, end_index: usize) {
        // Update just the modified portion of the SSBO
        unsafe {
            gl::BindBuffer(gl::SHADER_STORAGE_BUFFER, self.id);
            gl::BufferSubData(
                gl::SHADER_STORAGE_BUFFER,
                (start_index * std::mem::size_of::<T>()) as isize,
                ((end_index - start_index + 1) * std::mem::size_of::<T>()) as isize,
                vertices.as_ptr().add(start_index) as *const _,
            );
        }
    }

    pub fn bind(&self) {
        unsafe {
            gl::BindBuffer(gl::SHADER_STORAGE_BUFFER, self.id);
        }
    }

    pub fn unbind(&self) {
        unsafe {
            gl::BindBuffer(gl::SHADER_STORAGE_BUFFER, 0);
        }
    }

    pub fn delete(&self) {
        unsafe {
            gl::DeleteBuffers(1, &self.id);
        }
    }
}

impl Drop for SSBO {
    fn drop(&mut self) {
        self.delete();
    }
}