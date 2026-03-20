mod common;

use assert_cmd::{assert::OutputAssertExt, cargo::cargo_bin_cmd};
use serde::Deserialize;

const APP: &str = "proteinmpnn";
const ENGINE_NATIVE: &str = "none";
const ENGINE_SINGULARITY: &str = "singularity";
const ENGINE_DOCKER: &str = "docker";

/// Mirrors the per-app entry emitted by `rc config show --json`.
#[derive(Debug, Clone, Deserialize)]
pub struct AppConfig {
    pub name: String,
    pub container_image: String,
    pub hpc_image_path: String,
    pub native_root: Option<String>,
}

#[derive(Debug, Deserialize)]
struct ConfigOutput {
    apps: Vec<AppConfig>,
}

/// Runs `rc config show --json`, parses the output, and returns the [`AppConfig`]
/// for the given `app_name`.
///
/// # Panics
/// Panics if the command fails to execute, exits with a non-zero status, produces
/// invalid JSON, or does not contain an entry for `app_name`.
pub fn get_app_config(app_name: &str) -> AppConfig {
    let output = cargo_bin_cmd!()
        .args(["config", "show", "--json"])
        .output()
        .expect("failed to execute `rc config show --json`");

    assert!(
        output.status.success(),
        "`rc config show --json` exited with {}\nstderr: {}",
        output.status,
        String::from_utf8_lossy(&output.stderr),
    );

    let stdout = String::from_utf8(output.stdout)
        .expect("`rc config show --json` produced non-UTF-8 output");

    let config: ConfigOutput = serde_json::from_str(&stdout)
        .expect("failed to parse `rc config show --json` output as JSON");

    config
        .apps
        .into_iter()
        .find(|a| a.name == app_name)
        .unwrap_or_else(|| panic!("app '{app_name}' not found in `rc config show --json` output"))
}

#[test]
#[serial_test::file_serial]
#[cfg_attr(not(feature = "native-tests"), ignore)]
fn native_install_clean() {
    let app_config = get_app_config(APP);

    let native_root = app_config.native_root.unwrap_or_else(|| {
        panic!("{APP} native_root should be set — does it have a pixi_recipe()?")
    });

    let native_root_path = std::path::Path::new(&native_root);

    // Ensure a clean slate before installing.
    cargo_bin_cmd!()
        .args(["clean", APP, "-e", ENGINE_NATIVE])
        .unwrap();

    // --- Install ---
    let install_output = cargo_bin_cmd!()
        .args(["install", APP, "-e", ENGINE_NATIVE])
        .unwrap();

    install_output.assert().success();

    assert!(
        native_root_path.exists(),
        "native_root '{native_root}' should exist after `rc install {APP} -e {ENGINE_NATIVE}`",
    );

    // --- Clean ---
    let clean_output = cargo_bin_cmd!()
        .args(["clean", APP, "-e", ENGINE_NATIVE])
        .unwrap();

    clean_output.assert().success();

    assert!(
        !native_root_path.exists(),
        "native_root '{native_root}' should be removed after `rc clean {APP} -e {ENGINE_NATIVE}`",
    );
}

#[test]
#[serial_test::file_serial]
#[cfg_attr(not(feature = "hpc-tests"), ignore)]
fn singularity_install_clean() {
    let app_config = get_app_config(APP);

    let sif_path = std::path::Path::new(&app_config.hpc_image_path);

    // Ensure a clean slate before installing.
    cargo_bin_cmd!()
        .args(["clean", APP, "-e", ENGINE_SINGULARITY])
        .unwrap();

    // --- Install ---
    let install_output = cargo_bin_cmd!()
        .args(["install", APP, "-e", ENGINE_SINGULARITY])
        .unwrap();

    install_output.assert().success();

    assert!(
        sif_path.exists(),
        "hpc_image_path '{}' should exist after `rc install {APP} -e {ENGINE_SINGULARITY}`",
        app_config.hpc_image_path,
    );

    // --- Clean ---
    let clean_output = cargo_bin_cmd!()
        .args(["clean", APP, "-e", ENGINE_SINGULARITY])
        .unwrap();

    clean_output.assert().success();

    assert!(
        !sif_path.exists(),
        "hpc_image_path '{}' should be removed after `rc clean {APP} -e {ENGINE_SINGULARITY}`",
        app_config.hpc_image_path,
    );
}

fn docker_image_exists(image: &str) -> bool {
    std::process::Command::new("docker")
        .args(["image", "inspect", image])
        .stdout(std::process::Stdio::null())
        .stderr(std::process::Stdio::null())
        .status()
        .map(|s| s.success())
        .unwrap_or(false)
}

#[test]
#[serial_test::file_serial]
#[cfg_attr(not(feature = "docker-tests"), ignore)]
fn docker_install_clean() {
    let app_config = get_app_config(APP);

    // Ensure a clean slate before installing.
    cargo_bin_cmd!()
        .args(["clean", APP, "-e", ENGINE_DOCKER])
        .unwrap();

    // --- Install ---
    let install_output = cargo_bin_cmd!()
        .args(["install", APP, "-e", ENGINE_DOCKER])
        .unwrap();

    install_output.assert().success();

    assert!(
        docker_image_exists(&app_config.container_image),
        "docker image '{}' should exist after `rc install {APP} -e {ENGINE_DOCKER}`",
        app_config.container_image,
    );

    // --- Clean ---
    let clean_output = cargo_bin_cmd!()
        .args(["clean", APP, "-e", ENGINE_DOCKER])
        .unwrap();

    clean_output.assert().success();

    assert!(
        !docker_image_exists(&app_config.container_image),
        "docker image '{}' should be removed after `rc clean {APP} -e {ENGINE_DOCKER}`",
        app_config.container_image,
    );
}
