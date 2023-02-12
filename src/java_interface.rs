use crate::{EngineLock, PlayerInput};
use jni::{
    objects::JClass,
    sys::{jboolean, jfloat, jint, jlong},
    JNIEnv,
};

#[no_mangle]
pub unsafe extern "C" fn Java_org_farriswheel_voxelgame_VoxelEngine_initEngineNative(
    _env: JNIEnv,
    _: JClass,
) -> jlong {
    #[cfg(target_os = "android")]
    {
        android_log::init("VoxelTest").unwrap();
    }
    Box::into_raw(Box::new(EngineLock::default())) as jlong
}

#[no_mangle]
pub unsafe extern "C" fn Java_org_farriswheel_voxelgame_VoxelEngine_initGLNative(
    _env: JNIEnv,
    _: JClass,
    ptr: jlong,
    width: jint,
    height: jint,
) {
    let mut engine = (&mut *(ptr as *mut EngineLock)).engine.lock().unwrap();
    engine.init_gl(width as i32, height as i32);
}

#[no_mangle]
pub unsafe extern "C" fn Java_org_farriswheel_voxelgame_VoxelEngine_startTerrainThreadNative(
    _env: JNIEnv,
    _: JClass,
    ptr: jlong,
) {
    let mut engine = (&mut *(ptr as *mut EngineLock)).engine.lock().unwrap();
    engine.start_terrain_thread();
}

#[no_mangle]
pub unsafe extern "C" fn Java_org_farriswheel_voxelgame_VoxelEngine_updateNative(
    _env: JNIEnv,
    _: JClass,
    ptr: jlong,
) {
    let engine = &mut (&mut *(ptr as *mut EngineLock)).engine.lock().unwrap();
    engine.update();
}

#[no_mangle]
pub unsafe extern "C" fn Java_org_farriswheel_voxelgame_VoxelEngine_drawFrameNative(
    _env: JNIEnv,
    _: JClass,
    ptr: jlong,
) {
    let engine = &mut (&mut *(ptr as *mut EngineLock)).engine.lock().unwrap();
    engine.draw();
}

#[no_mangle]
pub unsafe extern "C" fn Java_org_farriswheel_voxelgame_VoxelEngine_resetGlResourcesNative(
    _env: JNIEnv,
    _: JClass,
    ptr: jlong,
) {
    let engine = &mut (&mut *(ptr as *mut EngineLock)).engine.lock().unwrap();
    engine.reset_gl_resources();
}

#[no_mangle]
pub unsafe extern "C" fn Java_org_farriswheel_voxelgame_VoxelEngine_isPausedNative(
    _env: JNIEnv,
    _: JClass,
    ptr: jlong,
) -> jboolean {
    if ptr == 0 {
        return true as u8;
    }
    let engine = &mut (&mut *(ptr as *mut EngineLock)).engine.lock().unwrap();
    engine.is_paused() as u8
}

#[no_mangle]
pub unsafe extern "C" fn Java_org_farriswheel_voxelgame_VoxelEngine_pauseGameNative(
    _env: JNIEnv,
    _: JClass,
    ptr: jlong,
) {
    if ptr == 0 {
        return;
    }
    let engine = &mut (&mut *(ptr as *mut EngineLock)).engine.lock().unwrap();
    engine.pause();
}

#[no_mangle]
pub unsafe extern "C" fn Java_org_farriswheel_voxelgame_VoxelEngine_resumeGameNative(
    _env: JNIEnv,
    _: JClass,
    ptr: jlong,
) {
    if ptr == 0 {
        return;
    }
    let engine = &mut (&mut *(ptr as *mut EngineLock)).engine.lock().unwrap();
    engine.resume();
}

#[no_mangle]
pub unsafe extern "C" fn Java_org_farriswheel_voxelgame_VoxelEngine_lookAroundNative(
    _env: JNIEnv,
    _: JClass,
    ptr: jlong,
    dx: jfloat,
    dy: jfloat,
) {
    let engine = &mut (&mut *(ptr as *mut EngineLock)).engine.lock().unwrap();
    engine.player_input(PlayerInput::Look(dy, dx));
}

#[no_mangle]
pub unsafe extern "C" fn Java_org_farriswheel_voxelgame_VoxelEngine_moveAroundNative(
    _env: JNIEnv,
    _: JClass,
    ptr: jlong,
    dx: jfloat,
    dy: jfloat,
    dz: jfloat,
) {
    let engine = &mut (&mut *(ptr as *mut EngineLock)).engine.lock().unwrap();
    engine.player_input(PlayerInput::Walk(dx, dy, dz));
}

#[no_mangle]
pub unsafe extern "C" fn Java_org_farriswheel_voxelgame_VoxelEngine_stopMovingNative(
    _env: JNIEnv,
    _: JClass,
    ptr: jlong,
) {
    let engine = &mut (&mut *(ptr as *mut EngineLock)).engine.lock().unwrap();
    engine.player_input(PlayerInput::Stop);
}

#[no_mangle]
pub unsafe extern "C" fn Java_org_farriswheel_voxelgame_VoxelEngine_playerJumpNative(
    _env: JNIEnv,
    _: JClass,
    ptr: jlong,
) {
    let engine = &mut (&mut *(ptr as *mut EngineLock)).engine.lock().unwrap();
    engine.player_input(PlayerInput::Jump);
}

#[no_mangle]
pub unsafe extern "C" fn Java_org_farriswheel_voxelgame_VoxelEngine_breakBlockNative(
    _env: JNIEnv,
    _: JClass,
    ptr: jlong,
) {
    let engine = &mut (&mut *(ptr as *mut EngineLock)).engine.lock().unwrap();
    engine.player_input(PlayerInput::Interact(false, true));
}

#[no_mangle]
pub unsafe extern "C" fn Java_org_farriswheel_voxelgame_VoxelEngine_placeBlockNative(
    _env: JNIEnv,
    _: JClass,
    ptr: jlong,
) {
    let _engine = &mut (&mut *(ptr as *mut EngineLock)).engine.lock().unwrap();
}

#[no_mangle]
pub unsafe extern "C" fn Java_org_farriswheel_voxelgame_VoxelEngine_prevInventoryNative(
    _env: JNIEnv,
    _: JClass,
    ptr: jlong,
) {
    let _engine = &mut (&mut *(ptr as *mut EngineLock)).engine.lock().unwrap();
}

#[no_mangle]
pub unsafe extern "C" fn Java_org_farriswheel_voxelgame_VoxelEngine_nextInventoryNative(
    _env: JNIEnv,
    _: JClass,
    ptr: jlong,
) {
    let _engine = &mut (&mut *(ptr as *mut EngineLock)).engine.lock().unwrap();
}

#[no_mangle]
pub unsafe extern "C" fn Java_org_farriswheel_voxelgame_VoxelEngine_invSqrt(
    _env: JNIEnv,
    _: JClass,
    num: jfloat,
) -> jfloat {
    crate::physics::vectormath::q_rsqrt(num)
}
