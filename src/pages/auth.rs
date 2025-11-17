use crate::enums::route::Route;
use dioxus::events::FormEvent;
use dioxus::prelude::*;

const CARD_CLASS: &str = "max-w-xl mx-auto bg-white dark:bg-gray-800 rounded-xl shadow-lg p-8 space-y-6 border border-gray-100 dark:border-gray-700";
const INPUT_CLASS: &str = "w-full px-4 py-3 rounded-lg border border-gray-300 dark:border-gray-600 focus:ring-2 focus:ring-indigo-500 focus:border-indigo-500 bg-white dark:bg-gray-900 text-gray-900 dark:text-gray-100 transition";
const LABEL_CLASS: &str = "block text-sm font-semibold text-gray-700 dark:text-gray-200 mb-2";
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
            error.set("請填寫姓名與Email，我們才能寄送邀請碼。".to_string());
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
                    p { class: "text-sm font-semibold text-indigo-600", "邀請碼申請" }
                    h1 { class: "text-2xl font-bold text-gray-900 dark:text-gray-100", "取得你的邀請碼" }
                    p { class: "text-gray-600 dark:text-gray-300", "填寫聯絡資訊，我們會將邀請碼寄送到你的信箱。" }
                }
                form { class: "space-y-6", onsubmit: handle_submit,
                    div { class: "space-y-1",
                        label { class: LABEL_CLASS, "姓名" }
                        input {
                            class: INPUT_CLASS,
                            r#type: "text",
                            placeholder: "你的名字",
                            value: "{name}",
                            oninput: move |evt| name.set(evt.value()),
                        }
                    }
                    div { class: "space-y-1",
                        label { class: LABEL_CLASS, "Email" }
                        input {
                            class: INPUT_CLASS,
                            r#type: "email",
                            placeholder: "name@example.com",
                            value: "{email}",
                            oninput: move |evt| email.set(evt.value()),
                        }
                    }
                    div { class: "space-y-1",
                        label { class: LABEL_CLASS, "邀請理由" }
                        textarea {
                            class: INPUT_CLASS,
                            rows: "4",
                            placeholder: "分享你想加入的原因，以及希望在平台完成的目標。",
                            value: "{message}",
                            oninput: move |evt| message.set(evt.value()),
                        }
                        p { class: "text-xs text-gray-500 dark:text-gray-400", "我們會優先處理清楚描述需求的申請。" }
                    }
                    if !error.read().is_empty() {
                        p { class: "text-sm text-red-500", "{error}" }
                    }
                    button { class: BUTTON_CLASS, r#type: "submit", "寄出邀請碼申請" }
                }
                div { class: "flex items-center gap-2 text-sm text-gray-600 dark:text-gray-300",
                    svg {
                        class: "w-5 h-5 text-indigo-500",
                        fill: "none",
                        stroke: "currentColor",
                        stroke_width: "1.5",
                        view_box: "0 0 24 24",
                        path { d: "M12 6v6l3 2", stroke_linecap: "round", stroke_linejoin: "round" }
                    }
                    span { "提交後請於24小時內留意你的Email信箱。" }
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
                    div { class: "mx-auto h-12 w-12 rounded-full bg-indigo-50 dark:bg-indigo-900/40 flex items-center justify-center",
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
                    h1 { class: "text-2xl font-bold text-gray-900 dark:text-gray-100", "已寄出邀請碼" }
                    p { class: "text-gray-600 dark:text-gray-300", "邀請碼已寄送至你的Email信箱，請前往收信並依照指引完成註冊。" }
                }
                div { class: "grid sm:grid-cols-2 gap-3 mt-4",
                    button {
                        class: "w-full py-3 px-4 border border-gray-300 dark:border-gray-700 rounded-lg font-semibold text-gray-700 dark:text-gray-200 hover:bg-gray-50 dark:hover:bg-gray-900 transition",
                        r#type: "button",
                        onclick: move |_| {
                            let _ = navigator_retry
                                .push(Route::InviteRequest { lang: retry_lang.clone() });
                        },
                        "重新填寫"
                    }
                    button {
                        class: BUTTON_CLASS,
                        r#type: "button",
                        onclick: move |_| {
                            let _ = navigator_register
                                .push(Route::Register { lang: register_lang.clone() });
                        },
                        "前往註冊"
                    }
                }
                p { class: "text-sm text-gray-500 dark:text-gray-400 text-center", "沒有收到信件？請確認垃圾郵件或稍後再試。" }
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
            error.set("請輸入邀請碼、姓名與Email。".to_string());
            return;
        }
        if password.read().len() < 8 {
            error.set("密碼長度至少8個字元。".to_string());
            return;
        }
        if password.read().ne(&*confirm.read()) {
            error.set("兩次密碼輸入不一致。".to_string());
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
                    p { class: "text-sm font-semibold text-indigo-600", "註冊帳號" }
                    h1 { class: "text-2xl font-bold text-gray-900 dark:text-gray-100", "使用邀請碼開啟帳戶" }
                    p { class: "text-gray-600 dark:text-gray-300", "輸入邀請碼並設定登入資訊，即可建立專屬帳號。" }
                }
                form { class: "space-y-5", onsubmit: handle_submit,
                    div { class: "space-y-1",
                        label { class: LABEL_CLASS, "邀請碼" }
                        input { class: INPUT_CLASS, r#type: "text", placeholder: "INVITE-XXXX", value: "{code}", oninput: move |evt| code.set(evt.value()) }
                        p { class: "text-xs text-gray-500 dark:text-gray-400", "邀請碼與Email需與信件內容一致。" }
                    }
                    div { class: "grid sm:grid-cols-2 gap-4",
                        div { class: "space-y-1",
                            label { class: LABEL_CLASS, "姓名" }
                            input { class: INPUT_CLASS, r#type: "text", placeholder: "你的名字", value: "{name}", oninput: move |evt| name.set(evt.value()) }
                        }
                        div { class: "space-y-1",
                            label { class: LABEL_CLASS, "Email" }
                            input { class: INPUT_CLASS, r#type: "email", placeholder: "name@example.com", value: "{email}", oninput: move |evt| email.set(evt.value()) }
                        }
                    }
                    div { class: "grid sm:grid-cols-2 gap-4",
                        div { class: "space-y-1",
                            label { class: LABEL_CLASS, "設定密碼" }
                            input { class: INPUT_CLASS, r#type: "password", placeholder: "至少8個字元", value: "{password}", oninput: move |evt| password.set(evt.value()) }
                        }
                        div { class: "space-y-1",
                            label { class: LABEL_CLASS, "再次輸入密碼" }
                            input { class: INPUT_CLASS, r#type: "password", placeholder: "請再次輸入", value: "{confirm}", oninput: move |evt| confirm.set(evt.value()) }
                        }
                    }
                    if !error.read().is_empty() {
                        p { class: "text-sm text-red-500", "{error}" }
                    }
                    button { class: BUTTON_CLASS, r#type: "submit", "建立帳號" }
                }
                div { class: "flex items-center justify-between text-sm text-gray-600 dark:text-gray-300",
                    span { "已經有帳號？" }
                    button {
                        class: "text-indigo-600 hover:text-indigo-700 font-semibold",
                        r#type: "button",
                        onclick: move |_| {
                            let _ = login_link_nav.push(Route::Login { lang: login_link_lang.clone() });
                        },
                        "立即登入"
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
            error.set("請輸入Email與密碼。".to_string());
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
                    p { class: "text-sm font-semibold text-indigo-600", "登入" }
                    h1 { class: "text-2xl font-bold text-gray-900 dark:text-gray-100", "使用邀請碼登入" }
                    p { class: "text-gray-600 dark:text-gray-300", "輸入Email、密碼與邀請碼即可登入，未申請邀請碼請先填寫申請表。" }
                }
                form { class: "space-y-5", onsubmit: handle_submit,
                    div { class: "space-y-1",
                        label { class: LABEL_CLASS, "Email" }
                        input { class: INPUT_CLASS, r#type: "email", placeholder: "name@example.com", value: "{email}", oninput: move |evt| email.set(evt.value()) }
                    }
                    div { class: "space-y-1",
                        label { class: LABEL_CLASS, "密碼" }
                        input { class: INPUT_CLASS, r#type: "password", placeholder: "你的密碼", value: "{password}", oninput: move |evt| password.set(evt.value()) }
                    }
                    div { class: "space-y-1",
                        label { class: LABEL_CLASS, "邀請碼 (可選)" }
                        input { class: INPUT_CLASS, r#type: "text", placeholder: "INVITE-XXXX", value: "{code}", oninput: move |evt| code.set(evt.value()) }
                        p { class: "text-xs text-gray-500 dark:text-gray-400", "若帳號綁定邀請碼，請一併輸入以便快速驗證。" }
                    }
                    if !error.read().is_empty() {
                        p { class: "text-sm text-red-500", "{error}" }
                    }
                    button { class: BUTTON_CLASS, r#type: "submit", "登入" }
                }
                div { class: "grid sm:grid-cols-2 gap-3 text-sm text-gray-600 dark:text-gray-300",
                    button {
                        class: "py-2 text-left text-indigo-600 hover:text-indigo-700 font-semibold",
                        r#type: "button",
                        onclick: move |_| {
                            let _ = invite_nav.push(Route::InviteRequest { lang: invite_lang.clone() });
                        },
                        "尚未申請邀請碼"
                    }
                    button {
                        class: "py-2 text-right text-indigo-600 hover:text-indigo-700 font-semibold",
                        r#type: "button",
                        onclick: move |_| {
                            let _ = register_nav.push(Route::Register { lang: register_lang.clone() });
                        },
                        "沒有帳號？立即註冊"
                    }
                }
            }
        }
    }
}
