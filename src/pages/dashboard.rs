use dioxus::prelude::*;
use serde::{Deserialize, Serialize};
use wasm_bindgen_futures::spawn_local;
use wasm_bindgen::prelude::*;
use crate::constants::config::config::{BASE_API_URL, SETTINGS};
use crate::enums::translations::Translations;
use crate::components::toast::Toast;
use crate::components::form::{InputField, TextareaField, ChoiceOptions};
use crate::components::story_content::Choice;
use dioxus::events::{FormEvent, FocusEvent};

#[derive(Props, Clone, PartialEq)]
pub struct DashboardProps {
    lang: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct Text {
    lang: String,
    paragraphs: String,
    choices: Vec<Choice>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct Paragraph {
    index: usize,
    choice_id: String,
    texts: Vec<Text>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct Data {
    items: Vec<Paragraph>,
}

#[derive(Clone, PartialEq)]
struct Language {
    code: &'static str,
    name: &'static str,
}

const AVAILABLE_LANGUAGES: &[Language] = &[
    Language { code: "zh-TW", name: "繁體中文" },
    Language { code: "zh-CN", name: "簡體中文" },
    Language { code: "en", name: "English" },
    Language { code: "ja", name: "日本語" },
    Language { code: "ko", name: "한국어" },
    Language { code: "es", name: "Español" },
    Language { code: "fr", name: "Français" },
    Language { code: "de", name: "Deutsch" },
    Language { code: "it", name: "Italiano" },
    Language { code: "pt", name: "Português" },
    Language { code: "ru", name: "Русский" },
    Language { code: "ar", name: "العربية" },
    Language { code: "hi", name: "हिंदी" },
    Language { code: "bn", name: "বাংলা" },
    Language { code: "id", name: "Bahasa Indonesia" },
    Language { code: "ms", name: "Bahasa Melayu" },
    Language { code: "th", name: "ไทย" },
    Language { code: "vi", name: "Tiếng Việt" },
    Language { code: "nl", name: "Nederlands" },
    Language { code: "pl", name: "Polski" },
    Language { code: "uk", name: "Українська" },
    Language { code: "el", name: "Ελληνικά" },
    Language { code: "he", name: "עברית" },
    Language { code: "tr", name: "Türkçe" },
    Language { code: "sv", name: "Svenska" },
    Language { code: "da", name: "Dansk" },
    Language { code: "fi", name: "Suomi" },
    Language { code: "no", name: "Norsk" },
    Language { code: "cs", name: "Čeština" },
    Language { code: "ro", name: "Română" },
    Language { code: "hu", name: "Magyar" },
    Language { code: "sk", name: "Slovenčina" },
    Language { code: "hr", name: "Hrvatski" },
    Language { code: "ca", name: "Català" },
    Language { code: "fil", name: "Filipino" },
    Language { code: "fa", name: "فارسی" },
    Language { code: "lv", name: "Latviešu" },
    Language { code: "af", name: "Afrikaans" },
    Language { code: "sw", name: "Kiswahili" },
    Language { code: "ga", name: "Gaeilge" },
    Language { code: "et", name: "Eesti" },
    Language { code: "eu", name: "Euskara" },
    Language { code: "is", name: "Íslenska" },
    Language { code: "mk", name: "Македонски" },
    Language { code: "hy", name: "Հայերեն" },
    Language { code: "ne", name: "नेपाली" },
    Language { code: "lb", name: "Lëtzebuergesch" },
    Language { code: "my", name: "မြန်မာဘာသာ" },
    Language { code: "gl", name: "Galego" },
    Language { code: "mr", name: "मराठी" },
    Language { code: "ka", name: "ქართული" },
    Language { code: "mn", name: "Монгол" },
    Language { code: "si", name: "සිංහල" },
    Language { code: "km", name: "ខ្មែរ" },
    Language { code: "sn", name: "chiShona" },
    Language { code: "yo", name: "Yorùbá" },
    Language { code: "so", name: "Soomaali" },
    Language { code: "ha", name: "Hausa" },
    Language { code: "zu", name: "isiZulu" },
    Language { code: "xh", name: "isiXhosa" },
    Language { code: "am", name: "አማርኛ" },
    Language { code: "be", name: "Беларуская" },
    Language { code: "az", name: "Azərbaycan" },
    Language { code: "uz", name: "O'zbek" },
    Language { code: "kk", name: "Қазақ" },
    Language { code: "ky", name: "Кыргызча" },
    Language { code: "tg", name: "Тоҷикӣ" },
    Language { code: "tk", name: "Türkmen" },
    Language { code: "ur", name: "اردو" },
    Language { code: "pa", name: "ਪੰਜਾਬੀ" },
    Language { code: "gu", name: "ગુજરાતી" },
    Language { code: "or", name: "ଓଡ଼ିଆ" },
    Language { code: "ta", name: "தமிழ்" },
    Language { code: "te", name: "తెలుగు" },
    Language { code: "kn", name: "ಕನ್ನಡ" },
    Language { code: "ml", name: "മലയാളം" },
    Language { code: "as", name: "অসমীয়া" },
    Language { code: "mai", name: "मैथिली" },
    Language { code: "mni", name: "মৈতৈলোন্" },
    Language { code: "doi", name: "डोगरी" },
    Language { code: "bho", name: "भोजपुरी" },
    Language { code: "sat", name: "ᱥᱟᱱᱛᱟᱲᱤ" },
    Language { code: "ks", name: "کٲشُر" },
    Language { code: "sa", name: "संस्कृतम्" },
    Language { code: "sd", name: "سنڌي" },
    Language { code: "kok", name: "कोंकणी" },
    Language { code: "gom", name: "कोंकणी" },
];

#[component]
pub fn Dashboard(props: DashboardProps) -> Element {
    let mut choices = use_signal(|| Vec::<Choice>::new());
    let mut choice_id = use_signal(|| String::new());
    let mut paragraphs = use_signal(|| String::new());
    let mut new_caption = use_signal(|| String::new());
    let mut new_goto = use_signal(|| String::new());
    let mut extra_captions = use_signal(|| Vec::<String>::new());
    let mut extra_gotos = use_signal(|| Vec::<String>::new());
    let mut show_extra_options = use_signal(|| Vec::<()>::new());
    let mut show_toast = use_signal(|| false);
    let mut toast_visible = use_signal(|| false);
    let mut selected_lang = use_signal(|| props.lang.clone());
    let mut is_open = use_signal(|| false);
    let mut search_query = use_signal(|| String::new());
    let t = Translations::get(&props.lang);

    let mut choice_id_error = use_signal(|| false);
    let mut paragraphs_error = use_signal(|| false);
    let mut new_caption_error = use_signal(|| false);
    let mut new_goto_error = use_signal(|| false);

    let filtered_languages = use_memo(move || {
        AVAILABLE_LANGUAGES.iter()
            .filter(|l| {
                let query = search_query.read().to_lowercase();
                l.name.to_lowercase().contains(&query) || 
                l.code.to_lowercase().contains(&query)
            })
            .collect::<Vec<_>>()
    });

    let dropdown_class = if *is_open.read() {
        "translate-y-0 opacity-100"
    } else {
        "-translate-y-2 opacity-0 pointer-events-none"
    };

    let current_language = AVAILABLE_LANGUAGES.iter()
        .find(|l| l.code == *selected_lang.read())
        .map(|l| l.name)
        .unwrap_or("繁體中文");

    let validate_field = |value: &str, error_signal: &mut Signal<bool>| {
        if value.trim().is_empty() {
            error_signal.set(true);
        } else {
            error_signal.set(false);
        }
    };

    let handle_submit = move |evt: Event<FormData>| {
        evt.stop_propagation();
        
        if choice_id.read().trim().is_empty() {
            return;
        }
        
        if paragraphs.read().trim().is_empty() {
            return;
        }
        
        if new_caption.read().trim().is_empty() || new_goto.read().trim().is_empty() {
            return;
        }

        let mut all_choices = Vec::new();
        
        if !new_caption.read().is_empty() && !new_goto.read().is_empty() {
            all_choices.push(Choice {
                caption: new_caption.read().clone(),
                goto: new_goto.read().clone(),
            });
        }

        for i in 0..extra_captions.read().len() {
            let caption = &extra_captions.read()[i];
            let goto = &extra_gotos.read()[i];
            if !caption.is_empty() && !goto.is_empty() {
                all_choices.push(Choice {
                    caption: caption.clone(),
                    goto: goto.clone(),
                });
            }
        }

        let text = Text {
            lang: selected_lang.read().clone(),
            paragraphs: paragraphs.read().clone(),
            choices: all_choices,
        };

        spawn_local(async move {
            let client = reqwest::Client::new();
            let url = format!("{}{}", BASE_API_URL, SETTINGS);
            
            match client.get(&url).send().await {
                Ok(response) => {
                    if let Ok(data) = response.json::<Data>().await {
                        let max_index = data.items.iter()
                            .map(|item| item.index)
                            .max()
                            .unwrap_or(0);
                        
                        let record = Paragraph {
                            index: max_index + 1,
                            choice_id: choice_id.read().clone(),
                            texts: vec![text],
                        };

                        match client.post(&url).json(&record).send().await {
                            Ok(response) => {
                                if response.status().is_success() {
                                    choice_id.set(String::new());
                                    paragraphs.set(String::new());
                                    choices.write().clear();
                                    new_caption.set(String::new());
                                    new_goto.set(String::new());
                                    extra_captions.write().clear();
                                    extra_gotos.write().clear();
                                    show_extra_options.write().clear();
                                    show_toast.set(true);
                                    
                                    let mut toast_visible = toast_visible.clone();
                                    spawn_local(async move {
                                        let window = web_sys::window().unwrap();
                                        let promise = js_sys::Promise::new(&mut |resolve, _| {
                                            window
                                                .set_timeout_with_callback_and_timeout_and_arguments_0(
                                                    &resolve,
                                                    50,
                                                )
                                                .unwrap();
                                        });
                                        let _ = wasm_bindgen_futures::JsFuture::from(promise).await;
                                        toast_visible.set(true);
                                    });
                                    
                                    let mut show_toast = show_toast.clone();
                                    let mut toast_visible = toast_visible.clone();
                                    spawn_local(async move {
                                        let window = web_sys::window().unwrap();
                                        let promise = js_sys::Promise::new(&mut |resolve, _| {
                                            window
                                                .set_timeout_with_callback_and_timeout_and_arguments_0(
                                                    &resolve,
                                                    2700,
                                                )
                                                .unwrap();
                                        });
                                        let _ = wasm_bindgen_futures::JsFuture::from(promise).await;
                                        
                                        toast_visible.set(false);
                                        
                                        let promise = js_sys::Promise::new(&mut |resolve, _| {
                                            window
                                                .set_timeout_with_callback_and_timeout_and_arguments_0(
                                                    &resolve,
                                                    300,
                                                )
                                                .unwrap();
                                        });
                                        let _ = wasm_bindgen_futures::JsFuture::from(promise).await;
                                        show_toast.set(false);
                                    });
                                }
                            }
                            Err(_) => {}
                        }
                    }
                }
                Err(_) => {}
            }
        });
    };

    let is_form_valid = move || {
        !choice_id.read().trim().is_empty() &&
        !paragraphs.read().trim().is_empty() &&
        !new_caption.read().trim().is_empty() &&
        !new_goto.read().trim().is_empty()
    };
    
    rsx! {
        crate::pages::layout::Layout { 
            title: Some("Dashboard"),
            {show_toast.read().then(|| {
                rsx!(
                    Toast {
                        visible: *toast_visible.read(),
                        message: t.submit_success.to_string()
                    }
                )
            })}
            form { 
                class: "max-w-3xl mx-auto p-8 bg-white dark:bg-gray-800 rounded-lg shadow-lg border border-gray-100 dark:border-gray-700",
                onsubmit: handle_submit,
                "onsubmit": "event.preventDefault();",
                
                div { class: "space-y-8",
                    div { class: "relative",
                        label { 
                            class: "block text-sm font-medium text-gray-700 dark:text-gray-300 mb-2",
                            "{t.select_language}"
                        }
                        div { 
                            class: "relative inline-block w-full",
                            button {
                                class: "w-full px-4 py-2.5 text-base border border-gray-300 dark:border-gray-600 rounded-lg bg-white dark:bg-gray-700 text-gray-900 dark:text-white shadow-sm focus:outline-none focus:ring-2 focus:ring-green-500 focus:border-transparent cursor-pointer transition-all duration-200 ease-in-out hover:border-green-500 dark:hover:border-green-500 flex justify-between items-center",
                                onclick: move |_| {
                                    let current = *is_open.read();
                                    is_open.set(!current);
                                },
                                span { "{current_language}" }
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
                                class: "absolute right-0 mt-2 w-full rounded-md shadow-lg bg-white dark:bg-gray-800 ring-1 ring-black ring-opacity-5 transition-all duration-200 ease-in-out transform origin-top-right {dropdown_class}",
                                div { 
                                    class: "p-2 border-b border-gray-200 dark:border-gray-700",
                                    input {
                                        class: "w-full px-3 py-2 text-sm border border-gray-300 dark:border-gray-600 rounded-md bg-white dark:bg-gray-700 text-gray-900 dark:text-white focus:outline-none focus:ring-2 focus:ring-green-500 focus:border-transparent",
                                        placeholder: "搜尋語言...",
                                        value: search_query.read().to_string(),
                                        oninput: move |evt| search_query.set(evt.value().to_string()),
                                    }
                                }
                                div { 
                                    class: "max-h-[calc(100vh_-_25rem)] overflow-y-auto scrollbar-thin scrollbar-thumb-gray-300 dark:scrollbar-thumb-gray-600 scrollbar-track-transparent",
                                    {filtered_languages.read().iter().map(|language| {
                                        let lang_code = language.code.to_string();
                                        let lang_name = language.name;
                                        rsx! {
                                            button {
                                                class: "block w-full text-left px-4 py-2 text-sm text-gray-700 dark:text-gray-200 hover:bg-gray-100 dark:hover:bg-gray-700 transition-colors duration-150",
                                                onclick: move |_| {
                                                    selected_lang.set(lang_code.clone());
                                                    is_open.set(false);
                                                    search_query.set(String::new());
                                                },
                                                {lang_name}
                                            }
                                        }
                                    })}
                                }
                            }
                        }
                    }

                    InputField {
                        label: t.choice_id,
                        placeholder: t.choice_id,
                        value: choice_id.read().to_string(),
                        required: true,
                        has_error: *choice_id_error.read(),
                        on_input: move |evt: FormEvent| {
                            choice_id.set(evt.value().clone());
                            validate_field(&evt.value(), &mut choice_id_error);
                        },
                        on_blur: move |evt: FocusEvent| validate_field(&choice_id.read(), &mut choice_id_error)
                    }

                    div { class: "space-y-2",
                        TextareaField {
                            label: t.paragraph,
                            placeholder: t.paragraph,
                            value: paragraphs.read().to_string(),
                            required: true,
                            has_error: *paragraphs_error.read(),
                            rows: 6,
                            on_input: move |evt: FormEvent| {
                                paragraphs.set(evt.value().clone());
                                validate_field(&evt.value(), &mut paragraphs_error);
                            },
                            on_blur: move |evt: FocusEvent| validate_field(&paragraphs.read(), &mut paragraphs_error)
                        }
                    }

                    ChoiceOptions {
                        t: t.clone(),
                        new_caption: new_caption.read().to_string(),
                        new_goto: new_goto.read().to_string(),
                        extra_captions: extra_captions.read().clone(),
                        extra_gotos: extra_gotos.read().clone(),
                        new_caption_error: *new_caption_error.read(),
                        new_goto_error: *new_goto_error.read(),
                        on_new_caption_change: move |evt: FormEvent| {
                            let value = evt.value().to_string();
                            validate_field(&value, &mut new_caption_error);
                            new_caption.set(value);
                        },
                        on_new_goto_change: move |evt: FormEvent| {
                            let value = evt.value().to_string();
                            validate_field(&value, &mut new_goto_error);
                            new_goto.set(value);
                        },
                        on_extra_caption_change: move |(i, evt): (usize, FormEvent)| {
                            let mut captions = extra_captions.write();
                            captions[i] = evt.value().to_string();
                        },
                        on_extra_goto_change: move |(i, evt): (usize, FormEvent)| {
                            let mut gotos = extra_gotos.write();
                            gotos[i] = evt.value().to_string();
                        },
                        on_add_choice: move |_| {
                            show_extra_options.write().push(());
                            extra_captions.write().push(String::new());
                            extra_gotos.write().push(String::new());
                        }
                    }

                    button {
                        class: "w-full px-6 py-3 bg-green-500 text-white rounded-md hover:bg-green-600 transition-colors duration-200 disabled:opacity-50 disabled:cursor-not-allowed font-medium text-lg",
                        r#type: "submit",
                        disabled: !is_form_valid(),
                        "{t.submit}"
                    }
                }
            }
        }
    }
}