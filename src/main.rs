use dialoguer::{theme::ColorfulTheme, Input, Select};
use rayon::prelude::*;
use std::fs::{self, File, OpenOptions};
use std::io::{BufRead, BufReader, Write};
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

fn main() {
    let version_options = vec!["Patch to CS:GO", "Patch back to CS2"];
    let version_selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Which version would you like to patch to?")
        .default(0)
        .items(&version_options)
        .interact()
        .expect("Failed to read input");

    let version = if version_selection == 0 { "2000303" } else { "1575" };

    let path_mode = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Search for CS:GO automatically or enter path manually?")
        .items(&["Auto-scan drives", "Enter a custom path"])
        .default(0)
        .interact()
        .expect("Failed to select path mode");

    let file_path_opt: Option<PathBuf> = if path_mode == 0 {
        println!("Scanning all possible drives on your PC...");
        let drives = get_drives();
        drives.par_iter().find_map_any(|drive| {
            println!("Scanning: {}..", drive);
            find_file(drive, "steam.inf", "csgo")
        })
    } else {
        let input_path: String = Input::with_theme(&ColorfulTheme::default())
            .with_prompt("Enter the path to your CS:GO folder")
            .interact_text()
            .expect("Failed to read path");
        let steam_inf_path = Path::new(&input_path).join("csgo").join("steam.inf");
        if steam_inf_path.exists() {
            Some(steam_inf_path)
        } else {
            println!("Invalid path or steam.inf not found at the specified location.");
            None
        }
    };

    match file_path_opt {
        Some(file_path) => {
            let version_name = match version {
                "2000303" => "CSGO",
                "1575" => "CS2",
                _ => version,
            };

            if patch_file_atomic(&file_path, version) {
                println!("File successfully patched to version: {}.", version_name);
                println!(
                    "{}",
                    if version == "2000303" {
                        "You should now be able to use your skins in CSGO Legacy. You shouldn't play CS2 while this is active."
                    } else {
                        "You can now play CS2 (again)."
                    }
                );
            }
        }
        None => println!("File not found or patching aborted."),
    }
}

fn get_drives() -> Vec<String> {
    let drives = vec!["C:\\", "D:\\", "E:\\", "F:\\", "G:\\", "H:\\"];
    drives
        .into_iter()
        .filter(|d| fs::metadata(d).is_ok())
        .map(String::from)
        .collect()
}

fn find_file(dir: &str, file_name: &str, parent_dir: &str) -> Option<PathBuf> {
    for entry in WalkDir::new(dir).into_iter().filter_map(Result::ok) {
        if entry
            .file_name()
            .to_str()
            .map_or(false, |name| name.eq_ignore_ascii_case(file_name))
        {
            if let Some(parent) = entry.path().parent() {
                if parent
                    .file_name()
                    .map_or(false, |n| n.eq_ignore_ascii_case(parent_dir))
                {
                    return Some(entry.path().to_path_buf());
                }
            }
        }
    }
    None
}

fn patch_file_atomic(file_path: &Path, version: &str) -> bool {
    let original_file = OpenOptions::new()
        .read(true)
        .open(file_path)
        .expect("Failed to open file");
    let reader = BufReader::new(&original_file);
    let mut lines: Vec<String> = reader.lines().filter_map(Result::ok).collect();

    let mut modified = false;
    for line in &mut lines {
        if line.starts_with("ClientVersion=") {
            *line = format!("ClientVersion={}", version);
            modified = true;
            break;
        }
    }

    if !modified {
        return false;
    }

    let new_contents = lines.join("\n");

    let parent_dir = file_path
        .parent()
        .expect("File has no parent directory");
    let temp_path = parent_dir.join("steam.inf.tmp");

    if let Ok(mut tmp_file) = File::create(&temp_path) {
        if tmp_file.write_all(new_contents.as_bytes()).is_ok() {
            if tmp_file.sync_all().is_ok() {
                return fs::rename(&temp_path, file_path).is_ok();
            }
        }
    }

    false
}