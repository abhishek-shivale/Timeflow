use dioxus::prelude::*;
use std::time::Duration;

const MAIN_CSS: Asset = asset!("/assets/main.css");
const FAVICON: Asset = asset!("/assets/favicon.ico");
const APPLE_TOUCH_ICON: Asset = asset!("/assets/apple-touch-icon.png");

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    let mut seconds = use_signal(|| 0_u64);
    let mut running = use_signal(|| false);

    use_future(move || async move {
        loop {
            tokio::time::sleep(Duration::from_secs(1)).await;

            if running() {
                *seconds.write() += 1;
            }
        }
    });

    let total_seconds = seconds();
    let hours = total_seconds / 3600;
    let minutes = (total_seconds / 60) % 60;
    let remaining_seconds = total_seconds % 60;

    rsx! {
        document::Title { "Timeflow" }
        document::Stylesheet { href: MAIN_CSS }
        document::Link { rel: "icon", href: FAVICON }
        document::Link { rel: "apple-touch-icon", href: APPLE_TOUCH_ICON }
        document::Meta {
            name: "description",
            content: "A calm, minimalist stopwatch built with Rust and Dioxus."
        }
        document::Meta { name: "theme-color", content: "#000000" }

        main { class: "app-shell",
            header { class: "top-bar",
                div { class: "brand",
                    span { class: "brand-mark" }
                    span { "Timeflow" }
                }

                div { class: "status",
                    span {
                        class: if running() { "status-dot status-dot--active" } else { "status-dot" },
                    }
                    if running() { "Running" } else { "Ready" }
                }
            }

            section { class: "timer-stage",
                p { class: "mode-label", "Stopwatch" }

                h1 { class: "sr-only", "Timeflow Stopwatch" }

                div {
                    class: "time-display",
                    aria_label: "Elapsed time: {hours} hours, {minutes} minutes, and {remaining_seconds} seconds",
                    span { class: "time-unit", "{hours:02}" }
                    span { class: "time-separator", ":" }
                    span { class: "time-unit", "{minutes:02}" }
                    span { class: "time-separator", ":" }
                    span { class: "time-unit", "{remaining_seconds:02}" }
                }

                p { class: "time-caption", "HOURS  ·  MINUTES  ·  SECONDS" }

                div { class: "controls",
                    button {
                        class: "control-button control-button--primary",
                        onclick: move |_| running.set(!running()),
                        if running() { "Pause" } else { "Start" }
                    }

                    button {
                        class: "control-button control-button--quiet",
                        disabled: total_seconds == 0,
                        onclick: move |_| {
                            seconds.set(0);
                            running.set(false);
                        },
                        "Reset"
                    }
                }
            }

            footer { class: "footer-note", "Take your time." }
        }
    }
}
