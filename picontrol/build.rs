use std::{env, path::PathBuf};

use bindgen::CargoCallbacks;

fn main() {
    let bindings = bindgen::builder()
        .clang_arg("--target=arm-unknown-linux-gnueabihf")
        .header("piControl.h")
        .parse_callbacks(Box::new(CargoCallbacks))
        .generate()
        .unwrap();

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
