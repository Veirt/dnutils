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
    let output_path = output.map(|path| Path::new(&path).display().to_string());

    pak.unpack_all(output_path, true).unwrap();

    pak.close_file_system();
}
