use crate::{
    app::{ContainerRunSpec, NativeRunSpec, RunSpec},
    util::include_asset,
};

pub fn spec(app_args: Vec<String>) -> RunSpec {
    let container = ContainerRunSpec::new(
        "rosettacommons/rfdiffusion",
        [
            "inference.output_prefix=/w/",
            "inference.model_directory_path=/app/RFdiffusion/models",
        ]
        .into_iter()
        .map(Into::into)
        .chain(app_args.clone())
        .collect(),
    )
    .scratch("/app/RFdiffusion/schedules")
    .working_dir("/w");

    let native = NativeRunSpec::new(include_asset!("pixi/rfdiffusion.toml"), app_args);

    RunSpec::new(container, Some(native))
}
