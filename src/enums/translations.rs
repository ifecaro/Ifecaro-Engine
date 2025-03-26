#[derive(Clone)]
pub struct Translations {
    pub add: &'static str,
    pub choice_id: &'static str,
    pub coming_soon: &'static str,
    pub dashboard: &'static str,
    pub goto_target: &'static str,
    pub options: &'static str,
    pub option_text: &'static str,
    pub paragraph: &'static str,
    pub settings: &'static str,
    pub story: &'static str,
    pub submit: &'static str,
    pub submit_success: &'static str,
    pub select_language: &'static str,
}

impl PartialEq for Translations {
    fn eq(&self, other: &Self) -> bool {
        self.add == other.add &&
        self.choice_id == other.choice_id &&
        self.coming_soon == other.coming_soon &&
        self.dashboard == other.dashboard &&
        self.goto_target == other.goto_target &&
        self.options == other.options &&
        self.option_text == other.option_text &&
        self.paragraph == other.paragraph &&
        self.settings == other.settings &&
        self.story == other.story &&
        self.submit == other.submit &&
        self.submit_success == other.submit_success &&
        self.select_language == other.select_language
    }
}

impl Translations {
    pub fn get(lang: &str) -> Self {
        match lang {
            "en-US" | "en-GB" => Self {
                add: "Add",
                choice_id: "Choice ID",
                coming_soon: "Coming soon...",
                dashboard: "Dashboard",
                goto_target: "Go to Target",
                options: "Options",
                option_text: "Option Text",
                paragraph: "Paragraph",
                settings: "Settings",
                story: "Story",
                submit: "Submit",
                submit_success: "Successfully submitted!",
                select_language: "Select Language",
            },
            "es-ES" => Self {
                add: "Añadir",
                choice_id: "ID de Opción",
                coming_soon: "Próximamente...",
                dashboard: "Panel de Control",
                goto_target: "Ir a Destino",
                options: "Opciones",
                option_text: "Texto de Opción",
                paragraph: "Párrafo",
                settings: "Configuración",
                story: "Historia",
                submit: "Enviar",
                submit_success: "¡Enviado con éxito!",
                select_language: "Seleccionar Idioma",
            },
            "es-CL" => Self {
                add: "Agregar",
                choice_id: "ID de Opción",
                coming_soon: "Próximamente...",
                dashboard: "Panel de Control",
                goto_target: "Ir a Destino",
                options: "Opciones",
                option_text: "Texto de Opción",
                paragraph: "Párrafo",
                settings: "Configuración",
                story: "Historia",
                submit: "Enviar",
                submit_success: "¡Enviado exitosamente!",
                select_language: "Seleccionar Idioma",
            },
            "zh-TW" => Self {
                add: "新增",
                choice_id: "選項代號",
                coming_soon: "即將推出...",
                dashboard: "儀表板",
                goto_target: "跳轉目標",
                options: "選項",
                option_text: "選項文字",
                paragraph: "段落",
                settings: "設定",
                story: "故事",
                submit: "送出",
                submit_success: "資料送出成功！",
                select_language: "選擇語言",
            },
            _ => Self::get("en-US"),
        }
    }
} 