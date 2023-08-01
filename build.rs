fn main() {
    // Re-run build script if the `zkp_auth` file has been modified or deleted
    // Tonic build reruns only for the protos if modified
    println!("cargo:rerun-if-changed=src/zkp_auth.rs");

    tonic_build::configure()
        .out_dir("src/")
        .compile(&["proto/zkp_auth.proto"], &["proto/"])
        .unwrap();
}
