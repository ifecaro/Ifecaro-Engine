mod common;

use common::*;

#[cfg(test)]
mod reader_mode_tests {
    use super::*;
    use ifecaro::pages::story::{merge_paragraphs_for_lang, Paragraph, Text, ComplexChoice};
    use rand::prelude::*;
    use std::collections::HashMap;

    fn create_test_paragraph_with_choices(
        id: &str, 
        chapter_id: &str, 
        lang: &str, 
        text: &str,
        choice_texts: Vec<&str>,
        choice_targets: Vec<Vec<&str>>
    ) -> Paragraph {
        let choices_text = Text {
            lang: lang.to_string(),
            paragraphs: text.to_string(),
            choices: choice_texts.into_iter().map(|s| s.to_string()).collect(),
        };
        
        let complex_choices: Vec<ComplexChoice> = choice_targets.into_iter().map(|targets| {
            ComplexChoice {
                to: targets.into_iter().map(|s| s.to_string()).collect(),
                type_: "goto".to_string(),
                key: None,
                value: None,
                same_page: None,
                time_limit: None,
            }
        }).collect();
        
        Paragraph {
            id: id.to_string(),
            chapter_id: chapter_id.to_string(),
            texts: vec![choices_text],
            choices: complex_choices,
            collection_id: String::new(),
            collection_name: String::new(),
            created: String::new(),
            updated: String::new(),
        }
    }

    #[test]
    fn test_reader_mode_displays_all_expanded_paragraphs() {
        // Create a story path with multiple paragraphs
        let paragraphs = vec![
            create_test_paragraph("start", "chapter1", "en", "Story begins here."),
            create_test_paragraph("middle1", "chapter1", "en", "First development."),
            create_test_paragraph("middle2", "chapter1", "en", "Second development."),
            create_test_paragraph("end", "chapter1", "en", "Story ends here."),
        ];
        
        // In reader mode, all paragraphs should be displayed
        let result = merge_paragraphs_for_lang(
            &paragraphs,
            "en",
            true,  // reader_mode = true
            false, // is_settings_chapter = false
            &[],   // choice_ids (not used in reader mode)
        );
        
        let expected = "Story begins here.\n\nFirst development.\n\nSecond development.\n\nStory ends here.";
        assert_eq!(result, expected);
    }

    #[test]
    fn test_reader_mode_vs_normal_mode_difference() {
        let paragraphs = vec![
            create_test_paragraph("p1", "chapter1", "en", "Paragraph 1"),
            create_test_paragraph("p2", "chapter1", "en", "Paragraph 2"),
            create_test_paragraph("p3", "chapter1", "en", "Paragraph 3"),
        ];
        
        // Normal mode should display all paragraphs
        let normal_result = merge_paragraphs_for_lang(
            &paragraphs,
            "en",
            false, // reader_mode = false
            false,
            &[],
        );
        
        // Reader mode should also display all paragraphs
        let reader_result = merge_paragraphs_for_lang(
            &paragraphs,
            "en",
            true, // reader_mode = true
            false,
            &[],
        );
        
        let expected = "Paragraph 1\n\nParagraph 2\n\nParagraph 3";
        assert_eq!(normal_result, expected);
        assert_eq!(reader_result, expected);
    }

    #[test]
    fn test_reader_mode_settings_chapter_handling() {
        let paragraphs = vec![
            create_test_paragraph("settings1", "settingschapter", "en", "Settings paragraph 1"),
            create_test_paragraph("settings2", "settingschapter", "en", "Settings paragraph 2"),
        ];
        
        // Settings chapter should use normal mode logic even when reader_mode is true
        let result = merge_paragraphs_for_lang(
            &paragraphs,
            "en",
            true, // reader_mode = true
            true, // is_settings_chapter = true
            &[],
        );
        
        let expected = "Settings paragraph 1\n\nSettings paragraph 2";
        assert_eq!(result, expected);
    }

    #[test]
    fn test_reader_mode_empty_paragraphs() {
        let paragraphs = vec![];
        
        let result = merge_paragraphs_for_lang(
            &paragraphs,
            "en",
            true,
            false,
            &[],
        );
        
        assert_eq!(result, "");
    }

    #[test]
    fn test_reader_mode_single_paragraph() {
        let paragraphs = vec![
            create_test_paragraph("single", "chapter1", "en", "Only one paragraph."),
        ];
        
        let result = merge_paragraphs_for_lang(
            &paragraphs,
            "en",
            true,
            false,
            &[],
        );
        
        assert_eq!(result, "Only one paragraph.");
    }

    #[test]
    fn test_reader_mode_language_filtering() {
        let paragraph = Paragraph {
            id: "multi_lang".to_string(),
            chapter_id: "chapter1".to_string(),
            texts: vec![
                Text {
                    lang: "en".to_string(),
                    paragraphs: "English text".to_string(),
                    choices: vec![],
                },
                Text {
                    lang: "zh".to_string(),
                    paragraphs: "中文文本".to_string(),
                    choices: vec![],
                },
            ],
            choices: vec![],
            collection_id: String::new(),
            collection_name: String::new(),
            created: String::new(),
            updated: String::new(),
        };
        
        let paragraphs = vec![paragraph];
        
        // Test English
        let en_result = merge_paragraphs_for_lang(
            &paragraphs,
            "en",
            true,
            false,
            &[],
        );
        assert_eq!(en_result, "English text");
        
        // Test Chinese
        let zh_result = merge_paragraphs_for_lang(
            &paragraphs,
            "zh",
            true,
            false,
            &[],
        );
        assert_eq!(zh_result, "中文文本");
        
        // Test non-existent language
        let missing_result = merge_paragraphs_for_lang(
            &paragraphs,
            "fr",
            true,
            false,
            &[],
        );
        assert_eq!(missing_result, "");
    }

    #[test]
    fn test_random_choice_selection_simulation() {
        // Simulate the auto-expansion logic for testing
        let mut choice_counts = HashMap::new();
        let iterations = 1000;
        
        let targets = vec!["option_a", "option_b", "option_c"];
        
        // Run random selection many times to test distribution
        for _ in 0..iterations {
            let chosen = targets.choose(&mut thread_rng()).unwrap();
            *choice_counts.entry(chosen.to_string()).or_insert(0) += 1;
        }
        
        // Each option should be chosen at least some times (allowing for randomness)
        for target in &targets {
            let count = choice_counts.get(&target.to_string()).unwrap_or(&0);
            assert!(
                *count > 0, 
                "Option {} should be chosen at least once in {} iterations", 
                target, 
                iterations
            );
            
            // Should be roughly distributed (within reasonable bounds)
            let expected = iterations / targets.len();
            let tolerance = expected / 3; // Allow 33% deviation
            assert!(
                *count > expected - tolerance && *count < expected + tolerance,
                "Option {} chosen {} times, expected around {} (tolerance: {})",
                target,
                count,
                expected,
                tolerance
            );
        }
    }

    #[test]
    fn test_complex_choice_structure() {
        // Test the ComplexChoice structure used in reader mode
        let paragraph = create_test_paragraph_with_choices(
            "test_choice",
            "chapter1",
            "en",
            "Choose your path:",
            vec!["Go left", "Go right", "Go straight"],
            vec![
                vec!["left_path"],
                vec!["right_path_a", "right_path_b"], // Multiple targets
                vec![], // No target
            ]
        );
        
        assert_eq!(paragraph.choices.len(), 3);
        assert_eq!(paragraph.choices[0].to, vec!["left_path"]);
        assert_eq!(paragraph.choices[1].to, vec!["right_path_a", "right_path_b"]);
        assert_eq!(paragraph.choices[2].to, Vec::<String>::new());
        
        // Test text choices
        assert_eq!(paragraph.texts[0].choices.len(), 3);
        assert_eq!(paragraph.texts[0].choices[0], "Go left");
        assert_eq!(paragraph.texts[0].choices[1], "Go right");
        assert_eq!(paragraph.texts[0].choices[2], "Go straight");
    }

    #[test]
    fn test_story_path_expansion_logic() {
        // Simulate the story path expansion that happens in reader mode
        let paragraphs = vec![
            create_test_paragraph_with_choices(
                "start",
                "chapter1", 
                "en",
                "Beginning of story",
                vec!["Choice A", "Choice B"],
                vec![vec!["middle_a"], vec!["middle_b"]]
            ),
            create_test_paragraph_with_choices(
                "middle_a",
                "chapter1",
                "en", 
                "Path A continues",
                vec!["Next step"],
                vec![vec!["end"]]
            ),
            create_test_paragraph_with_choices(
                "middle_b",
                "chapter1",
                "en",
                "Path B continues", 
                vec!["Another step"],
                vec![vec!["end"]]
            ),
            create_test_paragraph("end", "chapter1", "en", "Story conclusion"),
        ];
        
        // Test a specific path: start -> middle_a -> end
        let expanded_path = vec![
            paragraphs[0].clone(), // start
            paragraphs[1].clone(), // middle_a
            paragraphs[3].clone(), // end
        ];
        
        let result = merge_paragraphs_for_lang(
            &expanded_path,
            "en",
            true,
            false,
            &[],
        );
        
        let expected = "Beginning of story\n\nPath A continues\n\nStory conclusion";
        assert_eq!(result, expected);
    }

    #[test]
    fn test_reader_mode_with_no_valid_choices() {
        // Test behavior when reaching a paragraph with no valid choices
        let paragraph = create_test_paragraph_with_choices(
            "dead_end",
            "chapter1",
            "en",
            "This is the end of the story",
            vec!["Continue", "Restart"],
            vec![vec![], vec![]] // Both choices have no targets
        );
        
        let paragraphs = vec![paragraph];
        
        let result = merge_paragraphs_for_lang(
            &paragraphs,
            "en",
            true,
            false,
            &[],
        );
        
        assert_eq!(result, "This is the end of the story");
    }
} 