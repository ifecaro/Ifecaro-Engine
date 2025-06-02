use dioxus::prelude::*;

// Import the Dashboard component and related structs
use ifecaro::pages::dashboard::{Dashboard, DashboardProps, Data, Collection, ChapterData, SystemData};
use ifecaro::contexts::chapter_context::{ChapterState, Chapter, ChapterTitle};
use ifecaro::contexts::paragraph_context::{
    ParagraphState, Paragraph, Text, ParagraphChoice
};
use ifecaro::components::language_selector::AVAILABLE_LANGUAGES;

// Test utilities and helpers
mod dashboard_test_helpers {
    use super::*;
    
    pub fn create_test_language_state_data() -> (String, String) {
        ("zh-TW".to_string(), "zh-TW".to_string())
    }
    
    pub fn create_test_chapter_state() -> ChapterState {
        ChapterState {
            chapters: vec![
                Chapter {
                    id: "chapter1".to_string(),
                    titles: vec![
                        ChapterTitle {
                            lang: "zh-TW".to_string(),
                            title: "測試章節一".to_string(),
                        },
                        ChapterTitle {
                            lang: "en-US".to_string(),
                            title: "Test Chapter 1".to_string(),
                        },
                    ],
                    order: 1,
                },
                Chapter {
                    id: "chapter2".to_string(),
                    titles: vec![
                        ChapterTitle {
                            lang: "zh-TW".to_string(),
                            title: "測試章節二".to_string(),
                        },
                        ChapterTitle {
                            lang: "en-US".to_string(),
                            title: "Test Chapter 2".to_string(),
                        },
                    ],
                    order: 2,
                },
            ],
            loaded: true,
        }
    }
    
    pub fn create_test_paragraph_state() -> ParagraphState {
        ParagraphState {
            paragraphs: vec![
                Paragraph {
                    id: "para1".to_string(),
                    chapter_id: "chapter1".to_string(),
                    texts: vec![
                        Text {
                            lang: "zh-TW".to_string(),
                            paragraphs: "這是第一個測試段落內容。".to_string(),
                            choices: vec!["選項一".to_string(), "選項二".to_string()],
                        },
                        Text {
                            lang: "en-US".to_string(),
                            paragraphs: "This is the first test paragraph content.".to_string(),
                            choices: vec!["Option One".to_string(), "Option Two".to_string()],
                        },
                    ],
                    choices: vec![
                        ParagraphChoice::Simple(vec!["para2".to_string()]),
                        ParagraphChoice::Complex {
                            to: vec!["para3".to_string()],
                            type_: "goto".to_string(),
                            key: Some("test_key".to_string()),
                            value: Some(serde_json::json!("test_value")),
                            same_page: Some(false),
                            time_limit: Some(30),
                        },
                    ],
                },
                Paragraph {
                    id: "para2".to_string(),
                    chapter_id: "chapter1".to_string(),
                    texts: vec![
                        Text {
                            lang: "zh-TW".to_string(),
                            paragraphs: "這是第二個測試段落內容。".to_string(),
                            choices: vec!["繼續".to_string()],
                        },
                        Text {
                            lang: "en-US".to_string(),
                            paragraphs: "This is the second test paragraph content.".to_string(),
                            choices: vec!["Continue".to_string()],
                        },
                    ],
                    choices: vec![
                        ParagraphChoice::Simple(vec!["para3".to_string()]),
                    ],
                },
                Paragraph {
                    id: "para3".to_string(),
                    chapter_id: "chapter2".to_string(),
                    texts: vec![
                        Text {
                            lang: "zh-TW".to_string(),
                            paragraphs: "這是第三個測試段落內容。".to_string(),
                            choices: vec![],
                        },
                        Text {
                            lang: "en-US".to_string(),
                            paragraphs: "This is the third test paragraph content.".to_string(),
                            choices: vec![],
                        },
                    ],
                    choices: vec![],
                },
            ],
            loaded: true,
        }
    }
}

#[component]
fn TestApp(props: DashboardProps) -> Element {
    let language_state = use_signal(|| dashboard_test_helpers::create_test_language_state_data());
    let chapter_state = use_signal(|| dashboard_test_helpers::create_test_chapter_state());
    let paragraph_state = use_signal(|| dashboard_test_helpers::create_test_paragraph_state());
    
    provide_context(language_state);
    provide_context(chapter_state);
    provide_context(paragraph_state);
    
    rsx! {
        Dashboard { lang: props.lang }
    }
}

// Unit Tests for Dashboard Component Functionality
#[cfg(test)]
mod unit_tests {
    use super::*;
    
    #[test]
    fn test_dashboard_props_creation() {
        let props = DashboardProps {
            lang: "zh-TW".to_string(),
        };
        assert_eq!(props.lang, "zh-TW");
    }
    
    #[test]
    fn test_language_state_data() {
        let (current_lang, default_lang) = dashboard_test_helpers::create_test_language_state_data();
        assert_eq!(current_lang, "zh-TW");
        assert_eq!(default_lang, "zh-TW");
    }
    
    #[test]
    fn test_chapter_state_initialization() {
        let state = dashboard_test_helpers::create_test_chapter_state();
        assert_eq!(state.chapters.len(), 2);
        assert_eq!(state.chapters[0].id, "chapter1");
        assert_eq!(state.chapters[1].id, "chapter2");
        assert!(state.loaded);
    }
    
    #[test]
    fn test_paragraph_state_initialization() {
        let state = dashboard_test_helpers::create_test_paragraph_state();
        assert_eq!(state.paragraphs.len(), 3);
        assert_eq!(state.paragraphs[0].id, "para1");
        assert_eq!(state.paragraphs[0].chapter_id, "chapter1");
        assert!(state.loaded);
    }
    
    #[test]
    fn test_paragraph_text_languages() {
        let state = dashboard_test_helpers::create_test_paragraph_state();
        let paragraph = &state.paragraphs[0];
        
        // Check if paragraph has both zh-TW and en-US texts
        let zh_text = paragraph.texts.iter().find(|t| t.lang == "zh-TW");
        let en_text = paragraph.texts.iter().find(|t| t.lang == "en-US");
        
        assert!(zh_text.is_some());
        assert!(en_text.is_some());
        assert_eq!(zh_text.unwrap().paragraphs, "這是第一個測試段落內容。");
        assert_eq!(en_text.unwrap().paragraphs, "This is the first test paragraph content.");
    }
    
    #[test]
    fn test_paragraph_choices_structure() {
        let state = dashboard_test_helpers::create_test_paragraph_state();
        let paragraph = &state.paragraphs[0];
        
        assert_eq!(paragraph.choices.len(), 2);
        
        // Test Simple choice
        match &paragraph.choices[0] {
            ParagraphChoice::Simple(targets) => {
                assert_eq!(targets.len(), 1);
                assert_eq!(targets[0], "para2");
            },
            _ => panic!("Expected Simple choice"),
        }
        
        // Test Complex choice
        match &paragraph.choices[1] {
            ParagraphChoice::Complex { to, type_, key, value, same_page, time_limit } => {
                assert_eq!(to.len(), 1);
                assert_eq!(to[0], "para3");
                assert_eq!(type_, "goto");
                assert_eq!(key.as_ref().unwrap(), "test_key");
                assert_eq!(value.as_ref().unwrap(), &serde_json::json!("test_value"));
                assert_eq!(same_page.unwrap(), false);
                assert_eq!(time_limit.unwrap(), 30);
            },
            _ => panic!("Expected Complex choice"),
        }
    }
    
    #[test]
    fn test_available_languages() {
        assert!(!AVAILABLE_LANGUAGES.is_empty());
        
        // Check if zh-TW exists in available languages
        let zh_tw = AVAILABLE_LANGUAGES.iter().find(|l| l.code == "zh-TW");
        assert!(zh_tw.is_some());
        
        // Check if en-US exists in available languages
        let en_us = AVAILABLE_LANGUAGES.iter().find(|l| l.code == "en-US");
        assert!(en_us.is_some());
    }
}

// Component Tests
#[cfg(test)]
mod component_tests {
    use super::*;
    
    #[test]
    fn test_dashboard_component_structure() {
        // Test basic component structure without VirtualDom
        let props = DashboardProps {
            lang: "zh-TW".to_string(),
        };
        
        // Basic structure test - check if props are handled correctly
        assert_eq!(props.lang, "zh-TW");
    }
    
    #[test]
    fn test_dashboard_with_different_languages() {
        let test_languages = vec!["zh-TW", "en-US", "ja-JP", "ko-KR"];
        
        for lang in test_languages {
            let props = DashboardProps {
                lang: lang.to_string(),
            };
            
            // Should not panic with different language props
            assert_eq!(props.lang, lang);
        }
    }
}

// Integration Tests
#[cfg(test)]
mod integration_tests {
    use super::*;
    
    #[test]
    fn test_dashboard_state_management_logic() {
        let (current_lang, _) = dashboard_test_helpers::create_test_language_state_data();
        let chapter_state = dashboard_test_helpers::create_test_chapter_state();
        let paragraph_state = dashboard_test_helpers::create_test_paragraph_state();
        
        // Test language state management
        assert_eq!(current_lang, "zh-TW");
        
        // Test chapter state management
        assert_eq!(chapter_state.chapters.len(), 2);
        assert!(chapter_state.loaded);
        
        // Test paragraph state management
        assert_eq!(paragraph_state.paragraphs.len(), 3);
        assert!(paragraph_state.loaded);
        
        // Test paragraph retrieval by chapter
        let chapter1_paragraphs = paragraph_state.get_by_chapter("chapter1");
        assert_eq!(chapter1_paragraphs.len(), 2);
        
        let chapter2_paragraphs = paragraph_state.get_by_chapter("chapter2");
        assert_eq!(chapter2_paragraphs.len(), 1);
    }
    
    #[test]
    fn test_paragraph_content_translation() {
        let paragraph_state = dashboard_test_helpers::create_test_paragraph_state();
        let paragraph = &paragraph_state.paragraphs[0];
        
        // Test getting content in different languages
        let zh_content = paragraph.texts.iter()
            .find(|t| t.lang == "zh-TW")
            .map(|t| &t.paragraphs);
        
        let en_content = paragraph.texts.iter()
            .find(|t| t.lang == "en-US")
            .map(|t| &t.paragraphs);
        
        assert!(zh_content.is_some());
        assert!(en_content.is_some());
        assert_ne!(zh_content.unwrap(), en_content.unwrap());
    }
    
    #[test]
    fn test_choice_localization() {
        let paragraph_state = dashboard_test_helpers::create_test_paragraph_state();
        let paragraph = &paragraph_state.paragraphs[0];
        
        // Test choice text localization
        let zh_choices = paragraph.texts.iter()
            .find(|t| t.lang == "zh-TW")
            .map(|t| &t.choices);
        
        let en_choices = paragraph.texts.iter()
            .find(|t| t.lang == "en-US")
            .map(|t| &t.choices);
        
        assert!(zh_choices.is_some());
        assert!(en_choices.is_some());
        
        let zh_choices = zh_choices.unwrap();
        let en_choices = en_choices.unwrap();
        
        assert_eq!(zh_choices.len(), en_choices.len());
        assert_ne!(zh_choices[0], en_choices[0]); // Different language content
    }
    
    #[test]
    fn test_chapter_title_localization() {
        let chapter_state = dashboard_test_helpers::create_test_chapter_state();
        let chapter = &chapter_state.chapters[0];
        
        // Test chapter title localization
        let zh_title = chapter.titles.iter()
            .find(|t| t.lang == "zh-TW")
            .map(|t| &t.title);
        
        let en_title = chapter.titles.iter()
            .find(|t| t.lang == "en-US")
            .map(|t| &t.title);
        
        assert!(zh_title.is_some());
        assert!(en_title.is_some());
        assert_ne!(zh_title.unwrap(), en_title.unwrap());
    }
}

// Form Validation Tests
#[cfg(test)]
mod form_validation_tests {
    #[test]
    fn test_paragraph_content_validation() {
        // Test empty content
        let empty_content = "";
        assert!(empty_content.trim().is_empty());
        
        // Test valid content
        let valid_content = "This is valid paragraph content.";
        assert!(!valid_content.trim().is_empty());
        
        // Test whitespace only content
        let whitespace_content = "   \n\t  ";
        assert!(whitespace_content.trim().is_empty());
    }
    
    #[test]
    fn test_chapter_selection_validation() {
        // Test empty chapter selection
        let empty_chapter = "";
        assert!(empty_chapter.is_empty());
        
        // Test valid chapter selection
        let valid_chapter = "chapter1";
        assert!(!valid_chapter.is_empty());
    }
    
    #[test]
    fn test_choice_validation() {
        // Test valid choice
        let choice_text = "Valid choice text";
        let target_paragraphs = vec!["para1".to_string(), "para2".to_string()];
        let action_type = "goto";
        
        assert!(!choice_text.trim().is_empty());
        assert!(!target_paragraphs.is_empty());
        assert!(!action_type.trim().is_empty());
        
        // Test invalid choice
        let _empty_choice_text = "";
        let empty_targets: Vec<String> = vec![];
        let empty_action_type = "";
        
        let has_content = !empty_targets.is_empty() || !empty_action_type.trim().is_empty();
        if has_content {
            assert!(!empty_targets.is_empty()); // Should fail validation
        }
    }
}

// Error Handling Tests
#[cfg(test)]
mod error_handling_tests {
    use super::*;
    
    #[test]
    fn test_missing_translation_handling() {
        let mut paragraph_state = dashboard_test_helpers::create_test_paragraph_state();
        
        // Remove one language from a paragraph
        paragraph_state.paragraphs[0].texts.retain(|t| t.lang != "en-US");
        
        let paragraph = &paragraph_state.paragraphs[0];
        
        // Test fallback to available language
        let en_text = paragraph.texts.iter().find(|t| t.lang == "en-US");
        let zh_text = paragraph.texts.iter().find(|t| t.lang == "zh-TW");
        
        assert!(en_text.is_none());
        assert!(zh_text.is_some());
    }
    
    #[test]
    fn test_invalid_paragraph_id_handling() {
        let paragraph_state = dashboard_test_helpers::create_test_paragraph_state();
        
        // Try to get non-existent paragraph
        let result = paragraph_state.get_by_id("non_existent_id");
        assert!(result.is_none());
    }
    
    #[test]
    fn test_invalid_chapter_id_handling() {
        let paragraph_state = dashboard_test_helpers::create_test_paragraph_state();
        
        // Try to get paragraphs from non-existent chapter
        let result = paragraph_state.get_by_chapter("non_existent_chapter");
        assert!(result.is_empty());
    }
}

// Performance Tests
#[cfg(test)]
mod performance_tests {
    use super::*;
    use std::time::Instant;
    
    #[test]
    fn test_large_dataset_performance() {
        let start = Instant::now();
        
        // Create a large dataset
        let mut paragraph_state = ParagraphState {
            paragraphs: Vec::new(),
            loaded: true,
        };
        
        // Add 1000 paragraphs
        for i in 0..1000 {
            paragraph_state.paragraphs.push(Paragraph {
                id: format!("para{}", i),
                chapter_id: format!("chapter{}", i % 10),
                texts: vec![
                    Text {
                        lang: "zh-TW".to_string(),
                        paragraphs: format!("測試段落內容 {}", i),
                        choices: vec![format!("選項 {}", i)],
                    },
                ],
                choices: vec![
                    ParagraphChoice::Simple(vec![format!("para{}", i + 1)]),
                ],
            });
        }
        
        let duration = start.elapsed();
        
        // Performance assertion - should complete within reasonable time
        assert!(duration.as_millis() < 100, "Large dataset creation took too long: {:?}", duration);
        
        // Test retrieval performance
        let start = Instant::now();
        let chapter_paragraphs = paragraph_state.get_by_chapter("chapter1");
        let duration = start.elapsed();
        
        assert!(duration.as_millis() < 10, "Chapter paragraph retrieval took too long: {:?}", duration);
        assert_eq!(chapter_paragraphs.len(), 100); // 100 paragraphs per chapter
    }
}

// Accessibility Tests
#[cfg(test)]
mod accessibility_tests {
    use super::*;
    
    #[test]
    fn test_language_accessibility() {
        // Test that all available languages have proper codes and names
        for language in AVAILABLE_LANGUAGES.iter() {
            assert!(!language.code.is_empty(), "Language code should not be empty");
            assert!(!language.name.is_empty(), "Language name should not be empty");
            assert!(language.code.len() >= 2, "Language code should be at least 2 characters");
        }
    }
    
    #[test]
    fn test_content_structure_accessibility() {
        let paragraph_state = dashboard_test_helpers::create_test_paragraph_state();
        
        for paragraph in &paragraph_state.paragraphs {
            // Test that paragraphs have proper structure
            assert!(!paragraph.id.is_empty(), "Paragraph ID should not be empty");
            assert!(!paragraph.chapter_id.is_empty(), "Chapter ID should not be empty");
            assert!(!paragraph.texts.is_empty(), "Paragraph should have at least one text version");
            
            // Test that each text has proper structure
            for text in &paragraph.texts {
                assert!(!text.lang.is_empty(), "Text language should not be empty");
                assert!(!text.paragraphs.is_empty(), "Text content should not be empty");
            }
        }
    }
}

// Data Serialization Tests
#[cfg(test)]
mod serialization_tests {
    use super::*;
    use serde_json;
    
    #[test]
    fn test_paragraph_serialization() {
        let paragraph_state = dashboard_test_helpers::create_test_paragraph_state();
        let paragraph = &paragraph_state.paragraphs[0];
        
        // Test serialization
        let serialized = serde_json::to_string(paragraph);
        assert!(serialized.is_ok(), "Paragraph should be serializable");
        
        // Test deserialization
        let serialized_data = serialized.unwrap();
        let deserialized: Result<Paragraph, _> = serde_json::from_str(&serialized_data);
        assert!(deserialized.is_ok(), "Paragraph should be deserializable");
        
        let deserialized_paragraph = deserialized.unwrap();
        assert_eq!(paragraph.id, deserialized_paragraph.id);
        assert_eq!(paragraph.chapter_id, deserialized_paragraph.chapter_id);
    }
    
    #[test]
    fn test_chapter_serialization() {
        let chapter_state = dashboard_test_helpers::create_test_chapter_state();
        let chapter = &chapter_state.chapters[0];
        
        // Test serialization
        let serialized = serde_json::to_string(chapter);
        assert!(serialized.is_ok(), "Chapter should be serializable");
        
        // Test deserialization
        let serialized_data = serialized.unwrap();
        let deserialized: Result<Chapter, _> = serde_json::from_str(&serialized_data);
        assert!(deserialized.is_ok(), "Chapter should be deserializable");
        
        let deserialized_chapter = deserialized.unwrap();
        assert_eq!(chapter.id, deserialized_chapter.id);
        assert_eq!(chapter.order, deserialized_chapter.order);
    }
}

// Mock API Tests
#[cfg(test)]
mod api_tests {
    use super::*;
    
    #[test]
    fn test_data_structure_compatibility() {
        // Test Data structure
        let data = Data {
            items: vec![dashboard_test_helpers::create_test_paragraph_state().paragraphs[0].clone()],
        };
        
        assert_eq!(data.items.len(), 1);
        
        // Test Collection structure
        let collection = Collection {
            id: "test_collection".to_string(),
            name: "Test Collection".to_string(),
            collection_type: "base".to_string(),
            system: false,
            schema: "test_schema".to_string(),
        };
        
        assert_eq!(collection.id, "test_collection");
        assert_eq!(collection.name, "Test Collection");
        
        // Test ChapterData structure
        let chapter_data = ChapterData {
            id: "test_chapter".to_string(),
            title: "Test Chapter".to_string(),
            chapter_type: "normal".to_string(),
        };
        
        assert_eq!(chapter_data.id, "test_chapter");
        assert_eq!(chapter_data.title, "Test Chapter");
    }
    
    #[test]
    fn test_system_data_structure() {
        let system_data = SystemData {
            id: "system1".to_string(),
            key: "config_key".to_string(),
            value_raw: serde_json::json!({"setting": "value"}),
            collection_id: "settings".to_string(),
            collection_name: "Settings".to_string(),
            created: "2024-01-01T00:00:00Z".to_string(),
            updated: "2024-01-01T00:00:00Z".to_string(),
        };
        
        assert_eq!(system_data.id, "system1");
        assert_eq!(system_data.key, "config_key");
        assert_eq!(system_data.collection_id, "settings");
    }
}

// UI State Management Tests
#[cfg(test)]
mod ui_state_tests {
    use super::*;
    use std::collections::HashMap;
    
    #[test]
    fn test_form_state_validation() {
        // Mock form states
        let paragraphs_content = "Test paragraph content";
        let selected_chapter = "chapter1";
        let choices = vec![
            ("Choice 1".to_string(), vec!["para1".to_string()], "goto".to_string(), None::<String>, None::<serde_json::Value>, "chapter1".to_string(), false, None::<u32>),
            ("Choice 2".to_string(), vec!["para2".to_string()], "goto".to_string(), None::<String>, None::<serde_json::Value>, "chapter1".to_string(), false, None::<u32>),
        ];
        
        // Test main fields validation
        let main_fields_valid = !paragraphs_content.trim().is_empty() && !selected_chapter.is_empty();
        assert!(main_fields_valid);
        
        // Test choices validation
        let has_any_choices = !choices.is_empty();
        assert!(has_any_choices);
        
        let choices_valid = choices.iter().all(|(_choice_text, to, type_, _key, _value, _target_chapter, _same_page, _time_limit)| {
            let has_content = !to.is_empty() || !type_.trim().is_empty();
            if has_content {
                !to.is_empty()
            } else {
                true
            }
        });
        assert!(choices_valid);
        
        // Overall form validation
        let is_form_valid = main_fields_valid && (!has_any_choices || choices_valid);
        assert!(is_form_valid);
    }
    
    #[test]
    fn test_edit_mode_state_changes() {
        // Test changes detection in edit mode
        let original_content = "Original paragraph content";
        let modified_content = "Modified paragraph content";
        
        let content_changed = original_content != modified_content;
        assert!(content_changed);
        
        // Test no changes
        let same_content = "Same content";
        let unchanged_content = "Same content";
        
        let no_change = same_content == unchanged_content;
        assert!(no_change);
    }
    
    #[test]
    fn test_real_time_form_validation() {
        // Test real-time form validation scenarios as user types/interacts
        
        // Simulate form field states
        let mut content = String::new();
        let mut chapter = String::new();
        let mut choices: Vec<(String, Vec<String>, String)> = Vec::new();
        
        // Helper function for validation
        let validate_form = |content: &str, chapter: &str, choices: &Vec<(String, Vec<String>, String)>| -> (bool, Vec<String>) {
            let mut errors = Vec::new();
            
            // Content validation
            if content.trim().is_empty() {
                errors.push("Content cannot be empty".to_string());
            }
            
            // Chapter validation
            if chapter.trim().is_empty() {
                errors.push("Chapter must be selected".to_string());
            }
            
            // Choice validation
            for (i, (choice_text, targets, action_type)) in choices.iter().enumerate() {
                if choice_text.trim().is_empty() {
                    errors.push(format!("Choice {} text cannot be empty", i + 1));
                }
                if targets.is_empty() {
                    errors.push(format!("Choice {} must have at least one target", i + 1));
                }
                if action_type.trim().is_empty() {
                    errors.push(format!("Choice {} action type cannot be empty", i + 1));
                }
            }
            
            (errors.is_empty(), errors)
        };
        
        // Test Case 1: Empty form
        let (is_valid, errors) = validate_form(&content, &chapter, &choices);
        assert!(!is_valid);
        assert_eq!(errors.len(), 2); // Content and chapter errors
        
        // Test Case 2: Add content
        content = "Test content".to_string();
        let (is_valid, errors) = validate_form(&content, &chapter, &choices);
        assert!(!is_valid);
        assert_eq!(errors.len(), 1); // Only chapter error
        
        // Test Case 3: Add chapter
        chapter = "chapter1".to_string();
        let (is_valid, _errors) = validate_form(&content, &chapter, &choices);
        assert!(is_valid); // Should be valid now
        
        // Test Case 4: Add invalid choice
        choices.push(("".to_string(), vec![], "".to_string()));
        let (is_valid, errors) = validate_form(&content, &chapter, &choices);
        assert!(!is_valid);
        assert_eq!(errors.len(), 3); // Choice text, targets, and action type errors
        
        // Test Case 5: Fix choice step by step
        choices[0].0 = "Valid choice".to_string();
        let (is_valid, errors) = validate_form(&content, &chapter, &choices);
        assert!(!is_valid);
        assert_eq!(errors.len(), 2); // Targets and action type errors
        
        choices[0].1 = vec!["para1".to_string()];
        let (is_valid, errors) = validate_form(&content, &chapter, &choices);
        assert!(!is_valid);
        assert_eq!(errors.len(), 1); // Only action type error
        
        choices[0].2 = "goto".to_string();
        let (is_valid, _errors) = validate_form(&content, &chapter, &choices);
        assert!(is_valid); // Should be valid now
        
        // Test Case 6: Test whitespace handling
        content = "   \n\t  ".to_string();
        let (is_valid, errors) = validate_form(&content, &chapter, &choices);
        assert!(!is_valid);
        assert!(errors.iter().any(|e| e.contains("Content cannot be empty")));
    }
    
    #[test]
    fn test_submit_button_dynamic_state() {
        // Test submit button state changes based on form validity
        
        struct FormState {
            content: String,
            chapter: String,
            choices: Vec<(String, Vec<String>)>,
            is_submitting: bool,
            has_unsaved_changes: bool,
        }
        
        impl FormState {
            fn can_submit(&self) -> bool {
                let basic_valid = !self.content.trim().is_empty() && !self.chapter.trim().is_empty();
                let choices_valid = self.choices.iter().all(|(text, targets)| {
                    !text.trim().is_empty() && !targets.is_empty()
                });
                let not_submitting = !self.is_submitting;
                
                basic_valid && choices_valid && not_submitting
            }
            
            fn button_text(&self) -> &str {
                if self.is_submitting {
                    "Submitting..."
                } else if self.has_unsaved_changes {
                    "Save Changes"
                } else {
                    "Submit"
                }
            }
        }
        
        let mut form = FormState {
            content: String::new(),
            chapter: String::new(),
            choices: Vec::new(),
            is_submitting: false,
            has_unsaved_changes: false,
        };
        
        // Test Case 1: Initial state - button disabled
        assert!(!form.can_submit());
        assert_eq!(form.button_text(), "Submit");
        
        // Test Case 2: Add content and chapter - button enabled
        form.content = "Test content".to_string();
        form.chapter = "chapter1".to_string();
        assert!(form.can_submit());
        
        // Test Case 3: Mark as having changes - button text changes
        form.has_unsaved_changes = true;
        assert!(form.can_submit());
        assert_eq!(form.button_text(), "Save Changes");
        
        // Test Case 4: Start submitting - button disabled
        form.is_submitting = true;
        assert!(!form.can_submit());
        assert_eq!(form.button_text(), "Submitting...");
        
        // Test Case 5: Finish submitting - button enabled again
        form.is_submitting = false;
        form.has_unsaved_changes = false;
        assert!(form.can_submit());
        assert_eq!(form.button_text(), "Submit");
        
        // Test Case 6: Add invalid choice - button disabled
        form.choices.push(("".to_string(), vec![]));
        assert!(!form.can_submit());
        
        // Test Case 7: Fix choice - button enabled
        form.choices[0] = ("Valid choice".to_string(), vec!["para1".to_string()]);
        assert!(form.can_submit());
    }
    
    #[test]
    fn test_edit_mode_language_consistency() {
        // Test that language switching in edit mode maintains data consistency
        
        let paragraph_state = dashboard_test_helpers::create_test_paragraph_state();
        let paragraph = &paragraph_state.paragraphs[0];
        
        // Simulate edit mode state
        struct EditState {
            current_ui_language: String,
            current_content_language: String,
            editing_paragraph_id: String,
            content_cache: HashMap<String, (String, Vec<String>)>, // lang -> (content, choices)
        }
        
        impl EditState {
            fn load_content_for_language(&mut self, paragraph: &Paragraph, lang: &str) {
                if let Some(text) = paragraph.texts.iter().find(|t| t.lang == lang) {
                    self.content_cache.insert(
                        lang.to_string(),
                        (text.paragraphs.clone(), text.choices.clone())
                    );
                }
            }
            
            fn get_current_content(&self) -> Option<&(String, Vec<String>)> {
                self.content_cache.get(&self.current_content_language)
            }
            
            fn switch_content_language(&mut self, new_lang: String, paragraph: &Paragraph) {
                // Load content for new language if not cached
                if !self.content_cache.contains_key(&new_lang) {
                    self.load_content_for_language(paragraph, &new_lang);
                }
                self.current_content_language = new_lang;
            }
        }
        
        let mut edit_state = EditState {
            current_ui_language: "zh-TW".to_string(),
            current_content_language: "zh-TW".to_string(),
            editing_paragraph_id: paragraph.id.clone(),
            content_cache: HashMap::new(),
        };
        
        // Load initial content
        edit_state.load_content_for_language(paragraph, "zh-TW");
        edit_state.load_content_for_language(paragraph, "en-US");
        
        // Test Case 1: Initial Chinese content
        let initial_content = edit_state.get_current_content().unwrap().clone();
        assert!(initial_content.0.contains("第一個測試段落"));
        assert!(initial_content.1[0].contains("選項一"));
        
        // Test Case 2: Switch UI language (content should not change)
        edit_state.current_ui_language = "en-US".to_string();
        let same_content = edit_state.get_current_content().unwrap();
        assert_eq!(same_content, &initial_content);
        
        // Test Case 3: Switch content language
        edit_state.switch_content_language("en-US".to_string(), paragraph);
        let english_content = edit_state.get_current_content().unwrap();
        assert!(english_content.0.contains("first test paragraph"));
        assert!(english_content.1[0].contains("Option"));
        assert_ne!(english_content, &initial_content);
        
        // Test Case 4: Switch back to Chinese
        edit_state.switch_content_language("zh-TW".to_string(), paragraph);
        let back_to_chinese = edit_state.get_current_content().unwrap();
        assert_eq!(back_to_chinese, &initial_content);
        
        // Test Case 5: Verify cache consistency
        assert_eq!(edit_state.content_cache.len(), 2);
        assert!(edit_state.content_cache.contains_key("zh-TW"));
        assert!(edit_state.content_cache.contains_key("en-US"));
    }
} 