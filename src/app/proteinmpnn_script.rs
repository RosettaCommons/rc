use crate::app::ContainerRunSpec;

pub fn container_spec(mut app_args: Vec<String>) -> ContainerRunSpec {
    assert!(
        app_args.is_empty() || app_args[0].starts_with("-"),
        "ProteinmpnnScript arguments must include a script name as first argument"
    );

    let have_input_path_option = ![
        "helper_scripts/make_bias_AA.py",
        "helper_scripts/make_pssm_input_dict.py",
    ]
    .contains(&app_args[0].as_str());

    if have_input_path_option {
        app_args.splice(1..1, ["--input_path=/w".into()]);
    }
    app_args.splice(1..1, ["--output_path=/w".into()]);
    app_args[0].insert_str(0, "/app/proteinmpnn/helper_scripts/");

    ContainerRunSpec::new("rosettacommons/proteinmpnn", app_args).working_dir("/w")
}
