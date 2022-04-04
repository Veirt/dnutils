#![windows_subsystem = "windows"]
use std::{env, path::Path, process::exit};

use dnpak::EtFileSystem;

fn main() {
    let mut args = env::args();
    args.next();

    let input = args.next().unwrap_or_else(|| {
        eprintln!("Please input a directory.");
        exit(1);
    });
    let mut input_path = Path::new(&input).display().to_string();
    println!("{}", &input_path);

    let output = args.next();

    let mut new_pak;

    let output_path = {
        let mut tmp_path;
        if let Some(path) = output {
            tmp_path = Path::new(&path).display().to_string();
        } else {
            // to prevent output inside the folder
            let last_char = input_path.chars().last().unwrap();
            if last_char == '/' || last_char == '\\' {
                input_path.pop();
            }

            tmp_path = input_path.clone()
        }

        if !tmp_path.ends_with(".pak") {
            tmp_path.push_str(".pak");
        }

        tmp_path
    };

    new_pak = EtFileSystem::write(&output_path);
    new_pak.add_files(&input_path).unwrap_or_else(|err| {
        eprintln!("Something went wrong: {}", err);
        exit(1);
    });

    new_pak.close_file_system();
}
