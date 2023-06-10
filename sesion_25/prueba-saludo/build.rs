fn main() -> Result<(), Box<dyn std::error::Error>> {
	let path = "../api-prueba/prueba.proto";
	tonic_build::compile_protos(path)?;
	Ok(())
}
