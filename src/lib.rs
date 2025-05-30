use std::sync::mpsc;

use text_box::TextWidget;
// use ui::text;

pub mod graphics_engine;
pub mod text_box;
pub mod ui;
pub mod vao;
pub mod ssbo;
pub mod vbo;
pub mod ebo;
pub mod shader;

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

    // let mut bg_vert_handler = shader::ShaderHandler::try_new("src/shaders/background_rounded.vert")?;
    // let mut bg_frag_handler = shader::ShaderHandler::try_new("src/shaders/background_rounded.frag")?;
    // let mut bg_geom_handler = shader::ShaderHandler::try_new("src/shaders/background_rounded.geom")?;

    // let mut ui_elements = ui::UI::new();
    // ui_elements.init_shader("src/shaders/ui.vert", "src/shaders/ui.frag")?;
    // ui_elements.push_textblock(
    //     ui::text::TextBlock::from(String::from("I am the first one."),
    //     glm::Vec4::new(1.0, 1.0, 1.0, 1.0)),
    //     ui::TextBlockPosition::new(glm::Vec4::new(-0.8, 0.8, -1.0, 1.0), [32, 32], [1000, 1000], [0, 0]));
    // ui_elements.push_textblock(
    //     ui::text::TextBlock::from(String::from("Duckbill Studio :D"),
    //     glm::Vec4::new(1.0, 1.0, 1.0, 1.0)),
    //     ui::TextBlockPosition::new(glm::Vec4::new(-0.8, -0.5, -1.0, 1.0), [32, 32], [1000, 1000], [0, 0]));
    // ui_elements.reset_vao();
    // ui_elements.gen_ssbo();


    let mut text_widget = TextWidget::new();
    text_widget.init_shader("src/shaders/ui.vert", "src/shaders/ui.frag")?;
    text_widget.add_text_box(&String::from("Hello, World!"), glm::Vec2::new(-0.9, 0.7), glm::Vec2::new(-0.1, 0.3));
    text_widget.add_text_box(&String::from("Bottom :D"), glm::Vec2::new(-0.5, 0.0), glm::Vec2::new(0.5, -0.5));
    text_widget.init_vaos();
    text_widget.gen_ssbo();

    // let mut q = text_box::quad::Quad::new();
    // q.init_shader("src/shaders/background_rounded.vert", "src/shaders/background_rounded.frag", "src/shaders/background_rounded.geom")?;
    // q.reset_vao();

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

        if frag_handler.was_modified()? || vert_handler.was_modified()? { text_widget.reload_shader()?; }
        // if bg_frag_handler.was_modified()? || bg_vert_handler.was_modified()? || bg_geom_handler.was_modified()? { q.reload_shader()?; }

        // Updating textblock.
        match rx.try_recv() {
            // Ok(c) => ui_elements.append_textblock(0, c),
            Ok((19, _)) => text_widget.set_active_text_box(0),
            Ok((30, _)) => text_widget.set_active_text_box(1),
            Ok((25, _)) => {
                text_widget.debug_print();
            }
            Ok((23, _)) => {
                text_widget.replace_active_with_slice("REPLACED!");
                text_widget.debug_print();
            }
            Ok(_) => {
                text_widget.add_slice_to_active(":D");
                text_widget.debug_print();
            },
            Err(_) => {}
        }

        // ui_elements.draw();
        // q.draw();
        text_widget.draw();
        window.update();
    }

    Ok(())
}
