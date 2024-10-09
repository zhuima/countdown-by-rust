use chrono::{DateTime, Duration, Utc};
use dioxus::prelude::*;
use futures_lite::stream::StreamExt;
use gloo_timers::future::IntervalStream;
use log::info;
use web_sys::console;

#[component]
pub fn Countdown(target_date: Signal<DateTime<Utc>>) -> Element {
    info!("Countdown component function called");

    let mut remaining_time = use_signal(|| Duration::zero());

    // 使用use_effect来创建一个定时器
    use_future(move || async move {
        let mut interval = IntervalStream::new(1000);
        while let Some(_) = interval.next().await {
            let now = Utc::now();
            let target_value = *target_date.read();
            if now < target_value {
                *remaining_time.write() = target_value - now;
                info!("Remaining time: {:?}", remaining_time.read());
            } else {
                *remaining_time.write() = Duration::zero();
                info!("Countdown finished");
                break;
            }
        }
    });

    use_effect(move || {
        to_owned![target_date, remaining_time];
        info!("target_date changed: {:?}", *target_date.read());
        let now = Utc::now();
        let target_value = *target_date.read();
        if now < target_value {
            *remaining_time.write() = target_value - now;
        } else {
            *remaining_time.write() = Duration::zero();
        }
    });

    // 使用 web_sys::console 进行日志输出
    console::log_1(
        &format!(
            "Countdown component rendered with target date: {:?}",
            *target_date.read()
        )
        .into(),
    );

    console::log_1(
        &format!(
            "Countdown component remaining_time: {:?}",
            *remaining_time.read()
        )
        .into(),
    );

    let days = remaining_time.read().num_days();
    let hours = remaining_time.read().num_hours() % 24;
    let minutes = remaining_time.read().num_minutes() % 60;
    let seconds = remaining_time.read().num_seconds() % 60;

    rsx! {
        div { class: "grid grid-cols-2 gap-4 py-6 px-10 md:flex md:items-center md:justify-between md:mt-2  rounded-xl md:px-6 md:py-8 ",
            TimeUnit { value: days, label: "Days" }
            div { class: " hidden text-3xl -mt-8 md:inline-block md:text-5xl font-normal text-gray-50 ", ":" }
            TimeUnit { value: hours, label: "Hours" }
            div { class: " hidden text-3xl -mt-8 md:inline-block md:text-5xl font-normal text-gray-50 ", ":" }
            TimeUnit { value: minutes, label: "Minutes" }
            div { class: " hidden text-3xl -mt-8 md:inline-block md:text-5xl font-normal text-gray-50 ", ":" }
            TimeUnit { value: seconds, label: "Seconds" }
        }
    }
}

#[component]
fn TimeUnit(value: i64, label: &'static str) -> Element {
    rsx! {
        div { class: "flex flex-col items-center mx-2",
            div {
                class: "bg-white bg-opacity-20 rounded-xl p-4 mb-2 w-24 h-24 flex items-center justify-center",
                div { class: "text-4xl font-bold text-white", "{value:02}" }
            }
            div { class: "text-sm font-medium text-white uppercase tracking-wide", "{label}" }
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
