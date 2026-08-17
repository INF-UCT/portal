use std::{env, path::PathBuf};

fn main() -> Result<(), Box<dyn std::error::Error>> {
	let out_dir = PathBuf::from(env::var("OUT_DIR")?);
	let descriptor_path = out_dir.join("sword_descriptor_set.bin");

	let proto_dir = concat!(env!("CARGO_MANIFEST_DIR"), "/../../shared/proto/ldapi");

	tonic_prost_build::configure()
		.file_descriptor_set_path(&descriptor_path)
		.compile_protos(
			&[
				format!("{proto_dir}/users.proto"),
				format!("{proto_dir}/auth.proto"),
			],
			&[proto_dir.to_string()],
		)?;

	Ok(())
}
