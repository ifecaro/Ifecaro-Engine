// Import the Dashboard component and related structs
use ifecaro::contexts::chapter_context::{ChapterState, Chapter, ChapterTitle};
use ifecaro::contexts::paragraph_context::{
    ParagraphState, Paragraph, Text, ParagraphChoice
};

// Test user interaction scenarios
#[cfg(test)]
mod interaction_tests {
    use super::*;
    
    // Test helper to create a full dashboard state
    fn create_full_dashboard_state() -> (String, ChapterState, ParagraphState) {
        let current_language = "zh-TW".to_string();
        
        let chapter_state = ChapterState {
            chapters: vec![
                Chapter {
                    id: "chapter1".to_string(),
                    titles: vec![
                        ChapterTitle {
                            lang: "zh-TW".to_string(),
                            title: "第一章節".to_string(),
                        },
                        ChapterTitle {
                            lang: "en-US".to_string(),
                            title: "Chapter One".to_string(),
                        },
                    ],
                    order: 1,
                },
                Chapter {
                    id: "chapter2".to_string(),
                    titles: vec![
                        ChapterTitle {
                            lang: "zh-TW".to_string(),
                            title: "第二章節".to_string(),
                        },
                        ChapterTitle {
                            lang: "en-US".to_string(),
                            title: "Chapter Two".to_string(),
                        },
                    ],
                    order: 2,
                },
            ],
            loaded: true,
        };
        
        let paragraph_state = ParagraphState {
            paragraphs: vec![
                Paragraph {
                    id: "para1".to_string(),
                    chapter_id: "chapter1".to_string(),
                    texts: vec![
                        Text {
                            lang: "zh-TW".to_string(),
                            paragraphs: "這是測試段落一的內容。你站在森林的入口，看到兩條路。".to_string(),
                            choices: vec!["走左邊的路".to_string(), "走右邊的路".to_string()],
                        },
                        Text {
                            lang: "en-US".to_string(),
                            paragraphs: "This is test paragraph one. You stand at the forest entrance, seeing two paths.".to_string(),
                            choices: vec!["Take the left path".to_string(), "Take the right path".to_string()],
                        },
                    ],
                    choices: vec![
                        ParagraphChoice::Simple(vec!["para2".to_string()]),
                        ParagraphChoice::Complex {
                            to: vec!["para3".to_string()],
                            type_: "conditional".to_string(),
                            key: Some("forest_knowledge".to_string()),
                            value: Some(serde_json::json!(true)),
                            same_page: Some(false),
                            time_limit: Some(60),
                        },
                    ],
                },
                Paragraph {
                    id: "para2".to_string(),
                    chapter_id: "chapter1".to_string(),
                    texts: vec![
                        Text {
                            lang: "zh-TW".to_string(),
                            paragraphs: "你沿著左邊的路走，來到一個清澈的湖邊。".to_string(),
                            choices: vec!["飲用湖水".to_string(), "繼續前進".to_string()],
                        },
                        Text {
                            lang: "en-US".to_string(),
                            paragraphs: "You walk along the left path and arrive at a clear lake.".to_string(),
                            choices: vec!["Drink the lake water".to_string(), "Continue forward".to_string()],
                        },
                    ],
                    choices: vec![
                        ParagraphChoice::Simple(vec!["para4".to_string()]),
                        ParagraphChoice::Simple(vec!["para5".to_string()]),
                    ],
                },
                Paragraph {
                    id: "para3".to_string(),
                    chapter_id: "chapter1".to_string(),
                    texts: vec![
                        Text {
                            lang: "zh-TW".to_string(),
                            paragraphs: "你選擇了右邊的路，發現了一個古老的遺跡。".to_string(),
                            choices: vec!["探索遺跡".to_string(), "繞過遺跡".to_string()],
                        },
                        Text {
                            lang: "en-US".to_string(),
                            paragraphs: "You chose the right path and discovered an ancient ruin.".to_string(),
                            choices: vec!["Explore the ruin".to_string(), "Go around the ruin".to_string()],
                        },
                    ],
                    choices: vec![
                        ParagraphChoice::Complex {
                            to: vec!["para6".to_string()],
                            type_: "gain_item".to_string(),
                            key: Some("ancient_key".to_string()),
                            value: Some(serde_json::json!(1)),
                            same_page: Some(false),
                            time_limit: None,
                        },
                        ParagraphChoice::Simple(vec!["para7".to_string()]),
                    ],
                },
                Paragraph {
                    id: "para4".to_string(),
                    chapter_id: "chapter2".to_string(),
                    texts: vec![
                        Text {
                            lang: "zh-TW".to_string(),
                            paragraphs: "你喝了湖水，感到精神振奮。".to_string(),
                            choices: vec!["離開湖邊".to_string()],
                        },
                        Text {
                            lang: "en-US".to_string(),
                            paragraphs: "You drank the lake water and feel refreshed.".to_string(),
                            choices: vec!["Leave the lake".to_string()],
                        },
                    ],
                    choices: vec![
                        ParagraphChoice::Simple(vec!["para8".to_string()]),
                    ],
                },
            ],
            loaded: true,
        };
        
        (current_language, chapter_state, paragraph_state)
    }
    
    #[test]
    fn test_language_switching_logic() {
        let (mut current_language, _chapter_state, paragraph_state) = create_full_dashboard_state();
        
        // Initial state should be zh-TW
        assert_eq!(current_language, "zh-TW");
        
        // Simulate language change to English
        current_language = "en-US".to_string();
        assert_eq!(current_language, "en-US");
        
        // Test that paragraph content changes with language
        let paragraph = &paragraph_state.paragraphs[0];
        let zh_text = paragraph.texts.iter().find(|t| t.lang == "zh-TW").unwrap();
        let en_text = paragraph.texts.iter().find(|t| t.lang == "en-US").unwrap();
        
        assert_ne!(zh_text.paragraphs, en_text.paragraphs);
        assert_ne!(zh_text.choices[0], en_text.choices[0]);
    }
    
    #[test]
    fn test_chapter_selection_logic() {
        let (current_language, chapter_state, paragraph_state) = create_full_dashboard_state();
        
        // Test chapter selection
        let selected_chapter = "chapter1";
        assert!(!selected_chapter.is_empty());
        
        // Test getting paragraphs for selected chapter
        let chapter_paragraphs = paragraph_state.get_by_chapter(selected_chapter);
        assert_eq!(chapter_paragraphs.len(), 3); // para1, para2, para3 are in chapter1
        
        // Test chapter title retrieval
        let chapter = chapter_state.chapters.iter().find(|c| c.id == selected_chapter).unwrap();
        let title = chapter.titles.iter()
            .find(|t| t.lang == current_language)
            .map(|t| &t.title)
            .unwrap();
        assert_eq!(title, "第一章節");
    }
    
    #[test]
    fn test_paragraph_editing_logic() {
        let (current_language, _chapter_state, paragraph_state) = create_full_dashboard_state();
        
        // Simulate selecting a paragraph for editing
        let selected_paragraph = &paragraph_state.paragraphs[0];
        
        // Get existing content for current language
        let existing_text = selected_paragraph.texts.iter()
            .find(|t| t.lang == current_language)
            .unwrap();
        
        let original_content = &existing_text.paragraphs;
        let original_choices = &existing_text.choices;
        
        // Simulate editing
        let new_content = "這是修改過的段落內容。";
        let new_choices = vec!["新選項一".to_string(), "新選項二".to_string(), "新選項三".to_string()];
        
        // Test content change detection
        let content_changed = original_content != new_content;
        assert!(content_changed);
        
        let choices_changed = original_choices.len() != new_choices.len() || 
            original_choices.iter().zip(new_choices.iter()).any(|(old, new)| old != new);
        assert!(choices_changed);
    }
    
    #[test]
    fn test_choice_management_logic() {
        let (_current_language, _chapter_state, _paragraph_state) = create_full_dashboard_state();
        
        // Test creating new choices
        let mut choices = Vec::new();
        
        // Add a simple choice
        choices.push((
            "簡單選項".to_string(),
            vec!["target_para1".to_string()],
            "goto".to_string(),
            None::<String>,
            None::<serde_json::Value>,
            "chapter1".to_string(),
            false,
            None::<u32>,
        ));
        
        // Add a complex choice
        choices.push((
            "複雜選項".to_string(),
            vec!["target_para2".to_string(), "target_para3".to_string()],
            "conditional".to_string(),
            Some("inventory_item".to_string()),
            Some(serde_json::json!("magic_sword")),
            "chapter2".to_string(),
            true,
            Some(30),
        ));
        
        assert_eq!(choices.len(), 2);
        
        // Test choice validation
        let is_first_choice_valid = !choices[0].0.trim().is_empty() && !choices[0].1.is_empty();
        assert!(is_first_choice_valid);
        
        let is_second_choice_valid = !choices[1].0.trim().is_empty() && !choices[1].1.is_empty();
        assert!(is_second_choice_valid);
        
        // Test choice removal
        choices.remove(0);
        assert_eq!(choices.len(), 1);
        assert_eq!(choices[0].0, "複雜選項");
    }
    
    #[test]
    fn test_form_validation_logic() {
        let (_current_language, _chapter_state, _paragraph_state) = create_full_dashboard_state();
        
        // Test various form states
        let test_cases = vec![
            // (paragraphs, chapter, choices, expected_valid)
            ("Valid content", "chapter1", vec![("Choice", vec!["para1".to_string()], "goto", None::<String>, None::<serde_json::Value>, "chapter1", false, None::<u32>)], true),
            ("", "chapter1", vec![], false), // Empty content
            ("Valid content", "", vec![], false), // No chapter selected
            ("Valid content", "chapter1", vec![("Valid choice", vec!["para1".to_string()], "goto", None::<String>, None::<serde_json::Value>, "chapter1", false, None::<u32>)], true), // Valid choice with content
            ("Valid content", "chapter1", vec![("", vec![], "", None::<String>, None::<serde_json::Value>, "", false, None::<u32>)], false), // Invalid choice
            ("   ", "chapter1", vec![], false), // Whitespace only content
        ];
        
        for (content, chapter, choices, expected) in test_cases {
            let main_fields_valid = !content.trim().is_empty() && !chapter.is_empty();
            let has_any_choices = !choices.is_empty();
            let choices_valid = choices.iter().all(|(_choice_text, to, _type, _key, _value, _target_chapter, _same_page, _time_limit)| {
                // If there are target paragraphs, then it's valid
                !to.is_empty()
            });
            
            let is_form_valid = main_fields_valid && (!has_any_choices || choices_valid);
            assert_eq!(is_form_valid, expected, 
                "Failed for case: content='{}', chapter='{}', choices_count={}", 
                content, chapter, choices.len());
        }
    }
    
    #[test]
    fn test_edit_mode_toggle_logic() {
        let (_current_language, _chapter_state, paragraph_state) = create_full_dashboard_state();
        
        // Start in new mode
        let _is_edit_mode = false;
        let _selected_paragraph: Option<&Paragraph> = None;
        let mut paragraphs_content = String::new();
        let mut choices: Vec<(String, Vec<String>, String, Option<String>, Option<serde_json::Value>, String, bool, Option<u32>)> = Vec::new();
        
        // Toggle to edit mode
        let _is_edit_mode = true;
        
        // In edit mode, user should select a paragraph
        let _selected_paragraph = Some(&paragraph_state.paragraphs[0]);
        
        if let Some(paragraph) = _selected_paragraph {
            // Load paragraph content
            if let Some(text) = paragraph.texts.iter().find(|t| t.lang == "zh-TW") {
                paragraphs_content = text.paragraphs.clone();
                
                // Load choices
                for (i, choice_text) in text.choices.iter().enumerate() {
                    if let Some(paragraph_choice) = paragraph.choices.get(i) {
                        match paragraph_choice {
                            ParagraphChoice::Simple(targets) => {
                                choices.push((
                                    choice_text.clone(),
                                    targets.clone(),
                                    "goto".to_string(),
                                    None,
                                    None,
                                    "chapter1".to_string(),
                                    false,
                                    None,
                                ));
                            },
                            ParagraphChoice::Complex { to, type_, key, value, same_page, time_limit } => {
                                choices.push((
                                    choice_text.clone(),
                                    to.clone(),
                                    type_.clone(),
                                    key.clone(),
                                    value.clone(),
                                    "chapter1".to_string(),
                                    same_page.unwrap_or(false),
                                    *time_limit,
                                ));
                            },
                            _ => {}
                        }
                    }
                }
            }
        }
        
        assert!(_is_edit_mode);
        assert!(_selected_paragraph.is_some());
        assert!(!paragraphs_content.is_empty());
        assert_eq!(choices.len(), 2);
        
        // Toggle back to new mode
        let _is_edit_mode = false;
        let _selected_paragraph: Option<&Paragraph> = None;
        paragraphs_content.clear();
        choices.clear();
        
        assert!(!_is_edit_mode);
        assert!(_selected_paragraph.is_none());
        assert!(paragraphs_content.is_empty());
        assert!(choices.is_empty());
    }
    
    #[test]
    fn test_multi_language_content_logic() {
        let (_current_language, _chapter_state, paragraph_state) = create_full_dashboard_state();
        
        let paragraph = &paragraph_state.paragraphs[0];
        
        // Test content in Chinese
        let zh_text = paragraph.texts.iter().find(|t| t.lang == "zh-TW").unwrap();
        assert!(zh_text.paragraphs.contains("森林"));
        assert!(zh_text.choices[0].contains("左邊"));
        
        // Test content in English
        let en_text = paragraph.texts.iter().find(|t| t.lang == "en-US").unwrap();
        assert!(en_text.paragraphs.contains("forest"));
        assert!(en_text.choices[0].contains("left"));
        
        // Test fallback when translation doesn't exist
        let ja_text = paragraph.texts.iter().find(|t| t.lang == "ja-JP");
        assert!(ja_text.is_none());
        
        // Should fallback to first available language
        let fallback_text = paragraph.texts.first().unwrap();
        assert_eq!(fallback_text.lang, "zh-TW");
    }
    
    #[test]
    fn test_paragraph_filtering_by_chapter_logic() {
        let (_current_language, _chapter_state, paragraph_state) = create_full_dashboard_state();
        
        // Test filtering paragraphs by chapter
        let chapter1_paragraphs = paragraph_state.get_by_chapter("chapter1");
        let chapter2_paragraphs = paragraph_state.get_by_chapter("chapter2");
        let nonexistent_paragraphs = paragraph_state.get_by_chapter("chapter_nonexistent");
        
        assert_eq!(chapter1_paragraphs.len(), 3); // para1, para2, para3
        assert_eq!(chapter2_paragraphs.len(), 1); // para4
        assert_eq!(nonexistent_paragraphs.len(), 0);
        
        // Verify paragraph IDs
        let chapter1_ids: Vec<&str> = chapter1_paragraphs.iter().map(|p| p.id.as_str()).collect();
        assert!(chapter1_ids.contains(&"para1"));
        assert!(chapter1_ids.contains(&"para2"));
        assert!(chapter1_ids.contains(&"para3"));
        assert!(!chapter1_ids.contains(&"para4"));
    }
    
    #[test]
    fn test_complex_choice_structure_logic() {
        let (_current_language, _chapter_state, paragraph_state) = create_full_dashboard_state();
        
        let paragraph = &paragraph_state.paragraphs[0]; // para1
        
        // Test complex choice structure
        match &paragraph.choices[1] {
            ParagraphChoice::Complex { to, type_, key, value, same_page, time_limit } => {
                assert_eq!(to, &vec!["para3".to_string()]);
                assert_eq!(type_, "conditional");
                assert_eq!(key.as_ref().unwrap(), "forest_knowledge");
                assert_eq!(value.as_ref().unwrap(), &serde_json::json!(true));
                assert_eq!(same_page.unwrap(), false);
                assert_eq!(time_limit.unwrap(), 60);
            },
            _ => panic!("Expected Complex choice"),
        }
        
        let paragraph3 = &paragraph_state.paragraphs[2]; // para3
        match &paragraph3.choices[0] {
            ParagraphChoice::Complex { to, type_, key, value, same_page, time_limit } => {
                assert_eq!(to, &vec!["para6".to_string()]);
                assert_eq!(type_, "gain_item");
                assert_eq!(key.as_ref().unwrap(), "ancient_key");
                assert_eq!(value.as_ref().unwrap(), &serde_json::json!(1));
                assert_eq!(same_page.unwrap(), false);
                assert!(time_limit.is_none());
            },
            _ => panic!("Expected Complex choice"),
        }
    }
    
    #[test]
    fn test_edit_mode_language_switching_content_update() {
        let (_current_language, _chapter_state, paragraph_state) = create_full_dashboard_state();
        
        // Start in edit mode with a selected paragraph
        let selected_paragraph = &paragraph_state.paragraphs[0];
        let _current_ui_language = "zh-TW".to_string();
        let mut current_content_language = "zh-TW".to_string();
        
        // Initial content in Chinese
        let initial_text = selected_paragraph.texts.iter()
            .find(|t| t.lang == current_content_language)
            .unwrap();
        let initial_content = &initial_text.paragraphs;
        let initial_choices = &initial_text.choices;
        
        assert!(initial_content.contains("森林"));
        assert!(initial_choices[0].contains("左邊"));
        
        // Switch UI language to English (should not affect content being edited)
        let _current_ui_language = "en-US".to_string();
        
        // Content should remain the same as we're still editing Chinese version
        let same_text = selected_paragraph.texts.iter()
            .find(|t| t.lang == current_content_language)
            .unwrap();
        assert_eq!(same_text.paragraphs, *initial_content);
        assert_eq!(same_text.choices[0], initial_choices[0]);
        
        // Now switch content language to English
        current_content_language = "en-US".to_string();
        
        // Content should update to English version
        let english_text = selected_paragraph.texts.iter()
            .find(|t| t.lang == current_content_language)
            .unwrap();
        
        assert!(english_text.paragraphs.contains("forest"));
        assert!(english_text.choices[0].contains("left"));
        assert_ne!(english_text.paragraphs, *initial_content);
        assert_ne!(english_text.choices[0], initial_choices[0]);
        
        // Verify both language selections are independent
        let final_ui_language = "en-US";
        let final_content_language = &current_content_language;
        assert_eq!(final_ui_language, "en-US");
        assert_eq!(final_content_language, "en-US");
    }
    
    #[test]
    fn test_form_submit_button_state_management() {
        let (_current_language, _chapter_state, _paragraph_state) = create_full_dashboard_state();
        
        // Mock form state variables
        let mut paragraphs_content = String::new();
        let mut selected_chapter = String::new();
        let mut choices: Vec<(String, Vec<String>, String, Option<String>, Option<serde_json::Value>, String, bool, Option<u32>)> = Vec::new();
        
        // Function to check if submit button should be enabled
        let check_submit_enabled = |content: &str, chapter: &str, choices: &Vec<(String, Vec<String>, String, Option<String>, Option<serde_json::Value>, String, bool, Option<u32>)>| -> bool {
            let main_fields_valid = !content.trim().is_empty() && !chapter.is_empty();
            let has_any_choices = !choices.is_empty();
            let choices_valid = choices.iter().all(|(_choice_text, to, _type, _key, _value, _target_chapter, _same_page, _time_limit)| {
                !to.is_empty()
            });
            
            main_fields_valid && (!has_any_choices || choices_valid)
        };
        
        // Test Case 1: Empty form - button should be disabled
        assert!(!check_submit_enabled(&paragraphs_content, &selected_chapter, &choices));
        
        // Test Case 2: Add content but no chapter - button should be disabled
        paragraphs_content = "Test paragraph content".to_string();
        assert!(!check_submit_enabled(&paragraphs_content, &selected_chapter, &choices));
        
        // Test Case 3: Add chapter but no content - button should be disabled
        paragraphs_content = String::new();
        selected_chapter = "chapter1".to_string();
        assert!(!check_submit_enabled(&paragraphs_content, &selected_chapter, &choices));
        
        // Test Case 4: Valid content and chapter, no choices - button should be enabled
        paragraphs_content = "Test paragraph content".to_string();
        selected_chapter = "chapter1".to_string();
        assert!(check_submit_enabled(&paragraphs_content, &selected_chapter, &choices));
        
        // Test Case 5: Add invalid choice - button should be disabled
        choices.push((
            "Invalid choice".to_string(),
            vec![], // Empty target - invalid
            "goto".to_string(),
            None,
            None,
            "chapter1".to_string(),
            false,
            None,
        ));
        assert!(!check_submit_enabled(&paragraphs_content, &selected_chapter, &choices));
        
        // Test Case 6: Fix the choice - button should be enabled again
        choices[0].1 = vec!["para1".to_string()]; // Add valid target
        assert!(check_submit_enabled(&paragraphs_content, &selected_chapter, &choices));
        
        // Test Case 7: Add another valid choice - button should remain enabled
        choices.push((
            "Valid choice 2".to_string(),
            vec!["para2".to_string()],
            "conditional".to_string(),
            Some("key".to_string()),
            Some(serde_json::json!("value")),
            "chapter1".to_string(),
            false,
            Some(30),
        ));
        assert!(check_submit_enabled(&paragraphs_content, &selected_chapter, &choices));
        
        // Test Case 8: Clear content - button should be disabled
        paragraphs_content = String::new();
        assert!(!check_submit_enabled(&paragraphs_content, &selected_chapter, &choices));
        
        // Test Case 9: Whitespace only content - button should be disabled
        paragraphs_content = "   \n\t  ".to_string();
        assert!(!check_submit_enabled(&paragraphs_content, &selected_chapter, &choices));
        
        // Test Case 10: Restore valid content - button should be enabled
        paragraphs_content = "Valid content again".to_string();
        assert!(check_submit_enabled(&paragraphs_content, &selected_chapter, &choices));
    }
    
    #[test]
    fn test_edit_mode_comprehensive_language_switch_content_update() {
        let (_current_language, chapter_state, paragraph_state) = create_full_dashboard_state();
        
        // Simulate comprehensive edit mode state
        struct ComprehensiveEditState {
            // Language states
            current_ui_language: String,
            current_content_language: String,
            
            // Form states
            selected_paragraph_id: String,
            selected_chapter_id: String,
            
            // Content fields
            paragraph_content: String,
            choice_texts: Vec<String>,
            choice_targets: Vec<Vec<String>>,
            
            // UI Text states (what user sees in interface)
            ui_labels: std::collections::HashMap<String, String>,
            button_texts: std::collections::HashMap<String, String>,
            error_messages: std::collections::HashMap<String, String>,
            placeholder_texts: std::collections::HashMap<String, String>,
        }
        
        impl ComprehensiveEditState {
            fn get_ui_text(&self, key: &str, lang: &str) -> String {
                // Simulate getting UI text based on language
                match (key, lang) {
                    ("content_label", "zh-TW") => "段落內容".to_string(),
                    ("content_label", "en-US") => "Paragraph Content".to_string(),
                    ("chapter_label", "zh-TW") => "選擇章節".to_string(),
                    ("chapter_label", "en-US") => "Select Chapter".to_string(),
                    ("choices_label", "zh-TW") => "選項設定".to_string(),
                    ("choices_label", "en-US") => "Choice Settings".to_string(),
                    ("save_button", "zh-TW") => "儲存".to_string(),
                    ("save_button", "en-US") => "Save".to_string(),
                    ("cancel_button", "zh-TW") => "取消".to_string(),
                    ("cancel_button", "en-US") => "Cancel".to_string(),
                    ("content_placeholder", "zh-TW") => "請輸入段落內容...".to_string(),
                    ("content_placeholder", "en-US") => "Enter paragraph content...".to_string(),
                    ("choice_placeholder", "zh-TW") => "請輸入選項文字...".to_string(),
                    ("choice_placeholder", "en-US") => "Enter choice text...".to_string(),
                    ("required_error", "zh-TW") => "此欄位為必填".to_string(),
                    ("required_error", "en-US") => "This field is required".to_string(),
                    ("invalid_choice_error", "zh-TW") => "選項設定無效".to_string(),
                    ("invalid_choice_error", "en-US") => "Invalid choice configuration".to_string(),
                    _ => format!("Missing translation: {} ({})", key, lang),
                }
            }
            
            fn update_ui_language(&mut self, new_lang: String) {
                self.current_ui_language = new_lang.clone();
                
                // Update all UI text elements
                self.ui_labels.insert("content_label".to_string(), self.get_ui_text("content_label", &new_lang));
                self.ui_labels.insert("chapter_label".to_string(), self.get_ui_text("chapter_label", &new_lang));
                self.ui_labels.insert("choices_label".to_string(), self.get_ui_text("choices_label", &new_lang));
                
                self.button_texts.insert("save_button".to_string(), self.get_ui_text("save_button", &new_lang));
                self.button_texts.insert("cancel_button".to_string(), self.get_ui_text("cancel_button", &new_lang));
                
                self.placeholder_texts.insert("content_placeholder".to_string(), self.get_ui_text("content_placeholder", &new_lang));
                self.placeholder_texts.insert("choice_placeholder".to_string(), self.get_ui_text("choice_placeholder", &new_lang));
                
                self.error_messages.insert("required_error".to_string(), self.get_ui_text("required_error", &new_lang));
                self.error_messages.insert("invalid_choice_error".to_string(), self.get_ui_text("invalid_choice_error", &new_lang));
            }
            
            fn update_content_language(&mut self, new_lang: String, paragraph_state: &ParagraphState) {
                self.current_content_language = new_lang.clone();
                
                // Update paragraph content based on new language
                if let Some(paragraph) = paragraph_state.get_by_id(&self.selected_paragraph_id) {
                    if let Some(text) = paragraph.texts.iter().find(|t| t.lang == new_lang) {
                        self.paragraph_content = text.paragraphs.clone();
                        self.choice_texts = text.choices.clone();
                        
                        // Update choice targets (these don't change with language but validate consistency)
                        self.choice_targets = paragraph.choices.iter().map(|choice| {
                            match choice {
                                ParagraphChoice::Simple(targets) => targets.clone(),
                                ParagraphChoice::Complex { to, .. } => to.clone(),
                                _ => vec![],
                            }
                        }).collect();
                    }
                }
            }
            
            fn get_chapter_title(&self, chapter_id: &str, lang: &str, chapter_state: &ChapterState) -> String {
                if let Some(chapter) = chapter_state.chapters.iter().find(|c| c.id == chapter_id) {
                    if let Some(title) = chapter.titles.iter().find(|t| t.lang == lang) {
                        title.title.clone()
                    } else {
                        // Fallback to first available language
                        chapter.titles.first().map(|t| t.title.clone()).unwrap_or_default()
                    }
                } else {
                    String::new()
                }
            }
        }
        
        let mut edit_state = ComprehensiveEditState {
            current_ui_language: "zh-TW".to_string(),
            current_content_language: "zh-TW".to_string(),
            selected_paragraph_id: "para1".to_string(),
            selected_chapter_id: "chapter1".to_string(),
            paragraph_content: String::new(),
            choice_texts: Vec::new(),
            choice_targets: Vec::new(),
            ui_labels: std::collections::HashMap::new(),
            button_texts: std::collections::HashMap::new(),
            error_messages: std::collections::HashMap::new(),
            placeholder_texts: std::collections::HashMap::new(),
        };
        
        // Initialize with Chinese UI
        edit_state.update_ui_language("zh-TW".to_string());
        edit_state.update_content_language("zh-TW".to_string(), &paragraph_state);
        
        // Test Case 1: Initial state - all Chinese
        assert_eq!(edit_state.ui_labels.get("content_label").unwrap(), "段落內容");
        assert_eq!(edit_state.ui_labels.get("chapter_label").unwrap(), "選擇章節");
        assert_eq!(edit_state.button_texts.get("save_button").unwrap(), "儲存");
        assert_eq!(edit_state.placeholder_texts.get("content_placeholder").unwrap(), "請輸入段落內容...");
        assert_eq!(edit_state.error_messages.get("required_error").unwrap(), "此欄位為必填");
        
        // Content should be in Chinese
        assert!(edit_state.paragraph_content.contains("測試段落一"));
        assert!(edit_state.choice_texts[0].contains("走左邊"));
        
        // Chapter title should be in Chinese
        let chapter_title = edit_state.get_chapter_title(&edit_state.selected_chapter_id, &edit_state.current_ui_language, &chapter_state);
        assert_eq!(chapter_title, "第一章節");
        
        // Test Case 2: Switch UI language to English (content should remain Chinese)
        edit_state.update_ui_language("en-US".to_string());
        
        // UI elements should be in English
        assert_eq!(edit_state.ui_labels.get("content_label").unwrap(), "Paragraph Content");
        assert_eq!(edit_state.ui_labels.get("chapter_label").unwrap(), "Select Chapter");
        assert_eq!(edit_state.button_texts.get("save_button").unwrap(), "Save");
        assert_eq!(edit_state.placeholder_texts.get("content_placeholder").unwrap(), "Enter paragraph content...");
        assert_eq!(edit_state.error_messages.get("required_error").unwrap(), "This field is required");
        
        // Content should still be in Chinese (content language hasn't changed)
        assert!(edit_state.paragraph_content.contains("測試段落一"));
        assert!(edit_state.choice_texts[0].contains("走左邊"));
        
        // Chapter title should be in English (UI language)
        let chapter_title_en = edit_state.get_chapter_title(&edit_state.selected_chapter_id, &edit_state.current_ui_language, &chapter_state);
        assert_eq!(chapter_title_en, "Chapter One");
        
        // Test Case 3: Now switch content language to English
        edit_state.update_content_language("en-US".to_string(), &paragraph_state);
        
        // UI should remain in English
        assert_eq!(edit_state.ui_labels.get("content_label").unwrap(), "Paragraph Content");
        assert_eq!(edit_state.button_texts.get("save_button").unwrap(), "Save");
        
        // Content should now be in English
        assert!(edit_state.paragraph_content.contains("test paragraph one"));
        assert!(edit_state.choice_texts[0].contains("Take the left"));
        
        // Test Case 4: Switch UI back to Chinese while keeping content in English
        edit_state.update_ui_language("zh-TW".to_string());
        
        // UI should be in Chinese
        assert_eq!(edit_state.ui_labels.get("content_label").unwrap(), "段落內容");
        assert_eq!(edit_state.button_texts.get("save_button").unwrap(), "儲存");
        
        // Content should remain in English
        assert!(edit_state.paragraph_content.contains("test paragraph one"));
        assert!(edit_state.choice_texts[0].contains("Take the left"));
        
        // Chapter title should be in Chinese (UI language)
        let chapter_title_zh_again = edit_state.get_chapter_title(&edit_state.selected_chapter_id, &edit_state.current_ui_language, &chapter_state);
        assert_eq!(chapter_title_zh_again, "第一章節");
        
        // Test Case 5: Switch content back to Chinese
        edit_state.update_content_language("zh-TW".to_string(), &paragraph_state);
        
        // Everything should be in Chinese now
        assert_eq!(edit_state.ui_labels.get("content_label").unwrap(), "段落內容");
        assert!(edit_state.paragraph_content.contains("測試段落一"));
        assert!(edit_state.choice_texts[0].contains("走左邊"));
        
        // Test Case 6: Verify choice targets remain consistent across language switches
        assert_eq!(edit_state.choice_targets.len(), 2);
        assert_eq!(edit_state.choice_targets[0], vec!["para2".to_string()]);
        assert_eq!(edit_state.choice_targets[1], vec!["para3".to_string()]);
        
        // Test Case 7: Test validation error messages in different languages
        edit_state.update_ui_language("en-US".to_string());
        assert_eq!(edit_state.error_messages.get("required_error").unwrap(), "This field is required");
        assert_eq!(edit_state.error_messages.get("invalid_choice_error").unwrap(), "Invalid choice configuration");
        
        edit_state.update_ui_language("zh-TW".to_string());
        assert_eq!(edit_state.error_messages.get("required_error").unwrap(), "此欄位為必填");
        assert_eq!(edit_state.error_messages.get("invalid_choice_error").unwrap(), "選項設定無效");
    }
}

// Test edge cases and error scenarios
#[cfg(test)]
mod edge_case_tests {
    use super::*;
    
    #[test]
    fn test_empty_paragraph_content() {
        let paragraph = Paragraph {
            id: "empty_para".to_string(),
            chapter_id: "chapter1".to_string(),
            texts: vec![
                Text {
                    lang: "zh-TW".to_string(),
                    paragraphs: "".to_string(),
                    choices: vec![],
                },
            ],
            choices: vec![],
        };
        
        let text = &paragraph.texts[0];
        assert!(text.paragraphs.is_empty());
        assert!(text.choices.is_empty());
        assert!(paragraph.choices.is_empty());
    }
    
    #[test]
    fn test_mismatched_choice_counts() {
        let paragraph = Paragraph {
            id: "mismatched_para".to_string(),
            chapter_id: "chapter1".to_string(),
            texts: vec![
                Text {
                    lang: "zh-TW".to_string(),
                    paragraphs: "Test content".to_string(),
                    choices: vec!["Choice 1".to_string(), "Choice 2".to_string()],
                },
            ],
            choices: vec![
                ParagraphChoice::Simple(vec!["para1".to_string()]),
                // Missing second choice in paragraph.choices
            ],
        };
        
        let text_choices_count = paragraph.texts[0].choices.len();
        let paragraph_choices_count = paragraph.choices.len();
        
        assert_eq!(text_choices_count, 2);
        assert_eq!(paragraph_choices_count, 1);
        assert_ne!(text_choices_count, paragraph_choices_count);
    }
    
    #[test]
    fn test_invalid_target_references() {
        let paragraph_state = ParagraphState {
            paragraphs: vec![
                Paragraph {
                    id: "para_with_invalid_refs".to_string(),
                    chapter_id: "chapter1".to_string(),
                    texts: vec![
                        Text {
                            lang: "zh-TW".to_string(),
                            paragraphs: "Test content".to_string(),
                            choices: vec!["Go to nonexistent".to_string()],
                        },
                    ],
                    choices: vec![
                        ParagraphChoice::Simple(vec!["nonexistent_para".to_string()]),
                    ],
                },
            ],
            loaded: true,
        };
        
        // Try to find the referenced paragraph
        let referenced_paragraph = paragraph_state.get_by_id("nonexistent_para");
        assert!(referenced_paragraph.is_none());
    }
    
    #[test]
    fn test_circular_references() {
        let paragraph_state = ParagraphState {
            paragraphs: vec![
                Paragraph {
                    id: "para_a".to_string(),
                    chapter_id: "chapter1".to_string(),
                    texts: vec![
                        Text {
                            lang: "zh-TW".to_string(),
                            paragraphs: "Go to B".to_string(),
                            choices: vec!["To B".to_string()],
                        },
                    ],
                    choices: vec![
                        ParagraphChoice::Simple(vec!["para_b".to_string()]),
                    ],
                },
                Paragraph {
                    id: "para_b".to_string(),
                    chapter_id: "chapter1".to_string(),
                    texts: vec![
                        Text {
                            lang: "zh-TW".to_string(),
                            paragraphs: "Go to A".to_string(),
                            choices: vec!["To A".to_string()],
                        },
                    ],
                    choices: vec![
                        ParagraphChoice::Simple(vec!["para_a".to_string()]),
                    ],
                },
            ],
            loaded: true,
        };
        
        // Both paragraphs should exist
        let para_a = paragraph_state.get_by_id("para_a");
        let para_b = paragraph_state.get_by_id("para_b");
        
        assert!(para_a.is_some());
        assert!(para_b.is_some());
        
        // Check circular reference
        if let ParagraphChoice::Simple(targets) = &para_a.unwrap().choices[0] {
            assert_eq!(targets[0], "para_b");
        }
        
        if let ParagraphChoice::Simple(targets) = &para_b.unwrap().choices[0] {
            assert_eq!(targets[0], "para_a");
        }
    }
    
    #[test]
    fn test_malformed_json_values() {
        // Test with valid JSON value
        let valid_choice = ParagraphChoice::Complex {
            to: vec!["para1".to_string()],
            type_: "set_variable".to_string(),
            key: Some("player_health".to_string()),
            value: Some(serde_json::json!(100)),
            same_page: Some(false),
            time_limit: None,
        };
        
        match valid_choice {
            ParagraphChoice::Complex { value, .. } => {
                assert!(value.is_some());
                let val = value.unwrap();
                assert_eq!(val.as_i64().unwrap(), 100);
            },
            _ => panic!("Expected Complex choice"),
        }
        
        // Test with complex JSON object
        let complex_choice = ParagraphChoice::Complex {
            to: vec!["para2".to_string()],
            type_: "complex_action".to_string(),
            key: Some("action_data".to_string()),
            value: Some(serde_json::json!({
                "action": "move",
                "direction": "north",
                "distance": 5,
                "items": ["sword", "shield"]
            })),
            same_page: Some(true),
            time_limit: Some(120),
        };
        
        match complex_choice {
            ParagraphChoice::Complex { value, .. } => {
                assert!(value.is_some());
                let val = value.unwrap();
                assert_eq!(val["action"].as_str().unwrap(), "move");
                assert_eq!(val["distance"].as_i64().unwrap(), 5);
                assert!(val["items"].is_array());
            },
            _ => panic!("Expected Complex choice"),
        }
    }
} 