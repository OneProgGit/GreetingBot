fn main() -> Result<(), Box<dyn std::error::Error>> {
    tonic_prost_build::configure()
        .build_server(true)
        .build_client(true)
        .out_dir("src/generated")
        .compile_protos(
            &[
                "protos/models.proto",
                "protos/ai.proto",
                "protos/db.proto",
                "protos/platform.proto",
                "protos/weather.proto",
                "protos/commands.proto",
            ],
            &["protos"],
        )?;
    Ok(())
}
