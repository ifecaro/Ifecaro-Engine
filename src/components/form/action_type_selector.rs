use dioxus::prelude::*;
use crate::components::dropdown::Dropdown;
use dioxus_i18n::t;

#[derive(Debug, Clone, PartialEq)]
pub struct ActionType {
    pub value: String,
    pub label: String,
}

#[allow(dead_code)]
fn display_action_type(action_type: &ActionType) -> String {
    action_type.label.clone()
}

#[derive(Props, Clone, PartialEq)]
pub struct ActionTypeSelectorProps {
    pub label: String,
    pub value: String,
    pub is_open: bool,
    pub on_toggle: EventHandler<()>,
    pub on_select: EventHandler<String>,
    #[props(default = false)]
    pub has_error: bool,
    #[props(default = String::new())]
    pub class: String,
    #[props(default = false)]
    pub required: bool,
}

#[component]
pub fn ActionTypeSelector(props: ActionTypeSelectorProps) -> Element {
    // Define available action types
    let action_types = vec![
        ActionType { value: "".to_string(), label: t!("none") },
        ActionType { value: "setting".to_string(), label: t!("setting") },
    ];
    
    // Find currently selected action type
    let selected_label = action_types.iter()
        .find(|t| t.value == props.value)
        .map(|t| t.label.clone())
        .unwrap_or_else(|| t!("none"));

    rsx! {
        Dropdown {
            label: props.label,
            value: selected_label,
            options: action_types,
            is_open: props.is_open,
            search_query: String::new(),
            on_toggle: props.on_toggle,
            on_search: move |_| {},
            on_select: move |action_type: ActionType| {
                props.on_select.call(action_type.value);
            },
            display_fn: display_action_type,
            has_error: props.has_error,
            class: props.class,
            search_placeholder: t!("search_action_type"),
            button_class: None,
            label_class: None,
            dropdown_class: "",
            search_input_class: "",
            option_class: "",
            required: props.required,
        }
    }
} 