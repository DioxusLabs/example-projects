#![allow(non_snake_case)]

use dioxus::prelude::*;
use log::LevelFilter;

fn main() {
    // Init debug
    dioxus_logger::init(LevelFilter::Info).expect("failed to init logger");

    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    // Build cool things ✌️
    let mut files = use_signal(|| Files::new());

    rsx! {
        div {
            link { href: "https://fonts.googleapis.com/icon?family=Material+Icons", rel: "stylesheet" }
            link { rel: "stylesheet", href: "main.css" }
            header {
                i { class: "material-icons icon-menu", "menu" }
                h1 { "Files: " "{files.read().current()}" }
                span { }
                i { class: "material-icons", onclick: move |_| files.write().go_up(), "logout" }
            }
            main {
                for (dir_id, path) in files.read().path_names.iter().enumerate() {
                    div {
                        class: "folder", key: "{path.name}",
                        i { class: "material-icons",
                            onclick: move |_| files.write().enter_dir(dir_id),
                            // Change the icon
                            if path.is_directory { "folder" } else { "description" }
                            // The tooltip
                            p { class: "cooltip", "0 folders / 0 files" }
                        }
                        h1 { "{path.name}" }
                    }
                }
                {
                    files.read().err.as_ref().map(|err| {
                        rsx! (
                            div {
                                code { "{err}" }
                                button { onclick: move |_| files.write().clear_err(), "x" }
                            }
                        )
                    })
                }
            }
        }
    }
}


#[derive(Debug)]
struct File {
    is_directory: bool,
    name: String,
}

struct Files {
    path_stack: Vec<String>,
    path_names: Vec<File>,
    err: Option<String>,
}

impl Files {
    fn new() -> Self {
        let mut files = Self {
            path_stack: vec![".".to_string()],
            path_names: vec![],
            err: None,
        };

        files.reload_path_list();

        files
    }

    fn reload_path_list(&mut self) {
        let cur_path = self.path_stack.join("/");
        log::info!("Reloading path list for {:?}", cur_path);
        let paths = match std::fs::read_dir(&cur_path) {
            Ok(e) => e,
            Err(err) => { // Likely we're trying to open a file, so let's open it!
                if let Ok(_) = open::that(cur_path) {
                    log::info!("Opened file");
                    return;
                } else {
                    let err = format!("An error occurred: {:?}", err);
                    self.err = Some(err);
                    self.path_stack.pop();
                    return;
                }
            }
        };

        let collected = paths.collect::<Vec<_>>();
        log::info!("Path list reloaded {:#?}", collected);

        // clear the current state
        self.clear_err();
        self.path_names.clear();

        for path in collected {
            let file = path.unwrap();
            self.path_names
                .push(
                    File {
                        name: file.file_name().to_str().unwrap().to_string(),
                        is_directory: file.file_type().unwrap().is_dir(),
                    }
                );
        }
        log::info!("path names are {:#?}", self.path_names);
    }

    fn go_up(&mut self) {
        if self.path_stack.len() > 1 {
            self.path_stack.pop();
        }
        self.reload_path_list();
    }

    fn enter_dir(&mut self, dir_id: usize) {
        let path = &self.path_names[dir_id];
        self.path_stack.push(path.name.to_string());
        println!("{:#?}", self.path_stack);
        self.reload_path_list();
    }

    fn current(&self) -> String {
        self.path_stack.join("/")
    }

    fn clear_err(&mut self) {
        self.err = None;
    }
}
