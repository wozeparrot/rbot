use bindgen;
use std::env;

fn main() {
    generate_bindings();
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

    let path = env::current_dir().unwrap();
    println!("cargo:rustc-link-search=native={}/{}", path.display(), LIB_DIR);
}

fn generate_bindings() {
    const HEADER_DIR: &str = "headers";
    const BLOCK_REGEX_1: &str = r"HAL_\w+";
    const BLOCK_REGEX_2: &str = r"HALUsageReporting::.*";

    let bindings = bindgen::Builder::default()
        .header("HAL_wrapper.h")
        .whitelist_type(BLOCK_REGEX_1)
        .whitelist_function(BLOCK_REGEX_1)
        .whitelist_var(BLOCK_REGEX_1)
        .whitelist_type(BLOCK_REGEX_2)
        .clang_arg(format!("-I{}", HEADER_DIR))
        .clang_arg("-nostdinc")
        .clang_arg("-xc++")
        .clang_arg("-nostdinc++")
        .clang_arg("-std=c++14")
        .derive_default(true)
        .default_enum_style(bindgen::EnumVariation::ModuleConsts)
        .parse_callbacks(Box::new(Callbacks));
    
    println!("bindgen_args: {:?}", bindings.command_line_flags());
    
    let out = bindings.generate().expect("Unable to generate bindings.");

    out.write_to_file("src/hal_bindings.rs").expect("Could not write bindings to file.");

}

#[derive(Debug)]
struct Callbacks;

impl bindgen::callbacks::ParseCallbacks for Callbacks {
    fn enum_variant_name(&self, enum_name: Option<&str>, original_variant_name: &str, _variant_value: bindgen::callbacks::EnumVariantValue) -> Option<String> {
        match enum_name {
            Some("tResourceType") => {
                Some(original_variant_name["kResourceType_".len()..].to_owned())
            }
            Some(enum_name) if original_variant_name.starts_with(enum_name) => {
                Some(original_variant_name[enum_name.len() + 1..].to_owned())
            }
            _ => None,
        }
    }

    fn int_macro(&self, name: &str, _value: i64) -> Option<bindgen::callbacks::IntKind> {
        match name {
            "HAL_kInvalidHandle" => Some(bindgen::callbacks::IntKind::I32),
            _ => None,
        }
    }

    fn will_parse_macro(&self, name: &str) -> bindgen::callbacks::MacroParsingBehavior {
        if name.ends_with("_MESSAGE") {
            bindgen::callbacks::MacroParsingBehavior::Ignore
        } else {
            bindgen::callbacks::MacroParsingBehavior::Default
        }
    }
}