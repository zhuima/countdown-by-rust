use dioxus::prelude::*;

#[component]
pub fn Input(on_set_countdown: EventHandler<()>) -> Element {
    rsx! {
        div { class: "mt-8",
            button {
                class: "bg-blue-500 hover:bg-blue-600 px-4 py-2 rounded-lg transition-colors duration-200",
                onclick: move |_| on_set_countdown.call(()),
                "Set Countdown"
            }
        }
    }
}
