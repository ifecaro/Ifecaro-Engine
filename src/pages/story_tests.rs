use crate::pages::story::{Paragraph, Text, merge_paragraphs_for_lang, ComplexChoice};

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
fn test_merge_paragraphs_basic() {
    let p1 = make_paragraph("p1", "c1", "zh", "第一段");
    let p2 = make_paragraph("p2", "c1", "zh", "第二段");
    
    let paragraphs = vec![p1, p2];
    let choice_ids = vec!["p1".to_string(), "p2".to_string()];
    
    let result = merge_paragraphs_for_lang(&paragraphs, "zh", false, false, &choice_ids);
    assert_eq!(result, "第一段\n\n第二段");
}

#[test]
fn test_merge_paragraphs_reader_mode() {
    let p1 = make_paragraph("p1", "c1", "zh", "第一段");
    let p2 = make_paragraph("p2", "c1", "zh", "第二段");
    
    let paragraphs = vec![p1, p2];
    let choice_ids = vec!["p1".to_string(), "p2".to_string()];
    
    let result = merge_paragraphs_for_lang(&paragraphs, "zh", true, false, &choice_ids);
    assert_eq!(result, "第一段\n\n第二段");
}

#[test]
fn test_merge_paragraphs_with_exclusion() {
    let p1 = make_paragraph("p1", "c1", "zh", "第一段");
    let p2 = make_paragraph("p2", "c1", "zh", "第二段");
    
    let paragraphs = vec![p1, p2];
    let choice_ids = vec!["p1".to_string()]; // Only include p1, exclude p2
    
    // In normal mode (reader_mode = false), all paragraphs are included regardless of choice_ids
    let result = merge_paragraphs_for_lang(&paragraphs, "zh", false, false, &choice_ids);
    assert_eq!(result, "第一段\n\n第二段");
    
    // In reader mode (reader_mode = true), only first paragraph and those in choice_ids are included
    let reader_result = merge_paragraphs_for_lang(&paragraphs, "zh", true, false, &choice_ids);
    assert_eq!(reader_result, "第一段");
}

#[test]
fn test_paragraph_with_time_limit() {
    let mut p = make_paragraph("p1", "c1", "zh", "Paragraph");
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
    // Countdown generation logic
    let countdowns: Vec<u32> = p.choices.iter().map(|c| c.time_limit.unwrap_or(0)).collect();
    assert_eq!(countdowns, vec![10, 0, 5]);
}

#[test]
fn test_option_disabled_after_countdown() {
    // Assume one option has countdown of 5
    let mut countdowns = vec![5];
    let mut disabled_by_countdown = vec![false];

    // Simulate 5-second countdown
    for _ in 0..5 {
        for c in countdowns.iter_mut() {
            if *c > 0 {
                *c -= 1;
            }
        }
    }

    // When countdown ends, should trigger disable
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
        // Prepare props
        let props = StoryContentUIProps {
            paragraph: "這是一個故事".to_string(),
            choices: vec![Choice {
                caption: "選項一".to_string(),
                action: Action {
                    type_: "goto".to_string(),
                    key: None,
                    value: None,
                    to: "next".to_string(),
                },
            }],
            enabled_choices: vec!["選項一".to_string()], // Choice is enabled
            disabled_by_countdown: vec![false],
            chapter_title: "章節標題測試".to_string(),
        };
        let mut dom = VirtualDom::new_with_props(StoryContentUI, props);
        let mut mutations = NoOpMutations;
        dom.rebuild(&mut mutations);
        let html = dioxus_ssr::render(&dom);
        // Since choice is enabled, should have cursor-pointer not disabled classes
        assert!(html.contains("cursor-pointer"), "HTML: {}", html);
    }
}
