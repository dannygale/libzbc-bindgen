extern crate bindgen;

use std::env;
use std::path::PathBuf;

fn main() {
    let bindings = bindgen::Builder::default()
        .header("wrapper.h")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        .generate()
        .expect("unable to generate bindings");

    let outpath = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(outpath.join("bindings.rs"))
        .expect("couldn't write bindings");
}
