use std::collections::HashMap;

use dioxus::prelude::*;
use dioxus_router::prelude::*;
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
    baseUrl: String,
}

const FAVICON: Asset = asset!("/assets/favicon.ico");
const MAIN_CSS: Asset = asset!("/assets/main.css");
const HEADER_SVG: Asset = asset!("/assets/header.svg");
const TAILWIND_CSS: Asset = asset!("/assets/tailwind.css");

#[derive(Deserialize, Serialize)]
pub(crate) struct SmsMessageList {
    pub sms_messages: HashMap<String, Vec<Box<RawValue>>>,
    pub region: String,
}

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    let base_href = window().unwrap().location().href().unwrap();
    let mut url = Url::parse(&base_href).unwrap();
    url.path_segments_mut().unwrap().clear();

    let _ = use_context_provider(|| AppContext {
        baseUrl: url.to_string().to_owned(),
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
            id: "hero",
            img { src: HEADER_SVG, id: "header" }
            div { id: "links",
                a { href: "https://dioxuslabs.com/learn/0.6/", "📚 Learn Dioxus" }
                a { href: "https://dioxuslabs.com/awesome", "🚀 Awesome Dioxus" }
                a { href: "https://github.com/dioxus-community/", "📡 Community Libraries" }
                a { href: "https://github.com/DioxusLabs/sdk", "⚙️ Dioxus Development Kit" }
                a { href: "https://marketplace.visualstudio.com/items?itemName=DioxusLabs.dioxus", "💫 VSCode Extension" }
                a { href: "https://discord.gg/XgGxMSkvUM", "👋 Community Discord" }
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
           for (k,v) in ml.sms_messages.iter() {
              li {
                h3 { { k.to_string() } }
                ul {
                  for m in v {
                    li { "{m:?}" }
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

    let app_status_bu = app_status.baseUrl.clone();

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
