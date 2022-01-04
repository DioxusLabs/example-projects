//! Instagram for doggo breeds

use std::{collections::HashMap, iter::from_fn};

use dioxus::prelude::*;
use serde::{Deserialize, Serialize};
mod models;

fn main() {
    dioxus::desktop::launch_cfg(app, |c| c.with_window(|c| c.with_maximized(true)))
}

#[derive(Serialize, Deserialize)]
pub struct DogList {
    message: HashMap<String, Vec<String>>,
    status: String,
}

fn app(cx: Scope) -> Element {
    let search_input = use_state(&cx, || "".to_string());

    // when the component loads, we want to fetch the dog list
    let fut = use_future(&cx, || async move {
        reqwest::get("https://dog.ceo/api/breeds/list/all")
            .await
            .unwrap()
            .json::<DogList>()
            .await
            .unwrap()
    });

    cx.render(rsx! {
        // script { src: "https://cdn.tailwindcss.com" }
        link {
            rel: "stylesheet",
            href: "https://unpkg.com/tailwindcss@^2.0/dist/tailwind.min.css",
        }



        div { class: "insta-clone",

            page_nav(
                div {
                    input {
                        "type": "text",
                        value: "{search_input}",
                        placeholder: "Search for doggo",
                        oninput: move |evt| search_input.set(evt.value.clone()),
                        onkeydown: move |evt| {
                            if evt.key == "Enter" {
                                // search_for_dogs.start();
                            }
                        }
                    }
                }
            )
            div { class: "bg-gray-100 h-auto px-96 relative",
                user_profile()
                hr { class: "border-gray-500 mt-6"}
                hr { class: "border-gray-500 w-20 border-t-1 ml-64 border-gray-800"}

                if let Some(breeds) = fut.value() {
                    rsx!(cx, div {
                        ul {
                            {breeds.message.iter().map(|(breed, subbreeds)| rsx!(
                                li {
                                    key: "{breed}",
                                    "{breed}"
                                    ul {
                                        {subbreeds.iter().map(|subbreed| rsx!( li {
                                            key: "{subbreed}",
                                            "--- {subbreed}"
                                        }))}
                                    }
                                }
                            ))}
                        }
                    })
                } else {
                    rsx!(cx, "no dogs")
                }
            }
        }
    })
}

fn post_thumbnail(cx: Scope) -> Element {
    cx.render(rsx!(
        div { class: "flex-1 text-center px-4 py-2 m-2",
            img { class: "w-full",
                src: "https://images.unsplash.com/photo-1487530811176-3780de880c2d?ixlib=rb-1.2.1&ixid=eyJhcHBfaWQiOjEyMDd9&auto=format&fit=crop&w=800&q=80",
            }
        }
    ))
}

#[inline_props]
fn page_nav<'a>(cx: Scope<'a>, children: Element<'a>) -> Element {
    cx.render(rsx!(
        nav { class: "bg-white shadow px-48 border-b border-gray-400 sticky top-0 z-50",
            div { class: "max-w-7xl mx-auto px-2 sm:px-4 lg:px-8",
                div { class: "flex justify-between h-16",
                    platform_logos(),
                    children,
                    hamburger(),
                    div { class: "lg:ml-4 lg:flex lg:items-center",
                        profile_menu_items()
                        div { class: "ml-4 relative flex-shrink-0",
                            button { class: "flex rounded-full border-gray-700 transition duration-150 ease-in-out",
                                id: "user-menu",
                                "aria_label": "User menu",
                                aria_haspopup: "true",
                                img { class: "h-8 w-8 rounded-full",
                                    src: "https://lh3.googleusercontent.com/a-/AAuE7mADjk6Ww-zxpX1VxI6Q7f55LSUi1nYUWul2Gdxt=k-s256",
                                    alt: "",
                                }
                            }
                        }
                    }
                }
            }
        }
    ))
}

fn search_results(cx: Scope) -> Element {
    let results = (0..5).map(|f| rsx!(
        div {
            role: "none",
            a { class: "-qQT3",
                tabindex: "0",
                href: "/doubledownnews/",
                div { class: "qF0y9 Igw0E rBNOH eGOV_ ybXk5 _4EzTm XfCBB HVWg4",
                    aria_labelledby: "fa73a33391b0e2 f279cafbc6ce568 f2484eafbf70a86 f33432957a85d5c",
                    div { class: "qF0y9 Igw0E IwRSH eGOV_ _4EzTm yC0tu",
                        div { class: "RR-M-",
                            role: "button",
                            tabindex: "-1",
                            aria_disabled: "true",
                            canvas { class: "CfWVH",
                                width: "108",
                                style: "position: absolute; top: -5px; left: -5px; width: 54px; height: 54px;",
                                height: "108",
                            }
                            span { class: "_2dbep",
                                role: "link",
                                tabindex: "-1",
                                style: "width: 44px; height: 44px;",
                                img { class: "_6q-tv",
                                    draggable: "false",
                                    alt: "doubledownnews's profile picture",
                                    crossorigin: "anonymous",
                                    src: "https://scontent-atl3-1.cdninstagram.com/v/t51.2885-19/s150x150/36583374_1484432554995328_915325820757606400_n.jpg?_nc_ht=scontent-atl3-1.cdninstagram.com&amp;_nc_cat=104&amp;_nc_ohc=Twj2vEmxZ2IAX9ozgRb&amp;edm=AGeOuZUBAAAA&amp;ccb=7-4&amp;oh=00_AT-1fuiDaz7AUmYS_BHFxnNT5ng_LSIlf6G0zVqJqhZXNg&amp;oe=61CF1ED6&amp;_nc_sid=924bfa",
                                }
                            }
                        }
                    }
                    div { class: "qF0y9 Igw0E IwRSH YBx95 vwCYk",
                        div { class: "qF0y9 Igw0E IwRSH eGOV_ _4EzTm",
                            id: "f279cafbc6ce568",
                            div { class: "_7UhW9 xLCgt MMzan KV-D4 fDxYl",
                                div { class: "qF0y9 Igw0E rBNOH eGOV_ ybXk5 _4EzTm",
                                    div { class: "_7UhW9 xLCgt qyrsm KV-D4 uL8Hv",
                                        "doubledownnews"
                                    }
                                }
                            }
                        }
                        div { class: "qF0y9 Igw0E IwRSH eGOV_ _4EzTm DhRcB",
                            id: "f2484eafbf70a86",
                            div { class: "_7UhW9 xLCgt MMzan _0PwGv fDxYl",
                                "Double Down News"
                            }
                        }
                    }
                    div { class: "qF0y9 Igw0E rBNOH YBx95 ybXk5 _4EzTm soMvl",
                        id: "fa73a33391b0e2",
                        button { class: "wpO6b",
                            "type": "button",
                            div { class: "QBdPU",
                                svg { class: "_8-yf5",
                                    view_box: "0 0 24 24",
                                    height: "16",
                                    role: "img",
                                    fill: "#8e8e8e",
                                    width: "16",
                                    "aria_label": "Close",
                                    color: "#8e8e8e",
                                    polyline {
                                        fill: "none",
                                        points: "20.643 3.357 12 12 3.353 20.647",
                                        stroke: "currentColor",
                                        stroke_width: "3",
                                        stroke_linejoin: "round",
                                        stroke_linecap: "round",
                                    }
                                    line {
                                        stroke: "currentColor",
                                        stroke_linejoin: "round",
                                        fill: "none",
                                        y1: "20.649",
                                        y2: "3.354",
                                        x2: "3.354",
                                        stroke_width: "3",
                                        stroke_linecap: "round",
                                        x1: "20.649",
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    ));

    rsx!(cx,
        div { class: "",
            div { class: "",
                aria_hidden: "false",
                div { class: "",
                    style: "left: 187.5px;",
                }
                div { class: "",
                    div { class: "",
                        div { class: "",
                            h4 { class: "",
                                "Recent"
                            }
                            button { class: "",
                                "type": "button",
                                "Clear All"
                            }
                        }
                        ul { class: "",
                        }
                    }
                }
            }
        }
    )
}

fn user_profile(cx: Scope) -> Element {
    cx.render(rsx!(
        div { class: "flex md:flex-row-reverse flex-wrap",
            div { class: "w-full md:w-3/4 p-4 text-center",
                div { class: "text-left pl-4 pt-3",
                    span { class: "text-base text-gray-700 text-2xl mr-2", "HiraveSonali" }
                }
                div { class: "text-left pl-4 pt-3",
                    span { class: "text-base font-semibold text-gray-700 mr-2",
                        b { "220 " } "posts"
                    }
                    span { class: "text-base font-semibold text-gray-700 mr-2",
                        b { "114 " } "followers"
                    }
                    span { class: "text-base font-semibold text-gray-700",
                        b { "200 " } "following"
                    }
                }
                div { class: "text-left pl-4 pt-3",
                    span { class: "text-lg font-bold text-gray-700 mr-2",
                        "Sonali Hirave"
                    }
                }
                div { class: "text-left pl-4 pt-3",
                    p { class: "text-base font-medium text-blue-700 mr-2",
                        "#graphicsdesigner #traveller #reader #blogger #digitalmarketer"
                    }
                    p { class: "text-base font-medium text-gray-700 mr-2",
                        "https://www.behance.net/hiravesona7855"
                    }
                }
            }
            div { class: "w-full md:w-1/4 p-4 text-center",
                div { class: "w-full relative md:w-3/4 text-center mt-8",
                    button { class: "flex rounded-full",
                        id: "user-menu",
                        "aria_label": "User menu",
                        aria_haspopup: "true",
                        img { class: "h-40 w-40 rounded-full",
                            alt: "",
                            src: "https://lh3.googleusercontent.com/a-/AAuE7mADjk6Ww-zxpX1VxI6Q7f55LSUi1nYUWul2Gdxt=k-s256",
                        }
                    }
                }
            }
        }
    ))
}

fn platform_logos(cx: Scope) -> Element {
    rsx!(cx,
        div { class: "flex px-2 lg:px-0",
            div { class: "flex-shrink-0 flex items-center",
                img { class: "block lg:hidden h-8 w-auto",
                    src: "https://www.instagram.com/static/images/web/mobile_nav_type_logo.png/735145cfe0a4.png",
                    srcset: "https://www.instagram.com/static/images/web/mobile_nav_type_logo-2x.png/1b47f9d0e595.png",
                    alt: "Workflow logo",
                }
                img { class: "hidden lg:block h-8 w-auto",
                    src: "https://www.instagram.com/static/images/web/mobile_nav_type_logo.png/735145cfe0a4.png",
                    srcset: "https://www.instagram.com/static/images/web/mobile_nav_type_logo-2x.png/1b47f9d0e595.png",
                    alt: "Workflow logo",
                }
            }
        }
    )
}

fn hamburger(cx: Scope) -> Element {
    rsx!(cx,
        div { class: "flex items-center lg:hidden",
            button { class: "inline-flex items-center justify-center p-2 rounded-md text-gray-400 hover:text-gray-500 hover:bg-gray-100 focus:outline-none focus:bg-gray-100 focus:text-gray-500 transition duration-150 ease-in-out",
                aria_expanded: "false",
                "aria_label": "Main menu",
                svg { class: "block h-6 w-6",
                    stroke: "currentColor",
                    fill: "none",
                    view_box: "0 0 24 24",
                    path {
                        d: "M4 6h16M4 12h16M4 18h16",
                        stroke_linejoin: "round",
                        stroke_linecap: "round",
                        stroke_width: "2",
                    }
                }
                svg { class: "hidden h-6 w-6",
                    stroke: "currentColor",
                    fill: "none",
                    view_box: "0 0 24 24",
                    path {
                        d: "M6 18L18 6M6 6l12 12",
                        stroke_width: "2",
                        stroke_linecap: "round",
                        stroke_linejoin: "round",
                    }
                }
            }
        }
    )
}

fn profile_menu_items(cx: Scope) -> Element {
    rsx!(cx,
        button { class: "flex-shrink-0 p-1 border-transparent text-gray-700 rounded-full hover:text-gray-600 focus:outline-none focus:text-gray-600 transition duration-150 ease-in-out",
            "aria_label": "Notifications",
            svg { class: "h-6 w-6",
                stroke_linecap: "round",
                stroke_width: "1.5",
                fill: "none",
                stroke: "currentColor",
                view_box: "0 0 24 24",
                stroke_linejoin: "round",
                path {
                    d: "M3 12l2-2m0 0l7-7 7 7M5 10v10a1 1 0 001 1h3m10-11l2 2m-2-2v10a1 1 0 01-1 1h-3m-6 0a1 1 0 001-1v-4a1 1 0 011-1h2a1 1 0 011 1v4a1 1 0 001 1m-6 0h6",
                }
            }
        }
        button { class: "flex-shrink-0 p-1 border-transparent text-gray-700 rounded-full hover:text-gray-600 focus:outline-none focus:text-gray-600 transition duration-150 ease-in-out",
            "aria_label": "Notifications",
            svg { class: "h-6 w-6",
                fill: "none",
                stroke_linecap: "round",
                stroke_linejoin: "round",
                stroke_width: "1.5",
                stroke: "currentColor",
                view_box: "0 0 24 24",
                path {
                    d: "M16 15v-1a4 4 0 00-4-4H8m0 0l3 3m-3-3l3-3m9 14V5a2 2 0 00-2-2H6a2 2 0 00-2 2v16l4-2 4 2 4-2 4 2z",
                }
            }
        }
        button { class: "flex-shrink-0 p-1 border-transparent text-gray-700 rounded-full hover:text-gray-600 focus:outline-none focus:text-gray-600 transition duration-150 ease-in-out",
            "aria_label": "Notifications",
            svg { class: "h-6 w-6",
                stroke_linejoin: "round",
                fill: "none",
                view_box: "0 0 24 24",
                stroke_linecap: "round",
                stroke: "currentColor",
                stroke_width: "1.5",
                path {
                    d: "M18.364 5.636l-3.536 3.536m0 5.656l3.536 3.536M9.172 9.172L5.636 5.636m3.536 9.192l-3.536 3.536M21 12a9 9 0 11-18 0 9 9 0 0118 0zm-5 0a4 4 0 11-8 0 4 4 0 018 0z",
                }
            }
        }
        button { class: "flex-shrink-0 p-1 border-transparent text-gray-700 rounded-full hover:text-gray-600 focus:outline-none focus:text-gray-600 transition duration-150 ease-in-out",
            "aria_label": "Notifications",
            svg { class: "h-6 w-6",
                view_box: "0 0 24 24",
                stroke_linecap: "round",
                stroke_linejoin: "round",
                stroke_width: "1.5",
                stroke: "currentColor",
                fill: "none",
                path {
                    d: "M4.318 6.318a4.5 4.5 0 000 6.364L12 20.364l7.682-7.682a4.5 4.5 0 00-6.364-6.364L12 7.636l-1.318-1.318a4.5 4.5 0 00-6.364 0z",
                }
            }
        }
    )
}
