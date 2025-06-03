mod common;

#[cfg(test)]
mod reader_mode_integration_tests {
    use ifecaro::pages::story::{merge_paragraphs_for_lang, Paragraph, Text, ComplexChoice};
    use std::collections::HashMap;

    fn create_story_network() -> Vec<Paragraph> {
        // Create a network of connected paragraphs for testing auto-expansion
        vec![
            // Start paragraph with 2 choices
            create_paragraph_with_multiple_choices(
                "start",
                "chapter1",
                "en", 
                "You wake up in a mysterious forest.",
                vec!["Follow the path", "Enter the cave"],
                vec![vec!["path"], vec!["cave"]]
            ),
            
            // Path route with 1 choice leading to ending
            create_paragraph_with_multiple_choices(
                "path",
                "chapter1",
                "en",
                "The path leads to a beautiful meadow.",
                vec!["Rest in the meadow"],
                vec![vec!["meadow_end"]]
            ),
            
            // Cave route with 2 choices, one leading to treasure, one to danger
            create_paragraph_with_multiple_choices(
                "cave",
                "chapter1", 
                "en",
                "The cave is dark and mysterious.",
                vec!["Go deeper", "Turn back"],
                vec![vec!["treasure", "danger"], vec!["start"]] // Multiple targets for first choice
            ),
            
            // Treasure ending
            create_paragraph_with_no_choices(
                "treasure",
                "chapter1",
                "en",
                "You found a chest full of gold!"
            ),
            
            // Danger ending
            create_paragraph_with_no_choices(
                "danger", 
                "chapter1",
                "en",
                "You encountered a dangerous monster!"
            ),
            
            // Meadow ending
            create_paragraph_with_no_choices(
                "meadow_end",
                "chapter1", 
                "en",
                "You rest peacefully under the stars."
            ),
        ]
    }

    fn create_paragraph_with_multiple_choices(
        id: &str,
        chapter_id: &str, 
        lang: &str,
        text: &str,
        choice_texts: Vec<&str>,
        choice_targets: Vec<Vec<&str>>
    ) -> Paragraph {
        let text_obj = Text {
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
            texts: vec![text_obj],
            choices: complex_choices,
            collection_id: String::new(),
            collection_name: String::new(),
            created: String::new(),
            updated: String::new(),
        }
    }

    fn create_paragraph_with_no_choices(id: &str, chapter_id: &str, lang: &str, text: &str) -> Paragraph {
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

    fn simulate_auto_expansion(
        paragraphs: &[Paragraph],
        start_id: &str,
        max_depth: usize
    ) -> Vec<Paragraph> {
        // Simulate the auto-expansion logic from reader mode
        let mut visited = vec![start_id.to_string()];
        let mut path = vec![];
        let mut current_id = start_id.to_string();
        let mut depth = 0;
        
        while depth < max_depth {
            let current = match paragraphs.iter().find(|p| p.id == current_id) {
                Some(p) => p,
                None => break,
            };
            
            path.push(current.clone());
            
            // Find text for English
            let text = match current.texts.iter().find(|t| t.lang == "en") {
                Some(t) => t,
                None => break,
            };
            
            if text.choices.is_empty() {
                break;
            }
            
            // Get valid choice targets
            let mut valid_targets = vec![];
            for choice in &current.choices {
                if !choice.to.is_empty() {
                    // For multiple targets, just pick the first one for testing
                    valid_targets.extend(choice.to.iter().cloned());
                }
            }
            
            if valid_targets.is_empty() {
                break;
            }
            
            // Pick first valid target for deterministic testing
            let next_id = valid_targets[0].clone();
            
            if visited.contains(&next_id) {
                break; // Avoid loops
            }
            
            visited.push(next_id.clone());
            current_id = next_id;
            depth += 1;
        }
        
        path
    }

    #[test]
    fn test_reader_mode_story_network_expansion() {
        let paragraphs = create_story_network();
        
        // Test expansion from start following path route
        let expanded_path = simulate_auto_expansion(&paragraphs, "start", 10);
        
        // Should expand at least 2 paragraphs (start + one choice)
        assert!(expanded_path.len() >= 2, "Should expand at least 2 paragraphs, got {}", expanded_path.len());
        
        // First paragraph should be start
        assert_eq!(expanded_path[0].id, "start");
        
        // Merge the expanded path
        let result = merge_paragraphs_for_lang(
            &expanded_path,
            "en",
            true, // reader_mode
            false,
            &[],
        );
        
        // Should contain the start text
        assert!(result.contains("You wake up in a mysterious forest."));
        
        // Should contain at least one more paragraph
        assert!(result.contains("\n\n"), "Should contain multiple paragraphs separated by newlines");
    }

    #[test]
    fn test_reader_mode_multiple_ending_paths() {
        let paragraphs = create_story_network();
        
        // Test different possible paths
        let possible_endings = vec!["treasure", "danger", "meadow_end"];
        
        for ending in possible_endings {
            if let Some(ending_paragraph) = paragraphs.iter().find(|p| p.id == ending) {
                // Create a path that ends with this ending
                let path = vec![
                    paragraphs.iter().find(|p| p.id == "start").unwrap().clone(),
                    ending_paragraph.clone(),
                ];
                
                let result = merge_paragraphs_for_lang(
                    &path,
                    "en", 
                    true,
                    false,
                    &[],
                );
                
                // Should contain both start and ending texts
                assert!(result.contains("You wake up in a mysterious forest."));
                assert!(result.contains(&ending_paragraph.texts[0].paragraphs));
            }
        }
    }

    #[test]
    fn test_reader_mode_handles_empty_choice_targets() {
        // Test paragraph with choices that have no targets
        let paragraph = create_paragraph_with_multiple_choices(
            "empty_targets",
            "chapter1",
            "en",
            "This choice leads nowhere.",
            vec!["Try anyway", "Give up"],
            vec![vec![], vec![]] // Both choices have no targets
        );
        
        let paragraphs = vec![paragraph];
        let expanded = simulate_auto_expansion(&paragraphs, "empty_targets", 5);
        
        // Should only contain the starting paragraph
        assert_eq!(expanded.len(), 1);
        assert_eq!(expanded[0].id, "empty_targets");
        
        let result = merge_paragraphs_for_lang(
            &expanded,
            "en",
            true,
            false, 
            &[],
        );
        
        assert_eq!(result, "This choice leads nowhere.");
    }

    #[test]
    fn test_reader_mode_loop_prevention() {
        // Create paragraphs that could create a loop
        let loop_paragraphs = vec![
            create_paragraph_with_multiple_choices(
                "loop_a",
                "chapter1",
                "en",
                "You are at point A.",
                vec!["Go to B"],
                vec![vec!["loop_b"]]
            ),
            create_paragraph_with_multiple_choices(
                "loop_b", 
                "chapter1",
                "en",
                "You are at point B.",
                vec!["Go back to A"],
                vec![vec!["loop_a"]] // This would create a loop
            ),
        ];
        
        let expanded = simulate_auto_expansion(&loop_paragraphs, "loop_a", 10);
        
        // Should stop before creating infinite loop
        assert!(expanded.len() <= 2, "Should prevent infinite loops, got {} paragraphs", expanded.len());
        
        // Should contain both paragraphs at most once each
        let mut id_counts = HashMap::new();
        for p in &expanded {
            *id_counts.entry(&p.id).or_insert(0) += 1;
        }
        
        for (id, count) in id_counts {
            assert_eq!(count, 1, "Paragraph {} should appear only once, appeared {} times", id, count);
        }
    }

    #[test]
    fn test_reader_mode_language_consistency() {
        // Test that reader mode works with different languages
        let multi_lang_paragraph = Paragraph {
            id: "multi_lang".to_string(),
            chapter_id: "chapter1".to_string(),
            texts: vec![
                Text {
                    lang: "en".to_string(),
                    paragraphs: "English story text.".to_string(),
                    choices: vec!["English choice".to_string()],
                },
                Text {
                    lang: "zh".to_string(),
                    paragraphs: "中文故事內容。".to_string(),
                    choices: vec!["中文選擇".to_string()],
                },
            ],
            choices: vec![ComplexChoice {
                to: vec!["next".to_string()],
                type_: "goto".to_string(),
                key: None,
                value: None,
                same_page: None,
                time_limit: None,
            }],
            collection_id: String::new(),
            collection_name: String::new(),
            created: String::new(),
            updated: String::new(),
        };
        
        let next_paragraph = create_paragraph_with_no_choices(
            "next",
            "chapter1", 
            "en",
            "Next paragraph in English."
        );
        
        let paragraphs = vec![multi_lang_paragraph, next_paragraph];
        
        // Test English expansion
        let en_result = merge_paragraphs_for_lang(
            &paragraphs,
            "en",
            true,
            false,
            &[],
        );
        
        assert!(en_result.contains("English story text."));
        assert!(en_result.contains("Next paragraph in English."));
        
        // Test Chinese expansion (should not include next paragraph since it has no Chinese text)
        let zh_result = merge_paragraphs_for_lang(
            &paragraphs,
            "zh", 
            true,
            false,
            &[],
        );
        
        assert!(zh_result.contains("中文故事內容。"));
        assert!(!zh_result.contains("Next paragraph in English."));
    }

    #[test]
    fn test_reader_mode_performance_with_large_path() {
        // Test reader mode with a longer story path
        let mut paragraphs = vec![];
        
        // Create a linear story with 50 paragraphs
        for i in 0..50 {
            let id = format!("paragraph_{}", i);
            let next_id = if i < 49 { format!("paragraph_{}", i + 1) } else { String::new() };
            let text = format!("This is paragraph number {}.", i);
            
            let paragraph = if !next_id.is_empty() {
                create_paragraph_with_multiple_choices(
                    &id,
                    "chapter1", 
                    "en",
                    &text,
                    vec!["Continue"],
                    vec![vec![&next_id]]
                )
            } else {
                create_paragraph_with_no_choices(&id, "chapter1", "en", &text)
            };
            
            paragraphs.push(paragraph);
        }
        
        // Test that merging works efficiently even with many paragraphs
        let result = merge_paragraphs_for_lang(
            &paragraphs,
            "en",
            true,
            false,
            &[],
        );
        
        // Should contain all paragraphs
        assert!(result.contains("This is paragraph number 0."));
        assert!(result.contains("This is paragraph number 49."));
        
        // Count paragraph separators
        let separator_count = result.matches("\n\n").count();
        assert_eq!(separator_count, 49, "Should have 49 separators between 50 paragraphs");
    }
} 