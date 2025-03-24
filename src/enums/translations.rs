#[derive(Clone)]
pub struct Translations {
    pub add: &'static str,
    pub choice_id: &'static str,
    pub dashboard: &'static str,
    pub goto_target: &'static str,
    pub options: &'static str,
    pub option_text: &'static str,
    pub paragraph: &'static str,
    pub settings: &'static str,
    pub story: &'static str,
    pub submit: &'static str,
    pub submit_success: &'static str,
}

impl Translations {
    pub fn get(lang: &str) -> Self {
        match lang {
            "en" => Self {
                add: "Add",
                choice_id: "Choice ID",
                dashboard: "Dashboard",
                goto_target: "Go to Target",
                options: "Options",
                option_text: "Option Text",
                paragraph: "Paragraph",
                settings: "Settings",
                story: "Story",
                submit: "Submit",
                submit_success: "Successfully submitted!",
            },
            "zh-TW" => Self {
                add: "新增",
                choice_id: "選項代號",
                dashboard: "儀表板",
                goto_target: "跳轉目標",
                options: "選項",
                option_text: "選項文字",
                paragraph: "段落",
                settings: "設定",
                story: "故事",
                submit: "送出",
                submit_success: "資料送出成功！",
            },
            _ => Self::get("en"),
        }
    }
} 