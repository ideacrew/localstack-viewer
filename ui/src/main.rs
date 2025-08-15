use std::collections::HashMap;

use dioxus::prelude::*;
use reqwest::Url;
use web_sys::window;

use serde::{Deserialize, Serialize};
use serde_json::value::RawValue;

#[derive(Debug, Clone, Routable, PartialEq)]
enum Route {
    #[layout(Navbar)]
    #[route("/")]
    Home {},
    #[route("/sms-messages")]
    SmsMessages {},
}

#[derive(Clone)]
struct AppContext {
    base_url: String,
}

const FAVICON: Asset = asset!("/assets/favicon.ico");
const MAIN_CSS: Asset = asset!("/assets/main.css");
// const HEADER_SVG: Asset = asset!("/assets/header-light.svg");
const TAILWIND_CSS: Asset = asset!("/assets/tailwind.css");

#[derive(Deserialize, Serialize)]
pub(crate) struct SmsMessageList {
    pub sms_messages: HashMap<String, Vec<HashMap<String, Box<RawValue>>>>,
    pub region: String,
}

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
                            "This application provides the ability to browse look at localstack status and contents.\
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
        }

        Outlet::<Route> {}
    }
}

fn render_sms_messages(ml: &SmsMessageList) -> Element {
    rsx! {
        h1 { { ml.region.clone() } }
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

    let message_list = use_resource(move || {
        let app_status_base_url = app_status_bu.clone();
        async move {
            reqwest::get(app_status_base_url + "/api/sms-messages")
                .await?
                .json::<SmsMessageList>()
                .await
        }
    });

    match &*message_list.read_unchecked() {
        Some(Ok(ml)) => render_sms_messages(ml),
        Some(Err(e)) => rsx! {
            code { { format!("Error: {:?}",e) } }
        },
        None => rsx! {
            h1 { "No data yet..." }
        },
    }
}

/// Home page
#[component]
fn Home() -> Element {
    rsx! {
        Hero {}
    }
}
