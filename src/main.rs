mod alias;
mod cli;

use std::env;
use std::io;
use std::os::unix::process::CommandExt;
use std::process::Command;

use std::{collections::HashMap, error::Error, fs, path::PathBuf};

use alias::Aliases;
use clap::Parser;
use cli::Cli;

fn get_aliases_file_path() -> Result<PathBuf, Box<dyn Error>> {
    let home = env::var("HOME")?;
    let config_dir = PathBuf::from(home).join(".config").join("ga");

    fs::create_dir_all(&config_dir)?;

    Ok(config_dir.join("aliases.json"))
}

fn load_aliases() -> Result<Aliases, Box<dyn Error>> {
    let file_path = get_aliases_file_path()?;

    if !file_path.exists() {
        return Ok(Aliases {
            aliases: HashMap::new(),
        });
    }

    let contents = fs::read_to_string(&file_path)?;
    let aliases = serde_json::from_str(&contents)?;
    Ok(aliases)
}

fn save_aliases(aliases: &Aliases) -> Result<(), Box<dyn Error>> {
    let file_path = get_aliases_file_path()?;
    let json_aliases = serde_json::to_string_pretty(aliases)?;
    fs::write(&file_path, json_aliases)?;
    Ok(())
}

fn execute_interactive(command: &str) -> Result<(), Box<dyn std::error::Error>> {
    let trimmed = command.trim();

    if trimmed.starts_with("cd ") {
        let parts: Vec<&str> = trimmed.splitn(2, ' ').collect();
        if parts.len() == 2 {
            let dir = parts[1].trim();
            env::set_current_dir(dir)?;

            let shell = env::var("SHELL").unwrap_or_else(|_| "/bin/sh".to_string());

            // This ensures nvm is loaded before the hook executes
            let _ = Command::new(&shell).arg("-l").exec();
            return Err("exec failed".into());
        }
    }

    let shell = env::var("SHELL").unwrap_or_else(|_| "/bin/sh".to_string());

    let status = Command::new(&shell)
        .arg("-i")
        .arg("-c")
        .arg(command)
        .status()?;

    std::process::exit(status.code().unwrap_or(1));
}

fn add_cmd_if_exists(name: Option<String>, command: Option<String>, aliases: &mut Aliases) {
    if let (Some(name), Some(command)) = (name, command) {
        let entry = aliases.aliases.get(&name);
        if let Some(_entry) = entry {
            println!("short already exists! (Do you want to update it (y/n))");
            let mut op = String::new();
            io::stdin()
                .read_line(&mut op)
                .expect("Failed to read input");

            let should_update = match op.trim().to_lowercase().as_str() {
                "yes" | "y" => true,
                "no" | "n" => false,
                _ => {
                    println!("Invalid input, assuming no");
                    false
                }
            };

            if should_update {
                aliases.aliases.insert(name, command);
            } else {
                println!("Not updating alias.");
            }
        } else {
            aliases.aliases.insert(name, command);
        }

        if let Err(e) = save_aliases(&aliases) {
            eprintln!("Error saving: {}", e);
        }
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let cli = Cli::parse();
    let mut aliases = load_aliases().unwrap_or_else(|_| Aliases {
        aliases: HashMap::new(),
    });

    if cli.show {
        for (name, cmd) in &aliases.aliases {
            println!("{name} ~> {cmd}");
        }
        return Ok(());
    }

    if let Some(cmd) = cli.cmd {
        let mut execmd = &cmd;
        if let Some(command) = aliases.aliases.get(&cmd) {
            execmd = command;
        }

        execute_interactive(&execmd)?;

        return Ok(());
    }

    add_cmd_if_exists(cli.name, cli.command, &mut aliases);
    Ok(())
}
