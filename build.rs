extern crate pkg_config;

fn main() {
    if std::process::Command::new("pkg-config").output().is_err() {
        println!("cargo:rustc-link-lib=geos_c");
        return;
    }

    // TODO: handle library versions like this!
    //
    // pkg_config::probe_library("geos_c")
    //     .map(|lib| {
    //         if lib.version.starts_with("2.") {
    //             println!(r#"cargo:rustc-cfg=feature="v2""#);
    //         }
    //     })
    //     .expect("GEOS library not found");
}
