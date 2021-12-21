use std::convert::TryInto;
use glfw::Context;
use voxel::engine::core::ENGINE;

const WIDTH: u32 = 800;
const HEIGHT: u32 = 600;

fn main() {
    let mut glfw = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();

    #[cfg(target_arch = "aarch64")] {
        glfw.window_hint(glfw::WindowHint::ContextVersion(3, 1));
        glfw.window_hint(glfw::WindowHint::ClientApi(glfw::ClientApiHint::OpenGlEs));
        glfw.window_hint(glfw::WindowHint::OpenGlForwardCompat(true));
    }

    let (mut window, _events) = glfw.create_window(WIDTH, HEIGHT, "", glfw::WindowMode::Windowed).expect("Failed to create GLFW window");
    
    //window.make_current();
    window.set_key_polling(true);
    window.set_cursor_pos_polling(true);
    window.set_mouse_button_polling(true);
    window.set_cursor_pos(WIDTH as f64/2.0, HEIGHT as f64/2.0);

    unsafe {
        gl::load_with(|s| window.get_proc_address(s) as *const _);

        if let Err(e) = ENGINE.gl_setup(WIDTH.try_into().unwrap(), HEIGHT.try_into().unwrap()) {
            panic!("{}", e);
        } else {
            println!("Initialized OpenGL");
        }
        
        if let Err(e) = ENGINE.initialize() {
            panic!("{}", e);
        } else {
            println!("Initialized Engine");
        }
    }

    let start_time = glfw.get_time() as f32;
    while !window.should_close() {
        let current_time = glfw.get_time() as f32;
        let elapsed_time = current_time - start_time;
        
        unsafe {
            ENGINE.render(elapsed_time);
        }

        window.swap_buffers();
    }
}
