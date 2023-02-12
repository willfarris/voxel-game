#!/usr/bin/env python3

from argparse import ArgumentParser
import os

LIBNAME = "voxel"

def main():
    parser = ArgumentParser(
        prog = "link_android",
        description = "Compile and link library for use with ",
    )
    parser.add_argument(
        "project_path",
    )
    parser.add_argument(
        "-a",
        "--arch",
        default = "all",
        choices = ["all", "aarch64", "armv7", "i686", "x86_64"]
    )
    args = parser.parse_args()
    print(args.project_path)
    print(args.arch)

    arches = {
        "aarch64": ("aarch64-linux-android", ["arm64", "arm64-v8a"]),
        "armv7" : ("armv7-linux-androideabi", ["arm", "armeabi", "armeabi-v7a"]),
        "i686" : ("i686-linux-android", ["x86"]),
        "x86_64" : ("x86_64-linux-android", ["x86_64"]),
    }

    print("Checking for Android NDK")
    os.system("./get_ndk.sh")

    targets = []
    if args.arch == "all":
        for arch in arches.values():
            targets.append(arch)
    else:
        targets.append(arches[args.arch])
    print(targets)

    print("Making sure rustup has the following targets installed:")
    for target, folder in targets:
        print(f"  {target}", end='')
        cmd = "rustup target add " + target
        os.system(cmd)
    print()
    
    print("Building library")
    for target, folder in targets:
        cmd = "cargo build --release --lib --features=\"android-lib\" --target " + target
        build_success = os.system(cmd)
        if build_success != 0:
            print("Error: could not compile library")
            exit(-1)

    print("Linking library")
    for (target, folders) in targets:
        cargo_release_path = os.path.join("target", target, f"release/lib{LIBNAME}.so")
        for folder in folders:
            android_folder_path = os.path.join(args.project_path, "app/src/main/jniLibs", folder)
            mkdir_success = os.system("mkdir -p " + android_folder_path)
            if mkdir_success != 0:
                print(f"Error: could not create {android_folder_path}")
                exit(-1)
            full_android_path = os.path.join(android_folder_path, f"lib{LIBNAME}.so")
            cmd = "ln -sf " + os.path.abspath(cargo_release_path) + " " + full_android_path
            print(cmd)
            os.system(cmd)


if __name__ == "__main__":
    main()