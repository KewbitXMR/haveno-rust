use std::env;
use std::fs;
use std::path::PathBuf;

fn main() {
    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());

    // This writes the prost-generated modules to disk
    prost_build::Config::new()
        .out_dir(&out_dir)
        .compile_protos(
            &["proto/pb.proto", "proto/grpc.proto"], // adjust if more files
            &["proto"],
        )
        .expect("Failed to compile proto files");

    // Wrap the output files into a single includeable mod
    let generated_path = out_dir.join("generated.rs");

    let modules = vec![
        "io.haveno.protobuffer.rs", // match whatever the generated filename is
    ];

    let mod_wrappers: String = modules
        .iter()
        .map(|filename| {
            let module_name = filename.trim_end_matches(".rs").replace('.', "_");
            format!(
                "pub mod {} {{ include!(concat!(env!(\"OUT_DIR\"), \"/{}\")); }}",
                module_name, filename
            )
        })
        .collect::<Vec<_>>()
        .join("\n");

    fs::write(&generated_path, mod_wrappers)
        .expect("Failed to write generated.rs");
}