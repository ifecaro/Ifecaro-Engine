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

#[test]
fn test_countdowns_from_time_limit() {
    use crate::pages::story::ComplexChoice;
    // 準備一個段落，choices 有 time_limit
    let mut p = make_paragraph("p1", "c1", "zh", "段落");
    p.choices = vec![
        ComplexChoice {
            to: vec!["p2".to_string()],
            type_: "goto".to_string(),
            key: None,
            value: None,
            same_page: None,
            time_limit: Some(10),
        },
        ComplexChoice {
            to: vec!["p3".to_string()],
            type_: "goto".to_string(),
            key: None,
            value: None,
            same_page: None,
            time_limit: Some(0),
        },
        ComplexChoice {
            to: vec!["p4".to_string()],
            type_: "goto".to_string(),
            key: None,
            value: None,
            same_page: None,
            time_limit: Some(5),
        },
    ];
    // countdowns 產生邏輯
    let countdowns: Vec<u32> = p.choices.iter().map(|c| c.time_limit.unwrap_or(0)).collect();
    assert_eq!(countdowns, vec![10, 0, 5]);
}

#[test]
fn test_option_disabled_after_countdown() {
    // 假設有一個選項 countdown 為 5
    let mut countdowns = vec![5];
    let mut disabled_by_countdown = vec![false];

    // 模擬 5 秒倒數
    for _ in 0..5 {
        for c in countdowns.iter_mut() {
            if *c > 0 {
                *c -= 1;
            }
        }
    }

    // 倒數結束時，應該觸發禁用
    for (i, &c) in countdowns.iter().enumerate() {
        if c == 0 {
            disabled_by_countdown[i] = true;
        }
    }

    assert_eq!(disabled_by_countdown, vec![true]);
}

#[cfg(test)]
mod ssr_tests {
    use dioxus_core::NoOpMutations;
    use dioxus::prelude::VirtualDom;

    #[test]
    fn test_story_contentui_disabled_class() {
        use crate::components::story_content::{StoryContentUI, StoryContentUIProps, Choice, Action};
        // 準備 props
        let props = StoryContentUIProps {
            paragraph: "這是一段故事".to_string(),
            choices: vec![Choice {
                caption: "選項一".to_string(),
                action: Action {
                    type_: "goto".to_string(),
                    key: None,
                    value: None,
                    to: "p2".to_string(),
                },
            }],
            enabled_choices: vec!["選項一".to_string()],
            disabled_by_countdown: vec![true],
            chapter_title: "章節標題測試".to_string(),
        };
        let mut dom = VirtualDom::new_with_props(StoryContentUI, props);
        let mut mutations = NoOpMutations;
        dom.rebuild(&mut mutations);
        let html = dioxus_ssr::render(&dom);
        assert!(html.contains("opacity-50 cursor-not-allowed"), "HTML: {}", html);
    }
}
