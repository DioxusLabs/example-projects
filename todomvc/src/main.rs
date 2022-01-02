//! trunk only lets main.rs, not any binary
//!
//!
use todomvc::*;

pub fn main() {
    dioxus::web::launch(app);
}
