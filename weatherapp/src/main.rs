use dioxus::prelude::*;

fn main() {
    dioxus_web::launch(app);
}

fn app(cx: Scope) -> Element {
    cx.render(rsx!(
        link { rel: "stylesheet", href: "https://unpkg.com/tailwindcss@^2.0/dist/tailwind.min.css" },
        div { class: "mx-auto p-4 bg-gray-100 h-screen flex justify-center",
            div { class: "flex items-center justify-center flex-col",
                search_box{}
                div { class: "flex flex-wrap w-full px-2",
                    div { class: "bg-gray-900 text-white relative min-w-0 break-words rounded-lg overflow-hidden shadow-sm mb-4 w-full bg-white dark:bg-gray-600",
                        div { class: "px-6 py-6 relative",
                            country_info{}
                            country_data{}
                        }
                        week_weather{}
                    }
                }
            }
        }
    ))
}

fn country_data(cx: Scope) -> Element {
    cx.render(rsx!(
        div { class: "block sm:flex justify-between items-center flex-wrap",
            div { class: "w-full sm:w-1/2",
                div { class: "flex mb-2 justify-between items-center",
                    span { "Temp" }
                    small { class: "px-2 inline-block", "39.11°" }
                }
            }
            div { class: "w-full sm:w-1/2",
                div { class: "flex mb-2 justify-between items-center",
                    span { "Feels like" }
                    small { class: "px-2 inline-block", "33.33°" }
                }
            }
            div { class: "w-full sm:w-1/2",
                div { class: "flex mb-2 justify-between items-center",
                    span { "Temp min" }
                    small { class: "px-2 inline-block", "24.9°" }
                }
            }
            div { class: "w-full sm:w-1/2",
                div { class: "flex mb-2 justify-between items-center",
                    span { "Temp max" }
                    small { class: "px-2 inline-block", "39°" }
                }
            }
        }
    ))
}

fn country_info(cx: Scope) -> Element {
    cx.render(rsx!(
        div { class: "flex mb-4 justify-between items-center",
            div {
                h5 { class: "mb-0 font-medium text-xl",
                    "Delhi,IN"
                }
                h6 { class: "mb-0",
                    "April 04 2021"
                }
                small {
                    "Cloudy"
                }
            }
            div { class: "text-right",
                h3 { class: "font-bold text-4xl mb-0",
                    span {
                        "39°"
                    }
                }
            }
        }
    ))
}

fn week_weather(cx: Scope) -> Element {
    cx.render(rsx!(
        div { class: "divider table mx-2 text-center bg-transparent whitespace-nowrap",
            span { class: "inline-block px-3", small { "Forecast" } }
        }
        div { class: "px-6 py-6 relative",
            div { class: "text-center justify-between items-center flex",
                style: "flex-flow: initial;",
                div { class: "text-center mb-0 flex items-center justify-center flex-col",
                    span { class: "block my-1",
                        "Sun"
                    }
                    img { class: "block w-8 h-8",
                        src: "https://i.imgur.com/ffgW9JQ.png",
                    }
                    span { class: "block my-1",
                        "38.3°"
                    }
                }
                div { class: "text-center mb-0 flex items-center justify-center flex-col",
                    span { class: "block my-1",
                        "Mon"
                    }
                    img { class: "block w-8 h-8",
                        src: "https://i.imgur.com/BQbzoKt.png",
                    }
                    span { class: "block my-1",
                        "39.9°"
                    }
                }
                div { class: "text-center mb-0 flex items-center justify-center flex-col",
                    span { class: "block my-1",
                        "Mon"
                    }
                    img { class: "block w-8 h-8",
                        src: "https://i.imgur.com/BQbzoKt.png",
                    }
                    span { class: "block my-1",
                        "40.1°"
                    }
                }
                div { class: "text-center mb-0 flex items-center justify-center flex-col",
                    span { class: "block my-1",
                        "Mon"
                    }
                    img { class: "block w-8 h-8",
                        src: "https://i.imgur.com/ffgW9JQ.png",
                    }
                    span { class: "block my-1",
                        "41.5°"
                    }
                }
                div { class: "text-center mb-0 flex items-center justify-center flex-col",
                    span { class: "block my-1",
                        "Mon"
                    }
                    img { class: "block w-8 h-8",
                        src: "https://i.imgur.com/ffgW9JQ.png",
                    }
                    span { class: "block my-1",
                        "40.1°"
                    }
                }
                div { class: "text-center mb-0 flex items-center justify-center flex-col",
                    span { class: "block my-1",
                        "Mon"
                    }
                    img { class: "block w-8 h-8",
                        src: "https://i.imgur.com/BQbzoKt.png",
                    }
                    span { class: "block my-1",
                        "38°"
                    }
                }
            }
        }
    ))
}

fn search_box(cx: Scope) -> Element {
    cx.render(rsx!(
        div { 
            div { class: "inline-flex flex-col justify-center relative text-gray-500",
                div { class: "relative",
                    input { class: "p-2 pl-8 rounded border border-gray-200 bg-gray-200 focus:bg-white focus:outline-none focus:ring-2 focus:ring-yellow-600 focus:border-transparent",
                        placeholder: "search...",
                        "type": "text",
                        value: "Gar",
                    }
                    svg { class: "w-4 h-4 absolute left-2.5 top-3.5",
                        "viewBox": "0 0 24 24",
                        fill: "none",
                        stroke: "currentColor",
                        xmlns: "http://www.w3.org/2000/svg",
                        path { 
                            d: "M21 21l-6-6m2-5a7 7 0 11-14 0 7 7 0 0114 0z",
                            "stroke-linejoin": "round",
                            "stroke-linecap": "round",
                            "stroke-width": "2",
                        }
                    }
                }
                h3 { class: "mt-2 text-sm",
                    "Gevonden:"
                }
                ul { class: "bg-white border border-gray-100 w-full mt-2",
                    {(0..3).map(|f| rsx!{
                        li { class: "pl-8 pr-2 py-1 border-b-2 border-gray-100 relative cursor-pointer hover:bg-yellow-50 hover:text-gray-900",
                            svg { class: "stroke-current absolute w-4 h-4 left-2 top-2",
                                stroke: "currentColor",
                                xmlns: "http://www.w3.org/2000/svg",
                                "viewBox": "0 0 24 24",
                                fill: "none",
                                path { 
                                    "stroke-linejoin": "round",
                                    "stroke-width": "2",
                                    "stroke-linecap": "round",
                                    d: "M17.657 16.657L13.414 20.9a1.998 1.998 0 01-2.827 0l-4.244-4.243a8 8 0 1111.314 0z",
                                }
                                path { 
                                    "stroke-linecap": "round",
                                    "stroke-linejoin": "round",
                                    d: "M15 11a3 3 0 11-6 0 3 3 0 016 0z",
                                    "stroke-width": "2",
                                }
                            }
                            b { 
                                "Gar"
                            }
                            "dameer - Italië"
                        }                        
                    })}
                }
            }
        }

      
        // div { class: "flex border-2 rounded m-4",
        //     input { class: "px-4 py-2 w-80",
        //         "type": "text",
        //         placeholder: "Search...",
        //     }
        //     button { class: "flex items-center justify-center px-4 border-l bg-white",
        //         svg { class: "w-6 h-6 text-gray-600",
        //             "viewBox": "0 0 24 24",
        //             xmlns: "http://www.w3.org/2000/svg",
        //             fill: "currentColor",
        //             path {
        //                 d: "M16.32 14.9l5.39 5.4a1 1 0 0 1-1.42 1.4l-5.38-5.38a8 8 0 1 1 1.41-1.41zM10 16a6 6 0 1 0 0-12 6 6 0 0 0 0 12z",
        //             }
        //         }
        //     }
        // }
    ))
}


                    // li { class: "pl-8 pr-2 py-1 border-b-2 border-gray-100 relative cursor-pointer hover:bg-yellow-50 hover:text-gray-900",
                    //     svg { class: "stroke-current absolute w-4 h-4 left-2 top-2",
                    //         fill: "none",
                    //         stroke: "currentColor",
                    //         xmlns: "http://www.w3.org/2000/svg",
                    //         viewBox: "0 0 24 24",
                    //         path { 
                    //             stroke-linecap: "round",
                    //             stroke-width: "2",
                    //             stroke-linejoin: "round",
                    //             d: "M17.657 16.657L13.414 20.9a1.998 1.998 0 01-2.827 0l-4.244-4.243a8 8 0 1111.314 0z",
                    //         }
                    //         path { 
                    //             d: "M15 11a3 3 0 11-6 0 3 3 0 016 0z",
                    //             stroke-width: "2",
                    //             stroke-linejoin: "round",
                    //             stroke-linecap: "round",
                    //         }
                    //     }
                    //     b { 
                    //         "Gar"
                    //     }
                    //     "da - Veneto - Italië"
                    // }
                    // li { class: "pl-8 pr-2 py-1 border-b-2 border-gray-100 relative cursor-pointer hover:bg-yellow-50 hover:text-gray-900",
                    //     svg { class: "stroke-current absolute w-4 h-4 left-2 top-2",
                    //         stroke: "currentColor",
                    //         fill: "none",
                    //         xmlns: "http://www.w3.org/2000/svg",
                    //         viewBox: "0 0 24 24",
                    //         path { 
                    //             stroke-width: "2",
                    //             stroke-linejoin: "round",
                    //             d: "M3 12l2-2m0 0l7-7 7 7M5 10v10a1 1 0 001 1h3m10-11l2 2m-2-2v10a1 1 0 01-1 1h-3m-6 0a1 1 0 001-1v-4a1 1 0 011-1h2a1 1 0 011 1v4a1 1 0 001 1m-6 0h6",
                    //             stroke-linecap: "round",
                    //         }
                    //     }
                    //     b { 
                    //         "Gar"
                    //     }
                    //     "da Hotel - Italië"
                    // }
                    // li { class: "pl-8 pr-2 py-1 border-gray-100 relative cursor-pointer hover:bg-yellow-50 hover:text-gray-900",
                    //     svg { class: "stroke-current absolute w-4 h-4 left-2 top-2",
                    //         fill: "none",
                    //         xmlns: "http://www.w3.org/2000/svg",
                    //         viewBox: "0 0 24 24",
                    //         stroke: "currentColor",
                    //         path { 
                    //             stroke-linecap: "round",
                    //             stroke-linejoin: "round",
                    //             stroke-width: "2",
                    //             d: "M3 12l2-2m0 0l7-7 7 7M5 10v10a1 1 0 001 1h3m10-11l2 2m-2-2v10a1 1 0 01-1 1h-3m-6 0a1 1 0 001-1v-4a1 1 0 011-1h2a1 1 0 011 1v4a1 1 0 001 1m-6 0h6",
                    //         }
                    //     }
                    //     b { 
                    //         "Gar"
                    //     }
                    //     "dena Resort - Italië"
                    // }

