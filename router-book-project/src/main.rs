use dioxus::{
    prelude::*,
    router::{use_route, Link, Redirect, Route, Router},
};

fn main() {
    dioxus::web::launch(app);
}

fn app(cx: Scope) -> Element {
    cx.render(rsx! {
        Router {
            self::navbar {}
            Route { to: "/", self::homepage {}}
            Route {
                to: "/blog",
                p { "-- Dioxus Blog --" }
                Route { to: "/:post", self::blog_post {} }
            }
            Route { to: "", self::page_not_found {}}
        }
    })
}

fn navbar(cx: Scope) -> Element {
    cx.render(rsx! {
        ul {
            Link { to: "/", "Home"}
            br {}
            Link { to: "/blog", "Blog"}
        }
    })
}

fn homepage(cx: Scope) -> Element {
    cx.render(rsx! {
        p { "Welcome to Dioxus Blog!" }
    })
}

fn secret_page(cx: Scope) -> Element {
    cx.render(rsx! {
        p { "This page is not to be viewed!" }
        Redirect { to: "/" }
    })
}

fn blog_post(cx: Scope) -> Element {
    let route = use_route(&cx);
    let blog_text = match route.segment("post") {
        Some(val) => get_blog_post(val),
        None => "An unknown error occured".to_string(),
    };

    cx.render(rsx! {
        p { "{blog_text}" }
    })
}

fn get_blog_post(id: &str) -> String {
    match id {
        "foo" => "Welcome to the foo blog post!".to_string(),
        "bar" => "This is the bar blog post!".to_string(),
        id => format!("Blog post '{id}' does not exist!"),
    }
}

fn page_not_found(cx: Scope) -> Element {
    cx.render(rsx! {
        p { "Oops! The page you are looking for doesn't exist!" }
    })
}
