use std::env;

fn main() {
    let target = env::var("CARGO_MANIFEST_DIR").unwrap();

    let vc = std::path::Path::new(target.as_str())
        .join("vcpkg")
        .canonicalize().unwrap();

    env::set_var("VCPKG_ROOT", format!("{}", vc.display()));

    vcpkg::find_package("libsodium").unwrap();
}
