use crate::{EngineLock, engine::PlayerInput};
use jni::{
    objects::{JClass, JString},
    sys::{jboolean, jfloat, jint, jlong, jstring},
    JNIEnv,
};

#[no_mangle]
pub unsafe extern "C" fn Java_org_farriswheel_voxelgame_VoxelEngine_initEngineNative(
    env: JNIEnv,
    _: JClass,
    save_path: JString,
) -> jlong {
    #[cfg(features="android-lib")]
    {
        android_log::init("VoxelTest").unwrap();
    }
    let save_path_rs: String = env.get_string(save_path).unwrap().into();
    let save_file = std::path::Path::new(&save_path_rs);
    let engine = if save_file.exists() {
        debug!("Restoring from save file");
        Box::new(EngineLock::load_from_save(save_path_rs.as_str()))
    } else {
        debug!("Creating new world");
        Box::new(EngineLock::default())
    };

    Box::into_raw(engine) as jlong
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
pub unsafe extern "C" fn Java_org_farriswheel_voxelgame_VoxelEngine_startWorkerThreadsNative(
    _env: JNIEnv,
    _: JClass,
    ptr: jlong,
) {
    let mut engine = (&mut *(ptr as *mut EngineLock)).engine.lock().unwrap();
    engine.start_workers();
}

#[no_mangle]
pub unsafe extern "C" fn Java_org_farriswheel_voxelgame_VoxelEngine_saveGameNative(
    env: JNIEnv,
    _: JClass,
    ptr: jlong,
    save_path: JString
) {
    let save_path_rs: String = env.get_string(save_path).expect("unable to parse save path").into();
    let mut engine = (&mut *(ptr as *mut EngineLock)).engine.lock().unwrap();
    engine.save_to_file(save_path_rs.as_str());
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
    let engine = &mut (&mut *(ptr as *mut EngineLock)).engine.lock().unwrap();
    engine.player_input(PlayerInput::Interact(true, false));
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
