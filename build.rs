use std::{env, fs};
use std::path::PathBuf;

fn main() {
    // Re-run the build script if any proto changes
    println!("cargo:rerun-if-changed=proto/pb.proto");
    println!("cargo:rerun-if-changed=proto/grpc.proto");

    // Where prost will output the generated .rs files
    let out_dir = PathBuf::from(env::var("OUT_DIR").expect("OUT_DIR not set"));

    // Generate prost code into OUT_DIR
    let mut cfg = prost_build::Config::new();
    cfg.out_dir(&out_dir);
    cfg.compile_protos(&["proto/pb.proto", "proto/grpc.proto"], &["proto"]).expect("Failed to compile proto files");

    // Discover all generated files in OUT_DIR (e.g., io.haveno.protobuffer.rs)
    let entries = fs::read_dir(&out_dir).expect("Failed to read OUT_DIR");
    let mut modules: Vec<(String, String)> = Vec::new();
    for entry in entries {
        let path = entry.expect("bad entry").path();
        if path.extension().map(|e| e == "rs").unwrap_or(false) {
            let fname = path.file_name().unwrap().to_string_lossy().to_string();
            let module = fname.trim_end_matches(".rs").replace('.', "_");
            modules.push((module, fname));
        }
    }

    // Create a single wrapper that re-exports all generated modules
    // This file is written to OUT_DIR and should be included from src/lib.rs
    let wrapper_path = out_dir.join("generated.rs");
    let mut wrapper = String::new();
    for (module, filename) in modules {
        wrapper.push_str(&format!(
            "pub mod {module} {{ include!(concat!(env!(\"OUT_DIR\"), \"/{filename}\")); }}\n"
        ));
    }

    fs::write(&wrapper_path, wrapper).expect("Failed to write generated wrapper");
}
