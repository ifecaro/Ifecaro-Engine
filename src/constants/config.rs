// use serde::Deserialize;

#[allow(dead_code)]
pub struct Language<'a> {
    pub name: &'a str,
    pub code: &'a str,
}

pub static BASE_API_URL: &str = "http://localhost:8090/api";
pub static PARAGRAPHS: &str = "/collections/paragraphs/records";
pub static CHAPTERS: &str = "/collections/chapters/records";
pub static ACTIONS: &str = "/collections/actions/records";
#[allow(dead_code)]
pub static PUBLIC_COLLECTIONS: &str = "/collections/public";
#[allow(dead_code)]
pub static AUTH_TOKEN: &str = ""; // 不需要認證 token，因為 collections 應該可以被任何用戶訪問
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
