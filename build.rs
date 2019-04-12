extern crate pkg_config;

fn main() {
    let lib = "geos_c";

    match pkg_config::Config::new().probe(lib) {
        Ok(_) => {}
        Err(_) => {
            println!("cargo:rustc-link-lib=dylib={}", lib);
        }
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
