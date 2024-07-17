use std::env;
use cmake::Config;

fn build_and_link_soplex() {
    let profile = env::var("PROFILE").unwrap();
    let build_type = if profile == "release" { "Release" } else { "Debug" };

    // build soplex library with cmake
    let dst = Config::new("soplex")
        .profile(build_type)
        .define("GMP", "off")
        .define("BOOST", "off")
        .define("QUADMATH", "off")
        .define("PAPILO", "off")
        .build();

    println!("cargo:rustc-link-search={}/lib", dst.display());
    println!("cargo:rustc-link-search={}/lib64", dst.display());
    println!("cargo:rustc-link-lib=dylib=soplexshared");
    println!("cargo:rustc-link-lib=z");
}

fn main() {
    // skip building soplex on docs.rs
    if std::env::var("DOCS_RS").is_err() {
        build_and_link_soplex();
    }

    // generate bindings
    let bindings = bindgen::Builder::default()
        .header("soplex/src/soplex_interface.h")
        .allowlist_function("SoPlex_.*")
        .generate()
        .expect("Unable to generate bindings");

    let out_path = std::path::PathBuf::from(std::env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
