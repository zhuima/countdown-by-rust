#![allow(non_snake_case)]

mod components;

use chrono::{Duration, TimeZone, Utc};
use components::{Countdown, SettingsModal};
use dioxus::prelude::*;
use dioxus_logger::tracing::{info, Level};
use web_sys::window;

#[derive(Clone, Routable, Debug, PartialEq)]
enum Route {
    #[route("/")]
    Home {},
}

fn main() {
    dioxus_logger::init(Level::INFO).expect("failed to init logger");
    info!("starting app");
    launch(App);
}

fn App() -> Element {
    rsx! {
        Router::<Route> {}
        // title { "Countdown Timer" }
        // meta { name: "description", content: "A simple countdown timer with Rust" }
        // meta { name: "keywords", content: "countdown, timer, rust" }
        // meta { name: "canonical", content: "https://countdown.666222.best" }
        // meta { name: "viewport", content: "width=device-width, initial-scale=1.0" }
        // meta { name: "og:title", content: "Countdown Timer" }
        // meta { name: "og:type", content: "website" }
        // meta { name: "og:description", content: "A simple countdown timer with Rust" }
        // meta { name: "og:image", content: "https://countdown.666222.best/og-image.jpg" }
        // meta { name: "og:url", content: "https://countdown.666222.best" }
        // meta { name: "twitter:card", content: "summary_large_image" }
        // meta { name: "twitter:site", content: "https://x.com/ilovek8s" }
        // meta { name: "twitter:title", content: "Countdown Timer" }
        // meta { name: "twitter:description", content: "A simple countdown timer with Rust" }
        // meta { name: "twitter:image", content: "https://countdown.666222.best/og-image.jpg" }
    }
}

#[component]
fn Home() -> Element {
    let default_days = 7;
    let default_message = "We're launching soon";

    // 从 localStorage 读取保存的数据
    let saved_end_time = use_memo(move || {
        window()
            .and_then(|w| w.local_storage().ok())
            .flatten()
            .and_then(|storage| storage.get_item("countdown_end_time").ok())
            .flatten()
            .and_then(|value| value.parse::<i64>().ok())
    });

    let saved_message = use_memo(move || {
        window()
            .and_then(|w| w.local_storage().ok())
            .flatten()
            .and_then(|storage| storage.get_item("countdown_message").ok())
            .flatten()
            .unwrap_or_else(|| default_message.to_string())
    });

    let mut target_date = use_signal(|| {
        saved_end_time
            .as_ref()
            .map(|end_time| {
                let now = Utc::now().timestamp();
                if *end_time > now {
                    Utc.timestamp_opt(*end_time, 0).unwrap()
                } else {
                    Utc::now() + Duration::days(default_days)
                }
            })
            .unwrap_or_else(|| Utc::now() + Duration::days(default_days))
    });

    let mut custom_message = use_signal(|| saved_message.to_string());
    let mut show_settings = use_signal(|| false);

    let open_settings = move |_| show_settings.set(true);
    // 使用 use_effect 来监听 target_date 的变化
    use_effect(move || {
        info!("target_date updated: {}", target_date.read());
    });

    // info!(
    //     "Home component rendered with target date: {}",
    //     target_date.read()
    // );

    // let target_date = use_signal(|| Utc::now() + Duration::days(10));

    // info!(
    //     "Home component rendered with target date: {}",
    //     target_date.read()
    // );

    rsx! {
        div { class: "min-h-screen bg-gradient-to-b from-blue-900 to-purple-900 flex flex-col items-center justify-center text-white px-4 sm:px-6 lg:px-8",
            h1 {
                class: "text-3xl sm:text-4xl md:text-5xl lg:text-6xl font-light mb-6 sm:mb-8 md:mb-10 lg:mb-12 text-center",
                "{custom_message}"
            }
            Countdown { target_date: target_date }
            div { class: "mt-8 sm:mt-10 md:mt-12 lg:mt-16 flex space-x-4 sm:space-x-6",
                SocialIcon { icon: "facebook" }
                SocialIcon { icon: "github" }
                SocialIcon { icon: "twitter" }
            }
            footer {
                class: "mt-8 sm:mt-10 md:mt-12 lg:mt-16 text-sm font-light text-center",
                "Designed with ❤️ by "
                a {
                    href: "https://twitter.com/ilovek8s",
                    target: "_blank",
                    rel: "noopener noreferrer",
                    class: "text-gray-300 hover:text-white transition-colors font-light",
                    "@ilovek8s"
                }
            }
            div {
                class: "absolute bottom-8 right-8 cursor-pointer",
                onclick: open_settings,
                svg {
                    xmlns: "http://www.w3.org/2000/svg",
                    width: "24",
                    height: "24",
                    view_box: "0 0 24 24",
                    fill: "none",
                    stroke: "currentColor",
                    stroke_width: "2",
                    stroke_linecap: "round",
                    stroke_linejoin: "round",
                    path { d: "M19.4 15a1.65 1.65 0 0 0 .33 1.82l.06.06a2 2 0 0 1 0 2.83 2 2 0 0 1-2.83 0l-.06-.06a1.65 1.65 0 0 0-1.82-.33 1.65 1.65 0 0 0-1 1.51V21a2 2 0 0 1-2 2 2 2 0 0 1-2-2v-.09A1.65 1.65 0 0 0 9 19.4a1.65 1.65 0 0 0-1.82.33l-.06.06a2 2 0 0 1-2.83 0 2 2 0 0 1 0-2.83l.06-.06a1.65 1.65 0 0 0 .33-1.82 1.65 1.65 0 0 0-1.51-1H3a2 2 0 0 1-2-2 2 2 0 0 1 2-2h.09A1.65 1.65 0 0 0 4.6 9a1.65 1.65 0 0 0-.33-1.82l-.06-.06a2 2 0 0 1 0-2.83 2 2 0 0 1 2.83 0l.06.06a1.65 1.65 0 0 0 1.82.33H9a1.65 1.65 0 0 0 1-1.51V3a2 2 0 0 1 2-2 2 2 0 0 1 2 2v.09a1.65 1.65 0 0 0 1 1.51 1.65 1.65 0 0 0 1.82-.33l.06-.06a2 2 0 0 1 2.83 0 2 2 0 0 1 0 2.83l-.06.06a1.65 1.65 0 0 0-.33 1.82V9a1.65 1.65 0 0 0 1.51 1H21a2 2 0 0 1 2 2 2 2 0 0 1-2 2h-.09a1.65 1.65 0 0 0-1.51 1z" }
                    circle { cx: "12", cy: "12", r: "3" }
                }
            }
            // 修改这里,添加条件渲染
            // if *show_settings.read() {
            //     SettingsModal {
            //         on_close: close_settings,
            //         on_save: save_date
            //     }
            // }

             {show_settings().then(|| rsx!(
                SettingsModal {
                    on_close: move |_| show_settings.set(false),
                    on_save: move |(days, message): (i64, String)| {
                        let new_end_time = Utc::now() + Duration::days(days);
                        target_date.set(new_end_time);
                        custom_message.set(message.clone());
                        show_settings.set(false);

                        // 保存到 localStorage
                        if let Some(window) = window() {
                            if let Ok(Some(storage)) = window.local_storage() {
                                let _ = storage.set_item("countdown_end_time", &new_end_time.timestamp().to_string());
                                let _ = storage.set_item("countdown_message", &message);
                            }
                        }
                    }
                }
            ))}


        }
    }
}

#[component]
fn SocialIcon(icon: &'static str) -> Element {
    let (path, href) = match icon {
        "facebook" => (
            "M18 2h-3a5 5 0 00-5 5v3H7v4h3v8h4v-8h3l1-4h-4V7a1 1 0 011-1h3z",
            "https://x.com/i/communities/1503688305988169728"),
        "github" => (
            "M9 19c-5 1.5-5-2.5-7-3m14 6v-3.87a3.37 3.37 0 00-.94-2.61c3.14-.35 6.44-1.54 6.44-7A5.44 5.44 0 0020 4.77 5.07 5.07 0 0019.91 1S18.73.65 16 2.48a13.38 13.38 0 00-7 0C6.27.65 5.09 1 5.09 1A5.07 5.07 0 005 4.77a5.44 5.44 0 00-1.5 3.78c0 5.42 3.3 6.61 6.44 7A3.37 3.37 0 009 18.13V22",
            "https://github.com/zhuima/countdown-by-rust"),
        "twitter" => (
            "M23 3a10.9 10.9 0 01-3.14 1.53 4.48 4.48 0 00-7.86 3v1A10.66 10.66 0 013 4s-4 9 5 13a11.64 11.64 0 01-7 2c9 5 20 0 20-11.5a4.5 4.5 0 00-.08-.83A7.72 7.72 0 0023 3z",
            "https://x.com/ilovek8s"),
         _ => ("", "#"),
    };

    rsx! {
        a {
            href: "{href}",
            target: "_blank",
            class: "text-gray-400 hover:text-white transition-colors",
            svg {
                xmlns: "http://www.w3.org/2000/svg",
                width: "24",
                height: "24",
                view_box: "0 0 24 24",
                fill: "none",
                stroke: "currentColor",
                stroke_width: "2",
                stroke_linecap: "round",
                stroke_linejoin: "round",
                path { d: "{path}" }
            }
        }
    }
}
