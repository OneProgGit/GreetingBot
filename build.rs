fn main() -> Result<(), Box<dyn std::error::Error>> {
    tonic_prost_build::configure().compile_protos(
        &[
            "proto/models.proto",
            "proto/ai.proto",
            "proto/db.proto",
            "proto/platform.proto",
            "proto/weather.proto",
        ],
        &["proto"],
    )?;
    Ok(())
}
