use dioxus::prelude::*;

mod components {
    pub mod nav;
    pub mod product_page;
}

fn main() {
    dioxus::desktop::launch(app);
}

fn app(cx: Scope) -> Element {
    cx.render(rsx!(
        div {
            components::nav::nav()
            components::product_page::product_page()
        }
    ))
}
