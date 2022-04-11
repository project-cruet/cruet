extern crate bindgen;

use bindgen::callbacks::IntKind;
use std::env;
use std::path::PathBuf;

#[derive(Debug)]
struct LongForIntMacro;

impl bindgen::callbacks::ParseCallbacks for LongForIntMacro {
    fn int_macro(&self, name: &str, _value: i64) -> Option<IntKind> {
        if name.starts_with("__NR_") {
            Some(IntKind::Custom {
                name: "SyscallArg",
                is_signed: true,
            })
        } else {
            None
        }
    }
}

fn main() {
    println!("cargo:rerun-if-changed=wrapper.h");
    let bindings = bindgen::Builder::default()
        .header("wrapper.h")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        .parse_callbacks(Box::new(LongForIntMacro))
        .allowlist_var("__NR_.*")
        .generate()
        .expect("Unable to generate bindings");
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("unistd.rs"))
        .expect("Couldn't write bindings!");
}
