use dioxus::prelude::*;

use crate::{modal_dialog::Modal, AppContext};

#[component]
pub(crate) fn update_contact_gateway_blocklist() -> Element {
    let mut open = use_signal(|| false);
    let app_status: AppContext = use_context();
    let app_status_bu = app_status.base_url.clone();
    let app_status_base_url = app_status_bu.clone();
    let on_update_list_clicked = move |_e: MouseEvent| {
        let app_status_base_url = app_status_base_url.clone();
        spawn(async move {
            let app_status_base_url = app_status_base_url.clone();
            open.set(false);
            let client = reqwest::Client::new();
            let _ = client
                .post(app_status_base_url + "/api/contact-gateway/update-blocklist")
                .send()
                .await;
        });
    };
    rsx! {
        button {
          onclick: move |_e| open.set(true),
          class: "rounded-xl inset-ring inset-ring-gray-300 px-2",
          { "Update Blocklist"}
        }
        Modal {
            title: "Update Blocklist?", body: "This will request contact gateway to update it's blocklist.  This may also send blocklist messages to Enroll.  Are you sure?",
            open_signal: open,
            button {
                    class: "rounded-lg shadow-xs bg-white px-3 py-2 inset-ring inset-ring-black-300",
                    onclick: on_update_list_clicked,
                    { "Yes I'm Sure" }
            }
        }
    }
}
