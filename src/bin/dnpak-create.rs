use std::{env, path::Path, process::exit};

use dnpak::EtFileSystem;

fn main() {
    let mut args = env::args();
    args.next();

    let input = args.next().unwrap_or_else(|| {
        eprintln!("Please input a directory.");
        exit(1);
    });
    let input_path = Path::new(&input).display().to_string();

    let output = args.next().unwrap_or_else(|| {
        eprintln!("Please specify the output path.");
        exit(1);
    });
    let mut output_path = Path::new(&output).display().to_string();

    // add pak if it's not specified.
    if !output_path.ends_with(".pak") {
        output_path.push_str(".pak");
    }

    let mut new_pak = EtFileSystem::write(&output_path);

    new_pak.add_files(&input_path).unwrap_or_else(|err| {
        eprintln!("Something went wrong: {}", err);
        exit(1);
    });

    new_pak.close_file_system();
}
