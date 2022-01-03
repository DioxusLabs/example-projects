use axum::{response::Html, routing::get, Router};
use dioxus::prelude::*;
use std::net::SocketAddr;

mod components {
    pub mod nav;
    pub mod product_page;
}

#[tokio::main]
async fn main() {
    // build our application with a route
    let app = Router::new()
        .route("/", get(root))
        .route("/app", get(dioxusapp));

    // run it
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("listening on http://{}", addr);
    println!("- Route available on http://{}", addr);
    println!("- Route available on http://{}/app", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

// Just render a simple page directly from the request
async fn root() -> Html<String> {
    Html(dioxus::ssr::render_lazy(rsx! {
        div {
            h1 { "hello world!" }
            p { "goodbye world!" }
            (0..10).map(|f| rsx!{
                p { "abc: {f}" }
            })
        }
    }))
}

/// Render a more sophisticated page with ssr
async fn dioxusapp() -> Html<String> {
    fn dioxusapp(cx: Scope) -> Element {
        cx.render(rsx!(
            head {
                link { rel: "stylesheet", href: "https://unpkg.com/tailwindcss@^2.0/dist/tailwind.min.css" }
            }
            body {
                div {
                    components::nav::nav()
                    components::product_page::product_page()
                }
            }
        ))
    }
    let mut app = VirtualDom::new(dioxusapp);
    let _ = app.rebuild();
    Html(dioxus::ssr::render_vdom(&app))
}
