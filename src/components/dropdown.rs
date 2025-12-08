use dioxus::prelude::*;
use dioxus_core::NoOpMutations;
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::JsCast;
#[cfg(target_arch = "wasm32")]
use web_sys::{window, HtmlElement};

#[allow(unpredictable_function_pointer_comparisons)]
#[derive(Props, Clone, PartialEq)]
pub struct DropdownProps<T: Clone + PartialEq + 'static> {
    /// Dropdown label
    pub label: String,
    /// Dropdown label style (optional)
    pub label_class: Option<String>,
    /// Currently selected value
    pub value: String,
    /// Options list
    pub options: Vec<T>,
    /// Whether the dropdown is open
    pub is_open: bool,
    /// Search query
    pub search_query: String,
    /// Toggle dropdown event handler
    pub on_toggle: EventHandler<()>,
    /// Search event handler
    pub on_search: EventHandler<String>,
    /// Selection event handler
    pub on_select: EventHandler<T>,
    /// Display function to convert options to display strings
    pub display_fn: fn(&T) -> String,
    /// Optional error state
    #[props(default = false)]
    pub has_error: bool,
    /// Optional custom class name
    #[props(default = String::new())]
    pub class: String,
    /// Optional placeholder text
    #[props(default = "Search...".to_string())]
    pub search_placeholder: String,
    /// Optional button class name (optional)
    pub button_class: Option<String>,
    /// Optional dropdown class name
    #[props(default = String::new())]
    pub dropdown_class: String,
    /// Optional search input class name
    #[props(default = String::new())]
    pub search_input_class: String,
    /// Optional option class name
    #[props(default = String::new())]
    pub option_class: String,
    /// Whether to disable the dropdown
    #[props(default = false)]
    pub disabled: bool,
    /// Whether it's required
    #[props(default = false)]
    pub required: bool,
    /// Whether to show dropdown arrow
    #[props(default = true)]
    pub show_arrow: bool,
    /// Dropdown width class (optional)
    #[props(default = None)]
    pub dropdown_width: Option<String>,
    /// Dropdown position class (optional)
    #[props(default = None)]
    pub dropdown_position: Option<String>,
    /// Whether to show search box
    #[props(default = true)]
    pub show_search: bool,
    /// Whether in desktop mode
    #[props(default = false)]
    pub is_desktop: bool,
}

#[component]
pub fn Dropdown<T: Clone + PartialEq + 'static>(props: DropdownProps<T>) -> Element {
    let is_open = props.is_open;
    let selected_value = props.value.clone();

    use_effect(move || {
        #[cfg(target_arch = "wasm32")]
        {
            if is_open && !selected_value.is_empty() {
                if let Some(document) = window().and_then(|w| w.document()) {
                    if let Ok(Some(container)) =
                        document.query_selector(".dropdown-options[data-open='true']")
                    {
                        if let Ok(Some(selected_button)) =
                            container.query_selector("button[data-selected='true']")
                        {
                            if let (Some(container_el), Some(selected_el)) = (
                                container.dyn_into::<HtmlElement>().ok(),
                                selected_button.dyn_into::<HtmlElement>().ok(),
                            ) {
                                container_el.set_scroll_top(selected_el.offset_top());
                            }
                        }
                    }
                }
            }
        }
    });

    let closed_translation = if props.is_desktop {
        // Force the desktop animation even when the viewport width is small (e.g. desktop shell with a mobile-sized viewport)
        "-translate-y-2"
    } else {
        // Fallback to responsive behavior: mobile slides up, desktop slides down
        "translate-y-2 sm:-translate-y-2"
    };

    let dropdown_class = if props.is_open {
        "translate-y-0 opacity-100".to_string()
    } else {
        format!("{closed_translation} opacity-0 pointer-events-none")
    };

    let search_query = props.search_query.clone();
    let display_fn = props.display_fn;

    let button_class = props.button_class.clone().unwrap_or_else(|| "w-full px-4 py-2.5 text-sm border border-gray-300 dark:border-gray-600 paper:border-[#e4d5b2] rounded-lg bg-white dark:bg-gray-700 paper:bg-[#fef8e7] text-gray-900 dark:text-white paper:text-[#1f2937] shadow-sm focus:outline-none focus:ring-2 focus:ring-green-500 focus:border-transparent transition duration-200 ease-in-out hover:border-green-500 dark:hover:border-green-500 paper:hover:border-[#c6b17e] flex justify-between items-center relative will-change-transform will-change-opacity".to_string());
    let button_class = if props.disabled {
        format!("{} opacity-50 cursor-not-allowed", button_class)
    } else {
        button_class
    };
    let label_class = props.label_class.clone().unwrap_or_else(|| {
        "block text-sm font-medium text-gray-700 dark:text-gray-300 paper:text-[#374151] mb-2".to_string()
    });

    let width_class = props
        .dropdown_width
        .clone()
        .unwrap_or_else(|| "w-full sm:min-w-[16rem] sm:max-w-[60vw]".to_string());
    let position_class = props.dropdown_position.clone().unwrap_or_else(|| "fixed bottom-14 left-0 right-0 rounded-t-lg sm:absolute sm:bottom-auto sm:right-0 sm:top-full sm:left-auto sm:rounded-md".to_string());
    let base_panel_class = "z-[1000] transition duration-200 ease-in-out transform will-change-transform will-change-opacity shadow-lg bg-white dark:bg-gray-800 paper:bg-[#fef8e7] paper:text-[#1f2937] ring-1 ring-black ring-opacity-5 paper:ring-[#d4c29a] paper:ring-opacity-60 paper-surface";
    let dropdown_container_class = format!(
        "{} {} {} {}",
        base_panel_class, position_class, dropdown_class, width_class
    );

    let search_input_class = format!("w-full px-3 py-2 text-sm border border-gray-300 dark:border-gray-600 paper:border-[#e4d5b2] rounded-md bg-white dark:bg-gray-700 paper:bg-[#fef8e7] text-gray-900 dark:text-white paper:text-[#1f2937] focus:outline-none focus:ring-2 focus:ring-green-500 focus:border-transparent pen-texture-text {}", props.search_input_class);

    let base_option_class = format!("block w-full text-left px-4 py-2 text-sm text-gray-700 dark:text-gray-200 paper:text-[#374151] hover:bg-gray-100 dark:hover:bg-gray-700 paper:hover:bg-[#f0e6cf] transition duration-150 truncate pen-texture-text {}", props.option_class);

    rsx! {
        div {
            class: format!("relative {}", props.class),
            // Overlay
            {if props.is_open && !props.disabled {
                rsx! {
                    div {
                        class: "fixed inset-0 w-screen h-screen z-[999] bg-black/50",
                        onclick: move |_| props.on_toggle.call(()),
                    }
                }
            } else {
                rsx! {}
            }}

            label {
                class: label_class,
                "{props.label}"
                {if props.required {
                    rsx! {
                        span {
                            class: "text-red-500 ml-1",
                            "*"
                        }
                    }
                } else {
                    rsx! {}
                }}
            }
            div {
                class: "w-full",
                button {
                    class: button_class,
                    onclick: move |_| {
                        if !props.disabled {
                            props.on_toggle.call(())
                        }
                    },
                    disabled: props.disabled,
                    "aria-required": props.required.to_string(),
                    span {
                        class: "block truncate pen-texture-text",
                        "{props.value}"
                    }
                    if props.show_arrow {
                        svg {
                            class: "flex-shrink-0 fill-current h-4 w-4 transition-transform duration-200 ease-in-out",
                            xmlns: "http://www.w3.org/2000/svg",
                            view_box: "0 0 20 20",
                            path {
                                d: "M9.293 12.95l.707.707L15.657 8l-1.414-1.414L10 10.828 5.757 6.586 4.343 8z"
                            }
                        }
                    }
                }
                {if !props.disabled {
                    rsx! {
                        div {
                            class: dropdown_container_class,
                            {if props.show_search {
                                rsx! {
                                    div {
                                        class: "p-2 border-b border-gray-200 dark:border-gray-700",
                                        input {
                                            class: search_input_class,
                                            placeholder: props.search_placeholder,
                                            value: "{search_query}",
                                            oninput: move |e| props.on_search.call(e.value().clone())
                                        }
                                    }
                                }
                            } else {
                                rsx! {}
                            }}
                            div {
                                class: "max-h-[clamp(12rem,calc(100vh_-_16rem),24rem)] overflow-y-auto scrollbar-thin scrollbar-thumb-gray-300 dark:scrollbar-thumb-gray-600 scrollbar-track-transparent dropdown-options",
                                "data-open": props.is_open.to_string(),
                                {props.options.iter().map(|option| {
                                    let display_value = display_fn(option);
                                    let option_clone = option.clone();
                                    let is_selected = display_value == props.value;
                                    rsx! {
                                        button {
                                            "data-selected": is_selected.to_string(),
                                            class: format!("{} {}", base_option_class.clone(), if is_selected { "bg-blue-50 dark:bg-blue-900/50 text-blue-700 dark:text-blue-300" } else { "" }),
                                            onclick: move |_| props.on_select.call(option_clone.clone()),
                                            {display_value}
                                        }
                                    }
                                })}
                            }
                        }
                    }
                } else {
                    rsx! {}
                }}
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use dioxus::prelude::*;

    #[test]
    #[ignore]
    fn test_dropdown_default_max_width() {
        let props = DropdownProps::<String> {
            label: "Test".to_string(),
            label_class: None,
            value: "".to_string(),
            options: vec!["A".to_string(), "B".to_string()],
            is_open: true,
            search_query: "".to_string(),
            on_toggle: EventHandler::new(|_| {}),
            on_search: EventHandler::new(|_| {}),
            on_select: EventHandler::new(|_| {}),
            display_fn: |s: &String| s.clone(),
            has_error: false,
            class: String::new(),
            search_placeholder: "Search...".to_string(),
            button_class: None,
            dropdown_class: String::new(),
            search_input_class: String::new(),
            option_class: String::new(),
            disabled: false,
            required: false,
            show_arrow: true,
            dropdown_width: None,
            dropdown_position: None,
            show_search: true,
            is_desktop: true,
        };

        let mut dom = VirtualDom::new_with_props(Dropdown::<String>, props);
        let mut mutations = NoOpMutations;
        dom.rebuild(&mut mutations);
        let html = dioxus_ssr::render(&dom);
        assert!(
            html.contains("max-w-[60vw]"),
            "Dropdown should have responsive max width 60vw to prevent clipping"
        );
    }
}
