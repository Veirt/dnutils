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

    let mut pak = EtFileSystem::read(&input_path);

    let output = args.next();
    match output {
        Some(path) => {
            let output_path = Path::new(&path).display().to_string();

            pak.unpack(Some(output_path)).unwrap();
        }
        None => {
            pak.unpack(None).unwrap();
        }
    }

    pak.close_file_system();
}
