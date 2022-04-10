#![windows_subsystem = "windows"]
extern crate native_windows_gui as nwg;
use dnpak::EtFileSystem;
use md5::{Digest, Md5};
use std::{env, fs, io::Read, path::Path, process::exit, rc::Rc};

fn main() {
    let mut args = env::args();
    args.next();

    let input_path = Path::new(&args.next().unwrap()).display().to_string();
    if !input_path.ends_with(".pak") {
        eprintln!("File must be ended with .pak");
        nwg::stop_thread_dispatch();
        exit(1);
    }

    nwg::init().expect("Failed to init Native Windows GUI");
    nwg::Font::set_global_family("Segoe UI").expect("Failed to set default font");

    let mut window = Default::default();
    let mut version = Default::default();
    let mut patch_button = Default::default();
    let layout = Default::default();

    nwg::Window::builder()
        .size((300, 115))
        .position((300, 300))
        .title("dnpak-patch")
        .build(&mut window)
        .unwrap();

    nwg::TextInput::builder()
        .focus(true)
        .parent(&window)
        .build(&mut version)
        .unwrap();

    nwg::Button::builder()
        .text("Patch")
        .parent(&window)
        .build(&mut patch_button)
        .unwrap();

    nwg::GridLayout::builder()
        .parent(&window)
        .spacing(1)
        .child(0, 0, &version)
        .child_item(nwg::GridLayoutItem::new(&patch_button, 0, 1, 1, 2))
        .build(&layout)
        .unwrap();

    let window = Rc::new(window);
    let events_window = window.clone();

    let handler = nwg::full_bind_event_handler(&window.handle, move |evt, _evt_data, handle| {
        use nwg::Event as E;

        match evt {
            E::OnWindowClose => {
                if &handle == &events_window as &nwg::Window {
                    nwg::stop_thread_dispatch();
                }
            }
            E::OnButtonClick => {
                if &handle == &patch_button {
                    let pak = EtFileSystem::read(&input_path);
                    let pad_version = format!("{:0>8}", version.text());

                    let mut contents = String::new();
                    for file in pak.get_files() {
                        let path = {
                            let mut tmp = format!("{}\r\n", &file.to_string()[1..]);

                            if tmp.starts_with("resource") || tmp.starts_with("mapdata") {
                                tmp = "D ".to_owned() + &tmp;
                            } else {
                                tmp = "C ".to_owned() + &tmp;
                            }

                            tmp
                        };

                        contents.push_str(&path);
                    }

                    // create dir
                    fs::create_dir(&pad_version).unwrap();

                    // copy the pak file
                    fs::copy(
                        &input_path,
                        format!("{}/Patch{}.pak", &pad_version, &pad_version),
                    )
                    .expect("Cannot copy the pak file");

                    // txt
                    fs::write(
                        format!("{}/Patch{}.txt", &pad_version, &pad_version),
                        contents,
                    )
                    .expect("Cannot write the txt file");

                    // md5
                    let mut buffer = Vec::new();
                    fs::File::open(&input_path)
                        .expect("Cannot open the pak file")
                        .read_to_end(&mut buffer)
                        .expect("Cannot read the pak file");

                    let digest = Md5::digest(&buffer);
                    fs::write(
                        format!("{}/Patch{}.pak.md5", &pad_version, &pad_version),
                        format!("{:x}\r\n", &digest),
                    )
                    .expect("Cannot write the md5 file");

                    nwg::modal_info_message(
                        &events_window.handle,
                        "Success",
                        &format!("Patched version {}", version.text()),
                    );
                    nwg::stop_thread_dispatch();
                }
            }
            _ => {}
        }
    });

    nwg::dispatch_thread_events();
    nwg::unbind_event_handler(&handler);
}
