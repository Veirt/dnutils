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

    // add pak if it's not specified.
    if !input_path.ends_with(".pak") {
        eprintln!("File must be ended with .pak");
        exit(1);
    }

    let pak = EtFileSystem::read(&input_path);

    let file_list = pak.get_files();
    for file in file_list {
        println!("{}", file);
    }
}
