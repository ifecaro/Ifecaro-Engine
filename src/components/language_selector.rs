use dioxus::prelude::*;
use dioxus_i18n::t;

#[derive(Clone, PartialEq)]
pub struct Language {
    pub code: &'static str,
    pub name: &'static str,
}

#[allow(dead_code)]
pub const AVAILABLE_LANGUAGES: &[Language] = &[
    Language {
        code: "zh-TW",
        name: "繁體中文",
    },
    Language {
        code: "zh-CN",
        name: "简体中文",
    },
    Language {
        code: "en-US",
        name: "English (US)",
    },
    Language {
        code: "en-GB",
        name: "English (UK)",
    },
    Language {
        code: "es-ES",
        name: "Español (España)",
    },
    Language {
        code: "es-CL",
        name: "Español (Chile)",
    },
    Language {
        code: "ja-JP",
        name: "日本語",
    },
    Language {
        code: "ko-KR",
        name: "한국어",
    },
    Language {
        code: "fr-FR",
        name: "Français (France)",
    },
    Language {
        code: "de-DE",
        name: "Deutsch (Deutschland)",
    },
    Language {
        code: "it-IT",
        name: "Italiano (Italia)",
    },
    Language {
        code: "pt-PT",
        name: "Português (Portugal)",
    },
    Language {
        code: "pt-BR",
        name: "Português (Brasil)",
    },
    Language {
        code: "ru-RU",
        name: "Русский (Россия)",
    },
    Language {
        code: "ar",
        name: "العربية",
    },
    Language {
        code: "hi",
        name: "हिंदी",
    },
    Language {
        code: "bn",
        name: "বাংলা",
    },
    Language {
        code: "id",
        name: "Bahasa Indonesia",
    },
    Language {
        code: "ms",
        name: "Bahasa Melayu",
    },
    Language {
        code: "th",
        name: "ไทย",
    },
    Language {
        code: "vi",
        name: "Tiếng Việt",
    },
    Language {
        code: "nl",
        name: "Nederlands",
    },
    Language {
        code: "pl",
        name: "Polski",
    },
    Language {
        code: "uk",
        name: "Українська",
    },
    Language {
        code: "el",
        name: "Ελληνικά",
    },
    Language {
        code: "he",
        name: "עברית",
    },
    Language {
        code: "tr",
        name: "Türkçe",
    },
    Language {
        code: "sv",
        name: "Svenska",
    },
    Language {
        code: "da",
        name: "Dansk",
    },
    Language {
        code: "fi",
        name: "Suomi",
    },
    Language {
        code: "no",
        name: "Norsk",
    },
    Language {
        code: "cs",
        name: "Čeština",
    },
    Language {
        code: "ro",
        name: "Română",
    },
    Language {
        code: "hu",
        name: "Magyar",
    },
    Language {
        code: "sk",
        name: "Slovenčina",
    },
    Language {
        code: "hr",
        name: "Hrvatski",
    },
    Language {
        code: "ca",
        name: "Català",
    },
    Language {
        code: "fil",
        name: "Filipino",
    },
    Language {
        code: "fa",
        name: "فارسی",
    },
    Language {
        code: "lv",
        name: "Latviešu",
    },
    Language {
        code: "af",
        name: "Afrikaans",
    },
    Language {
        code: "sw",
        name: "Kiswahili",
    },
    Language {
        code: "ga",
        name: "Gaeilge",
    },
    Language {
        code: "et",
        name: "Eesti",
    },
    Language {
        code: "eu",
        name: "Euskara",
    },
    Language {
        code: "is",
        name: "Íslenska",
    },
    Language {
        code: "mk",
        name: "Македонски",
    },
    Language {
        code: "hy",
        name: "Հայերեն",
    },
    Language {
        code: "ne",
        name: "नेपाली",
    },
    Language {
        code: "lb",
        name: "Lëtzebuergesch",
    },
    Language {
        code: "my",
        name: "မြန်မာဘာသာ",
    },
    Language {
        code: "gl",
        name: "Galego",
    },
    Language {
        code: "mr",
        name: "मराठी",
    },
    Language {
        code: "ka",
        name: "ქართული",
    },
    Language {
        code: "mn",
        name: "Монгол",
    },
    Language {
        code: "si",
        name: "සිංහල",
    },
    Language {
        code: "km",
        name: "ខ្មែរ",
    },
    Language {
        code: "sn",
        name: "chiShona",
    },
    Language {
        code: "yo",
        name: "Yorùbá",
    },
    Language {
        code: "so",
        name: "Soomaali",
    },
    Language {
        code: "ha",
        name: "Hausa",
    },
    Language {
        code: "zu",
        name: "isiZulu",
    },
    Language {
        code: "xh",
        name: "isiXhosa",
    },
    Language {
        code: "am",
        name: "አማርኛ",
    },
    Language {
        code: "be",
        name: "Беларуская",
    },
    Language {
        code: "az",
        name: "Azərbaycan",
    },
    Language {
        code: "uz",
        name: "O'zbek",
    },
    Language {
        code: "kk",
        name: "Қазақ",
    },
    Language {
        code: "ky",
        name: "Кыргызча",
    },
    Language {
        code: "tg",
        name: "Тоҷикӣ",
    },
    Language {
        code: "tk",
        name: "Türkmen",
    },
    Language {
        code: "ur",
        name: "اردو",
    },
    Language {
        code: "pa",
        name: "ਪੰਜਾਬੀ",
    },
    Language {
        code: "gu",
        name: "ગુજરાતી",
    },
    Language {
        code: "or",
        name: "ଓଡ଼ିଆ",
    },
    Language {
        code: "ta",
        name: "தமிழ்",
    },
    Language {
        code: "te",
        name: "తెలుగు",
    },
    Language {
        code: "kn",
        name: "ಕನ್ನಡ",
    },
    Language {
        code: "ml",
        name: "മലയാളം",
    },
    Language {
        code: "as",
        name: "অসমীয়া",
    },
    Language {
        code: "mai",
        name: "मैथिली",
    },
    Language {
        code: "mni",
        name: "मैथिली",
    },
    Language {
        code: "doi",
        name: "डोगरी",
    },
    Language {
        code: "bho",
        name: "भोजपुरी",
    },
    Language {
        code: "sat",
        name: "ᱥᱟᱱᱛᱟᱲᱤ",
    },
    Language {
        code: "ks",
        name: "کٲشُر",
    },
    Language {
        code: "sa",
        name: "संस्कृतम्",
    },
    Language {
        code: "sd",
        name: "سنڌي",
    },
    Language {
        code: "kok",
        name: "कोंकणी",
    },
    Language {
        code: "gom",
        name: "कोंकणी",
    },
    Language {
        code: "ar-AE",
        name: "العربية (الإمارات)",
    },
    Language {
        code: "ar-DZ",
        name: "العربية (الجزائر)",
    },
    Language {
        code: "ar-EG",
        name: "العربية (مصر)",
    },
    Language {
        code: "ar-IQ",
        name: "العربية (العراق)",
    },
    Language {
        code: "ar-LB",
        name: "العربية (لبنان)",
    },
    Language {
        code: "ar-MA",
        name: "العربية (المغرب)",
    },
    Language {
        code: "ar-SA",
        name: "العربية (السعودية)",
    },
    Language {
        code: "ar-SY",
        name: "العربية (سوريا)",
    },
    Language {
        code: "ar-TN",
        name: "العربية (تونس)",
    },
    Language {
        code: "bg-BG",
        name: "Български",
    },
    Language {
        code: "bn-BD",
        name: "বাংলা (বাংলাদেশ)",
    },
    Language {
        code: "bn-IN",
        name: "বাংলা (ভারত)",
    },
    Language {
        code: "cs-CZ",
        name: "Čeština (Česko)",
    },
    Language {
        code: "da-DK",
        name: "Dansk (Danmark)",
    },
    Language {
        code: "de-AT",
        name: "Deutsch (Österreich)",
    },
    Language {
        code: "de-BE",
        name: "Deutsch (Belgien)",
    },
    Language {
        code: "de-CH",
        name: "Deutsch (Schweiz)",
    },
    Language {
        code: "de-LU",
        name: "Deutsch (Luxemburg)",
    },
    Language {
        code: "el-GR",
        name: "Ελληνικά (Ελλάδα)",
    },
    Language {
        code: "en",
        name: "English",
    },
    Language {
        code: "en-AU",
        name: "English (Australia)",
    },
    Language {
        code: "en-CA",
        name: "English (Canada)",
    },
    Language {
        code: "en-HK",
        name: "English (Hong Kong)",
    },
    Language {
        code: "en-IE",
        name: "English (Ireland)",
    },
    Language {
        code: "en-IN",
        name: "English (India)",
    },
    Language {
        code: "en-NZ",
        name: "English (New Zealand)",
    },
    Language {
        code: "en-PH",
        name: "English (Philippines)",
    },
    Language {
        code: "en-SG",
        name: "English (Singapore)",
    },
    Language {
        code: "en-ZA",
        name: "English (South Africa)",
    },
    Language {
        code: "es",
        name: "Español",
    },
    Language {
        code: "es-419",
        name: "Español (Latinoamérica)",
    },
    Language {
        code: "es-AR",
        name: "Español (Argentina)",
    },
    Language {
        code: "es-BO",
        name: "Español (Bolivia)",
    },
    Language {
        code: "es-CO",
        name: "Español (Colombia)",
    },
    Language {
        code: "es-EC",
        name: "Español (Ecuador)",
    },
    Language {
        code: "es-MX",
        name: "Español (México)",
    },
    Language {
        code: "es-PE",
        name: "Español (Perú)",
    },
    Language {
        code: "es-UY",
        name: "Español (Uruguay)",
    },
    Language {
        code: "es-VE",
        name: "Español (Venezuela)",
    },
    Language {
        code: "et-EE",
        name: "Eesti (Eesti)",
    },
    Language {
        code: "fa-IR",
        name: "فارسی (ایران)",
    },
    Language {
        code: "fi-FI",
        name: "Suomi (Suomi)",
    },
    Language {
        code: "fr",
        name: "Français",
    },
    Language {
        code: "fr-BE",
        name: "Français (Belgique)",
    },
    Language {
        code: "fr-CA",
        name: "Français (Canada)",
    },
    Language {
        code: "fr-CH",
        name: "Français (Suisse)",
    },
    Language {
        code: "fr-LU",
        name: "Français (Luxembourg)",
    },
    Language {
        code: "fr-MA",
        name: "Français (Maroc)",
    },
    Language {
        code: "he-IL",
        name: "עברית (ישראל)",
    },
    Language {
        code: "hi-IN",
        name: "हिन्दी (भारत)",
    },
    Language {
        code: "hr-HR",
        name: "Hrvatski (Hrvatska)",
    },
    Language {
        code: "hu-HU",
        name: "Magyar (Magyarország)",
    },
    Language {
        code: "id-ID",
        name: "Bahasa Indonesia (Indonesia)",
    },
    Language {
        code: "it-CH",
        name: "Italiano (Svizzera)",
    },
    Language {
        code: "lt-LT",
        name: "Lietuvių",
    },
    Language {
        code: "lv-LV",
        name: "Latviešu (Latvija)",
    },
    Language {
        code: "ms-MY",
        name: "Bahasa Melayu (Malaysia)",
    },
    Language {
        code: "ms-SG",
        name: "Bahasa Melayu (Singapura)",
    },
    Language {
        code: "nb",
        name: "Norsk bokmål",
    },
    Language {
        code: "nb-NO",
        name: "Norsk bokmål (Norge)",
    },
    Language {
        code: "nl-BE",
        name: "Nederlands (België)",
    },
    Language {
        code: "nl-NL",
        name: "Nederlands (Nederland)",
    },
    Language {
        code: "nn-NO",
        name: "Norsk nynorsk",
    },
    Language {
        code: "pl-PL",
        name: "Polski (Polska)",
    },
    Language {
        code: "ro-RO",
        name: "Română (România)",
    },
    Language {
        code: "ru-BY",
        name: "Русский (Беларусь)",
    },
    Language {
        code: "ru-KZ",
        name: "Русский (Казахстан)",
    },
    Language {
        code: "ru-UA",
        name: "Русский (Украина)",
    },
    Language {
        code: "sk-SK",
        name: "Slovenčina (Slovensko)",
    },
    Language {
        code: "sl-SI",
        name: "Slovenščina",
    },
    Language {
        code: "sr-RS",
        name: "Српски",
    },
    Language {
        code: "sv-FI",
        name: "Svenska (Finland)",
    },
    Language {
        code: "sv-SE",
        name: "Svenska (Sverige)",
    },
    Language {
        code: "ta-IN",
        name: "தமிழ் (இந்தியா)",
    },
    Language {
        code: "ta-SG",
        name: "தமிழ் (சிங்கப்பூர்)",
    },
    Language {
        code: "th-TH",
        name: "ไทย (ประเทศไทย)",
    },
    Language {
        code: "tr-TR",
        name: "Türkçe (Türkiye)",
    },
    Language {
        code: "uk-UA",
        name: "Українська (Україна)",
    },
    Language {
        code: "vi-VN",
        name: "Tiếng Việt (Việt Nam)",
    },
    Language {
        code: "zh",
        name: "中文",
    },
    Language {
        code: "zh-HK",
        name: "中文（香港）",
    },
    Language {
        code: "zh-Hans",
        name: "简体中文",
    },
    Language {
        code: "zh-Hans-CN",
        name: "简体中文（中国大陆）",
    },
    Language {
        code: "zh-Hant",
        name: "繁體中文",
    },
    Language {
        code: "zh-Hant-HK",
        name: "繁體中文（香港）",
    },
    Language {
        code: "zh-Hant-TW",
        name: "繁體中文（台灣）",
    },
    Language {
        code: "zh-MO",
        name: "中文（澳門）",
    },
    Language {
        code: "zh-SG",
        name: "中文（新加坡）",
    },
];

#[allow(dead_code)]
pub fn display_language(lang: &&Language) -> String {
    lang.name.to_string()
}

#[derive(Props, Clone, PartialEq)]
pub struct LanguageSelectorProps {
    selected_lang: String,
    on_language_change: EventHandler<String>,
}

#[component]
pub fn LanguageSelector(props: LanguageSelectorProps) -> Element {
    let mut is_open = use_signal(|| false);
    let mut search_query = use_signal(|| String::new());

    let filtered_languages = use_memo(move || {
        let query = search_query.read().to_lowercase();
        AVAILABLE_LANGUAGES
            .iter()
            .filter(|l| {
                l.name.to_lowercase().contains(&query) || l.code.to_lowercase().contains(&query)
            })
            .collect::<Vec<_>>()
    });

    let _dropdown_class = use_memo(move || {
        if *is_open.read() {
            "translate-y-0 opacity-100"
        } else {
            "-translate-y-2 opacity-0 pointer-events-none"
        }
    });

    let current_language = use_memo(move || {
        AVAILABLE_LANGUAGES
            .iter()
            .find(|l| l.code == props.selected_lang)
            .map(|l| l.name)
            .unwrap_or("繁體中文")
    });

    rsx! {
        crate::components::dropdown::Dropdown {
            label: t!("select_language"),
            value: current_language.read().to_string(),
            options: filtered_languages.read().clone(),
            is_open: *is_open.read(),
            search_query: search_query.read().to_string(),
            on_toggle: move |_| {
                let current = *is_open.read();
                is_open.set(!current);
            },
            on_search: move |query| search_query.set(query),
            on_select: move |lang: &Language| {
                props.on_language_change.call(lang.code.to_string());
                is_open.set(false);
                search_query.set(String::new());
            },
            display_fn: display_language,
            button_class: None,
            label_class: None,
            search_placeholder: t!("search_language"),
        }
    }
}
