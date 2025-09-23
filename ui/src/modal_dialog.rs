use dioxus::prelude::*;

#[derive(PartialEq, Clone, Props)]
pub(crate) struct ModalProps {
    title: String,
    body: String,
    open_signal: Signal<bool>,
    children: Element,
}

#[component]
pub fn Modal(props: ModalProps) -> Element {
    let mut open = props.open_signal.clone();
    let extra_class_value = if open() { "" } else { " hidden" };
    rsx! {
          div {
            class: "fixed inset-0 bg-black/50 items-center justify-center flex".to_owned() + extra_class_value,
            div {
              class: "justify-center p-6 text-center focus:outline-none",
              tabindex: "0",
              div {
                    class: "relative transform bg-white rounded-2xl shadow-xl",
                    h3 { class: "text-left px-4 pt-4",
                          { props.title }
                        }
                    div { class: "text-left px-4 py-4",
                            { props.body }
                        }
                    div { class: "bg-gray-200 py-4 px-3 rounded-b-2xl",
                      button {
                                    class: "rounded-lg shadow-xs bg-white px-3 py-2 inset-ring inset-ring-black-300 mr-4",
                                    onclick: move |_e| open.set(false),
                                    { "Cancel" }
                                }
                      { props.children }
                    }
              }
            }
          }
    }
}
