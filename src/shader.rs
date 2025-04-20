use std::{error::Error, ffi::CString, fs, path, ptr::null, time::SystemTime};

pub struct Shader<'a> {
	id: Option<u32>,
	vert_file: &'a str,
	frag_file: &'a str,
	geom_file: Option<&'a str>
}

impl<'a> Shader<'a> {
	pub fn try_new(vert_file: &'a str, frag_file: &'a str, geom_file: Option<&'a str>) -> Result<Self, Box<dyn std::error::Error>> {
		// Returning the Shader program.
		let mut shader = Shader {
			id: None,
			vert_file,
			frag_file,
			geom_file
		};

		shader.reload()?;
		Ok(shader)
	}


	// Reloads the shader program, if it has been changed.
	pub fn reload(&mut self) -> Result<(), Box<dyn Error>> {
		match self.id {
			Some(_) => self.delete(),
			None => {}
		}

		let vert_contents = CString::new(fs::read_to_string(path::Path::new(self.vert_file))?)?;
		let frag_contents = CString::new(fs::read_to_string(path::Path::new(self.frag_file))?)?;
		let geom_contents = match self.geom_file {
			None => CString::new("")?,
			Some(geom_file) => CString::new(fs::read_to_string(path::Path::new(geom_file))?)?
		};
		
		let id;

		unsafe {
			let vertex_shader = gl::CreateShader(gl::VERTEX_SHADER);
			gl::ShaderSource(vertex_shader, 1, &(vert_contents.as_ptr()) as *const *const gl::types::GLchar, null());
			gl::CompileShader(vertex_shader);
			Self::compile_errors(vertex_shader, "VERTEX");

			let fragment_shader = gl::CreateShader(gl::FRAGMENT_SHADER);
			gl::ShaderSource(fragment_shader, 1, &(frag_contents.as_ptr()) as *const *const gl::types::GLchar, null());
			gl::CompileShader(fragment_shader);
			Self::compile_errors(fragment_shader, "FRAGMENT");

			id = gl::CreateProgram();
			
			gl::AttachShader(id, vertex_shader);
			gl::AttachShader(id, fragment_shader);

			if self.geom_file.is_some() {
				let geometry_shader = gl::CreateShader(gl::GEOMETRY_SHADER);
				gl::ShaderSource(geometry_shader, 1, &(geom_contents.as_ptr()) as *const *const gl::types::GLchar, null());
				gl::CompileShader(geometry_shader);
				Self::compile_errors(geometry_shader, "GEOMETRY");
			
				gl::AttachShader(id, geometry_shader);
				
				gl::LinkProgram(id);
				
				gl::DeleteShader(geometry_shader);
			} else {
				gl::LinkProgram(id);
			}

        	let error = gl::GetError();
        	if error != gl::NO_ERROR { panic!("HERE 4 {}", error); }

			gl::DeleteShader(vertex_shader);
			gl::DeleteShader(fragment_shader);
		}

		self.id = Some(id);

		Ok(())
	}

	pub fn activate(&self) {
		unsafe {
			if let Some(id) = self.id {
				gl::UseProgram(id);
			}
		}
	}

	pub fn delete(&self) {
		unsafe {
			if let Some(id) = self.id {
				gl::DeleteProgram(id);
			}	
		}
	}

	pub fn get_id(&self) -> Option<u32> {
		self.id
	}

	// Checks if the different Shaders have compiled properly
	pub fn compile_errors(shader: u32, shader_type: &str) {
   		let mut success = 0;
		// if shader_type != "PROGRAM" {
		unsafe {
			// Check if shader compiled successfully
	 		gl::GetShaderiv(shader,
				if shader_type == "PROGRAM" { gl::LINK_STATUS } else { gl::COMPILE_STATUS },
				&mut success);

	 		if success == gl::FALSE as i32 {
	 		    // First, query the length of the info log
	 		    let mut len = 0;
	 		    gl::GetShaderiv(shader, gl::INFO_LOG_LENGTH, &mut len);
			 
	 		    // Create a buffer for the info log
	 		    let mut buffer: Vec<u8> = Vec::with_capacity(len as usize);
	 		    buffer.extend([0].iter().cycle().take(len as usize));
			 
	 		    // Get the info log
	 		    let mut written = 0;
	 		    gl::GetShaderInfoLog(
	 		        shader,
	 		        len,
	 		        &mut written,
	 		        buffer.as_mut_ptr() as *mut gl::types::GLchar
	 		    );
			 
	 		    // Convert the buffer to a string
	 		    let error = String::from_utf8_lossy(&buffer[..written as usize]).to_string();
				eprintln!("{}", error);
	 		}
		}
	}
}


// This struct just helps keep track of when the file has been changed, so we can re-load it automatically for testing.
// This won't be used in the release build.
pub struct ShaderHandler<'a> {
    modified_metadata: SystemTime,
    file_path: &'a str
}
impl<'a> ShaderHandler<'a> {
    pub fn try_new(file_path: &'a str) -> Result<Self, Box<dyn Error>> {
        let modified_metadata = fs::metadata(&path::Path::new(file_path))?.modified()?;
        Ok(ShaderHandler { file_path, modified_metadata })
    }

    // Checks if the ShaderHandler has been modified since it was last checked.
    pub fn was_modified(&mut self) -> Result<bool, Box<dyn Error>> {
        let new_modified_metadata = fs::metadata(&self.file_path)?.modified()?;
        if new_modified_metadata == self.modified_metadata {
            Ok(false)
        } else {
            self.modified_metadata = new_modified_metadata;
            Ok(true)
        }
    }
}

