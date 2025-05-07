use dioxus::prelude::*;
use dioxus_i18n::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct SectionProps {
    pub title: String,
    pub description: String,
    pub children: Element,
}

pub fn Section(props: SectionProps) -> Element {
    rsx! {
        div { class: "space-y-6",
            div { class: "space-y-1",
                h2 { class: "text-xl font-semibold text-gray-900 dark:text-white",
                    {props.title}
                }
                p { class: "text-sm text-gray-500 dark:text-gray-400",
                    {props.description}
                }
            }
            div { class: "space-y-6",
                {props.children}
            }
        }
    }
} 