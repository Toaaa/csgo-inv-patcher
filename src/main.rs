use dialoguer::{theme::ColorfulTheme, Select};
use rayon::prelude::*;
use std::fs::{self, OpenOptions};
use std::io::{BufRead, BufReader};
use std::path::Path;
use walkdir::WalkDir;

fn main() {
    let options: Vec<&str> = vec!["Patch to CS:GO", "Patch back to CS2"];
    let selection: usize = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Which version would you like to patch to?")
        .default(0)
        .items(&options)
        .interact()
        .expect("Failed to read input");

    let version: &str = if selection == 0 { "2000303" } else { "2000370" }; // 2000303 = CS:GO Legacy, 2000370 = CS2

    let drives: Vec<String> = get_drives();

    println!("Scanning all possible drives on your PC...");

    let found_file = drives.par_iter().find_map_any(|drive: &String| {
        println!("Scanning: {}..", drive);
        find_file(drive, "steam.inf", "csgo")
    });

    match found_file {
        Some(file_path) => {
            // println!("File found at: {}", file_path.display()); // just for debugging

            let version_name: &str = match version {
                "2000303" => "CSGO",
                "2000370" => "CS2",
                _ => version,
            };

            if patch_file(&file_path, version) {
                println!("File successfully patched to version: {}.", version_name);

                let note: &str = if version == "2000303" {
                    "You should now be able to use your skins in CSGO Legacy. You shouldn't play CS2 while this is active."
                } else {
                    "You can now play CS2 (again)."
                };

                println!("{}", note);
                // open_in_notepad(&file_path); // This was only used for testing reasons and is not (longer) necessary.
            } else {
                println!("Failed to patch the file.");
            }
        }
        None => {
            println!("File not found on any drive.");
        }
    }
}

fn get_drives() -> Vec<String> {
    let drives: Vec<&str> = vec!["C:\\", "D:\\", "E:\\", "F:\\", "G:\\"];
    drives
        .into_iter()
        .filter(|d: &&str| fs::metadata(d).is_ok())
        .map(String::from)
        .collect()
}

fn find_file(dir: &str, file_name: &str, parent_dir: &str) -> Option<std::path::PathBuf> {
    for entry in WalkDir::new(dir).into_iter().filter_map(Result::ok) {
        if entry
            .file_name()
            .to_str()
            .map_or(false, |name: &str| name.eq_ignore_ascii_case(file_name))
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

fn patch_file(file_path: &Path, version: &str) -> bool {
    let file: fs::File = OpenOptions::new()
        .read(true)
        .write(true)
        .open(file_path)
        .expect("Failed to open file");
    let reader = BufReader::new(&file);
    let mut contents: Vec<String> = reader.lines().filter_map(Result::ok).collect();

    for line in &mut contents {
        if line.starts_with("ClientVersion=") {
            *line = format!("ClientVersion={}", version);
            break;
        }
    }

    let new_contents: String = contents.join("\n");
    fs::write(file_path, new_contents).is_ok()
}

/* This was only used for testing reasons and is not (longer) necessary. */

// // Function to open the file in Notepad
// fn open_in_notepad(file_path: &Path) {
//     Command::new("notepad.exe")
//         .arg(file_path)
//         .spawn()
//         .expect("Failed to open Notepad");
// }
