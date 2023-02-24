use dioxus::prelude::*;

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
    let todos = use_state(&cx, im_rc::HashMap::<u32, TodoItem>::default);
    let filter = use_state(&cx, || FilterState::All);
    let draft = use_state(&cx, || "".to_string());
    let todo_id = use_state(&cx, || 0);

    // Filter the todos based on the filter state
    let mut filtered_todos = todos.get()
        .iter()
        .filter(|(_, item)| match *filter.get() {
            FilterState::All => true,
            FilterState::Active => !item.checked,
            FilterState::Completed => item.checked,
        })
        .map(|f| *f.0)
        .collect::<Vec<_>>();
    filtered_todos.sort_unstable();

    let show_clear_completed = todos.get().values().any(|todo| todo.checked);
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
                        value: "{*draft.get()}",
                        autofocus: "true",
                        oninput: move |evt| draft.set(evt.value.clone()),
                        onkeydown: move |evt| {
                            if evt.key() == keyboard_types::Key::Enter && !draft.get().is_empty() {
                                todos.make_mut().insert(
                                    *todo_id.get(),
                                    TodoItem {
                                        id: *todo_id.get(),
                                        checked: false,
                                        contents: draft.get().clone(),
                                    },
                                );
                                todo_id.set(*todo_id.get() + 1);
                                draft.set("".to_string());
                            }
                        }
                    }
                }
                ul { class: "todo-list",
                    filtered_todos.iter().map(|id| rsx! {
                        todo_entry { key: "{id}", id: *id, todos: todos }
                    })
                }
                (!todos.get().is_empty()).then(|| rsx!(
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
                        (show_clear_completed).then(|| rsx!(
                            button {
                                class: "clear-completed",
                                onclick: move |_| todos.make_mut().retain(|_, todo| !todo.checked),
                                "Clear completed"
                            }
                        ))
                    }
                ))
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
    todos: &'a UseState<Todos>,
    id: u32,
}

pub fn todo_entry<'a>(cx: Scope<'a, TodoEntryProps<'a>>) -> Element {
    use keyboard_types::Key;

    let is_editing = use_state(&cx, || false);

    let todos = cx.props.todos.get();
    let todo = &todos[&cx.props.id];
    let completed = if todo.checked { "completed" } else { "" };
    let editing = if *is_editing.get() { "editing" } else { "" };

    cx.render(rsx! {
        li {
            class: "{completed} {editing}",
            onclick: move |_| is_editing.set(true),
            onfocusout: move |_| is_editing.set(false),
            div { class: "view",
                input { class: "toggle", r#type: "checkbox", id: "cbg-{todo.id}", checked: "{todo.checked}",
                    onchange: move |evt| {
                        cx.props.todos.make_mut()[&cx.props.id].checked = evt.value.parse().unwrap();
                    }
                }
                label { r#for: "cbg-{todo.id}", pointer_events: "none", "{todo.contents}" }
            }
            is_editing.get().then(|| rsx!{
                input {
                    class: "edit",
                    value: "{todo.contents}",
                    oninput: move |evt| {
                        cx.props.todos.make_mut()[&cx.props.id].contents = evt.value.clone()
                    },
                    autofocus: "true",
                    onkeydown: move |evt| {
                        match evt.key() {
                            Key::Enter | Key::Escape | Key::Tab => is_editing.set(false),
                            _ => {}
                        }
                    },
                }
            })
        }
    })
}

