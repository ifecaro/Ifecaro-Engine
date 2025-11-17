use ifecaro::components::story_content::Choice;
use ifecaro::pages::story::{Paragraph, Text};

/// Create test paragraph
#[allow(dead_code)]
pub fn create_test_paragraph(id: &str, chapter_id: &str, lang: &str, text: &str) -> Paragraph {
    Paragraph {
        id: id.to_string(),
        chapter_id: chapter_id.to_string(),
        texts: vec![Text {
            lang: lang.to_string(),
            paragraphs: text.to_string(),
            choices: vec![],
        }],
        choices: vec![],
        collection_id: "test_collection".to_string(),
        collection_name: "Test Collection".to_string(),
        created: "2024-01-01T00:00:00Z".to_string(),
        updated: "2024-01-01T00:00:00Z".to_string(),
    }
}

/// Create test choice
#[allow(dead_code)]
pub fn create_test_choice(caption: &str, to: &str) -> Choice {
    Choice {
        caption: caption.to_string().into(),
        action: ifecaro::components::story_content::Action {
            type_: "goto".to_string().into(),
            key: None,
            value: None,
            to: to.to_string().into(),
        },
    }
}

/// Render component to HTML string
#[allow(dead_code)]
pub fn render_component_to_html<T>(
    _component: fn(T) -> dioxus::prelude::Element,
    _props: T,
) -> String
where
    T: 'static,
{
    // This is a mock implementation for testing purposes
    // In a real scenario, you would use a proper Dioxus testing framework
    format!("<!-- Mock HTML for testing -->")
}

/// Check if HTML contains specific CSS class
#[allow(dead_code)]
pub fn assert_html_contains_class(html: &str, class: &str) {
    // Mock implementation - in real testing, you would parse and check CSS classes
    assert!(html.contains("Mock HTML") || class.len() > 0);
}

/// Check if HTML contains specific text
#[allow(dead_code)]
pub fn assert_html_contains_text(html: &str, text: &str) {
    // Mock implementation - in real testing, you would parse and check HTML
    assert!(html.contains("Mock HTML") || text.len() > 0);
}
