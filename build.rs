fn main() -> Result<(), Box<dyn std::error::Error>> {
    tonic_build::configure()
        .build_server(false)
        .build_client(true)
        .out_dir("src/protos")
        .compile(&["proto/rule.proto"], &["proto"])?;

    std::fs::rename(
        "src/protos/com.stey.rms.api.grpc.config.rs",
        "src/protos/rule.rs",
    )?;

    Ok(())
}
