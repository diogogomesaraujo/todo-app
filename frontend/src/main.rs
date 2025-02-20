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
    let mut input_text = use_signal(|| String::new());

    let get_tasks = move || async move {
        let response = reqwest::get("http://localhost:3000/tasks")
            .await
            .expect("Could not access link")
            .json::<Vec<Task>>()
            .await
            .unwrap();

        tasks.set(response);

        println!("{:?}", tasks);
    };

    let add_task = move |text: String| async move {
        let client = reqwest::Client::new();

        // handle response
        client
            .post("http://localhost:3000/tasks")
            .header("Content-Type", "application/json")
            .body(text)
            .send()
            .await
            .unwrap();
    };

    use_future(get_tasks);

    rsx! {
        div {
            id: "hero",
            div {
                input {
                    // and what to do when the value changes
                    value: "{input_text}",
                    oninput: move |event| input_text.set(event.value())
                }
            }
            div {
                {
                    rsx!(
                        {tasks.iter().map(|item| rsx! {
                            div { id: "buttons",
                                a { id: "task", "{item.content}" }
                            }
                        })}
                    )
                }
            },
            Padding {}
            div {
                id: "buttons", button {
                    onclick: move |_| {
                        let task_text = input_text.clone();
                        spawn(async move {
                            println!("Added Task: {}", task_text);
                            add_task(task_text.to_string()).await;
                        });
                    },
                    id: "add_task", "Add a Task", }
            }
        }
    }
}
