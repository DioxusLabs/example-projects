#![allow(non_snake_case)]
use dioxus::prelude::*;
use todomvc::*;

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    console_error_panic_hook::set_once();
    dioxus::web::launch(APP)
}

pub struct TodoModel {
    todos: Todos,
    draft: String,
    filter: FilterState,
    todogen: u32,
}

impl TodoModel {
    pub fn new() -> Self {
        TodoModel {
            todos: Todos::new(),
            draft: String::new(),
            filter: FilterState::All,
            todogen: 0,
        }
    }

    pub fn set_draft(&mut self, text: &str) {
        self.draft = text.to_string();
    }

    pub fn submit_todo(&mut self) {
        if self.draft.is_empty() {
            return;
        }
        self.todos.insert(
            self.todogen,
            TodoItem {
                id: self.todogen,
                checked: false,
                contents: self.draft.clone(),
            },
        );
        self.todogen += 1;
        self.draft.clear();
    }

    pub fn set_filter(&mut self, filter: FilterState) {
        self.filter = filter;
    }

    pub fn show_clear_completed(&self) -> bool {
        self.todos.values().any(|todo| todo.checked)
    }

    pub fn clear_completed(&mut self) {
        self.todos.retain(|_, todo| todo.checked == false);
    }

    pub fn filtered_todos(&self) -> Vec<u32> {
        self.todos
            .iter()
            .filter(|(_, item)| match self.filter {
                FilterState::All => true,
                FilterState::Active => !item.checked,
                FilterState::Completed => item.checked,
            })
            .map(|f| *f.0)
            .collect::<Vec<_>>()
    }

    pub fn is_empty(&self) -> bool {
        self.todos.is_empty()
    }
}

static APP: Component<()> = |cx| {
    let todos = use_ref(&cx, || TodoModel::new());

    let filtered_todos = todos.read().filtered_todos();
    let items_left = filtered_todos.len();

    let item_text = match items_left {
        1 => "item",
        _ => "items",
    };

    let show_clear_completed = todos.read().show_clear_completed();
    let draft = todos.read().draft.clone();

    let todo_list = todos.write().filtered_todos().into_iter().map(|id| {
        rsx!(TodoEntry {
            id: id,
            todos: todos
        })
    });

    cx.render(rsx!{
        section {
            class: "todoapp",
            style { [include_str!("../src/style.css")] }
            div {
                header {
                    class: "header",
                    h1 {"todos"}
                    input {
                        class: "new-todo",
                        placeholder: "What needs to be done?",
                        value: "{draft}",
                        oninput: move |evt| todos.write().set_draft(&evt.value),
                        onkeydown: move |evt| {
                            if evt.key == "Enter" {
                                todos.write().submit_todo();
                            }
                        }
                    }
                }
                ul { class: "todo-list",
                    todo_list
                }
                {(!todos.read().is_empty()).then(|| rsx!(
                    footer { class: "footer",
                        span { class: "todo-count", strong {"{items_left} "} span {"{item_text} left"} }
                        ul { class: "filters",
                            li { class: "All", a { onclick: move |_| todos.write().set_filter(FilterState::All), "All" }}
                            li { class: "Active", a { onclick: move |_| todos.write().set_filter(FilterState::Active), "Active" }}
                            li { class: "Completed", a { onclick: move |_| todos.write().set_filter(FilterState::Completed), "Completed" }}
                        }
                        {(show_clear_completed).then(|| rsx!(
                            button {
                                class: "clear-completed",
                                onclick: move |_| todos.write().clear_completed(),
                                "Clear completed"
                            }
                        ))}
                    }
                ))}
            }
        }
        footer { class: "info",
            p {"Double-click to edit a todo"}
            p { "Created by ", a {href: "http://github.com/jkelleyrtp/", "jkelleyrtp" }}
            p { "Part of ", a { href: "http://todomvc.com", "TodoMVC" }}
        }
    })
};

#[derive(Props)]
pub struct TodoEntryProps<'a> {
    id: u32,
    todos: &'a UseRef<TodoModel>,
}

pub fn TodoEntry<'a>(cx: Scope<'a, TodoEntryProps<'a>>) -> Element {
    let _todos = cx.props.todos.read();
    let todo = _todos.todos.get(&cx.props.id)?;

    let (is_editing, _set_is_editing) = use_state(&cx, || false);
    let completed = if todo.checked { "completed" } else { "" };

    cx.render(rsx!{
        li {
            class: "{completed}",
            div {
                class: "view",
                input {
                    class: "toggle",
                    r#type: "checkbox",
                    id: "cbg-{todo.id}",
                    checked: "{todo.checked}",
                    onchange: move |e| {
                        log::info!("setting checked {:?}", &cx.props.id);
                        cx.props.todos.write().todos.get_mut(&cx.props.id).map(|todo| todo.checked = e.value.parse().unwrap());
                    }
                }

                label {
                    r#for: "cbg-{todo.id}",
                    pointer_events: "none",
                    "{todo.contents}"
                }

               {is_editing.then(|| rsx!{
                    input {
                        value: "{todo.contents}",
                        oninput: move |evt| {
                            cx.props.todos.write().todos.get_mut(&cx.props.id).map(|todo| todo.contents = evt.value.clone());
                        },
                    }
                })}
            }
        }
    })
}
