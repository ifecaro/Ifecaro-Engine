pub mod config {
    // use serde::Deserialize;

    pub struct Language<'a> {
        pub name: &'a str,
        pub code: &'a str,
    }

    pub static BASE_API_URL: &str = "http://0.0.0.0:8090";
    pub static SETTINGS: &str = "/api/collections/settings/records";
    pub static LANGUAGES: [Language; 5] = [
        Language {
            name: "English (US)",
            code: "en-US",
        },
        Language {
            name: "English (UK)",
            code: "en-GB",
        },
        Language {
            name: "Español (España)",
            code: "es-ES",
        },
        Language {
            name: "Español (Chile)",
            code: "es-CL",
        },
        Language {
            name: "正體中文",
            code: "zh-TW",
        },
    ];
}
