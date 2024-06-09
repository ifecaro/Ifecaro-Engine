use dioxus::{
    hooks::use_signal_sync,
    prelude::{component, dioxus_core, dioxus_elements, fc_to_builder, rsx, Element, IntoDynNode},
    signals::{Readable, Writable, WritableVecExt},
};

#[component]
pub fn Dashboard() -> Element {
    let mut choices = use_signal_sync(|| vec![""]);

    rsx! {
        crate::pages::layout::Layout { title: "Dashboard",

            div { class: "mb-5",
                label { class: "block mb-2 text-sm font-medium text-gray-900 dark:text-white",
                    "Language"
                }
                select {
                    class: "bg-gray-50 border border-gray-300 text-gray-900 text-sm rounded-lg focus:ring-blue-500 focus:border-blue-500 block w-full p-2.5 dark:bg-gray-700 dark:border-gray-600 dark:placeholder-gray-400 dark:text-white dark:focus:ring-blue-500 dark:focus:border-blue-500",
                    disabled: true,
                    option { "English" }
                }
            }

            div { class: "mb-5 text-2xl font-semibold tracking-tight text-gray-900 dark:text-white group",
                "English"
            }
            div { class: "mb-5",
                label { class: "block mb-2 text-sm font-medium text-gray-900 dark:text-white",
                    "Paragraph"
                }
                textarea {
                    class: "block p-2.5 w-full text-sm text-gray-900 bg-gray-50 rounded-lg border border-gray-300 focus:ring-blue-500 focus:border-blue-500 dark:bg-gray-700 dark:border-gray-600 dark:placeholder-gray-400 dark:text-white dark:focus:ring-blue-500 dark:focus:border-blue-500",
                    rows: 10
                }
            }
            div { class: "mb-5",
                label { class: "block mb-2 text-sm font-medium text-gray-900 dark:text-white",
                    "Choices"
                }
                article { class: "prose dark:prose-invert lg:prose-xl indent-10",
                    ol {
                        {
                            choices.read().iter().map(|choice| {
                                return rsx!{
                                            li {
                                                class: "!ml-0",
                                                input {
                                                    class: "bg-gray-50 border border-gray-300 text-gray-900 text-sm rounded-lg focus:ring-blue-500 focus:border-blue-500 block w-full p-2.5 dark:bg-gray-700 dark:border-gray-600 dark:placeholder-gray-400 dark:text-white dark:focus:ring-blue-500 dark:focus:border-blue-500"
                                                }
                                            }
                                }
                            })
                        }
                    }
                }

                button {
                    class: "text-gray-900 hover:text-white border border-gray-800 hover:bg-gray-900 focus:ring-4 focus:outline-none focus:ring-gray-300 font-medium rounded-lg text-sm px-5 py-2.5 text-center me-2 mb-2 dark:border-gray-600 dark:text-gray-400 dark:hover:text-white dark:hover:bg-gray-600 dark:focus:ring-gray-800",
                    onclick: move |_| {
                        choices.push("");
                    },
                    "Add a new choice"
                }
            }

            div { class: "mb-5 grid justify-center",
                button { class: "text-gray-900 bg-white border border-gray-300 focus:outline-none hover:bg-gray-100 focus:ring-4 focus:ring-gray-100 font-medium rounded-full text-sm px-5 py-2.5 me-2 mb-2 dark:bg-gray-800 dark:text-white dark:border-gray-600 dark:hover:bg-gray-700 dark:hover:border-gray-600 dark:focus:ring-gray-700",
                    "Submit"
                }
            }
        }
    }
}
