fn main() -> Result<(), Box<dyn std::error::Error>> {
	let path = "../api-grpc/primos.proto";
	tonic_build::compile_protos(path)?;
	Ok(())
}
