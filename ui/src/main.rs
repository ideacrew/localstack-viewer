use dioxus::prelude::*;
use reqwest::Url;
use web_sys::window;

use serde_json::value::RawValue;
use std::cmp::Eq;

use localstack_viewer_data::SmsMessageList;

#[derive(Debug, Clone, Routable, PartialEq)]
enum Route {
    #[layout(Navbar)]
    #[route("/")]
    Home {},
    #[route("/sms/messages")]
    SmsMessages {},
    #[route("/sms/settings")]
    SmsSettings {},
}

#[derive(Clone)]
struct AppContext {
    base_url: String,
}

const FAVICON: Asset = asset!("/assets/favicon.ico");
const MAIN_CSS: Asset = asset!("/assets/main.css");
// const HEADER_SVG: Asset = asset!("/assets/header-light.svg");
const TAILWIND_CSS: Asset = asset!("/assets/tailwind.css");

fn main() {
    dioxus::prelude::launch(App);
}

fn read_base_url() -> String {
    let base_href = window().unwrap().location().href().unwrap();
    let mut url = Url::parse(&base_href).unwrap();
    url.path_segments_mut().unwrap().clear();
    url.to_string().trim_end_matches("/").to_owned()
}

#[component]
fn App() -> Element {
    let _ = use_context_provider(|| AppContext {
        base_url: read_base_url(),
    });

    rsx! {
        document::Link { rel: "icon", href: FAVICON }
        document::Link { rel: "stylesheet", href: MAIN_CSS } document::Link { rel: "stylesheet", href: TAILWIND_CSS }
        Router::<Route> {}
    }
}

#[component]
pub fn Hero() -> Element {
    rsx! {
        div {
            class: "relative isolate mt-8 max-w-3/4 place-content-center",
                div {
                    class: "text-center",
                    div {
                        h1 {
                            "Welcome to Localstack Viewer!"
                        }
                        p {
                            class: "text-lg pt-3",
                            "This application provides the ability to look at localstack status and contents. \
                            It covers cases not available in the default LocalStack UI - or that are normally only in the paid versions."
                        }
                    }
                }
        }
    }
}

/// Shared navbar component.
#[component]
fn Navbar() -> Element {
    rsx! {
        div {
            id: "navbar",
            Link {
                to: Route::Home {},
                "Home"
            }
            Link {
                to: Route::SmsMessages {  },
                "SMS Messages"
            }
            Link {
                to: Route::SmsSettings {  },
                "SMS Settings"
            }
        }

        Outlet::<Route> {}
    }
}

fn render_sms_messages(ml: &SmsMessageList, messages_deleted: Signal<bool>) -> Element {
    rsx! {
        h1 { { ml.region.clone() } }
        message_purge_dialog {
            messages_deleted
        }
        ul {
           class: "ml-8",
           for (k,v) in ml.sms_messages.iter() {
              li {
                h3 {
                  { k.to_string() }
                }
                ol {
                  class: "list-decimal ml-16",
                  for m in v {
                    li {
                        class: "mb-4",
                        dl {
                            for (sk, val) in m {
                                dt {
                                    class: "font-bold",
                                    "{sk}"
                                }
                                dd {
                                    class: "ml-8",
                                    { format!("{}", val.as_ref()) }
                                }
                            }
                        }
                    }
                  }
                }
              }
           }
        }
    }
}

#[component]
fn SmsMessages() -> Element {
    let app_status: AppContext = use_context();

    let app_status_bu = app_status.base_url.clone();

    let messages_deleted = use_signal(|| false);

    let message_list = use_resource(move || {
        let _val = messages_deleted.read();
        let app_status_base_url = app_status_bu.clone();
        async move {
            reqwest::get(app_status_base_url + "/api/sms/messages")
                .await?
                .json::<SmsMessageList>()
                .await
        }
    });

    match &*message_list.read_unchecked() {
        Some(Ok(ml)) => render_sms_messages(ml, messages_deleted),
        Some(Err(e)) => rsx! {
            code { { format!("Error: {:?}",e) } }
        },
        None => rsx! {
            h1 { "No data yet..." }
        },
    }
}

#[component]
fn message_purge_dialog(mut messages_deleted: Signal<bool>) -> Element {
    let app_status: AppContext = use_context();
    let app_status_bu = app_status.base_url.clone();
    let mut open = use_signal(|| false);
    let extra_class_value = if open() { "" } else { " hidden" };
    let app_status_base_url = app_status_bu.clone();
    let on_message_delete_clicked = move |_e: MouseEvent| {
        let app_status_base_url = app_status_base_url.clone();
        spawn(async move {
            let app_status_base_url = app_status_base_url.clone();
            let m_val = messages_deleted.clone().read().clone();
            open.set(false);
            let client = reqwest::Client::new();
            let _ = client
                .post(app_status_base_url + "/api/sms/purge-messages")
                .send()
                .await;
            messages_deleted.set(!m_val);
        });
    };
    rsx! {
      button {
        onclick: move |_e| open.set(true),
        class: "rounded-xl inset-ring inset-ring-gray-300 px-2",
        { "Purge Messages"}
      }
      div {
          class: "fixed inset-0 bg-black/50 items-center justify-center flex".to_owned() + extra_class_value,
          div {
              class: "justify-center p-6 text-center focus:outline-none",
              tabindex: "0",
              div {
                class: "relative transform bg-white rounded-2xl shadow-xl",
                    h3 { class: "text-left px-4 pt-4",
                       { "Purge Messages?"} }
                    div { class: "text-left px-4 py-4",
                        {"This will purge all messages, are you sure?" }
                    }
                    div {
                        class: "bg-gray-200 py-4 px-3 rounded-b-2xl",
                            button {
                                class: "rounded-lg shadow-xs bg-white px-3 py-2 inset-ring inset-ring-black-300 mr-4",
                                onclick: move |_e| open.set(false),
                                { "Cancel" }
                            }
                            button {
                                class: "rounded-lg shadow-xs bg-white px-3 py-2 inset-ring inset-ring-black-300",
                                onclick: on_message_delete_clicked,
                                { "Delete" }
                            }
                    }
              }
          }
      }
    }
}

#[component]
fn blocked_sms_numbers(blocked_list: Resource<Result<Vec<String>, reqwest::Error>>) -> Element {
    match &*blocked_list.read_unchecked() {
        Some(Ok(l)) => rsx! {
            ul {
                class: "ml-8",
                for v in l {
                  li { "{v}" }
                }
            }
        },
        Some(Err(e)) => rsx! { code { { format!("Error: {:?}",e) } } },
        _ => rsx! { div { { "No data yet..."} } },
    }
}

#[component]
fn SmsSettings() -> Element {
    let app_status: AppContext = use_context();

    let app_status_bu = app_status.base_url.clone();

    let blocked_list = use_resource(move || {
        let app_status_base_url = app_status_bu.clone();
        async move {
            reqwest::get(app_status_base_url + "/api/sms/blocked-numbers")
                .await?
                .json::<Vec<String>>()
                .await
        }
    });
    rsx! {
        h1 { "Blocked Numbers" }
        blocked_sms_numbers { blocked_list }
    }
}

/// Home page
#[component]
fn Home() -> Element {
    rsx! {
        Hero {}
    }
}
