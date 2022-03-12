fn main() -> Result<(), Box<dyn std::error::Error>> {
    tonic_build::compile_protos("../salty-service/proto/salty.proto")?;
    Ok(())
}
