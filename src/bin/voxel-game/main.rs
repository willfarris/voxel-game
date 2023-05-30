use std::env;

use glfw::Context;
use voxel::q_rsqrt;
use voxel::engine::{Engine, PlayerInput};

const WIDTH: i32 = 1920;
const HEIGHT: i32 = 900;
const GAME_SAVE_PATH: &str = "blockcraft";

fn main() {
    let args: Vec<String> = env::args().collect();
    assert!(args.len() > 1);

    let mut voxel_game = match args[1].as_str() {
        "--restore" => Engine::load_from_save(GAME_SAVE_PATH),
        "--new" => Engine::default(),
        _ => Engine::default()
    };
    
    let mut glfw = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();

    glfw.window_hint(glfw::WindowHint::ContextVersion(3, 1));
    glfw.window_hint(glfw::WindowHint::ClientApi(glfw::ClientApiHint::OpenGlEs));
    glfw.window_hint(glfw::WindowHint::OpenGlForwardCompat(true));

    let (mut window, events) = glfw
        .create_window(
            WIDTH as u32,
            HEIGHT as u32,
            "Voxel Game",
            glfw::WindowMode::Windowed,
        )
        .expect("Failed to create GLFW window");

    gl::load_with(|s| window.get_proc_address(s) as *const _);
    window.make_current();
    window.set_key_polling(true);
    window.set_cursor_pos_polling(true);
    window.set_mouse_button_polling(true);
    window.set_cursor_pos(WIDTH as f64 / 2.0, HEIGHT as f64 / 2.0);
    window.set_cursor_mode(glfw::CursorMode::Hidden);

    println!("P - Pause\nO - Save");

    voxel_game.init_gl(WIDTH, HEIGHT);
    voxel_game.start_workers();

    const NUM_AVG_FRAMES: usize = 60;
    let mut averages = [0f32; NUM_AVG_FRAMES];
    let mut i = 0;

    let mut wasd_pressed = [false; 4];
    let mut jump = false;
    let start_time = glfw.get_time() as f32;
    let mut last_time = start_time;

    // Render + Input thread
    let mut should_close = false;
    while !should_close {
        should_close = window.should_close();

        let current_time = glfw.get_time() as f32;
        let delta_time = current_time - last_time;
        averages[i] = 1.0 / delta_time;
        i = if i < (NUM_AVG_FRAMES - 1) {
            i + 1
        } else {
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

        voxel_game.update();
        voxel_game.draw();
        window.swap_buffers();

        glfw.poll_events();
        for (_, event) in glfw::flush_messages(&events) {
            match event {
                glfw::WindowEvent::CursorPos(x, y) => {
                    let delta = (x - WIDTH as f64 / 2.0, y - HEIGHT as f64 / 2.0);
                    window.set_cursor_pos(WIDTH as f64 / 2.0, HEIGHT as f64 / 2.0);
                    voxel_game.player_input(PlayerInput::Look(
                        0.001 * delta.1 as f32,
                        0.001 * delta.0 as f32,
                    ));
                }
                glfw::WindowEvent::MouseButton(button, state, x) => {
                    match button {
                        glfw::MouseButton::Button1 => {
                            if state == glfw::Action::Press {
                                voxel_game.player_input(PlayerInput::Interact(false, true));
                            }
                        }
                        glfw::MouseButton::Button2 => {
                            if state == glfw::Action::Press {
                                voxel_game.player_input(PlayerInput::Interact(true, false));
                            }
                        }
                        glfw::MouseButton::Button3 => {
                            if state == glfw::Action::Release {
                                //voxel_game.should_interact = true;
                            }
                        }
                        _ => println!("{:?} {:?} {:?}", button, state, x),
                    }
                }
                glfw::WindowEvent::Key(k, _, state, _) => {
                    let pressed_or_held = if state == glfw::Action::Release {
                        false
                    } else {
                        true
                    };
                    match k {
                        glfw::Key::Escape => window.set_should_close(true),
                        glfw::Key::W => wasd_pressed[0] = pressed_or_held,
                        glfw::Key::A => wasd_pressed[1] = pressed_or_held,
                        glfw::Key::S => wasd_pressed[2] = pressed_or_held,
                        glfw::Key::D => wasd_pressed[3] = pressed_or_held,
                        glfw::Key::Space => {
                            if state == glfw::Action::Press {
                                jump = true;
                            } else if state == glfw::Action::Release {
                                jump = false;
                            }
                        }

                        glfw::Key::O => {
                            if state == glfw::Action::Release {
                                voxel_game.pause();
                                voxel_game.save_to_file(GAME_SAVE_PATH);
                                voxel_game.resume();
                            }
                        }

                        glfw::Key::P => {
                            if state == glfw::Action::Release {
                                if voxel_game.is_paused() {
                                    voxel_game.resume();
                                } else {
                                    voxel_game.pause();
                                }
                            }
                        }

                        glfw::Key::Num1 => {
                            if state == glfw::Action::Press {
                                voxel_game.player_input(PlayerInput::Inventory(0));
                            }
                        }
                        glfw::Key::Num2 => {
                            if state == glfw::Action::Press {
                                voxel_game.player_input(PlayerInput::Inventory(1));
                            }
                        }
                        glfw::Key::Num3 => {
                            if state == glfw::Action::Press {
                                voxel_game.player_input(PlayerInput::Inventory(2));
                            }
                        }
                        glfw::Key::Num4 => {
                            if state == glfw::Action::Press {
                                voxel_game.player_input(PlayerInput::Inventory(3));
                            }
                        }
                        glfw::Key::Num5 => {
                            if state == glfw::Action::Press {
                                voxel_game.player_input(PlayerInput::Inventory(4));
                            }
                        }
                        glfw::Key::Num6 => {
                            if state == glfw::Action::Press {
                                voxel_game.player_input(PlayerInput::Inventory(5));
                            }
                        }
                        glfw::Key::Num7 => {
                            if state == glfw::Action::Press {
                                voxel_game.player_input(PlayerInput::Inventory(6));
                            }
                        }
                        glfw::Key::Num8 => {
                            if state == glfw::Action::Press {
                                voxel_game.player_input(PlayerInput::Inventory(7));
                            }
                        }
                        glfw::Key::Num9 => {
                            if state == glfw::Action::Press {
                                voxel_game.player_input(PlayerInput::Inventory(8));
                            }
                        }

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
        move_direction *=
            q_rsqrt(move_direction.x * move_direction.x + move_direction.z * move_direction.z);
        if !wasd_pressed[0] && !wasd_pressed[1] && !wasd_pressed[2] && !wasd_pressed[3] {
            voxel_game.player_input(PlayerInput::Stop);
        } else {
            voxel_game.player_input(PlayerInput::Walk(
                move_direction.x,
                move_direction.y,
                move_direction.z,
            ));
        }

        if jump {
            voxel_game.player_input(PlayerInput::Jump);
        }
    }
}
