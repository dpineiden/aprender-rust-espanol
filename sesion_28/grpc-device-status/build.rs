fn main() -> Result<(), Box<dyn std::error::Error>> {
	let path = "../api/status.proto";
	tonic_build::compile_protos(path)?;
	Ok(())
}
