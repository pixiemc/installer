use std::error::Error;
use std::io::{self, Read, Write};
use std::process;

mod os;

fn pause() {
    print!("  \n  Press any key to continue!");
    io::stdout().flush().unwrap();
    let _ = io::stdin().read(&mut [0u8]).unwrap();
}

fn run() -> Result<(), Box<dyn Error>> {
    let default_name = "ESSENTIAL_CM_HOST".to_string();
    let default_value = "wss://connect.pixie.rip/v1".to_string();
    let args: Vec<String> = std::env::args().collect();
    let (name, value) = if args.len() > 4 && args[1] == "--name" && args[3] == "--value" {
        (&args[2], &args[4])
    } else {
        (&default_name, &default_value)
    };

    println!("\n  Pixie.rip - The best Essential client modification.");
    println!("  -----------------------------------------------\n");

    if !(cfg!(target_os = "windows") || cfg!(target_os = "macos")) {
        println!("  The installer only supports Windows and macOS, Sorry!");
        pause();
        return Err("Unsupported OS".into());
    }

    if !os::check_server_reachable() {
        println!("  Failed to connect to Pixie, please check your internet connection!");
        pause();
        return Err("Failed to connect to Pixie".into());
    }

    match os::set_env_var(name, value) {
        Ok(()) => {
            println!("  Successfully installed!\n");

            println!("  Important:");
            println!("   - Restart your Launcher/Game to apply changes!\n");

            println!("  Need help?");
            println!("  Website: pixie.rip");
            println!("  Discord: discord.gg/pixiemc\n");
            println!("  If you run into any problems, join our Discord for support!\n");
        }
        Err(e) => {
            println!("  \nInstallation failed!");
            println!("  Please run the installer again as administrator.\n");
            println!("  Need help? Join our Discord: discord.gg/pixiemc\n");
            pause();
            return Err(e);
        }
    }

    pause();
    Ok(())
}

fn main() {
    if let Err(e) = run() {
        println!("  Error: {}", e);
        process::exit(1);
    }
}
