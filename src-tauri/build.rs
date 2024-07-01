fn main() -> Result<(), Box<dyn std::error::Error>> {
    tonic_build::configure()
    .build_server(true)
    .compile(
        // &["src/api/general.proto", "src/api/data_service.proto"],
        &["proto/data_service.proto", "proto/data_service.proto"],
        &["proto"],
    )?;
    tauri_build::build();
    Ok(())
}
