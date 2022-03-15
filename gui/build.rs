fn main() {
    #[cfg(all(feature = "wingui", feature = "gtk"))]
    compile_error!("feature \"wingui\" and feature \"gtk\" cannot be enabled at the same time");

    #[cfg(all(feature = "wingui", not(target_os = "windows")))]
    compile_error!("feature \"wingui\" need a windows plateform");

    #[cfg(all(not(feature = "wingui"), not(feature = "gtk")))]
    compile_error!("No gui framework enabled");
}
