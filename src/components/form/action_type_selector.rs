use dioxus::prelude::*;
use crate::components::dropdown::Dropdown;

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
    // 定義可用的動作類型
    let action_types = vec![
        ActionType { value: "".to_string(), label: "None".to_string() },
        ActionType { value: "setting".to_string(), label: "Setting".to_string() },
    ];
    
    // 找到當前選中的動作類型
    let selected_label = action_types.iter()
        .find(|t| t.value == props.value)
        .map(|t| t.label.clone())
        .unwrap_or_else(|| "None".to_string());

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
            search_placeholder: "搜尋動作類型...",
            button_class: None,
            label_class: None,
            dropdown_class: "",
            search_input_class: "",
            option_class: "",
            required: props.required,
        }
    }
} 