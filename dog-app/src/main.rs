#![allow(non_snake_case)]

use dioxus::{html::input_data::keyboard_types::Key, prelude::*};
use dioxus_desktop::{Config, WindowBuilder};

use crate::models::{list_all_breeds, random_image_by_breed};
mod models;

fn main() {
    dioxus_desktop::launch_cfg(
        app,
        Config::default().with_window(
            WindowBuilder::new()
                .with_maximized(true)
                .with_title("Dog App"),
        ),
    )
}

fn app(cx: Scope) -> Element {
    let selected_breed = use_state::<Option<String>>(cx, || None);
    let search_input = use_state(cx, String::new);

    // Fetch the dog list when the component mounts
    let fut = use_future!(cx, |()| async move { list_all_breeds().await });

    render!(
        link {
            rel: "stylesheet",
            href: "https://unpkg.com/tailwindcss@^2.0/dist/tailwind.min.css"
        }
        div {
            nav { class: "bg-white shadow p-8 border-b border-gray-400 sticky top-0",
                div { class: "flex justify-center align-vertical",
                    input {
                        class: "p-4 w-full",
                        "type": "text",
                        value: "{search_input}",
                        placeholder: "Search for doggo",
                        oninput: move |evt| search_input.set(evt.value.clone()),
                        onkeydown: move |evt| { if evt.key() == Key::Enter {} }
                    }
                }
            }
            div { class: "px-2 flex",
                div { class: "grow w-full h-full",
                    if let Some(Ok(breeds)) = fut.value() {
                        let current_search = search_input.get();
                        rsx!{
                            breeds.message.iter().filter_map(|(breed, subbreeds)| {
                                if current_search.is_empty() || breed.contains(current_search) {
                                    let onclick = move |_| {
                                        to_owned![selected_breed, breed];
                                        cx.spawn(async move {
                                            if let Ok(image) = random_image_by_breed(&breed).await {
                                                selected_breed.set(Some(image.message));
                                            }
                                        })
                                    };

                                    Some(rsx! {
                                        Card {
                                            key: "{breed}",
                                            title: breed.to_string(),
                                            list: subbreeds.clone(),
                                            onclick: onclick,
                                        }
                                    })
                                } else {
                                    None
                                }
                            })
                        }
                    } else {
                        rsx!("No dogs 🐶😔")
                    }
                }
                if let Some(selected_breed) = selected_breed.get() {
                    rsx!(
                        div {
                            class: "w-1/2",
                            img {
                                src: "{selected_breed}"
                            }
                        }
                    )
                }
            }
        }
    )
}

#[allow(non_snake_case)]
#[inline_props]
fn Card<'a>(cx: Scope, title: String, list: Vec<String>, onclick: EventHandler<'a>) -> Element {
    render!(
        div {
            onclick: |_| onclick.call(()),
            class: "my-2 bg-gray-100 w-full rounded-sm p-2",
            h3 { class: "text-2xl", "{title}" }
            ul { class: "list-disc ml-8",
                {list.iter().map(|item| rsx!( li {
                    key: "{item}",
                    "{item}"
                }))}
            }
        }
    )
}
