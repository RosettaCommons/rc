use crate::app::RunSpec;

pub fn rfdiffusion(mut app_args: Vec<String>) -> RunSpec {
    app_args.splice(
        0..0,
        [
            "inference.output_prefix=/w/".into(),
            "inference.model_directory_path=/app/RFdiffusion/models".into(),
        ],
    );
    RunSpec::new("rosettacommons/rfdiffusion", app_args)
        .scratch("/app/RFdiffusion/schedules")
        .working_dir("/w")
}
