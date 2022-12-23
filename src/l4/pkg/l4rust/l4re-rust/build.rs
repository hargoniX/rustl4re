use std::env;
use std::path::PathBuf;

fn main() {
    let mut bindings = bindgen::Builder::default()
        .use_core()
        .ctypes_prefix("core::ffi")
        .rustified_enum(".*")
        .header("bindgen.h");
    if let Ok(include_dirs) = ::std::env::var("L4_INCLUDE_DIRS") {
        for item in include_dirs.split(" ") {
            bindings = bindings.clang_arg(item);
        }
    }
    let bindings = bindings.generate().expect("Unable to generate bindings");
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
