use dioxus::prelude::*;
use chrono::{DateTime, Utc, Duration};
use log::info;
use gloo_timers::callback::Interval;
use web_sys::console;

#[component]
pub fn Countdown(target_date: Signal<DateTime<Utc>>) -> Element {
    info!("Countdown component function called");

    // 使用use_memo计算remaining_time
    let remaining_time = use_memo(move || {
        let now = Utc::now();
        let target = *target_date.read();
        if now < target {
            target - now
        } else {
            Duration::zero()
        }
    });

        // 使用 web_sys::console 进行日志输出
        console::log_1(&format!("Countdown component rendered with target date: {:?}", *target_date.read()).into());



    info!("Countdown component rendered with target date: {}", target_date);


    console::log_1(&format!("Countdown component remaining_time: {:?}", *remaining_time.read()).into());
    let days = remaining_time.read().num_days();
    let hours = remaining_time.read().num_hours() % 24;
    let minutes = remaining_time.read().num_minutes() % 60;
    let seconds = remaining_time.read().num_seconds() % 60;




    rsx! {
        div { class: "flex flex-col items-center",
            div { class: "flex space-x-4",
                TimeUnit { value: days, label: "Days" }
                div { class: "text-2xl font-bold self-center", ":" }
                TimeUnit { value: hours, label: "Hours" }
                div { class: "text-2xl font-bold self-center", ":" }
                TimeUnit { value: minutes, label: "Minutes" }
                div { class: "text-2xl font-bold self-center", ":" }
                TimeUnit { value: seconds, label: "Seconds" }
            }
        }
    }
}

#[component]
fn TimeUnit(value: i64, label: &'static str) -> Element {
    rsx! {
        div { class: "flex flex-col items-center",
            div { 
                class: "bg-gray-800 rounded-lg p-1 mb-2 relative overflow-hidden w-24 h-24",
                FlipCard { value: value }
            }
            div { class: "text-sm font-medium text-gray-500 uppercase", "{label}" }
        }
    }
}

#[component]
fn FlipCard(value: i64) -> Element {
    rsx! {
        div { class: "flip-card",
            div { class: "flip-card-inner",
                div { class: "flip-card-front",
                    div { class: "flip-card-number", "{value:02}" }
                }
                div { class: "flip-card-back",
                    div { class: "flip-card-number", "{value:02}" }
                }
            }
        }
    }
}