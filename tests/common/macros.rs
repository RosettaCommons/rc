#[allow(unused_macros)]
macro_rules! engine_tests {
    // Default: all engines (including none), optional features
    ($test_fn:ident $(; $($feature:literal),* $(,)?)? ) => {
        $crate::common::engine_tests!(
            @gen $test_fn;
            //engines(docker, singularity, apptainer);
            engines(docker, singularity, apptainer, none);
            features($($($feature),*)?)
        );
    };

    // Explicit engines override (+ optional features)
    ($test_fn:ident;
        engines($($eng:ident),+ $(,)? )
        $(; $($feature:literal),* $(,)?)?
    ) => {
        $crate::common::engine_tests!(
            @gen $test_fn;
            engines($($eng),+);
            features($($($feature),*)?)
        );
    };

    // Normalize: features are passed as ONE token tree: ( ... )
    (@gen $test_fn:ident; engines($($eng:ident),+); features $features:tt) => {
        $(
            $crate::common::engine_tests!(@emit $test_fn; $eng; $features);
        )+
    };

    // ---- Per-engine expansion (re-parse features here) ----

    (@emit $test_fn:ident; docker; ($($feature:literal),*) ) => {
        ::paste::paste! {
            #[test]
            #[serial_test::serial]
            #[cfg_attr(
                not(all(feature = "docker-tests" $(, feature = $feature)*)),
                ignore
            )]
            fn [<docker_ $test_fn>]() {
                $crate::common::docker_clear_cache();
                $test_fn("docker");
            }
        }
    };

    (@emit $test_fn:ident; singularity; ($($feature:literal),*) ) => {
        ::paste::paste! {
            #[test]
            #[serial_test::serial]
            #[cfg_attr(
                not(all(feature = "hpc-tests" $(, feature = $feature)*)),
                ignore
            )]
            fn [<singularity_ $test_fn>]() {
                $test_fn("singularity");
            }
        }
    };

    (@emit $test_fn:ident; apptainer; ($($feature:literal),*) ) => {
        ::paste::paste! {
            #[test]
            #[serial_test::serial]
            #[cfg_attr(
                not(all(feature = "hpc-tests" $(, feature = $feature)*)),
                ignore
            )]
            fn [<apptainer_ $test_fn>]() {
                $test_fn("apptainer");
            }
        }
    };

    (@emit $test_fn:ident; none; ($($feature:literal),*) ) => {
        ::paste::paste! {
            #[test]
            #[serial_test::serial]
            #[cfg_attr(
                not(all(feature = "native-tests" $(, feature = $feature)*)),
                ignore
            )]
            fn [<native_ $test_fn>]() {
                $test_fn("none");
            }
        }
    };

    // Nice error for typos / unsupported engines
    (@emit $test_fn:ident; $other:ident; $features:tt ) => {
        compile_error!(concat!(
            "engine_tests!: unknown engine '",
            stringify!($other),
            "'. Expected: docker, singularity, apptainer, none."
        ));
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
