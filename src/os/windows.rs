#[cfg(target_os = "windows")]
use std::env;
#[cfg(target_os = "windows")]
use std::error::Error;
#[cfg(target_os = "windows")]
use std::ptr;
#[cfg(target_os = "windows")]
use winapi::um::shellapi::ShellExecuteW;
#[cfg(target_os = "windows")]
use winreg::enums::*;
#[cfg(target_os = "windows")]
use winreg::RegKey;

#[cfg(target_os = "windows")]
pub fn set_env_var(name: &str, value: &str) -> Result<(), Box<dyn Error>> {
    if !is_admin() {
        println!("  Requesting administrator privileges!!!");
        return elevate_and_set_var(name, value);
    }

    set_system_env_var(name, value)
}

#[cfg(target_os = "windows")]
fn elevate_and_set_var(name: &str, value: &str) -> Result<(), Box<dyn Error>> {
    let exe_path = env::current_exe()?;
    let exe_path_str = exe_path.to_string_lossy();
    let args = format!("--name \"{}\" --value \"{}\"", name, value);

    unsafe {
        let operation: Vec<u16> = "runas\0".encode_utf16().collect();
        let file: Vec<u16> = exe_path_str.encode_utf16().collect();
        let parameters: Vec<u16> = args.encode_utf16().collect();
        let directory: Vec<u16> = exe_path
            .parent()
            .unwrap()
            .to_string_lossy()
            .encode_utf16()
            .collect();

        let result = ShellExecuteW(
            ptr::null_mut(),
            operation.as_ptr(),
            file.as_ptr(),
            parameters.as_ptr(),
            directory.as_ptr(),
            1,
        );

        if (result as isize) <= 32 {
            return Err("Failed to elevate privileges!".into());
        }
    }

    std::process::exit(0);
}

#[cfg(target_os = "windows")]
fn set_system_env_var(name: &str, value: &str) -> Result<(), Box<dyn Error>> {
    let hklm = RegKey::predef(HKEY_LOCAL_MACHINE);
    let path = "SYSTEM\\CurrentControlSet\\Control\\Session Manager\\Environment";

    let (env, _) = hklm.create_subkey(path)?;
    env.set_value(name, &value)?;

    std::env::set_var(name, value);
    Ok(())
}

#[cfg(target_os = "windows")]
fn is_admin() -> bool {
    if let Ok(_) = RegKey::predef(HKEY_LOCAL_MACHINE)
        .create_subkey("SYSTEM\\CurrentControlSet\\Control\\Session Manager\\Environment")
    {
        return true;
    }
    false
}