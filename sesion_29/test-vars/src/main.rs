use std::env;

fn main() {
    if let Some(manifest_dir) = env::var("CARGO_MANIFEST_DIR").ok() {
        let some_file_path = format!("{}", manifest_dir);
        // Resto de tu código...
        println!("{}", some_file_path);
    }
}
