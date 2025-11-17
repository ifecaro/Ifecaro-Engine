use ifecaro::contexts::paragraph_context::{Paragraph, ParagraphChoice, Text};
use ifecaro::utils::find_paragraph_by_id;

#[test]
fn find_paragraph_by_id_returns_updated_object() {
    // Updated paragraph with new content
    let updated = Paragraph {
        id: "p1".to_string(),
        chapter_id: "c1".to_string(),
        texts: vec![Text {
            lang: "zh-TW".to_string(),
            paragraphs: "更新後的段落內容".to_string(),
            choices: vec![],
        }],
        choices: vec![ParagraphChoice::Simple(Vec::new())],
    };

    // The paragraph list now only contains the updated version
    let list = vec![updated.clone()];

    // Helper should return the updated version
    let result = find_paragraph_by_id(&list, "p1").expect("Paragraph should be found");
    assert_eq!(result, updated);
}
