use std::convert::TryInto;

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

    let (mut window, events) = glfw.create_window(WIDTH, HEIGHT, "", glfw::WindowMode::Windowed).expect("Failed to create GLFW window");
    
    //window.make_current();
    window.set_key_polling(true);
    window.set_cursor_pos_polling(true);
    window.set_mouse_button_polling(true);
    window.set_cursor_pos(WIDTH as f64/2.0, HEIGHT as f64/2.0);

    unsafe {
        if let Err(error) = ENGINE.gl_setup(WIDTH.try_into().unwrap(), HEIGHT.try_into().unwrap()) {
            panic!("Could not load OpenGL!");
        }
        
        if let Err(error) = ENGINE.start_engine() {
            panic!("Could not start engine!");
        }
    }

    let start_time = glfw.get_time() as f32;
    let mut previous_time = start_time;
    while !window.should_close() {
        let current_time = glfw.get_time() as f32;
        let delta_time = (current_time - previous_time) as f32;
        let elapsed_time = current_time - start_time;
        
        unsafe {
            ENGINE.render(elapsed_time);
        }
    }
}
