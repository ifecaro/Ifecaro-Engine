#[derive(Clone)]
pub struct DashboardTranslations {
    pub choice_id: &'static str,
    pub paragraph: &'static str,
    pub options: &'static str,
    pub option_text: &'static str,
    pub goto_target: &'static str,
    pub add: &'static str,
    pub submit: &'static str,
    pub submit_success: &'static str,
}

impl DashboardTranslations {
    pub fn get(lang: &str) -> Self {
        match lang {
            "en" => Self {
                choice_id: "Choice ID",
                paragraph: "Paragraph",
                options: "Options",
                option_text: "Option Text",
                goto_target: "Go to Target",
                add: "Add",
                submit: "Submit",
                submit_success: "Successfully submitted!",
            },
            "zh-TW" => Self {
                choice_id: "選項代號",
                paragraph: "段落",
                options: "選項",
                option_text: "選項文字",
                goto_target: "跳轉目標",
                add: "新增",
                submit: "送出",
                submit_success: "資料送出成功！",
            },
            _ => Self::get("en"),
        }
    }
} 