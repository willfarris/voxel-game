

use glfw::Context;
use voxel::engine::engine::{PlayerInteraction, Engine};
use voxel::physics::vectormath::q_rsqrt;

const WIDTH: i32 = 1600;
const HEIGHT: i32 = 900;

fn main() {
    let mut glfw = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();

    #[cfg(target_arch = "aarch64")] {
        glfw.window_hint(glfw::WindowHint::ContextVersion(3, 1));
        glfw.window_hint(glfw::WindowHint::ClientApi(glfw::ClientApiHint::OpenGlEs));
        glfw.window_hint(glfw::WindowHint::OpenGlForwardCompat(true));
    }

    let (mut window, events) = glfw.create_window(WIDTH as u32, HEIGHT as u32, "VoxelGame", glfw::WindowMode::Windowed).expect("Failed to create GLFW window");

    gl::load_with(|s| window.get_proc_address(s) as *const _);
    window.make_current();
    window.set_key_polling(true);
    window.set_cursor_pos_polling(true);
    window.set_mouse_button_polling(true);
    window.set_cursor_pos(WIDTH as f64/2.0, HEIGHT as f64/2.0);
    window.set_cursor_mode(glfw::CursorMode::Hidden);
    
    
    let seed = 4;//0xFF221234;
    let world_radius = 4;

    let mut engine = Engine::new(WIDTH, HEIGHT, seed, world_radius);

    const NUM_AVG_FRAMES: usize = 60;
    let mut averages = [0f32; NUM_AVG_FRAMES];
    let mut i = 0;

    let mut wasd_pressed = [false; 4];
    let start_time = glfw.get_time() as f32;
    let mut last_time = start_time;

    // Render + Input thread
    let mut should_close = false;
    while !should_close {
        should_close = window.should_close();

        let current_time = glfw.get_time() as f32;
        let delta_time = current_time - last_time;
        averages[i] = 1.0 / delta_time;
        i = if i < (NUM_AVG_FRAMES - 1) { i + 1 } else {
            let mut average_fps = 0.0;
            for n in 0..NUM_AVG_FRAMES {
                average_fps += averages[n];
            }
            average_fps /= NUM_AVG_FRAMES as f32;
            window.should_close();
            window.set_title(format!("Voxel Game - {} FPS", average_fps).as_str());
            0
        };
        last_time = current_time;

        engine.update(delta_time);
        engine.render();
        window.swap_buffers();

        glfw.poll_events();
        for (_, event) in glfw::flush_messages(&events) {
            match event {
                glfw::WindowEvent::CursorPos(x, y) => {
                    let delta = (x-WIDTH as f64/2.0, y-HEIGHT as f64/2.0);

                    window.set_cursor_pos(WIDTH as f64/2.0, HEIGHT as f64/2.0);
                    
                    engine.player.camera.rotate_on_x_axis(0.001 * delta.1 as f32);
                    engine.player.camera.rotate_on_y_axis(0.001 * delta.0 as f32);
                    
                },
                glfw::WindowEvent::MouseButton(button, state, x) => {
                    match button {
                        glfw::MouseButton::Button1 => {
                            if state == glfw::Action::Press {
                                engine.player_interaction = PlayerInteraction::RightHand;
                            }
                        },
                        glfw::MouseButton::Button2 => {
                            if state == glfw::Action::Release {
                                if let Some(block_id) = &engine.player.inventory.consume_currently_selected() {
                                    if let Some((intersect_position, world_index)) = voxel::physics::vectormath::dda(
                                        &engine.terrain, 
                                        &engine.player.camera.position, 
                                        &engine.player.camera.forward,
                                        6.0) {
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
                                        engine.terrain.place_at_global_pos(place_index, *block_id);
                                    }
                                }
                                
                            }
                        }
                        glfw::MouseButton::Button3 => {
                            if state == glfw::Action::Release {
                                //engine.should_interact = true;
                            }
                        }
                        _ => println!("{:?} {:?} {:?}", button, state, x),
                    }
                },
                glfw::WindowEvent::Key(k, _, state, _) => {
                    let pressed_or_held = if state == glfw::Action::Release {false} else {true};
                    match k {
                        glfw::Key::Escape => {
                            window.set_should_close(true)
                        },
                        glfw::Key::W => wasd_pressed[0] = pressed_or_held,
                        glfw::Key::A => wasd_pressed[1] = pressed_or_held,
                        glfw::Key::S => wasd_pressed[2] = pressed_or_held,
                        glfw::Key::D => wasd_pressed[3] = pressed_or_held,
                        glfw::Key::Space => if state == glfw::Action::Press {
                            engine.player.jump()
                        },

                        glfw::Key::P => if state == glfw::Action::Release {
                            engine.play_state = if engine.play_state == voxel::engine::engine::PlayState::Running {
                                voxel::engine::engine::PlayState::Paused
                            } else {
                                voxel::engine::engine::PlayState::Running
                            }
                        },

                        glfw::Key::Num1 => if state == glfw::Action::Press {engine.player.inventory.selected = 0;},
                        glfw::Key::Num2 => if state == glfw::Action::Press {engine.player.inventory.selected = 1;},
                        glfw::Key::Num3 => if state == glfw::Action::Press {engine.player.inventory.selected = 2;},
                        glfw::Key::Num4 => if state == glfw::Action::Press {engine.player.inventory.selected = 3;},
                        glfw::Key::Num5 => if state == glfw::Action::Press {engine.player.inventory.selected = 4;},
                        glfw::Key::Num6 => if state == glfw::Action::Press {engine.player.inventory.selected = 5;},
                        glfw::Key::Num7 => if state == glfw::Action::Press {engine.player.inventory.selected = 6;},
                        glfw::Key::Num8 => if state == glfw::Action::Press {engine.player.inventory.selected = 7;},
                        glfw::Key::Num9 => if state == glfw::Action::Press {engine.player.inventory.selected = 8;},

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
        if !wasd_pressed[0] && !wasd_pressed[1] && !wasd_pressed[2] && !wasd_pressed[3] {
            //todo!("Stop player movement")
            engine.player.stop_move();
        } else {
            //todo!("Update player velocity")
            engine.player.move_direction(move_direction);
        }
        
    }
}
