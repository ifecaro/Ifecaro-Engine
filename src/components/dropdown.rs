use dioxus::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Props, Clone, PartialEq)]
pub struct DropdownProps<T: Clone + PartialEq + 'static> {
    label: String,
    value: String,
    options: Vec<T>,
    is_open: bool,
    search_query: String,
    on_toggle: EventHandler<()>,
    on_search: EventHandler<String>,
    on_select: EventHandler<T>,
    display_fn: fn(&T) -> String,
}

#[component]
pub fn Dropdown<T: Clone + PartialEq + 'static>(props: DropdownProps<T>) -> Element {
    let dropdown_class = if props.is_open {
        "translate-y-0 opacity-100"
    } else {
        "-translate-y-2 opacity-0 pointer-events-none"
    };

    let search_query = props.search_query.clone();
    let display_fn = props.display_fn;

    rsx! {
        div { 
            class: "relative",
            label { 
                class: "block text-sm font-medium text-gray-700 dark:text-gray-300 mb-2",
                "{props.label}"
            }
            div { 
                class: "w-full",
                button {
                    class: "w-full px-4 py-2.5 text-base border border-gray-300 dark:border-gray-600 rounded-lg bg-white dark:bg-gray-700 text-gray-900 dark:text-white shadow-sm focus:outline-none focus:ring-2 focus:ring-green-500 focus:border-transparent cursor-pointer transition-all duration-200 ease-in-out hover:border-green-500 dark:hover:border-green-500 flex justify-between items-center",
                    onclick: move |_| props.on_toggle.call(()),
                    span { "{props.value}" }
                    svg { 
                        class: "fill-current h-4 w-4 transition-transform duration-200 ease-in-out",
                        xmlns: "http://www.w3.org/2000/svg",
                        view_box: "0 0 20 20",
                        path { 
                            d: "M9.293 12.95l.707.707L15.657 8l-1.414-1.414L10 10.828 5.757 6.586 4.343 8z"
                        }
                    }
                }
                div {
                    class: "absolute left-0 right-0 mt-2 rounded-md shadow-lg bg-white dark:bg-gray-800 ring-1 ring-black ring-opacity-5 transition-all duration-200 ease-in-out transform origin-top {dropdown_class} z-50",
                    div { 
                        class: "p-2 border-b border-gray-200 dark:border-gray-700",
                        input {
                            class: "w-full px-3 py-2 text-sm border border-gray-300 dark:border-gray-600 rounded-md bg-white dark:bg-gray-700 text-gray-900 dark:text-white focus:outline-none focus:ring-2 focus:ring-green-500 focus:border-transparent",
                            placeholder: "搜尋...",
                            value: "{search_query}",
                            oninput: move |e| props.on_search.call(e.value().clone())
                        }
                    }
                    div { 
                        class: "max-h-[calc(100vh_-_25rem)] overflow-y-auto scrollbar-thin scrollbar-thumb-gray-300 dark:scrollbar-thumb-gray-600 scrollbar-track-transparent",
                        {props.options.iter().map(|option| {
                            let display_value = display_fn(option);
                            let option_clone = option.clone();
                            rsx! {
                                button {
                                    class: "block w-full text-left px-4 py-2 text-sm text-gray-700 dark:text-gray-200 hover:bg-gray-100 dark:hover:bg-gray-700 transition-colors duration-150",
                                    onclick: move |_| props.on_select.call(option_clone.clone()),
                                    {display_value}
                                }
                            }
                        })}
                    }
                }
            }
        }
    }
} 