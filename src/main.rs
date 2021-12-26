use std::convert::TryInto;
use glfw::Context;
use voxel::engine::core::ENGINE;
use voxel::physics::vectormath::q_rsqrt;

const WIDTH: u32 = 1600;
const HEIGHT: u32 = 900;

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
    window.set_cursor_mode(glfw::CursorMode::Hidden);


    let seed = 4;//0xFF221234;
    let world_radius = 4;

    unsafe {
        gl::load_with(|s| window.get_proc_address(s) as *const _);

        if let Err(e) = ENGINE.gl_setup(WIDTH.try_into().unwrap(), HEIGHT.try_into().unwrap()) {
            panic!("{}", e);
        } else {
            println!("Initialized OpenGL");
        }
        
        if let Err(e) = ENGINE.initialize(seed, world_radius) {
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
                    match button {
                        glfw::MouseButton::Button1 => {
                            if state == glfw::Action::Press {unsafe {ENGINE.should_break_block = true}}
                        },
                        glfw::MouseButton::Button2 => {
                            if state == glfw::Action::Release {
                                unsafe {
                                    let player = ENGINE.player.as_mut().unwrap();
                                    let world = ENGINE.world.as_mut().unwrap();
                                    if let Some(block_id) = player.inventory.consume_currently_selected() {
                                        if let Some((intersect_position, world_index)) = voxel::physics::vectormath::dda(&world, &player.camera.position, &player.camera.forward, 6.0) {
                                            let place_index = cgmath::Vector3 {
                                                x: if intersect_position.x == world_index.x as f32 {
                                                    world_index.x - 1
                                                } else if intersect_position.x-1.0 == world_index.x as f32 {
                                                    world_index.x + 1
                                                } else {
                                                    world_index.x
                                                },
                                                y: if intersect_position.y== world_index.y as f32 {
                                                    world_index.y - 1
                                                } else if intersect_position.y-1.0 == world_index.y as f32 {
                                                    world_index.y + 1
                                                } else {
                                                    world_index.y
                                                },
                                                z: if intersect_position.z == world_index.z as f32 {
                                                    world_index.z - 1
                                                } else if intersect_position.z-1.0 == world_index.z as f32 {
                                                    world_index.z + 1
                                                } else {
                                                    world_index.z
                                                },
                                            };
                                            world.place_at_global_pos(place_index, block_id);
                                        }
                                    }
                                }
                            }
                        }
                        glfw::MouseButton::Button3 => {
                            if state == glfw::Action::Release {unsafe {ENGINE.should_interact = true}}
                        }
                        _ => println!("{:?} {:?} {:?}", button, state, x),
                    }
                },
                glfw::WindowEvent::Key(k, _, state, _) => {
                    let pressed_or_held = if state == glfw::Action::Release {false} else {true};
                    match k {
                        glfw::Key::Escape => window.set_should_close(true),
                        glfw::Key::W => wasd_pressed[0] = pressed_or_held,
                        glfw::Key::A => wasd_pressed[1] = pressed_or_held,
                        glfw::Key::S => wasd_pressed[2] = pressed_or_held,
                        glfw::Key::D => wasd_pressed[3] = pressed_or_held,
                        glfw::Key::Space => if state == glfw::Action::Press {unsafe {ENGINE.player.as_mut().unwrap().jump()}},

                        glfw::Key::Num1 => unsafe {if state == glfw::Action::Press {ENGINE.player.as_mut().unwrap().inventory.selected = 0;}},
                        glfw::Key::Num2 => unsafe {if state == glfw::Action::Press {ENGINE.player.as_mut().unwrap().inventory.selected = 1;}},
                        glfw::Key::Num3 => unsafe {if state == glfw::Action::Press {ENGINE.player.as_mut().unwrap().inventory.selected = 2;}},
                        glfw::Key::Num4 => unsafe {if state == glfw::Action::Press {ENGINE.player.as_mut().unwrap().inventory.selected = 3;}},
                        glfw::Key::Num5 => unsafe {if state == glfw::Action::Press {ENGINE.player.as_mut().unwrap().inventory.selected = 4;}},
                        glfw::Key::Num6 => unsafe {if state == glfw::Action::Press {ENGINE.player.as_mut().unwrap().inventory.selected = 5;}},
                        glfw::Key::Num7 => unsafe {if state == glfw::Action::Press {ENGINE.player.as_mut().unwrap().inventory.selected = 6;}},
                        glfw::Key::Num8 => unsafe {if state == glfw::Action::Press {ENGINE.player.as_mut().unwrap().inventory.selected = 7;}},
                        glfw::Key::Num9 => unsafe {if state == glfw::Action::Press {ENGINE.player.as_mut().unwrap().inventory.selected = 8;}},

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
