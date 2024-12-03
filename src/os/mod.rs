mod macos;
mod windows;

#[cfg(target_os = "windows")]
pub use self::windows::check_server_reachable;
#[cfg(target_os = "windows")]
pub use self::windows::set_env_var;

#[cfg(target_os = "macos")]
pub use self::macos::check_server_reachable;
#[cfg(target_os = "macos")]
pub use self::macos::set_env_var;

#[cfg(not(any(target_os = "windows", target_os = "macos")))]
pub fn check_server_reachable() -> bool {
    unreachable!();
}

#[cfg(not(any(target_os = "windows", target_os = "macos")))]
pub fn set_env_var(_name: &str, _value: &str) -> Result<(), Box<dyn std::error::Error>> {
    Err("The installer only supports Windows and macOS, Sorry!".into())
}
