fn profile_navigator(cx: Scope) -> Element {
    cx.render(rsx!(
        div { class: "flex flex-row mt-4 justify-center mr-16",
            div { class: "flex text-gray-700 text-center py-2 m-2 pr-5",
                div { class: "flex inline-flex",
                    button { class: "border-transparent text-gray-800 rounded-full hover:text-blue-600 focus:outline-none focus:text-gray-600",
                        "aria_label": "Notifications",
                        svg { class: "h-6 w-6",
                            stroke_width: "2",
                            stroke_linejoin: "round",
                            stroke_linecap: "round",
                            view_box: "0 0 24 24",
                            stroke: "currentColor",
                            fill: "none",
                            path {
                                d: "M17 14v6m-3-3h6M6 10h2a2 2 0 002-2V6a2 2 0 00-2-2H6a2 2 0 00-2 2v2a2 2 0 002 2zm10 0h2a2 2 0 002-2V6a2 2 0 00-2-2h-2a2 2 0 00-2 2v2a2 2 0 002 2zM6 20h2a2 2 0 002-2v-2a2 2 0 00-2-2H6a2 2 0 00-2 2v2a2 2 0 002 2z",
                            }
                        }
                    }
                }
                div { class: "flex inline-flex ml-2 mt-1",
                    h3 { class: "text-sm font-bold text-gray-800 mr-2",
                        "POSTS"
                    }
                }
            }
            div { class: "flex text-gray-700 text-center py-2 m-2 pr-5",
                div { class: "flex inline-flex",
                    button { class: "border-transparent text-gray-600 rounded-full hover:text-blue-600 focus:outline-none focus:text-gray-600",
                        aria_label: "Notifications",
                        svg { class: "h-6 w-6",
                            stroke_linecap: "round",
                            stroke: "currentColor",
                            view_box: "0 0 24 24",
                            fill: "none",
                            stroke_width: "2",
                            stroke_linejoin: "round",
                            path {
                                d: "M9.75 17L9 20l-1 1h8l-1-1-.75-3M3 13h18M5 17h14a2 2 0 002-2V5a2 2 0 00-2-2H5a2 2 0 00-2 2v10a2 2 0 002 2z",
                            }
                        }
                    }
                }
                div { class: "flex inline-flex ml-2 mt-1",
                    h3 { class: "text-sm font-medium text-gray-700 mr-2",
                        "IGTV"
                    }
                }
            }
            div { class: "flex text-gray-700 text-center py-2 m-2 pr-5",
                div { class: "flex inline-flex",
                    button { class: "border-transparent text-gray-600 rounded-full hover:text-blue-600 focus:outline-none focus:text-gray-600",
                        "aria_label": "Notifications",
                        svg { class: "h-6 w-6",
                            fill: "none",
                            stroke_width: "2",
                            view_box: "0 0 24 24",
                            stroke_linejoin: "round",
                            stroke: "currentColor",
                            stroke_linecap: "round",
                            path {
                                d: "M5 5a2 2 0 012-2h10a2 2 0 012 2v16l-7-3.5L5 21V5z",
                            }
                        }
                    }
                }
                div { class: "flex inline-flex ml-2 mt-1",
                    h3 { class: "text-sm font-medium text-gray-700 mr-2",
                        "SAVED"
                    }
                }
            }
            div { class: "flex text-gray-700 text-center py-2 m-2 pr-5",
                div { class: "flex inline-flex",
                    button { class: "border-transparent text-gray-600 rounded-full hover:text-blue-600 focus:outline-none focus:text-gray-600",
                        aria_label: "Notifications",
                        svg { class: "h-6 w-6",
                            stroke_width: "2",
                            stroke_linecap: "round",
                            stroke_linejoin: "round",
                            view_box: "0 0 24 24",
                            fill: "none",
                            stroke: "currentColor",
                            path {
                                d: "M18 9v3m0 0v3m0-3h3m-3 0h-3m-2-5a4 4 0 11-8 0 4 4 0 018 0zM3 20a6 6 0 0112 0v1H3v-1z",
                            }
                        }
                    }
                }
                div { class: "flex inline-flex ml-2 mt-1",
                    h3 { class: "text-sm font-medium text-gray-700 mr-2",
                        "TAGGED"
                    }
                }
            }
        }
    ))
}

fn user_stories(cx: Scope) -> Element {
    cx.render(rsx!(
        div { class: "inline-flex ml-36 mt-16",
            div { class: "flex-1 text-center px-4 py-2 m-2",
                div { class: "relative shadow-xl mx-auto h-24 w-24 -my-12 border-white rounded-full overflow-hidden border-4",
                    img { class: "object-cover w-full h-full",
                        src: "https://images.unsplash.com/photo-1502164980785-f8aa41d53611?ixlib=rb-1.2.1&ixid=eyJhcHBfaWQiOjEyMDd9&auto=format&fit=crop&w=967&q=80",
                    }
                }
                h1 { class: "pt-16 text-base font-semibold text-gray-900",
                    "Fun"
                }
            }
            div { class: "flex-1 text-center px-4 py-2 m-2",
                div { class: "relative shadow-xl mx-auto h-24 w-24 -my-12 border-white rounded-full overflow-hidden border-4",
                    img { class: "object-cover w-full h-full",
                        src: "https://images.unsplash.com/photo-1456415333674-42b11b9f5b7b?ixlib=rb-1.2.1&ixid=eyJhcHBfaWQiOjEyMDd9&auto=format&fit=crop&w=967&q=80",
                    }
                }
                h1 { class: "pt-16 text-base font-semibold text-gray-900",
                    "Travel"
                }
            }
            div { class: "flex-1 text-center px-4 py-2 m-2",
                div { class: "relative shadow-xl mx-auto h-24 w-24 -my-12 border-white rounded-full overflow-hidden border-4",
                    img { class: "object-cover w-full h-full",
                        src: "https://images.unsplash.com/photo-1494972308805-463bc619d34e?ixlib=rb-1.2.1&ixid=eyJhcHBfaWQiOjEyMDd9&auto=format&fit=crop&w=1052&q=80",
                    }
                }
                h1 { class: "pt-16 text-base font-semibold text-gray-900",
                    "Food"
                }
            }
            div { class: "flex-1 text-center px-4 py-2 m-2",
                div { class: "relative shadow-xl mx-auto h-24 w-24 -my-12 border-white rounded-full overflow-hidden border-4",
                    img { class: "object-cover w-full h-full",
                        src: "https://images.unsplash.com/photo-1516834474-48c0abc2a902?ixlib=rb-1.2.1&ixid=eyJhcHBfaWQiOjEyMDd9&auto=format&fit=crop&w=1053&q=80",
                    }
                }
                h1 { class: "pt-16 text-base font-semibold text-gray-900",
                    "Sketch"
                }
            }
            div { class: "flex-1 text-center px-4 py-2 m-2",
                div { class: "relative shadow-xl mx-auto h-24 w-24 -my-12 border-white rounded-full overflow-hidden border-4",
                    img { class: "object-cover w-full h-full",
                        src: "https://images.unsplash.com/photo-1444021465936-c6ca81d39b84?ixlib=rb-1.2.1&ixid=eyJhcHBfaWQiOjEyMDd9&auto=format&fit=crop&w=800&q=80",
                    }
                }
                h1 { class: "pt-16 text-base font-semibold text-gray-900",
                    "My Work"
                }
            }
        }
    ))
}
