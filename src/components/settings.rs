use dioxus::prelude::*;

#[component]
pub fn SettingsModal(
    on_close: EventHandler<()>,
    on_save: EventHandler<(i64, String)>,
) -> Element {
    let mut input_date = use_signal(|| String::new());
    let mut input_message = use_signal(|| String::from("We're launching soon"));

    let save_and_close = move |_| {
        if let Ok(duration) = input_date.read().parse::<i64>() {
            on_save.call((duration, input_message.read().to_string()));
        }
        on_close.call(());
    };

    rsx! {
        div {
            class: "fixed inset-0 bg-black bg-opacity-50 flex items-center justify-center",
            div {
                class: "bg-white p-6 rounded-lg",
                h2 { class: "text-2xl mb-4 text-gray-800", "Set Countdown Settings" }
                input {
                    r#type: "number",
                    placeholder: "Enter number of days",
                    value: "{input_date}",
                    oninput: move |e| input_date.set(e.value()),
                    class: "border p-2 mb-4 w-full text-black"
                }
                input {
                    r#type: "text",
                    placeholder: "Enter custom message",
                    value: "{input_message}",
                    oninput: move |e| input_message.set(e.value()),
                    class: "border p-2 mb-4 w-full text-black"
                }
                div {
                    class: "flex justify-end",
                    button {
                        onclick: move |_| on_close.call(()),
                        class: "mr-2 px-4 py-2 bg-gray-200 rounded text-gray-800",
                        "Cancel"
                    }
                    button {
                        onclick: save_and_close,
                        class: "px-4 py-2 bg-blue-500 text-white rounded",
                        "Save"
                    }
                }
            }
        }
    }
}