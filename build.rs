extern crate gcc;

fn main() {
    println!("cargo:rustc-flags=-l dylib=stdc++");

    println!("cargo:rustc-link-search=/home/pi/rust-scoreboard/rust-rpi-rgb-led-matrix/rpi-rgb-led-matrix/lib");
}
