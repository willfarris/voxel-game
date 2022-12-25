use jni::{sys::{jlong, jint, jfloat, jboolean}, objects::JClass, JNIEnv};
use std::sync::Mutex;
use crate::{Engine, PlayerMovement};

struct EngineLock {
    engine: Mutex<Engine>
}

impl EngineLock {
    pub fn new() -> Self {
        Self {
            engine: Mutex::new(Engine::new()),
        }
    }
}


#[no_mangle]
pub unsafe extern fn Java_org_farriswheel_voxelgame_VoxelEngine_initEngineNative(_env: JNIEnv, _: JClass) -> jlong {
    #[cfg(target_os = "android")] {
        android_log::init("VoxelTest").unwrap();
    }
    Box::into_raw(Box::new(EngineLock::new())) as jlong
}


#[no_mangle]
pub unsafe extern fn Java_org_farriswheel_voxelgame_VoxelEngine_initGLNative(_env: JNIEnv, _: JClass, ptr: jlong, width: jint, height: jint) {
    let engine = (&mut *(ptr as *mut EngineLock)).engine.get_mut().unwrap();
    engine.init_gl(width as i32, height as i32);
}

#[no_mangle]
pub unsafe extern fn Java_org_farriswheel_voxelgame_VoxelEngine_updateNative(_env: JNIEnv, _: JClass, ptr: jlong, delta_time: jfloat) {
    let engine = &mut (&mut *(ptr as *mut EngineLock)).engine.lock().unwrap();
    engine.update(delta_time);
}

#[no_mangle]
pub unsafe extern fn Java_org_farriswheel_voxelgame_VoxelEngine_drawFrameNative(_env: JNIEnv, _: JClass, ptr: jlong) {
    let engine = &mut (&mut *(ptr as *mut EngineLock)).engine.lock().unwrap();
    engine.draw();
}

#[no_mangle]
pub unsafe extern fn Java_org_farriswheel_voxelgame_VoxelEngine_isPausedNative(_env: JNIEnv, _: JClass, ptr: jlong) -> jboolean {
    if ptr == 0 {
        return true as u8;
    }
    let engine = &mut *(ptr as *mut Engine);
    engine.is_paused() as u8
}

#[no_mangle]
pub unsafe extern fn Java_org_farriswheel_voxelgame_VoxelEngine_pauseGameNative(_env: JNIEnv, _: JClass, ptr: jlong) {
    if ptr == 0 {
        return;
    }
    let engine = &mut *(ptr as *mut Engine);
    engine.pause();
}

#[no_mangle]
pub unsafe extern fn Java_org_farriswheel_voxelgame_VoxelEngine_resumeGameNative(_env: JNIEnv, _: JClass, ptr: jlong) {
    if ptr == 0 {
        return;
    }
    let engine = &mut (&mut *(ptr as *mut EngineLock)).engine.lock().unwrap();
    engine.resume();
}

#[no_mangle]
pub unsafe extern fn Java_org_farriswheel_voxelgame_VoxelEngine_lookAroundNative(_env: JNIEnv, _: JClass, ptr: jlong, dx: jfloat, dy: jfloat) {
    let engine = &mut (&mut *(ptr as *mut EngineLock)).engine.lock().unwrap();
    debug!("Look {} {}", dx, dy);
    engine.player_movement(PlayerMovement::Look(dy, dx));
}

#[no_mangle]
pub unsafe extern fn Java_org_farriswheel_voxelgame_VoxelEngine_moveAroundNative(_env: JNIEnv, _: JClass, ptr: jlong, dx: jfloat, dy: jfloat, dz: jfloat) {
    let engine = &mut (&mut *(ptr as *mut EngineLock)).engine.lock().unwrap();
    engine.player_movement(PlayerMovement::Walk(dx, dy, dz));
}

#[no_mangle]
pub unsafe extern fn Java_org_farriswheel_voxelgame_VoxelEngine_stopMovingNative(_env: JNIEnv, _: JClass, ptr: jlong) {
    let engine = &mut (&mut *(ptr as *mut EngineLock)).engine.lock().unwrap();
    engine.player_movement(PlayerMovement::Stop);
}

#[no_mangle]
pub unsafe extern fn Java_org_farriswheel_voxelgame_VoxelEngine_playerJumpNative(_env: JNIEnv, _: JClass, ptr: jlong) {
    let engine = &mut (&mut *(ptr as *mut EngineLock)).engine.lock().unwrap();
    engine.player_movement(PlayerMovement::Jump);
}

#[no_mangle]
pub unsafe extern fn Java_org_farriswheel_voxelgame_VoxelEngine_breakBlockNative(_env: JNIEnv, _: JClass, ptr: jlong) {
    let engine = &mut (&mut *(ptr as *mut EngineLock)).engine.lock().unwrap();
    
}

#[no_mangle]
pub unsafe extern fn Java_org_farriswheel_voxelgame_VoxelEngine_placeBlockNative(_env: JNIEnv, _: JClass, ptr: jlong) {
    let engine = &mut (&mut *(ptr as *mut EngineLock)).engine.lock().unwrap();
    
}

#[no_mangle]
pub unsafe extern fn Java_org_farriswheel_voxelgame_VoxelEngine_prevInventoryNative(_env: JNIEnv, _: JClass, ptr: jlong) {
    let engine = &mut (&mut *(ptr as *mut EngineLock)).engine.lock().unwrap();

}

#[no_mangle]
pub unsafe extern fn Java_org_farriswheel_voxelgame_VoxelEngine_nextInventoryNative(_env: JNIEnv, _: JClass, ptr: jlong) {
    let engine = &mut (&mut *(ptr as *mut EngineLock)).engine.lock().unwrap();
    
}
