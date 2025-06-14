use ifecaro::pages::story::{Paragraph, Text, paragraph_has_translation};

#[test]
fn test_paragraph_has_translation_exists() {
    // Prepare mock paragraph with English translation
    let paragraph = Paragraph {
        id: "p1".to_string(),
        chapter_id: String::new(),
        texts: vec![Text {
            lang: "en-US".to_string(),
            paragraphs: "Hello world".to_string(),
            choices: vec![],
        }],
        choices: vec![],
        collection_id: String::new(),
        collection_name: String::new(),
        created: String::new(),
        updated: String::new(),
    };

    let paragraphs = vec![paragraph];
    assert!(paragraph_has_translation(&paragraphs, "p1", "en-US"));
}

#[test]
fn test_paragraph_has_translation_missing() {
    // Prepare paragraph without Chinese translation
    let paragraph = Paragraph {
        id: "p2".to_string(),
        chapter_id: String::new(),
        texts: vec![Text {
            lang: "en-US".to_string(),
            paragraphs: "Hello".to_string(),
            choices: vec![],
        }],
        choices: vec![],
        collection_id: String::new(),
        collection_name: String::new(),
        created: String::new(),
        updated: String::new(),
    };

    let paragraphs = vec![paragraph];
    assert!(!paragraph_has_translation(&paragraphs, "p2", "zh-TW"));
} 