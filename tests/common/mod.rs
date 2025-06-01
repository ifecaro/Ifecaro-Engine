use ifecaro::*;
use dioxus_core::NoOpMutations;

/// 建立測試用的段落
pub fn create_test_paragraph(id: &str, chapter_id: &str, lang: &str, text: &str) -> ifecaro::pages::story::Paragraph {
    use ifecaro::pages::story::{Paragraph, Text};
    
    Paragraph {
        id: id.to_string(),
        chapter_id: chapter_id.to_string(),
        texts: vec![Text {
            lang: lang.to_string(),
            paragraphs: text.to_string(),
            choices: vec![],
        }],
        choices: vec![],
        collection_id: String::new(),
        collection_name: String::new(),
        created: String::new(),
        updated: String::new(),
    }
}

/// 建立測試用的選擇
pub fn create_test_choice(caption: &str, to: &str) -> ifecaro::components::story_content::Choice {
    use ifecaro::components::story_content::{Choice, Action};
    
    Choice {
        caption: caption.to_string(),
        action: Action {
            type_: "goto".to_string(),
            key: None,
            value: None,
            to: to.to_string(),
        },
    }
}

/// 渲染組件為 HTML 字串
pub fn render_component_to_html<T>(component: fn(T) -> Element, props: T) -> String 
where
    T: 'static + Clone,
{
    let mut dom = VirtualDom::new_with_props(component, props);
    let mut mutations = NoOpMutations;
    dom.rebuild(&mut mutations);
    dioxus_ssr::render(&dom)
}

/// 檢查 HTML 是否包含特定的 CSS 類別
#[allow(dead_code)]
pub fn assert_html_contains_class(html: &str, class: &str) {
    assert!(html.contains(class), "HTML 應包含 CSS 類別 '{}'\nHTML: {}", class, html);
}

/// 檢查 HTML 是否包含特定文字
pub fn assert_html_contains_text(html: &str, text: &str) {
    assert!(html.contains(text), "HTML 應包含文字 '{}'\nHTML: {}", text, html);
} 