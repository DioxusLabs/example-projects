#![allow(non_snake_case)]

use dioxus::prelude::*;
use todomvc::*;

fn main() {
    console_error_panic_hook::set_once();
    dioxus::web::launch(APP)
}

pub static APP: Component<()> = |cx| {
    // Share our TodoList to the todos themselves
    use_context_provider(&cx, || Todos::new());

    // Save state for the draft, filter
    let (draft, set_draft) = use_state(&cx, || "".to_string());
    let (filter, set_filter) = use_state(&cx, || FilterState::All);
    let (todo_id, set_todo_id) = use_state(&cx, || 0);

    // Consume the todos
    let todos = use_context::<Todos>(&cx)?;

    // Filter the todos based on the filter state
    let mut filtered_todos = todos
        .read()
        .iter()
        .filter(|(_, item)| match *filter {
            FilterState::All => true,
            FilterState::Active => !item.checked,
            FilterState::Completed => item.checked,
        })
        .map(|f| *f.0)
        .collect::<Vec<_>>();
    filtered_todos.sort_unstable();

    // Define the actions to manage the todolist
    let submit_todo = move || {
        if !draft.is_empty() {
            todos.write().insert(
                *todo_id,
                TodoItem {
                    id: *todo_id,
                    checked: false,
                    contents: draft.to_string(),
                },
            );
            set_todo_id(todo_id + 1);
            set_draft("".to_string());
        }
    };
    let clear_completed = move || {
        todos.write().retain(|_, todo| todo.checked == false);
    };

    // Some assists in actually rendering the content
    let show_clear_completed = todos.read().values().any(|todo| todo.checked);
    let items_left = filtered_todos.len();
    let item_text = match items_left {
        1 => "item",
        _ => "items",
    };

    cx.render(rsx!{
        section { class: "todoapp",
            style { [include_str!("../src/style.css")] }
            div {
                header { class: "header",
                    h1 {"todos"}
                    input {
                        class: "new-todo",
                        placeholder: "What needs to be done?",
                        value: "{draft}",
                        autofocus: "true",
                        oninput: move |evt| set_draft(evt.value.clone()),
                        onkeydown: move |evt| {
                            if evt.key == "Enter" {
                                submit_todo();
                            }
                        }
                    }
                }
                ul { class: "todo-list",
                    filtered_todos.iter().map(|id| rsx!(TodoEntry { key: "{id}", id: *id }))
                }
                {(!todos.read().is_empty()).then(|| rsx!(
                    footer { class: "footer",
                        span { class: "todo-count", strong {"{items_left} "} span {"{item_text} left"} }
                        ul { class: "filters",
                            li { class: "All", a { onclick: move |_| set_filter(FilterState::All), "All" }}
                            li { class: "Active", a { onclick: move |_| set_filter(FilterState::Active), "Active" }}
                            li { class: "Completed", a { onclick: move |_| set_filter(FilterState::Completed), "Completed" }}
                        }
                        {(show_clear_completed).then(|| rsx!(
                            button { class: "clear-completed", onclick: move |_| clear_completed(),
                                "Clear completed"
                            }
                        ))}
                    }
                ))}
            }
        }
        footer { class: "info",
            p {"Double-click to edit a todo"}
            p { "Created by ", a { href: "http://github.com/jkelleyrtp/", "jkelleyrtp" }}
            p { "Part of ", a { href: "http://todomvc.com", "TodoMVC" }}
        }
    })
};

#[derive(PartialEq, Props)]
pub struct TodoEntryProps {
    id: u32,
}

pub fn TodoEntry(cx: Scope<TodoEntryProps>) -> Element {
    let todos = use_context::<Todos>(&cx)?;

    let _todos = todos.read();
    let todo = _todos.get(&cx.props.id)?;

    let (is_editing, _set_is_editing) = use_state(&cx, || false);
    let completed = if todo.checked { "completed" } else { "" };

    cx.render(rsx!{
        li { class: "{completed}",
            div { class: "view",
                input { class: "toggle", r#type: "checkbox", id: "cbg-{todo.id}", checked: "{todo.checked}",
                    onchange: move |evt| {
                        todos.write().get_mut(&cx.props.id).map(|todo| todo.checked = evt.value.parse().unwrap());
                    }
                }

                label { r#for: "cbg-{todo.id}", pointer_events: "none",
                    "{todo.contents}"
                }

               {is_editing.then(|| rsx!{
                    input { value: "{todo.contents}",
                        oninput: move |evt| {
                            todos.write().get_mut(&cx.props.id).map(|todo| todo.contents = evt.value.clone());
                        },
                    }
                })}
            }
        }
    })
}
