extern crate gl_generator;

use gl_generator::{Registry, Api, Profile, Fallbacks, StructGenerator, DebugStructGenerator};
use std::env;
use std::fs::{File, copy};
use std::path::{Path};

const FILE_NAME: &str = "bindings.rs";
//const OUT_DIR_ENV: &str = "OUT_DIR";
//const CRATE_DIR_ENV: &str = "CARGO_MANIFEST_DIR";

fn main() {
    let out_dir = env::var("OUT_DIR").unwrap();
    let mut file_gl = File::create(&Path::new(&out_dir).join("bindings.rs")).unwrap();

    let registry = Registry::new(
        Api::Gl,
        (3, 3),
        Profile::Core,
        Fallbacks::All,
        [],
    );

    if env::var("CARGO_FEATURE_DEBUG").is_ok() {
        registry.write_bindings(
            DebugStructGenerator,
            &mut file_gl,
        ).unwrap();
    } else {
        registry.write_bindings(
            StructGenerator,
            &mut file_gl,
        ).unwrap();
    }

    let out_dir = env::var("OUT_DIR").unwrap();
    let crate_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    let out_file = Path::new(&out_dir).join(FILE_NAME);
    let src_file = Path::new(&crate_dir).join("src").join(FILE_NAME);
    copy(out_file.as_path(), src_file.as_path()).expect("文件拷贝失败");
}


