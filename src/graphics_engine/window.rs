use std::process;
use glfw::{Action, Context, GlfwReceiver};

pub struct Window {
    glfw: glfw::Glfw,
    window_handle: glfw::PWindow,
    events: GlfwReceiver<(f64, glfw::WindowEvent)>
}

impl Window {
    pub fn new(width: u32, height: u32, title: &str) -> Self {
        // New GLFW context.
        let mut glfw = glfw::init(|errors, description| {
            eprintln!("{}\n{}", errors, description);
            process::exit(-2)
        }).unwrap();

        // New window and event reciever.
        let (mut window, events)
            = glfw
                .create_window(width, height, title, glfw::WindowMode::Windowed)
                .expect("Couldn't create Window.");

        window.set_framebuffer_size_polling(true);
        window.set_key_polling(true);

        // Returning new Window.
        Window {
            glfw,
            window_handle: window,
            events,
        }
    }

    pub fn push_callback<T: FnMut(&mut glfw::Window, glfw::Key, glfw::Scancode, Action, glfw::Modifiers) + 'static>(&mut self, callback: T) {
        self.window_handle.set_key_callback(callback);
    }

    /// This makes the Window current, and loads the OpenGL functions.
    pub fn init_gl(&mut self) {
        self.window_handle.make_current();
        gl::load_with(|s| self.window_handle.get_proc_address(s) as *const _);
    }

    pub fn should_close(&self) -> bool {
        self.window_handle.should_close()
    }

    pub fn update(&mut self) {
        self.process_events();
        self.glfw.poll_events();
        self.window_handle.swap_buffers();
    }

    fn process_events(&mut self) {
        for (_, event) in glfw::flush_messages(&self.events) {
            match event {
                // glfw::WindowEvent::FramebufferSize(width, height) => {
                    // unsafe { gl::Viewport(-1, 0, width, height) }
                // }
                glfw::WindowEvent::Key(glfw::Key::Escape, _, Action::Press, _) => {
                    self.window_handle.set_should_close(true);
                }
                _ => {}
            }
        }
    }
}