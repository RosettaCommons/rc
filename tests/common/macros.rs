#[allow(unused_macros)]
macro_rules! engine_tests {
    ($test_fn:ident) => {
        ::paste::paste! {

            #[test]
            #[serial_test::serial]
            #[cfg_attr(not(feature = "docker-tests"), ignore)]
            fn [<docker_ $test_fn>]() {
                $test_fn("docker");
                common::docker_clear_cache();
            }

            #[test]
            #[serial_test::serial]
            #[cfg_attr(not(feature = "hpc-tests"), ignore)]
            fn [<singularity_ $test_fn>]() {
                $test_fn("singularity");
            }
            #[test]
            #[serial_test::serial]
            #[cfg_attr(not(feature = "hpc-tests"), ignore)]
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
