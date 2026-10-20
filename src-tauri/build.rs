fn main() {
    println!("cargo:rerun-if-changed=icons/icon.ico");
    println!("cargo:rerun-if-changed=tauri.conf.json");
    #[cfg(feature = "desktop")]
    tauri_build::build();
}
