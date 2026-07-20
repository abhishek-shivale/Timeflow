use dioxus::prelude::*;
use std::time::Duration;

const MAIN_CSS: Asset = asset!("/assets/main.css");

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
        document::Title { "Stopwatch" }
        document::Stylesheet { href: MAIN_CSS }

        main { class: "app-shell",
            header { class: "top-bar",
                div { class: "brand",
                    span { class: "brand-mark" }
                    span { "Moment" }
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

                h1 { class: "sr-only", "Stopwatch" }

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
