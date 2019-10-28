use bindgen;
use std::env;
use std::path::*;

fn rbot_dir() -> PathBuf {
    PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap())
}

fn main() {
    link_libs();
}

const LIB_DIR: &'static str = "libs";
const LIB_LIST: &'static [&'static str] = &[
    "FRC_NetworkCommunication",
    "NiFpga",
    "NiFpgaLv",
    "niriodevenum",
    "niriosession",
    "NiRioSrv",
    "RoboRIO_FRC_ChipObject",
    "visa",
    "wpiHal",
    "wpiutil",
];

fn link_libs() {
    for lib in LIB_LIST.iter() {
        println!("cargo:rustc-link-lib=dylib={}", lib);
    }

    println!("cargo:rustc-link-search=native={}{}", rbot_dir().join("libs/").display(), LIB_DIR);
}