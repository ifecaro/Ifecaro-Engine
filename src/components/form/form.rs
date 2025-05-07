use dioxus::prelude::*;
use dioxus_i18n::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct FormProps {
    pub on_submit: EventHandler<()>,
    pub children: Element,
}

pub fn Form(props: FormProps) -> Element {
    rsx! {
        form {
            class: "space-y-6",
            onsubmit: move |e| {
                e.prevent_default();
                props.on_submit.call(());
            },
            {props.children}
        }
    }
} 