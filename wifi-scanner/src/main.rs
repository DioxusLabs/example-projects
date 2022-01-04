use dioxus::prelude::*;
use futures::StreamExt;
use futures_channel::mpsc::{unbounded, UnboundedReceiver};
use std::cell::Cell;
use std::time::Duration;
use wifiscanner::Wifi;

fn main() {
    let (sender, receiver) = unbounded();

    // launch our IO thread
    std::thread::spawn(move || {
        tokio::runtime::Builder::new_multi_thread()
            .enable_all()
            .build()
            .unwrap()
            .block_on(async move {
                loop {
                    tokio::time::sleep(Duration::from_secs(1)).await;
                    if let Ok(devices) = wifiscanner::scan() {
                        let _ = sender.unbounded_send(devices);
                    } else {
                        break;
                    }
                }
            });
    });

    // launch our app on the current thread - important because we spawn a window
    dioxus::desktop::launch_with_props(
        app,
        AppProps {
            receiver: Cell::new(Some(receiver)),
        },
        |c| c,
    )
}

struct AppProps {
    receiver: Cell<Option<UnboundedReceiver<Vec<Wifi>>>>,
}

fn app(cx: Scope<AppProps>) -> Element {
    let list = use_state(&cx, || vec![]);

    let scan = use_coroutine(&cx, || {
        let receiver = cx.props.receiver.take();
        let list = list.for_async();
        async move {
            if let Some(mut receiver) = receiver {
                while let Some(msg) = receiver.next().await {
                    list.set(msg);
                }
            }
        }
    });

    cx.render(rsx!(
        div {
            div { class: "py-8 px-6",
                div { class: "container px-4 mx-auto",
                    h2 { class: "text-2xl font-bold", "Scan for WiFi Networks" }
                    button {
                        class: "inline-block w-full md:w-auto px-6 py-3 font-medium text-white bg-indigo-500 hover:bg-indigo-600 rounded transition duration-200",
                        onclick: move |_| {
                            // todo: wire up the coroutine properly and add a loading state
                            scan.start();
                            list.set(wifiscanner::scan().unwrap());
                        },
                        "scan"
                    }
                }
            }

            section { class: "py-8",
                div { class: "container px-4 mx-auto",
                    div { class: "p-4 mb-6 bg-white shadow rounded overflow-x-auto",
                        table { class: "table-auto w-full",
                            thead {
                                tr { class: "text-xs text-gray-500 text-left",
                                    th { class: "pl-6 pb-3 font-medium", "Strength" }
                                    th { class: "pb-3 font-medium", "Network" }
                                    th { class: "pb-3 font-medium", "Channel" }
                                    th { class: "pb-3 font-medium", "Security" }
                                }
                            }
                            match list.len() {
                                0 => rsx!("No networks found. Try scanning"),
                                _ => {
                                    let mut sorted_wifis = list
                                        .iter()
                                        .map(|wif: &Wifi| (wif, wif.signal_level.parse::<f32>().unwrap()))
                                        .collect::<Vec<_>>();
                                    sorted_wifis.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap());

                                    rsx! {
                                        tbody {
                                            sorted_wifis.into_iter().rev().map(|(wi, _)|{
                                                let Wifi { mac, ssid, channel, signal_level, security } = wi;
                                                rsx!(
                                                    tr { class: "text-xs bg-gray-50",
                                                        td { class: "py-5 px-6 font-medium", "{signal_level}" }
                                                        td { class: "flex px-4 py-3",
                                                            div {
                                                                p { class: "font-medium", "{ssid}" }
                                                                p { class: "text-gray-500", "{mac}" }
                                                            }
                                                        }
                                                        td { span { class: "inline-block py-1 px-2 text-white bg-green-500 rounded-full", "{channel}" } }
                                                        td {  span { class: "inline-block py-1 px-2 text-purple-500 bg-purple-50 rounded-full", "{security}" } }
                                                    }
                                                )
                                            })
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    ))
}
