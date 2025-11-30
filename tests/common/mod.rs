mod macros;
mod path_shim;

#[allow(unused)]
pub(crate) use macros::engine_tests;

#[allow(unused)]
pub use path_shim::ContainerPathShim;

#[allow(dead_code)]
#[cfg_attr(not(feature = "docker-tests"), ignore)]
pub fn docker_clear_cache() {
    // detect if run under GitHub CI and clear Docker cache if so
    if std::env::var("GITHUB_ACTIONS").is_ok() {
        eprintln!("Running in GitHub CI, clearing Docker cache...");
        let output = std::process::Command::new("docker")
            .args(["system", "prune", "-af", "--volumes"])
            .output();

        match output {
            Ok(result) => {
                if result.status.success() {
                    eprintln!("Docker cache cleared successfully");
                } else {
                    eprintln!(
                        "Docker prune failed: {}",
                        String::from_utf8_lossy(&result.stderr)
                    );
                }
            }
            Err(e) => {
                eprintln!("Failed to run docker system prune: {}", e);
            }
        }
    }
}
