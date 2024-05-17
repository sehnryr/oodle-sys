#[cfg(feature = "bindgen")]
fn generate_bindings() {
    let bindings = bindgen::Builder::default()
        .header("oodle2.h")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        .generate()
        .expect("Unable to generate bindings");

    let out_path = std::path::PathBuf::from(std::env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}

#[cfg(not(feature = "bindgen"))]
fn generate_bindings() {}

fn main() {
    println!("cargo:rustc-link-search=native=/usr/local/lib");
    #[cfg(target_os = "linux")]
    println!("cargo:rustc-link-lib=oo2corelinux64");
    #[cfg(target_os = "windows")]
    println!("cargo:rustc-link-lib=oo2core_win64");
    println!("cargo:rerun-if-changed=oodle2.h");

    generate_bindings();
}
