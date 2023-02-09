# Voxel Game
![Screenshot](https://user-images.githubusercontent.com/9190155/187536563-14793048-66b4-4bc6-b040-4403f08ec179.png)

A simple Minecraft-like game written in Rust and OpenGL. Runs on desktop Linux & Android.

# Building and Usage

This project was written in Rust and requires the Rust toolchain (available at [rustup.rs](https://rustup.rs/)). The desktop version requires [GLFW](https://www.glfw.org/). After setting up the required dependencies, the desktop app can be built and run as follows:

```
git clone git@github.com:WillFarris/voxel-game.git
cd voxel-game
cargo run --release --features=desktop
```

## Android
See [WillFarris/VoxelGameAndroid](https://github.com/WillFarris/VoxelGameAndroid) for the Android Studio project. The code can be compiled to a .so file for use on Android as seen below. Requires Python and wget.
```
git clone git@github.com:WillFarris/voxel-game.git
git clone git@github.com:WillFarris/VoxelGameAndroid $HOME/AndroidStudioProjects/
cd voxel-game
./link_android.py $HOME/AndroidStudioProjects/VoxelGameAndroid
```

Then build `VoxelGameAndroid` in Android Studio.

# Content

So far there isn't much to do in the game besides walk around. Features so far include:
* Simple 3D engine
* Infinite terrain

The following are roughly planned for the future:
* Deferred rendering pipeline + postprocessing effects
* Proper inventory system + the ability to break and collect blocks (WIP)
* Simple AI/mobs
* Persistent storage for worlds
* Better shaders/rendering pipeline
