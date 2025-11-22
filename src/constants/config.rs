// use serde::Deserialize;

#[allow(dead_code)]
pub struct Language<'a> {
    pub name: &'a str,
    pub code: &'a str,
}

pub static BASE_API_URL: &str = "https://ifecaro.com/db/api";
pub static PARAGRAPHS: &str = "/collections/paragraphs/records";
pub static CHAPTERS: &str = "/collections/chapters/records";
#[allow(dead_code)]
pub static ACTIONS: &str = "/collections/actions/records";
#[allow(dead_code)]
pub static CHARACTERS: &str = "/collections/characters/records";
#[allow(dead_code)]
pub static ATTRIBUTES: &str = "/collections/attributes/records";
#[allow(dead_code)]
pub static RELATIONSHIPS: &str = "/collections/relationships/records";
#[allow(dead_code)]
pub static PUBLIC_COLLECTIONS: &str = "/collections/public";
#[allow(dead_code)]
pub static AUTH_TOKEN: &str = ""; // No auth token needed, as collections should be accessible to any user
pub static LANGUAGES: [Language; 7] = [
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
        name: "Français (France)",
        code: "fr-FR",
    },
    Language {
        name: "中文（台灣）",
        code: "zh-TW",
    },
    Language {
        name: "中文（中国）",
        code: "zh-CN",
    },
];
