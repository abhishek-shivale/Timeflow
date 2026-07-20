use dioxus::prelude::*;
use std::time::Duration;

const MAIN_CSS: Asset = asset!("/assets/main.css");
const FAVICON: Asset = asset!("/assets/favicon.ico");
const APPLE_TOUCH_ICON: Asset = asset!("/assets/apple-touch-icon.png");
const APP_ICON: Asset = asset!("/assets/timeflow-icon.png");
#[used]
static IOS_ICONS: Asset = asset!("/ios/icons", AssetOptions::folder());

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    let mut seconds = use_signal(|| 0_u64);
    let mut running = use_signal(|| false);
    let mut focus_mode = use_signal(|| false);

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

        main {
            class: if focus_mode() { "app-shell app-shell--focus" } else { "app-shell" },
            header { class: "top-bar",
                div { class: "brand",
                    img {
                        class: "brand-mark",
                        src: APP_ICON,
                        alt: "Timeflow icon",
                    }
                    span { "Timeflow" }
                }

                div { class: "top-actions",
                    div { class: "status",
                        span {
                            class: if running() { "status-dot status-dot--active" } else { "status-dot" },
                        }
                        if running() { "Running" } else { "Ready" }
                    }

                    button {
                        class: "icon-button",
                        aria_label: if focus_mode() { "Exit fullscreen" } else { "Enter fullscreen" },
                        title: if focus_mode() { "Exit fullscreen" } else { "Enter fullscreen" },
                        onclick: move |_| {
                            let entering_fullscreen = !focus_mode();
                            focus_mode.set(entering_fullscreen);

                            let script = if entering_fullscreen {
                                r#"
                                const root = document.documentElement;
                                const request = root.requestFullscreen || root.webkitRequestFullscreen;
                                if (request) request.call(root).catch?.(() => {});
                                "#
                            } else {
                                r#"
                                const exit = document.exitFullscreen || document.webkitExitFullscreen;
                                if (exit) exit.call(document).catch?.(() => {});
                                "#
                            };

                            let _ = document::eval(script);
                        },
                        svg {
                            view_box: "0 0 24 24",
                            fill: "none",
                            stroke: "currentColor",
                            stroke_width: "1.8",
                            stroke_linecap: "round",
                            stroke_linejoin: "round",
                            if focus_mode() {
                                path { d: "M8 3v3a2 2 0 0 1-2 2H3" }
                                path { d: "M16 3v3a2 2 0 0 0 2 2h3" }
                                path { d: "M8 21v-3a2 2 0 0 0-2-2H3" }
                                path { d: "M16 21v-3a2 2 0 0 1 2-2h3" }
                            } else {
                                path { d: "M8 3H5a2 2 0 0 0-2 2v3" }
                                path { d: "M16 3h3a2 2 0 0 1 2 2v3" }
                                path { d: "M8 21H5a2 2 0 0 1-2-2v-3" }
                                path { d: "M16 21h3a2 2 0 0 0 2-2v-3" }
                            }
                        }
                    }
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
