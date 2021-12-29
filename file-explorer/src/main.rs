//! Example: File Explorer
//! -------------------------
//!
//! This is a fun little desktop application that lets you explore the file system.
//!
//! This example is interesting because it's mixing filesystem operations and GUI, which is typically hard for UI to do.

use dioxus::prelude::*;

fn main() {
    // simple_logger::init_with_level(log::Level::Debug).unwrap();
    dioxus::desktop::launch_cfg(App, |c| {
        c.with_window(|w| {
            w.with_resizable(true).with_inner_size(
                dioxus::desktop::wry::application::dpi::LogicalSize::new(400.0, 800.0),
            )
        })
    });
}

static App: Component<()> = |cx| {
    let file_manager = use_ref(&cx, || Files::new());
    let files = file_manager.read();

    let file_list = files.path_names.iter().enumerate().map(|(dir_id, path)| {
        let path_end = path.split('/').last().unwrap_or(path.as_str());
        let icon_type = if path_end.contains(".") {
            "description"
        } else {
            "folder"
        };
        rsx! (
            div { class: "folder",
                key: "{path}"
                i { class: "material-icons",
                    "{icon_type}"
                    onclick: move |_| file_manager.write().enter_dir(dir_id),
                    p { class: "cooltip", "0 folders / 0 files" }
                }
                h1 { "{path_end}" }
            }
        )
    });

    let err_disp = files.err.as_ref().map(|err| {
        rsx! (
            div {
                code {"{err}"}
                button {"x", onclick: move |_| file_manager.write().clear_err() }
            }
        )
    });

    let current_dir = files.current();

    rsx!(cx, div {
        link { href:"https://fonts.googleapis.com/icon?family=Material+Icons" rel:"stylesheet" }
        style { {[include_str!("./style.css")]} }
        header {
            i { class: "material-icons icon-menu", "menu" }
            h1 { "Files: {current_dir}" }
            span { }
            i { class: "material-icons", "logout", onclick: move |_| file_manager.write().go_up() }
        }
        main {
            {file_list}
            {err_disp}
        }

    })
};

struct Files {
    path_stack: Vec<String>,
    path_names: Vec<String>,
    err: Option<String>,
}

impl Files {
    fn new() -> Self {
        let mut files = Self {
            path_stack: vec!["./".to_string()],
            path_names: vec![],
            err: None,
        };

        files.reload_path_list();

        files
    }

    fn reload_path_list(&mut self) {
        let cur_path = self.path_stack.last().unwrap();
        log::info!("Reloading path list for {:?}", cur_path);
        let paths = match std::fs::read_dir(cur_path) {
            Ok(e) => e,
            Err(err) => {
                let err = format!("An error occured: {:?}", err);
                self.err = Some(err);
                self.path_stack.pop();
                return;
            }
        };
        let collected = paths.collect::<Vec<_>>();
        log::info!("Path list reloaded {:#?}", collected);

        // clear the current state
        self.clear_err();
        self.path_names.clear();

        for path in collected {
            self.path_names
                .push(path.unwrap().path().display().to_string());
        }
        log::info!("path namees are {:#?}", self.path_names);
    }

    fn go_up(&mut self) {
        if self.path_stack.len() > 1 {
            self.path_stack.pop();
        }
        self.reload_path_list();
    }

    fn enter_dir(&mut self, dir_id: usize) {
        let path = &self.path_names[dir_id];
        self.path_stack.push(path.clone());
        self.reload_path_list();
    }

    fn current(&self) -> &str {
        self.path_stack.last().unwrap()
    }
    fn clear_err(&mut self) {
        self.err = None;
    }
}
