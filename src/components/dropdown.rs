use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct DropdownProps<T: Clone + PartialEq + 'static> {
    /// 下拉選單的標籤
    pub label: String,
    /// 下拉選單的 label 樣式（選填）
    pub label_class: Option<String>,
    /// 當前選中的值
    pub value: String,
    /// 可選項列表
    pub options: Vec<T>,
    /// 是否打開下拉選單
    pub is_open: bool,
    /// 搜尋查詢
    pub search_query: String,
    /// 切換下拉選單的事件處理器
    pub on_toggle: EventHandler<()>,
    /// 搜尋事件處理器
    pub on_search: EventHandler<String>,
    /// 選擇事件處理器
    pub on_select: EventHandler<T>,
    /// 顯示函數，用於將選項轉換為顯示字符串
    pub display_fn: fn(&T) -> String,
    /// 可選的錯誤狀態
    #[props(default = false)]
    pub has_error: bool,
    /// 可選的自定義類名
    #[props(default = String::new())]
    pub class: String,
    /// 可選的佔位符文本
    #[props(default = "搜尋...".to_string())]
    pub search_placeholder: String,
    /// 可選的按鈕類名（選填）
    pub button_class: Option<String>,
    /// 可選的下拉選單類名
    #[props(default = String::new())]
    pub dropdown_class: String,
    /// 可選的搜尋輸入框類名
    #[props(default = String::new())]
    pub search_input_class: String,
    /// 可選的選項類名
    #[props(default = String::new())]
    pub option_class: String,
    /// 是否禁用下拉選單
    #[props(default = false)]
    pub disabled: bool,
    /// 是否為必填
    #[props(default = false)]
    pub required: bool,
    /// 是否顯示下拉箭頭
    #[props(default = true)]
    pub show_arrow: bool,
    /// 下拉選單寬度 class（選填）
    #[props(default = None)]
    pub dropdown_width: Option<String>,
    /// 下拉選單位置 class（選填）
    #[props(default = None)]
    pub dropdown_position: Option<String>,
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
    
    let button_class = props.button_class.clone().unwrap_or_else(|| "w-full px-4 py-2.5 text-sm border border-gray-300 dark:border-gray-600 rounded-lg bg-white dark:bg-gray-700 text-gray-900 dark:text-white shadow-sm focus:outline-none focus:ring-2 focus:ring-green-500 focus:border-transparent transition-all duration-200 ease-in-out hover:border-green-500 dark:hover:border-green-500 flex justify-between items-center relative".to_string());
    let label_class = props.label_class.clone().unwrap_or_else(|| "block text-sm font-medium text-gray-700 dark:text-gray-300 mb-2".to_string());
    
    let width_class = props.dropdown_width.clone().unwrap_or_else(|| "w-full".to_string());
    let position_class = props.dropdown_position.clone().unwrap_or_else(|| "left-0".to_string());
    let dropdown_container_class = format!("absolute {} top-full mt-2 rounded-md shadow-lg bg-white dark:bg-gray-800 ring-1 ring-black ring-opacity-5 transition-all duration-200 ease-in-out transform origin-top {dropdown_class} z-[1000] {} {}", position_class, props.dropdown_class, width_class);
    
    let search_input_class = format!("w-full px-3 py-2 text-sm border border-gray-300 dark:border-gray-600 rounded-md bg-white dark:bg-gray-700 text-gray-900 dark:text-white focus:outline-none focus:ring-2 focus:ring-green-500 focus:border-transparent {}", props.search_input_class);
    
    let base_option_class = format!("block w-full text-left px-4 py-2 text-sm text-gray-700 dark:text-gray-200 hover:bg-gray-100 dark:hover:bg-gray-700 transition-colors duration-150 truncate {}", props.option_class);

    rsx! {
        div { 
            class: format!("relative {}", props.class),
            // 遮罩層
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
                        class: "block truncate",
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
                            div { 
                                class: "p-2 border-b border-gray-200 dark:border-gray-700",
                                input {
                                    class: search_input_class,
                                    placeholder: props.search_placeholder,
                                    value: "{search_query}",
                                    oninput: move |e| props.on_search.call(e.value().clone())
                                }
                            }
                            div { 
                                class: "max-h-[clamp(12rem,calc(100vh_-_16rem),24rem)] overflow-y-auto scrollbar-thin scrollbar-thumb-gray-300 dark:scrollbar-thumb-gray-600 scrollbar-track-transparent",
                                {props.options.iter().map(|option| {
                                    let display_value = display_fn(option);
                                    let option_clone = option.clone();
                                    let is_selected = display_value == props.value;
                                    rsx! {
                                        button {
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