#[derive(Clone)]
pub struct Translations {
    pub add: &'static str,
    pub choice_id: &'static str,
    pub coming_soon: &'static str,
    pub dashboard: &'static str,
    pub goto_target: &'static str,
    pub options: &'static str,
    pub option_text: &'static str,
    pub caption: &'static str,
    pub paragraph: &'static str,
    pub settings: &'static str,
    pub story: &'static str,
    pub submit: &'static str,
    pub submit_success: &'static str,
    pub select_language: &'static str,
    pub select_chapter: &'static str,
    pub option: &'static str,
    pub action_settings: &'static str,
    pub action_type: &'static str,
    pub action_key: &'static str,
    pub action_value: &'static str,
    pub add_option: &'static str,
    pub paragraph_title: &'static str,
    pub paragraph_content: &'static str,
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
        self.caption == other.caption &&
        self.paragraph == other.paragraph &&
        self.settings == other.settings &&
        self.story == other.story &&
        self.submit == other.submit &&
        self.submit_success == other.submit_success &&
        self.select_language == other.select_language &&
        self.select_chapter == other.select_chapter &&
        self.option == other.option &&
        self.action_settings == other.action_settings &&
        self.action_type == other.action_type &&
        self.action_key == other.action_key &&
        self.action_value == other.action_value &&
        self.add_option == other.add_option &&
        self.paragraph_title == other.paragraph_title &&
        self.paragraph_content == other.paragraph_content
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
                goto_target: "Target Paragraph",
                options: "Options",
                option_text: "Option Text",
                caption: "Caption",
                paragraph: "Paragraph",
                settings: "Settings",
                story: "Story",
                submit: "Submit",
                submit_success: "Successfully submitted!",
                select_language: "Select Language",
                select_chapter: "Select Chapter",
                option: "Option",
                action_settings: "Action Settings",
                action_type: "Action Type",
                action_key: "Action Key",
                action_value: "Action Value",
                add_option: "Add Option",
                paragraph_title: "Paragraph Title",
                paragraph_content: "Paragraph Content",
            },
            "es-ES" => Self {
                add: "Añadir",
                choice_id: "ID de Opción",
                coming_soon: "Próximamente...",
                dashboard: "Panel de Control",
                goto_target: "Ir a Destino",
                options: "Opciones",
                option_text: "Texto de Opción",
                caption: "Leyenda",
                paragraph: "Párrafo",
                settings: "Configuración",
                story: "Historia",
                submit: "Enviar",
                submit_success: "¡Enviado con éxito!",
                select_language: "Seleccionar Idioma",
                select_chapter: "Seleccionar Capítulo",
                option: "Opción",
                action_settings: "Configuración de Acción",
                action_type: "Tipo de Acción",
                action_key: "Clave de Acción",
                action_value: "Valor de Acción",
                add_option: "Añadir Opción",
                paragraph_title: "Título del Párrafo",
                paragraph_content: "Contenido del Párrafo",
            },
            "es-CL" => Self {
                add: "Agregar",
                choice_id: "ID de Opción",
                coming_soon: "Próximamente...",
                dashboard: "Panel de Control",
                goto_target: "Ir a Destino",
                options: "Opciones",
                option_text: "Texto de Opción",
                caption: "Leyenda",
                paragraph: "Párrafo",
                settings: "Configuración",
                story: "Historia",
                submit: "Enviar",
                submit_success: "¡Enviado exitosamente!",
                select_language: "Seleccionar Idioma",
                select_chapter: "Seleccionar Capítulo",
                option: "Opción",
                action_settings: "Configuración de Acción",
                action_type: "Tipo de Acción",
                action_key: "Clave de Acción",
                action_value: "Valor de Acción",
                add_option: "Agregar Opción",
                paragraph_title: "Título del Párrafo",
                paragraph_content: "Contenido del Párrafo",
            },
            "zh-TW" => Self {
                add: "新增",
                choice_id: "選項代號",
                coming_soon: "即將推出...",
                dashboard: "儀表板",
                goto_target: "目標段落",
                options: "選項",
                option_text: "選項文字",
                caption: "標題",
                paragraph: "段落",
                settings: "設定",
                story: "故事",
                submit: "送出",
                submit_success: "資料送出成功！",
                select_language: "選擇語言",
                select_chapter: "選擇章節",
                option: "選項",
                action_settings: "動作設定",
                action_type: "動作類型",
                action_key: "動作鍵值",
                action_value: "動作數值",
                add_option: "新增選項",
                paragraph_title: "段落標題",
                paragraph_content: "段落內容",
            },
            _ => Self::get("en-US"),
        }
    }
} 