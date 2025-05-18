use dioxus::prelude::*;

#[component]
pub fn Toast(
    visible: bool,
    message: String,
) -> Element {
    let slide_class = if visible { 
        "translate-y-0 opacity-100" 
    } else { 
        "translate-y-full opacity-0" 
    };
    
    rsx! {
        div {
            class: "fixed bottom-4 right-4 bg-green-500 text-white px-6 py-3 rounded shadow-lg z-50 \
                   transform transition-transform transition-opacity duration-300 ease-in-out {slide_class}",
            "{message}"
        }
    }
} 