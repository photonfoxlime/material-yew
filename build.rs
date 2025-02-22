use std::env;
use std::path::Path;
use std::process::Command;

fn main() {
    let manifest_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    let build_dir = Path::new(&manifest_dir).join("build");

    // Run npm install in the build directory
    let status = Command::new("npm")
        .arg("install")
        .current_dir(build_dir.as_path())
        .status()
        .expect("Failed to execute npm install");

    if !status.success() {
        panic!("npm install failed");
    }

    // Run npm run build in the build directory
    let status = Command::new("npm")
        .arg("run")
        .arg("build")
        .current_dir(build_dir.as_path())
        .status()
        .expect("Failed to execute npm run build");

    if !status.success() {
        panic!("npm run build failed");
    }
}
