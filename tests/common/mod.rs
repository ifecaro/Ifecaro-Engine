use ifecaro::*;
use dioxus_core::NoOpMutations;

/// Create test paragraph
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

/// Create test choice
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

/// Render component to HTML string
pub fn render_component_to_html<T>(component: fn(T) -> Element, props: T) -> String 
where
    T: 'static + Clone,
{
    let mut dom = VirtualDom::new_with_props(component, props);
    let mut mutations = NoOpMutations;
    dom.rebuild(&mut mutations);
    dioxus_ssr::render(&dom)
}

/// Check if HTML contains specific CSS class
#[allow(dead_code)]
pub fn assert_html_contains_class(html: &str, class: &str) {
    assert!(html.contains(class), "HTML should contain CSS class '{}'\nHTML: {}", class, html);
}

/// Check if HTML contains specific text
pub fn assert_html_contains_text(html: &str, text: &str) {
    assert!(html.contains(text), "HTML should contain text '{}'\nHTML: {}", text, html);
} 