use std::sync::mpsc;

pub mod graphics_engine;
pub mod ui;
pub mod vao;
pub mod vbo;
pub mod ebo;
pub mod shader;
pub mod quad;

pub fn run() -> Result<(), Box<dyn std::error::Error>> {
    let mut window = graphics_engine::window::Window::new(1080, 720, "Plz");

    let (tx, rx) =  mpsc::channel::<(usize, glfw::Modifiers)>();
    

    // Explains itself.
    #[allow(unused)]
    let yee_old_scancode_resolver = |_: &mut glfw::Window, key: glfw::Key, scancode: glfw::Scancode, action: glfw::Action, _: glfw::Modifiers| {
        if action == glfw::Action::Press && key != glfw::Key::LeftShift {
            print!("{}\t", scancode);
            if let Some(key_name) = key.get_name() {
                print!("\'{}\'", key_name);
            }
            println!();
        }
    };

    let scancode_to_char = move |_: &mut glfw::Window, _: glfw::Key, scancode: glfw::Scancode, action: glfw::Action, modifiers: glfw::Modifiers| {
        if action == glfw::Action::Press || action == glfw::Action::Repeat {
            match tx.send((scancode as usize, modifiers)) {
                Ok(_) => {}
                Err(_) => { return; }
            }
        }
    };

    window.push_callback(scancode_to_char);
    // window.push_callback(yee_old_scancode_resolver);

    window.init_gl();

    let mut vert_handler = shader::ShaderHandler::try_new("src/shaders/ui.vert")?;
    let mut frag_handler = shader::ShaderHandler::try_new("src/shaders/ui.frag")?;

    let mut ui_elements = ui::UI::new();
    ui_elements.init_shader("src/shaders/ui.vert", "src/shaders/ui.frag")?;
    ui_elements.push_textblock(
        ui::text::TextBlock::from(String::from(""),
        glm::Vec4::new(1.0, 1.0, 1.0, 1.0)),
        ui::TextBlockPosition::new(glm::Vec3::new(0.1, 0.1, -0.1), glm::UVec2::new(10, 10), glm::Vec2::new(0.1, 0.1)));
    ui_elements.reset_vao();

	// Enables the Depth Buffer and does backface culling.
    unsafe {
	    // gl::Enable(gl::DEPTH_TEST);
       	gl::Enable(gl::BLEND);
       	gl::BlendFunc(gl::SRC_ALPHA, gl::ONE_MINUS_SRC_ALPHA);
       	// Enabling backface culling :D
       	gl::Enable(gl::CULL_FACE);
       	gl::CullFace(gl::BACK);
       	gl::FrontFace(gl::CCW);
    }

    while !window.should_close() {
        unsafe {
            // gl::ClearColor(0.3, 0.5, 0.3, 1.0);
            gl::ClearColor(0.22, 0.2, 0.2, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);
        }

        if frag_handler.was_modified()? || vert_handler.was_modified()? {
            ui_elements.reload_shader()?;
        }

        // Updating textblock.
        match rx.try_recv() {
            Ok(c) => ui_elements.append_textblock(0, c),
            Err(_) => {}
        }

        ui_elements.draw();
        window.update();
    }

    Ok(())
}
