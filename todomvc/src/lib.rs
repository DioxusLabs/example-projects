use dioxus::{html::input_data::keyboard_types::Key, prelude::*};

#[derive(PartialEq)]
pub enum FilterState {
    All,
    Active,
    Completed,
}

pub type Todos = im_rc::HashMap<u32, TodoItem>;

#[derive(Debug, PartialEq, Clone)]
pub struct TodoItem {
    pub id: u32,
    pub checked: bool,
    pub contents: String,
}

pub fn app(cx: Scope<()>) -> Element {
    let todos = use_ref(cx, im_rc::HashMap::<u32, TodoItem>::default);
    let filter = use_state(cx, || FilterState::All);
    let draft = use_ref(cx, String::new);
    let todo_id = use_state(cx, || 0);

    // Filter the todos based on the filter state
    let mut filtered_todos = todos
        .read()
        .iter()
        .filter(|(_, item)| match filter.get() {
            FilterState::All => true,
            FilterState::Active => !item.checked,
            FilterState::Completed => item.checked,
        })
        .map(|f| *f.0)
        .collect::<Vec<_>>();
    filtered_todos.sort_unstable();

    let show_clear_completed = todos.read().values().any(|todo| todo.checked);
    let items_left = filtered_todos.len();
    let item_text = match items_left {
        1 => "item",
        _ => "items",
    };

    cx.render(rsx!{
        section { class: "todoapp",
            style { include_str!("../src/style.css") }
            div {
                header { class: "header",
                    h1 {"todos"}
                    input {
                        class: "new-todo",
                        placeholder: "What needs to be done?",
                        value: "{draft.read()}",
                        autofocus: "true",
                        oninput: move |evt| draft.set(evt.value.clone()),
                        onkeydown: move |evt| {
                            if evt.key() == Key::Enter && !draft.read().is_empty() {
                                todos.write().insert(
                                    *todo_id.get(),
                                    TodoItem {
                                        id: *todo_id.get(),
                                        checked: false,
                                        contents: draft.read().clone(),
                                    },
                                );
                                todo_id.set(todo_id + 1);
                                draft.set("".to_string());
                            }
                        }
                    }
                }
                ul { class: "todo-list",
                    filtered_todos.iter().map(|id| rsx!(todo_entry{ key: "{id}", id: *id, set_todos: todos }))
                }
                if !todos.read().is_empty() {
                    rsx! {
                        footer { class: "footer",
                            span { class: "todo-count",
                                strong {"{items_left} "}
                                span {"{item_text} left"}
                            }
                            ul { class: "filters",
                                li { class: "All", a { onclick: move |_| filter.set(FilterState::All), "All" }}
                                li { class: "Active", a { onclick: move |_| filter.set(FilterState::Active), "Active" }}
                                li { class: "Completed", a { onclick: move |_| filter.set(FilterState::Completed), "Completed" }}
                            }
                            if show_clear_completed {
                                rsx!(
                                    button {
                                        class: "clear-completed",
                                        onclick: move |_| todos.write().retain(|_, todo| !todo.checked),
                                        "Clear completed"
                                    }
                                )
                            }
                        }
                    }
                }
            }
        }
        footer { class: "info",
            p {"Double-click to edit a todo"}
            p { "Created by ", a {  href: "http://github.com/jkelleyrtp/", "jkelleyrtp" }}
            p { "Part of ", a { href: "http://todomvc.com", "TodoMVC" }}
        }
    })
}

#[derive(Props)]
pub struct TodoEntryProps<'a> {
    set_todos: &'a UseRef<Todos>,
    id: u32,
}

pub fn todo_entry<'a>(cx: Scope<'a, TodoEntryProps<'a>>) -> Element {
    let editing = use_state(cx, || false);

    let todos = cx.props.set_todos.read();
    let todo = &todos[&cx.props.id];
    let is_checked = todo.checked;
    let completed = if is_checked { "completed" } else { "" };
    let is_editing = (**editing).then_some("editing").unwrap_or_default();

    render!(li {
        class: "{completed} {is_editing}",
        onclick: move |_| {
            if !is_checked {
                editing.set(true)
            }
        },
        onfocusout: move |_| editing.set(false),
        div { class: "view",
            input { class: "toggle", r#type: "checkbox", id: "cbg-{todo.id}", checked: "{is_checked}",
                onchange: move |evt| {
                    cx.props.set_todos.write()[&cx.props.id].checked = evt.value.parse().unwrap();
                }
            }
            label { r#for: "cbg-{todo.id}", pointer_events: "none", "{todo.contents}" }
        }
        if **editing {
            rsx!{
                input {
                    class: "edit",
                    value: "{todo.contents}",
                    oninput: move |evt| cx.props.set_todos.write()[&cx.props.id].contents = evt.value.clone(),
                    autofocus: "true",
                    onkeydown: move |evt| {
                        match evt.key().to_string().as_str() {
                            "Enter" | "Escape" | "Tab" => editing.set(false),
                            _ => {}
                        }
                    },
                }
            }
        }
    })
}
