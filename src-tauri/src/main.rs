// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

fn main() {
    #[cfg(feature = "desktop")]
    justlegal_os_lib::run();
    #[cfg(not(feature = "desktop"))]
    eprintln!("JustLegal OS desktop UI disabled in headless build. Run justlegalos-cli instead.");
}
