use ifecaro::*;
use dioxus_core::NoOpMutations;

#[cfg(test)]
mod story_integration_tests {
    use super::*;
    use ifecaro::pages::story::{Paragraph, Text, merge_paragraphs_for_lang};
    
    fn make_test_paragraph(id: &str, chapter_id: &str, lang: &str, text: &str) -> Paragraph {
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
    fn test_story_complete_flow() {
        let p1 = make_test_paragraph("p1", "c1", "zh", "故事開始");
        let p2 = make_test_paragraph("p2", "c1", "zh", "劇情發展");
        let p3 = make_test_paragraph("p3", "c1", "zh", "故事結束");
        
        let expanded = vec![p1.clone(), p2.clone(), p3.clone()];
        let choice_ids = vec!["p2".to_string(), "p3".to_string()];
        
        let result = merge_paragraphs_for_lang(
            &expanded,
            "zh",
            true, // reader_mode
            false, // is_settings_chapter
            &choice_ids,
        );
        
        assert_eq!(result, "故事開始\n\n劇情發展\n\n故事結束");
    }

    #[test]
    fn test_story_ui_integration() {
        use ifecaro::components::story_content::{StoryContentUI, StoryContentUIProps, Choice, Action};
        
        let choices = vec![
            Choice { 
                caption: "繼續故事".to_string(), 
                action: Action { 
                    type_: "goto".to_string(), 
                    key: None, 
                    value: None, 
                    to: "next_paragraph".to_string() 
                } 
            },
        ];
        
        let props = StoryContentUIProps {
            paragraph: "這是一個完整的故事段落測試".to_string(),
            choices: choices.clone(),
            enabled_choices: vec!["繼續故事".to_string()],
            disabled_by_countdown: vec![false],
            chapter_title: "整合測試章節".to_string(),
        };
        
        let mut dom = VirtualDom::new_with_props(StoryContentUI, props);
        let mut mutations = NoOpMutations;
        dom.rebuild(&mut mutations);
        let html = dioxus_ssr::render(&dom);
        
        assert!(html.contains("這是一個完整的故事段落測試"));
        assert!(html.contains("繼續故事"));
        assert!(html.contains("整合測試章節"));
    }
}

#[cfg(test)]
mod ui_integration_tests {
    use super::*;
    
    #[test]
    fn test_keyboard_state_integration() {
        use ifecaro::components::story_content::{Choice, Action};
        
        let mut keyboard_state = KeyboardState::default();
        
        let choices = vec![
            Choice {
                caption: "選項一".to_string(),
                action: Action {
                    type_: "goto".to_string(),
                    key: None,
                    value: None,
                    to: "p1".to_string(),
                },
            },
            Choice {
                caption: "選項二".to_string(),
                action: Action {
                    type_: "goto".to_string(),
                    key: None,
                    value: None,
                    to: "p2".to_string(),
                },
            },
        ];
        
        keyboard_state.choices = std::sync::Arc::new(choices);
        keyboard_state.enabled_choices = std::sync::Arc::new(vec!["p1".to_string(), "p2".to_string()]);
        
        // Test default state
        assert_eq!(keyboard_state.selected_index, 0);
        assert_eq!(keyboard_state.choices.len(), 2);
        assert_eq!(keyboard_state.enabled_choices.len(), 2);
        
        // Simulate keyboard navigation
        keyboard_state.selected_index = 1;
        assert_eq!(keyboard_state.selected_index, 1);
        assert!(keyboard_state.selected_index < keyboard_state.choices.len() as i32);
    }
}

#[cfg(test)]
mod context_integration_tests {
    #[test]
    fn test_settings_context_default() {
        use ifecaro::contexts::settings_context::SettingsContext;
        
        let _settings = SettingsContext::default();
        // Test successful creation of SettingsContext
    }
} 