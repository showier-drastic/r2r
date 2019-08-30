use common::*;
use msg_gen::*;
use std::env;
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;

fn main() {
    let msgs_str = read_file("./msgs.txt").expect("You need to create msgs.txt");
    let msgs_list = parse_msgs(&msgs_str);
    let msgs = as_map(&msgs_list);

    let mut codegen = String::new();

    for (module, prefixes) in &msgs {
        println!(
            "cargo:rustc-link-lib=dylib={}__rosidl_typesupport_c",
            module
        );
        println!(
            "cargo:rustc-link-lib=dylib={}__rosidl_typesupport_introspection_c",
            module
        );
        println!("cargo:rustc-link-lib=dylib={}__rosidl_generator_c", module);

        codegen.push_str(&format!("pub mod {} {{\n", module));

        for (prefix, msgs) in prefixes {
            codegen.push_str(&format!("  pub mod {} {{\n", prefix));
            codegen.push_str("    use super::super::*;\n");

            for msg in msgs {
                codegen.push_str(&generate_rust_msg(module, prefix, msg));
            }

            codegen.push_str("  }\n");
        }

        codegen.push_str("}\n");
    }

    let untyped_helper = generate_untyped_helper(&msgs_list);

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    let msgs_fn = out_path.join("generated_msgs.rs");
    let untyped_fn = out_path.join("generated_untyped_helper.rs");

    let mut f = File::create(msgs_fn).unwrap();
    write!(f, "{}", codegen).unwrap();
    let mut f = File::create(untyped_fn).unwrap();
    write!(f, "{}", untyped_helper).unwrap();
}