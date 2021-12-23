use std::convert::TryInto;
use glfw::Context;
use voxel::engine::core::ENGINE;
use voxel::physics::vectormath::q_rsqrt;

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
    let mut wasd_pressed = [false; 4];
    let start_time = glfw.get_time() as f32;
    while !window.should_close() {
        let current_time = glfw.get_time() as f32;
        let elapsed_time = current_time - start_time;
        
        unsafe {
            ENGINE.render(elapsed_time);
        }

        window.swap_buffers();

        glfw.poll_events();
        for (_, event) in glfw::flush_messages(&events) {
            match event {
                glfw::WindowEvent::CursorPos(x, y) => {
                    let delta = (x-WIDTH as f64/2.0, y-HEIGHT as f64/2.0);
                    window.set_cursor_pos(WIDTH as f64/2.0, HEIGHT as f64/2.0);
                    unsafe {
                        ENGINE.player.as_mut().unwrap().camera.rotate_on_x_axis(0.001 * delta.1 as f32);
                        ENGINE.player.as_mut().unwrap().camera.rotate_on_y_axis(0.001 * delta.0 as f32);
                    }
                },
                glfw::WindowEvent::MouseButton(button, state, x) => {
                    println!("{:?} {:?} {:?}", button, state, x);
                    if state == glfw::Action::Press {unsafe {ENGINE.should_break_block = true}}
                },
                glfw::WindowEvent::Key(k, _, state, _) => {
                    let pressed = if state == glfw::Action::Release {false} else {true};
                    let released = if state == glfw::Action::Release {true} else {false};
                    match k {
                        glfw::Key::Escape => window.set_should_close(true),
                        glfw::Key::W => wasd_pressed[0] = pressed,
                        glfw::Key::A => wasd_pressed[1] = pressed,
                        glfw::Key::S => wasd_pressed[2] = pressed,
                        glfw::Key::D => wasd_pressed[3] = pressed,
                        glfw::Key::Space => if state == glfw::Action::Press {unsafe {ENGINE.player.as_mut().unwrap().jump()}},
                        _ => {
                            println!("{:?}", k);
                        }
                    }
                }
                _ => println!("{:?}", event),
            }
        }

        let mut move_direction = cgmath::Vector3 {
            x: wasd_pressed[3] as i32 as f32 - wasd_pressed[1] as i32 as f32,
            y: 0.0,
            z: wasd_pressed[0] as i32 as f32 - wasd_pressed[2] as i32 as f32,
        };
        move_direction *= q_rsqrt(move_direction.x * move_direction.x + move_direction.z * move_direction.z);
        
        unsafe {
            
            if !wasd_pressed[0] && !wasd_pressed[1] && !wasd_pressed[2] && !wasd_pressed[3] {
                ENGINE.player.as_mut().unwrap().stop_move();
            } else {
                ENGINE.player.as_mut().unwrap().move_direction(move_direction);
            }
        }
    }
}
