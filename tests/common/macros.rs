#[allow(unused_macros)]
macro_rules! engine_tests {
    // Handles both cases:
    //   common::engine_tests!(rfdiffusion);
    //   common::engine_tests!(rfdiffusion; "high-memory-tests");
    //   common::engine_tests!(rfdiffusion; "high-memory-tests", "some-awesome-feature");
    //
    ($test_fn:ident $(; $($feature:literal),+ $(,)?)?) => {
        ::paste::paste! {

            #[test]
            #[serial_test::serial]
            #[cfg_attr(
                not(all(
                    feature = "docker-tests"
                    $( $(, feature = $feature)*)?
                )),
                ignore
            )]
            fn [<docker_ $test_fn>]() {
                common::docker_clear_cache();
                $test_fn("docker");
            }

            #[test]
            #[serial_test::serial]
            #[cfg_attr(
                not(all(
                    feature = "hpc-tests"
                    $($(, feature = $feature)*)?
                )),
                ignore
            )]
            fn [<singularity_ $test_fn>]() {
                $test_fn("singularity");
            }

            #[test]
            #[serial_test::serial]
            #[cfg_attr(
                not(all(
                    feature = "hpc-tests"
                    $($(, feature = $feature)*)?
                )),
                ignore
            )]
            fn [<apptainer_ $test_fn>]() {
                $test_fn("apptainer");
            }
        }
    };
}

// macro_rules! engine_tests {
//     ($test_fn:ident, $($engine:ident),+ $(,)?) => {
//         $(
//             ::paste::paste! {
//                 #[test]
//                 #[cfg_attr(not(feature = "hpc-tests"), ignore)]
//                 fn [<$engine:lower _ $test_fn>]() {
//                     $test_fn($engine);
//                 }
//             }
//         )*
//     };
// }
// engine_tests!(rosetta_score, APPTAINER, SINGULARITY);

pub(crate) use engine_tests;
