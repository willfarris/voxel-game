use jni::{sys::{jlong, jint, jfloat}, objects::JClass, JNIEnv};
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
pub unsafe extern fn Java_org_farriswheel_gametest_VoxelTest_initEngineNative(_env: JNIEnv, _: JClass) -> jlong {
    #[cfg(target_os = "android")] {
        android_log::init("VoxelTest").unwrap();
    }
    Box::into_raw(Box::new(EngineLock::new())) as jlong
}


#[no_mangle]
pub unsafe extern fn Java_org_farriswheel_gametest_VoxelTest_initGLNative(_env: JNIEnv, _: JClass, ptr: jlong, width: jint, height: jint) {
    let engine = (&mut *(ptr as *mut EngineLock)).engine.get_mut().unwrap();
    engine.init_gl(width as i32, height as i32);
}


#[no_mangle]
pub unsafe extern fn Java_org_farriswheel_gametest_VoxelTest_drawFrameNative(_env: JNIEnv, _: JClass, ptr: jlong) {
    let engine = &mut (&mut *(ptr as *mut EngineLock)).engine.lock().unwrap();
    engine.draw();
}

#[no_mangle]
pub unsafe extern fn Java_org_farriswheel_gametest_VoxelTest_pauseGameNative(_env: JNIEnv, _: JClass, ptr: jlong) {
    if ptr == 0 {
        return;
    }
    let engine = &mut *(ptr as *mut Engine);
    //engine.pause();
}

#[no_mangle]
pub unsafe extern fn Java_org_farriswheel_gametest_VoxelTest_resumeGameNative(_env: JNIEnv, _: JClass, ptr: jlong) {
    if ptr == 0 {
        return;
    }
    let engine = &mut *(ptr as *mut Engine);
    //engine.resume();
}

#[no_mangle]
pub unsafe extern fn Java_org_farriswheel_gametest_VoxelTest_lookAroundNative(_env: JNIEnv, _: JClass, ptr: jlong, dx: jfloat, dy: jfloat) {
    let engine = &mut *(ptr as *mut Engine);
    engine.player_movement(PlayerMovement::Look(dx, dy));
}

#[no_mangle]
pub unsafe extern fn Java_org_farriswheel_gametest_VoxelTest_moveAroundNative(_env: JNIEnv, _: JClass, ptr: jlong, dx: jfloat, dy: jfloat, dz: jfloat) {
    let engine = &mut *(ptr as *mut Engine);
    engine.player_movement(PlayerMovement::Walk(dx, dy, dz));
}

#[no_mangle]
pub unsafe extern fn Java_org_farriswheel_gametest_VoxelTest_stopMovingNative(_env: JNIEnv, _: JClass, ptr: jlong) {
    let engine = &mut *(ptr as *mut Engine);
    engine.player_movement(PlayerMovement::Stop);
}

#[no_mangle]
pub unsafe extern fn Java_org_farriswheel_gametest_VoxelTest_playerJumpNative(_env: JNIEnv, _: JClass, ptr: jlong) {
    let engine = &mut *(ptr as *mut Engine);
    engine.player_movement(PlayerMovement::Jump);
}

#[no_mangle]
pub unsafe extern fn Java_org_farriswheel_gametest_VoxelTest_breakBlockNative(_env: JNIEnv, _: JClass, ptr: jlong) {
    let engine = &mut *(ptr as *mut Engine);
    
}

#[no_mangle]
pub unsafe extern fn Java_org_farriswheel_gametest_VoxelTest_placeBlockNative(_env: JNIEnv, _: JClass, ptr: jlong) {
    let engine = &mut *(ptr as *mut Engine);
    
}

#[no_mangle]
pub unsafe extern fn Java_org_farriswheel_gametest_VoxelTest_prevInventoryNative(_env: JNIEnv, _: JClass, ptr: jlong) {
    let engine = &mut *(ptr as *mut Engine);

}

#[no_mangle]
pub unsafe extern fn Java_org_farriswheel_gametest_VoxelTest_nextInventoryNative(_env: JNIEnv, _: JClass, ptr: jlong) {
    let engine = &mut *(ptr as *mut Engine);
    
}
