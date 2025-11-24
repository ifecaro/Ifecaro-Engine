use crate::enums::route::Route;
use dioxus::events::FormEvent;
use dioxus::prelude::*;
use dioxus_i18n::t;

const CARD_CLASS: &str = "max-w-xl mx-auto bg-white dark:bg-gray-800 paper:bg-[#fef8e7] paper:text-[#1f2937] rounded-xl shadow-lg p-8 space-y-6 border border-gray-100 dark:border-gray-700 paper:border-[#e4d5b2]";
const INPUT_CLASS: &str = "w-full px-4 py-3 rounded-lg border border-gray-300 dark:border-gray-600 paper:border-[#e4d5b2] focus:ring-2 focus:ring-indigo-500 focus:border-indigo-500 bg-white dark:bg-gray-900 paper:bg-[#fef8e7] text-gray-900 dark:text-gray-100 paper:text-[#1f2937] transition";
const LABEL_CLASS: &str = "block text-sm font-semibold text-gray-700 dark:text-gray-200 paper:text-[#374151] mb-2";
const BUTTON_CLASS: &str = "w-full py-3 px-4 bg-indigo-600 hover:bg-indigo-700 text-white font-semibold rounded-lg shadow transition disabled:opacity-60 disabled:cursor-not-allowed";

#[derive(Props, Clone, PartialEq)]
pub struct InviteRequestProps {
    pub lang: String,
}

#[component]
pub fn InviteRequest(props: InviteRequestProps) -> Element {
    let mut name = use_signal(String::new);
    let mut email = use_signal(String::new);
    let mut message = use_signal(String::new);
    let mut error = use_signal(String::new);
    let navigator = use_navigator();

    let handle_submit = move |evt: FormEvent| {
        evt.prevent_default();
        let has_missing = name.read().is_empty() || email.read().is_empty();
        if has_missing {
            error.set(t!("invite_request_error_required"));
            return;
        }

        error.set(String::new());
        let lang = props.lang.clone();
        let nav = navigator.clone();
        nav.push(Route::InviteCheckEmail { lang });
    };

    rsx! {
        section { class: "py-10",
            div { class: CARD_CLASS,
                div { class: "space-y-2",
                    p { class: "text-sm font-semibold text-indigo-600", "{t!(\"invite_request\")}" }
                    h1 { class: "text-2xl font-bold text-gray-900 dark:text-gray-100", "{t!(\"invite_request_title\")}" }
                    p { class: "text-gray-600 dark:text-gray-300 paper:text-[#374151]", "{t!(\"invite_request_description\")}" }
                }
                form { class: "space-y-6", onsubmit: handle_submit,
                    div { class: "space-y-1",
                        label { class: LABEL_CLASS, "{t!(\"invite_request_name_label\")}" }
                        input {
                            class: INPUT_CLASS,
                            r#type: "text",
                            placeholder: "{t!(\"invite_request_name_placeholder\")}",
                            value: "{name}",
                            oninput: move |evt| name.set(evt.value()),
                        }
                    }
                    div { class: "space-y-1",
                        label { class: LABEL_CLASS, "{t!(\"invite_request_email_label\")}" }
                        input {
                            class: INPUT_CLASS,
                            r#type: "email",
                            placeholder: "{t!(\"invite_request_email_placeholder\")}",
                            value: "{email}",
                            oninput: move |evt| email.set(evt.value()),
                        }
                    }
                    div { class: "space-y-1",
                        label { class: LABEL_CLASS, "{t!(\"invite_request_reason_label\")}" }
                        textarea {
                            class: INPUT_CLASS,
                            rows: "4",
                            placeholder: "{t!(\"invite_request_reason_placeholder\")}",
                            value: "{message}",
                            oninput: move |evt| message.set(evt.value()),
                        }
                        p { class: "text-xs text-gray-500 dark:text-gray-400 paper:text-[#4b5563]", "{t!(\"invite_request_reason_helper\")}" }
                    }
                    if !error.read().is_empty() {
                        p { class: "text-sm text-red-500", "{error}" }
                    }
                    button { class: BUTTON_CLASS, r#type: "submit", "{t!(\"invite_request_submit\")}" }
                }
                div { class: "flex items-center gap-2 text-sm text-gray-600 dark:text-gray-300 paper:text-[#374151]",
                    svg {
                        class: "w-5 h-5 text-indigo-500 paper:text-[#1f2937]",
                        fill: "none",
                        stroke: "currentColor",
                        stroke_width: "1.5",
                        view_box: "0 0 24 24",
                        path { d: "M12 6v6l3 2", stroke_linecap: "round", stroke_linejoin: "round" }
                    }
                    span { "{t!(\"invite_request_notice\")}" }
                }
            }
        }
    }
}

#[derive(Props, Clone, PartialEq)]
pub struct InviteCheckEmailProps {
    pub lang: String,
}

#[component]
pub fn InviteCheckEmail(props: InviteCheckEmailProps) -> Element {
    let navigator = use_navigator();
    let navigator_retry = navigator.clone();
    let navigator_register = navigator.clone();
    let retry_lang = props.lang.clone();
    let register_lang = props.lang.clone();

    rsx! {
        section { class: "py-10",
            div { class: CARD_CLASS,
                div { class: "space-y-3 text-center",
                    div { class: "mx-auto h-12 w-12 rounded-full bg-indigo-50 dark:bg-indigo-900/40 paper:bg-[#eae0c9] paper:text-[#1f2937] flex items-center justify-center",
                        svg {
                            class: "w-6 h-6 text-indigo-600 dark:text-indigo-300",
                            fill: "none",
                            stroke: "currentColor",
                            stroke_width: "1.5",
                            view_box: "0 0 24 24",
                            path { d: "M3 8l9 6 9-6", stroke_linecap: "round", stroke_linejoin: "round" }
                            path { d: "M5 6h14a2 2 0 0 1 2 2v8a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2V8a2 2 0 0 1 2-2z", stroke_linecap: "round", stroke_linejoin: "round" }
                        }
                    }
                    h1 { class: "text-2xl font-bold text-gray-900 dark:text-gray-100", "{t!(\"invite_check_email_title\")}" }
                    p { class: "text-gray-600 dark:text-gray-300 paper:text-[#374151]", "{t!(\"invite_check_email_description\")}" }
                }
                div { class: "grid sm:grid-cols-2 gap-3 mt-4",
                    button {
                        class: "w-full py-3 px-4 border border-gray-300 dark:border-gray-700 rounded-lg font-semibold text-gray-700 dark:text-gray-200 hover:bg-gray-50 dark:hover:bg-gray-900 transition",
                        r#type: "button",
                        onclick: move |_| {
                            let _ = navigator_retry
                                .push(Route::InviteRequest { lang: retry_lang.clone() });
                        },
                        "{t!(\"invite_check_email_retry\")}" 
                    }
                    button {
                        class: BUTTON_CLASS,
                        r#type: "button",
                        onclick: move |_| {
                            let _ = navigator_register
                                .push(Route::Register { lang: register_lang.clone() });
                        },
                        "{t!(\"invite_check_email_register\")}" 
                    }
                }
                p { class: "text-sm text-gray-500 dark:text-gray-400 paper:text-[#4b5563] text-center", "{t!(\"invite_check_email_helper\")}" }
            }
        }
    }
}

#[derive(Props, Clone, PartialEq)]
pub struct RegisterProps {
    pub lang: String,
}

#[component]
pub fn Register(props: RegisterProps) -> Element {
    let mut code = use_signal(String::new);
    let mut name = use_signal(String::new);
    let mut email = use_signal(String::new);
    let mut password = use_signal(String::new);
    let mut confirm = use_signal(String::new);
    let mut error = use_signal(String::new);
    let navigator = use_navigator();
    let submit_nav = navigator.clone();
    let login_link_nav = navigator.clone();
    let submit_lang = props.lang.clone();
    let login_link_lang = props.lang.clone();

    let handle_submit = move |evt: FormEvent| {
        evt.prevent_default();
        if code.read().is_empty() || name.read().is_empty() || email.read().is_empty() {
            error.set(t!("register_error_missing"));
            return;
        }
        if password.read().len() < 8 {
            error.set(t!("register_error_password_length"));
            return;
        }
        if password.read().ne(&*confirm.read()) {
            error.set(t!("register_error_password_match"));
            return;
        }
        error.set(String::new());
        let lang = submit_lang.clone();
        let _ = submit_nav.push(Route::Login { lang });
    };

    rsx! {
        section { class: "py-10",
            div { class: CARD_CLASS,
                div { class: "space-y-2 text-center",
                    p { class: "text-sm font-semibold text-indigo-600", "{t!(\"register_badge\")}" }
                    h1 { class: "text-2xl font-bold text-gray-900 dark:text-gray-100", "{t!(\"register_title\")}" }
                    p { class: "text-gray-600 dark:text-gray-300 paper:text-[#374151]", "{t!(\"register_description\")}" }
                }
                form { class: "space-y-5", onsubmit: handle_submit,
                    div { class: "space-y-1",
                        label { class: LABEL_CLASS, "{t!(\"register_code_label\")}" }
                        input { class: INPUT_CLASS, r#type: "text", placeholder: "{t!(\"register_code_placeholder\")}", value: "{code}", oninput: move |evt| code.set(evt.value()) }
                        p { class: "text-xs text-gray-500 dark:text-gray-400 paper:text-[#4b5563]", "{t!(\"register_code_helper\")}" }
                    }
                    div { class: "grid sm:grid-cols-2 gap-4",
                        div { class: "space-y-1",
                            label { class: LABEL_CLASS, "{t!(\"register_name_label\")}" }
                            input { class: INPUT_CLASS, r#type: "text", placeholder: "{t!(\"register_name_placeholder\")}", value: "{name}", oninput: move |evt| name.set(evt.value()) }
                        }
                        div { class: "space-y-1",
                            label { class: LABEL_CLASS, "{t!(\"register_email_label\")}" }
                            input { class: INPUT_CLASS, r#type: "email", placeholder: "{t!(\"register_email_placeholder\")}", value: "{email}", oninput: move |evt| email.set(evt.value()) }
                        }
                    }
                    div { class: "grid sm:grid-cols-2 gap-4",
                        div { class: "space-y-1",
                            label { class: LABEL_CLASS, "{t!(\"register_password_label\")}" }
                            input { class: INPUT_CLASS, r#type: "password", placeholder: "{t!(\"register_password_placeholder\")}", value: "{password}", oninput: move |evt| password.set(evt.value()) }
                        }
                        div { class: "space-y-1",
                            label { class: LABEL_CLASS, "{t!(\"register_confirm_label\")}" }
                            input { class: INPUT_CLASS, r#type: "password", placeholder: "{t!(\"register_confirm_placeholder\")}", value: "{confirm}", oninput: move |evt| confirm.set(evt.value()) }
                        }
                    }
                    if !error.read().is_empty() {
                        p { class: "text-sm text-red-500", "{error}" }
                    }
                    button { class: BUTTON_CLASS, r#type: "submit", "{t!(\"register_submit\")}" }
                }
                div { class: "flex items-center justify-between text-sm text-gray-600 dark:text-gray-300 paper:text-[#374151]",
                    span { "{t!(\"register_login_prompt\")}" }
                    button {
                        class: "text-indigo-600 hover:text-indigo-700 font-semibold",
                        r#type: "button",
                        onclick: move |_| {
                            let _ = login_link_nav.push(Route::Login { lang: login_link_lang.clone() });
                        },
                        "{t!(\"register_login_link\")}" 
                    }
                }
            }
        }
    }
}

#[derive(Props, Clone, PartialEq)]
pub struct LoginProps {
    pub lang: String,
}

#[component]
pub fn Login(props: LoginProps) -> Element {
    let mut email = use_signal(String::new);
    let mut password = use_signal(String::new);
    let mut code = use_signal(String::new);
    let mut error = use_signal(String::new);

    let navigator = use_navigator();
    let submit_nav = navigator.clone();
    let invite_nav = navigator.clone();
    let register_nav = navigator.clone();
    let submit_lang = props.lang.clone();
    let invite_lang = props.lang.clone();
    let register_lang = props.lang.clone();

    let handle_submit = move |evt: FormEvent| {
        evt.prevent_default();
        if email.read().is_empty() || password.read().is_empty() {
            error.set(t!("login_error_missing"));
            return;
        }
        error.set(String::new());
        // 後端驗證成功後即可導向故事體驗或儀表板
        let _ = submit_nav.push(Route::Story { lang: submit_lang.clone() });
    };

    rsx! {
        section { class: "py-10",
            div { class: CARD_CLASS,
                div { class: "space-y-2 text-center",
                    p { class: "text-sm font-semibold text-indigo-600", "{t!(\"login_badge\")}" }
                    h1 { class: "text-2xl font-bold text-gray-900 dark:text-gray-100", "{t!(\"login_title\")}" }
                    p { class: "text-gray-600 dark:text-gray-300 paper:text-[#374151]", "{t!(\"login_description\")}" }
                }
                form { class: "space-y-5", onsubmit: handle_submit,
                    div { class: "space-y-1",
                        label { class: LABEL_CLASS, "{t!(\"login_email_label\")}" }
                        input { class: INPUT_CLASS, r#type: "email", placeholder: "{t!(\"login_email_placeholder\")}", value: "{email}", oninput: move |evt| email.set(evt.value()) }
                    }
                    div { class: "space-y-1",
                        label { class: LABEL_CLASS, "{t!(\"login_password_label\")}" }
                        input { class: INPUT_CLASS, r#type: "password", placeholder: "{t!(\"login_password_placeholder\")}", value: "{password}", oninput: move |evt| password.set(evt.value()) }
                    }
                    div { class: "space-y-1",
                        label { class: LABEL_CLASS, "{t!(\"login_code_label\")}" }
                        input { class: INPUT_CLASS, r#type: "text", placeholder: "{t!(\"login_code_placeholder\")}", value: "{code}", oninput: move |evt| code.set(evt.value()) }
                        p { class: "text-xs text-gray-500 dark:text-gray-400 paper:text-[#4b5563]", "{t!(\"login_code_helper\")}" }
                    }
                    if !error.read().is_empty() {
                        p { class: "text-sm text-red-500", "{error}" }
                    }
                    button { class: BUTTON_CLASS, r#type: "submit", "{t!(\"login_submit\")}" }
                }
                div { class: "grid sm:grid-cols-2 gap-3 text-sm text-gray-600 dark:text-gray-300 paper:text-[#374151]",
                    button {
                        class: "py-2 text-left text-indigo-600 hover:text-indigo-700 font-semibold",
                        r#type: "button",
                        onclick: move |_| {
                            let _ = invite_nav.push(Route::InviteRequest { lang: invite_lang.clone() });
                        },
                        "{t!(\"login_invite_link\")}" 
                    }
                    button {
                        class: "py-2 text-right text-indigo-600 hover:text-indigo-700 font-semibold",
                        r#type: "button",
                        onclick: move |_| {
                            let _ = register_nav.push(Route::Register { lang: register_lang.clone() });
                        },
                        "{t!(\"login_register_link\")}" 
                    }
                }
            }
        }
    }
}
