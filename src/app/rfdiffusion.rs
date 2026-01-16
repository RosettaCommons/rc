use crate::{
    app::{ContainerRunSpec, IoSpec, NativeRunSpec, RunSpec},
    util::include_asset,
};

pub fn spec(app_args: Vec<String>) -> RunSpec {
    let container = ContainerRunSpec::with_prefixed_args(
        "rosettacommons/rfdiffusion",
        [
            "inference.output_prefix=/w/",
            "inference.model_directory_path=/app/RFdiffusion/models",
        ],
        app_args.clone(),
    )
    .scratch("/app/RFdiffusion/schedules")
    .working_dir("/w");

    let native = NativeRunSpec::new(
        include_asset!("pixi/rfdiffusion.toml"),
        IoSpec::InputDir("aaa".into()),
        app_args,
    );

    RunSpec::new(container, Some(native))
}
