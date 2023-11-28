#[cfg(windows)]
use std::process::{Stdio, Command};
#[cfg(windows)]
use std::os::windows::process::CommandExt;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
  #[cfg(windows)]
  {
    if !is_windows_11() {
      panic!("The app only runs on Windows 11 or above");
    }
  }

  tauri::Builder::default()
    .plugin(tauri_plugin_dialog::init())
    .plugin(tauri_plugin_notification::init())
    .plugin(tauri_plugin_http::init())
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}

#[cfg(windows)]
fn is_windows_11() -> bool {
  let version = Command::new("cmd")
    .creation_flags(0x08000000)
    .args(["/c", "ver"])
    .stdout(Stdio::piped())
    .spawn()
    .unwrap()
    .wait_with_output()
    .unwrap()
    .stdout;

  let string = String::from_utf8(version).unwrap();
  let splitted = string
    .replace("\n", "")
    .replace("Microsoft Windows [", "")
    .replace("]", "");
  let version: Vec<&str> = splitted.split(".").collect();

  let version: usize = version[2].parse().unwrap_or(0);

  version >= 22000
}