/* A simple project to inspire e show how to use Framework Dioxus and query OpenAI to gerenate images.*/
use dioxus::prelude::*;
use serde::{Deserialize, Serialize};
use serde_json::json;

#[tokio::main]
async fn main() {
    dioxus::desktop::launch(app)
}

fn app(cx: Scope) -> Element {
    let api = use_state(&cx, || "".to_string());
    let prompt = use_state(&cx, || "".to_string());
    let n_image = use_state(&cx, || 1.to_string());
    let image = use_state(&cx, || ImageResponse {
        created: 0,
        data: Vec::new(),
    });
    let loading = use_state(&cx, || "".to_string());
    
    cx.render(rsx! {
        head {
            link {
                rel: "stylesheet",
                href: "https://unpkg.com/bulma@0.9.0/css/bulma.min.css",
            }
        }
        div { class: "container",
        div { class: "columns",
            div { class: "column",
                input { class: "input is-primary mt-4",
                value:"{api}",
                    r#type: "text",
                    placeholder: "API",
                    oninput: move |evt| {
                        api.set(evt.value.clone());
                    },
                }

                input { class: "input is-primary mt-4",
                    placeholder: "MAX 1000 Dgts",
                    r#type: "text",
                    value:"{prompt}",
                    oninput: move |evt| {
                        prompt.set(evt.value.clone());
                    },
                }

                input { class: "input is-primary mt-4",
                    r#type: "number",
                    min:"1",
                     max:"10",
                    value:"{n_image}",
                    oninput: move |evt| {
                        n_image.set(evt.value.clone());
                    },
                }
            }
        }

        button { class: "button is-primary {loading}",

        onclick: move |_| cx.spawn({
            
            let loading =   loading.clone();
            loading.set("is-loading".to_string());
            let  image_request = image.clone();
            let  api_request = api.clone();
            let  prompt_request = prompt.clone();
            let  n_image_request = n_image.clone();
            async move {
                image_request.set(request(api_request.current().to_string(), prompt_request.current().to_string(), n_image_request.current().to_string()).await);
                loading.set("".to_string());
            }
        }),
            "Generate image"
        }
        br {
        }
    }
    image.data.iter().map(|image| {
            rsx!(
                section { class: "is-flex",
            div { class: "container is-fluid",
                div { class: "container has-text-centered",
                    div { class: "is-justify-content-center",
                        div { class: "level",
                            div { class: "level-item",
                                figure { class: "image",
                                    img {
                                        alt: "",
                                        src: "{image.url}",
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
            )
        })
    })
}
async fn request(api: String, prompt: String, n_image: String) -> ImageResponse {
    let client = reqwest::Client::new();
    let body = json!({
        "prompt":  prompt,
        "n":n_image.parse::<i32>().unwrap_or(1),
        "size":"1024x1024",
    });

    let mut authorization = "Bearer ".to_string();
    authorization.push_str(&api);

    let res = client
        .post("https://api.openai.com/v1/images/generations")
        .body(body.to_string())
        .header("Content-Type", "application/json")
        .header("Authorization", authorization)
        .send()
        .await
        .unwrap()
        .text()
        .await
        .unwrap();
    let deserialized: ImageResponse = serde_json::from_str(&res).unwrap();
    deserialized
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Props, Clone)]
struct UrlImage {
    url: String,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Props, Clone)]
struct ImageResponse {
    created: i32,
    data: Vec<UrlImage>,
}
