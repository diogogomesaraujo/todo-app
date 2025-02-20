use dioxus::prelude::*;
use serde::{Deserialize, Serialize};

const FAVICON: Asset = asset!("/assets/favicon.ico");
const MAIN_CSS: Asset = asset!("/assets/main.css");

#[derive(Serialize, Deserialize, Debug)]
struct Task {
    id: u32,
    content: String,
    checked: bool,
}

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    rsx! {
        document::Link { rel: "icon", href: FAVICON }
        document::Link { rel: "stylesheet", href: MAIN_CSS }
        Padding {}
        Header {}
        Padding {}
        Hero {}
    }
}

#[component]
pub fn Header() -> Element {
    rsx!(div { id: "header", h1 { "Just Another Todo App 📋" } })
}

#[component]
pub fn Padding() -> Element {
    rsx!(div { id: "padding", a { } })
}

#[component]
pub fn Hero() -> Element {
    let mut tasks = use_signal(|| Vec::<Task>::new());
    let mut input_text = use_signal(|| String::from("Type here..."));

    let get_tasks = move || async move {
        let response = reqwest::get("http://localhost:3000/tasks")
            .await
            .expect("Could not access link")
            .json::<Vec<Task>>()
            .await
            .unwrap();

        tasks.set(response);
    };

    let add_task = move |text: String| async move {
        let client = reqwest::Client::new();

        // handle response
        client
            .post("http://localhost:3000/tasks")
            .header("Content-Type", "application/json")
            .body(format!("\"{}\"", text))
            .send()
            .await
            .unwrap();
    };

    let remove_task = move |text: String| async move {
        let client = reqwest::Client::new();
        let id: u32 = text.trim().parse().unwrap();

        // handle response
        client
            .delete(format!("http://localhost:3000/tasks/{}", id))
            .header("Content-Type", "application/json")
            .send()
            .await
            .unwrap();
    };

    use_future(get_tasks);

    rsx! {
        div {
            id: "hero",
            div {
                id: "buttons",
                input {
                    id: "input",
                    value: "{input_text}",
                    oninput: move |event| input_text.set(event.value())
                }
            }
            Padding {}
            div {
                {
                    rsx!(
                        {tasks.iter().map(|item| rsx! {
                            div { id: "buttons",
                                a { id: "task", strong { "{item.id}." }, "   {item.content}" }
                            }
                        })}
                    )
                }
            },
            Padding {}
            Padding {}
            div {
                id: "buttons",
                button {
                    onclick: move |_| {
                        let task_text = input_text.clone();
                        spawn(async move {
                            println!("Added Task: {}", task_text);
                            add_task(task_text.to_string()).await;
                        });
                    },
                    id: "add_task", "Add a Task",
                },
                Padding {}
                button {
                    onclick: move |_| {
                        let task_text = input_text.clone();
                        spawn(async move {
                            println!("Added Task: {}", task_text);
                            remove_task(task_text.to_string()).await;
                        });
                    },
                    id: "remove_task", "Remove a Task",
                }
            }
        }
    }
}
