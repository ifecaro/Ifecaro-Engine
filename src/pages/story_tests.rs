use crate::pages::story::{Paragraph, Text, merge_paragraphs_for_lang};

fn make_paragraph(id: &str, chapter_id: &str, lang: &str, text: &str) -> Paragraph {
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

#[test]
fn test_merge_paragraphs_reader_mode() {
    let p1 = make_paragraph("p1", "c1", "zh", "第一段");
    let p2 = make_paragraph("p2", "c1", "zh", "第二段");
    let expanded = vec![p1.clone(), p2.clone()];
    let choice_ids = vec!["p2".to_string()];
    let result = merge_paragraphs_for_lang(
        &expanded,
        "zh",
        true, // reader_mode
        false, // is_settings_chapter
        &choice_ids,
    );
    assert_eq!(result, "第一段\n\n第二段");
}

#[test]
fn test_merge_paragraphs_normal_mode() {
    let p1 = make_paragraph("p1", "c1", "zh", "第一段");
    let p2 = make_paragraph("p2", "c1", "zh", "第二段");
    let expanded = vec![p1.clone(), p2.clone()];
    let choice_ids = vec![];
    let result = merge_paragraphs_for_lang(
        &expanded,
        "zh",
        false, // reader_mode
        false, // is_settings_chapter
        &choice_ids,
    );
    assert_eq!(result, "第一段\n\n第二段");
}

#[test]
fn test_merge_paragraphs_reader_mode_only_first() {
    let p1 = make_paragraph("p1", "c1", "zh", "第一段");
    let p2 = make_paragraph("p2", "c1", "zh", "第二段");
    let expanded = vec![p1.clone(), p2.clone()];
    let choice_ids = vec![]; // 沒有 p2
    let result = merge_paragraphs_for_lang(
        &expanded,
        "zh",
        true, // reader_mode
        false, // is_settings_chapter
        &choice_ids,
    );
    assert_eq!(result, "第一段");
} 