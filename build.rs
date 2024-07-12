use cmake::Config;

fn main() {
    // build soplex library with cmake
    let dst = Config::new("soplex")
        .define("GMP", "off")
        .define("BOOST", "off")
        .define("QUADMATH", "off")
        .define("PAPILO", "off")
        .build();

    println!("cargo:rustc-link-search={}/lib", dst.display());
    println!("cargo:rustc-link-search={}/lib64", dst.display());
    println!("cargo:rustc-link-lib=dylib=soplexshared");
    println!("cargo:rustc-link-lib=z");

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
