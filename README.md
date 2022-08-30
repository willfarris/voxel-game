# Voxel Game

A simple Minecraft-like game written in Rust and OpenGL. This is the desktop version, an Android project is available at [WillFarris/VoxelGameAndroid](https://github.com/WillFarris/VoxelGameAndroid).

The core code for the game is found at [WillFarris/libvoxel](https://github.com/WillFarris/libvoxel). This is so the same engine code can be reused for the desktop and Android versions of the game.

# Building and Usage

This project was written in Rust and requires the Rust toolchain, available at [rustup.rs](https://rustup.rs/). The game requires the GLFW library, which can be installed with e.g. `sudo pacman -S glfw` or your OS/distro's equivalent.

After setting up the required dependencies, the code can be built and run as follows:

```
git clone git@github.com:WillFarris/voxel-game.git
cd voxel-game
git submodule update --init
cargo run
```