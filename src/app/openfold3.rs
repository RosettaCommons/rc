use camino::Utf8Path;

use crate::{
    app::{AppSpec, ContainerConfig, NativeRunSpec, make_absolute},
    util::include_asset,
};

pub struct Openfold3;
pub static OPENFOLD3: Openfold3 = Openfold3;

impl AppSpec for Openfold3 {
    fn container_image(&self) -> &'static str {
        "openfoldconsortium/openfold3:stable"
    }

    fn pixi_recipe(&self) -> Option<&'static str> {
        Some(include_asset!("pixi/openfold3.toml"))
    }

    fn container_spec(&self, app_args: Vec<String>) -> ContainerConfig {
        ContainerConfig::with_prefixed_args(["run_openfold"], app_args).working_dir("/w")
    }

    fn native_spec(&self, app_args: Vec<String>, working_dir: &Utf8Path) -> NativeRunSpec {
        assert!(
            app_args.len() == 1,
            "Openfold3 arguments must include a input-json file as first argument"
        );
        let json_file = make_absolute(working_dir, &app_args[0]);

        let app_args = vec![format!(
            "--output-dir={working_dir} --query_json={json_file}"
        )];

        NativeRunSpec::new(app_args)
    }
}
