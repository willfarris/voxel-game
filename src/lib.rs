mod entity;
mod item;
mod macros;
mod physics;
mod player;
mod terrain;
mod graphics;
pub mod engine;

#[cfg(feature = "android-lib")]
#[macro_use]
extern crate log;
#[cfg(feature = "android-lib")]
extern crate android_log;
#[cfg(feature = "android-lib")]
extern crate jni;
#[cfg(feature = "android-lib")]
mod java_interface;


pub use physics::vectormath::q_rsqrt;

pub struct EngineLock {
    #[allow(dead_code)]
    engine: std::sync::Mutex<engine::Engine>,
}

impl EngineLock {
    pub fn load_from_save(save_path: &str) -> Self {
        Self {
            engine: std::sync::Mutex::new(engine::Engine::load_from_save(save_path))
        }
    }
}

impl Default for EngineLock {
    fn default() -> Self {
        Self {
            engine: std::sync::Mutex::new(engine::Engine::default()),
        }
    }
}



