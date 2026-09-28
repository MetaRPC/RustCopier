use std::path::Path;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("cargo:rerun-if-changed=proto/copier.proto");
    println!("cargo:rerun-if-changed=proto");

    let out_dir = std::env::var("OUT_DIR")?;
    let dest_path = Path::new(&out_dir).join("copier.rs");

    // Try compiling with protoc; if protoc is not installed, fall back to pre-generated code
    if let Err(_) = tonic_build::compile_protos("proto/copier.proto") {
        let fallback_path = Path::new("src/copier.rs");
        if fallback_path.exists() {
            std::fs::copy(fallback_path, dest_path)?;
        }
    }

    Ok(())
}
