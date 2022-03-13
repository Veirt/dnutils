use std::{env, path::Path, process::exit};

use dnpak::EtFileSystem;

fn main() {
    let mut args = env::args();
    args.next();

    let input = args.next().unwrap_or_else(|| {
        eprintln!("Please input a pak.");
        exit(1);
    });
    let input_path = Path::new(&input).display().to_string();
    if !input_path.ends_with(".pak") {
        eprintln!("File must be ended with .pak");
        exit(1);
    }

    let pak = EtFileSystem::read(&input_path);
    let query = args.next();

    let files;
    if let Some(query) = query {
        files = pak.find_files(&query);
    } else {
        files = pak.get_files();
    }

    for file in files {
        println!("{}", file);
    }
}
