mod common;

use common::*;
#[cfg(target_arch = "wasm32")]
use wasm_bindgen_test::wasm_bindgen_test;
#[cfg(target_arch = "wasm32")]
wasm_bindgen_test::wasm_bindgen_test_configure!(run_in_browser);

#[cfg(test)]
mod reader_mode_tests {
    use super::*;
    use ifecaro::pages::story::{merge_paragraphs_for_lang, Paragraph, Text, ComplexChoice};
    use rand::prelude::*;
    use std::collections::HashMap;
    use tracing;
    use tracing_subscriber;

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

    #[cfg_attr(target_arch = "wasm32", wasm_bindgen_test)]
    #[cfg_attr(not(target_arch = "wasm32"), test)]
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

    #[cfg_attr(target_arch = "wasm32", wasm_bindgen_test)]
    #[cfg_attr(not(target_arch = "wasm32"), test)]
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

    #[cfg_attr(target_arch = "wasm32", wasm_bindgen_test)]
    #[cfg_attr(not(target_arch = "wasm32"), test)]
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

    #[cfg_attr(target_arch = "wasm32", wasm_bindgen_test)]
    #[cfg_attr(not(target_arch = "wasm32"), test)]
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

    #[cfg_attr(target_arch = "wasm32", wasm_bindgen_test)]
    #[cfg_attr(not(target_arch = "wasm32"), test)]
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

    #[cfg_attr(target_arch = "wasm32", wasm_bindgen_test)]
    #[cfg_attr(not(target_arch = "wasm32"), test)]
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

    #[cfg_attr(target_arch = "wasm32", wasm_bindgen_test)]
    #[cfg_attr(not(target_arch = "wasm32"), test)]
    fn test_random_choice_selection_simulation() {
        use tracing::info;
        #[cfg(not(target_arch = "wasm32"))]
        let _ = tracing_subscriber::fmt::try_init();
        use std::collections::HashSet;
        use rand::seq::SliceRandom;
        // 測試說明：
        // 準備一個有多分支的段落結構，start 有兩個選項（Go A, Go B），
        // 每個選項都連到兩個不同的段落（a1, a2, b1, b2），
        // 每次測試隨機選擇一個分支，檢查合併結果是否有多種不同路徑。
        let start = create_test_paragraph_with_choices(
            "start", "chapter1", "en", "Start", vec!["Go A", "Go B"], vec![vec!["a1", "a2"], vec!["b1", "b2"]]
        );
        let a1 = create_test_paragraph_with_choices("a1", "chapter1", "en", "A1", vec![], vec![]);
        let a2 = create_test_paragraph_with_choices("a2", "chapter1", "en", "A2", vec![], vec![]);
        let b1 = create_test_paragraph_with_choices("b1", "chapter1", "en", "B1", vec![], vec![]);
        let b2 = create_test_paragraph_with_choices("b2", "chapter1", "en", "B2", vec![], vec![]);
        let paragraphs = vec![start.clone(), a1.clone(), a2.clone(), b1.clone(), b2.clone()];
        info!(?paragraphs, "Test setup: Paragraphs used for random path test");
        // 模擬多次隨機展開
        let mut seen_paths = HashSet::new();
        let iterations = 100;
        for i in 0..iterations {
            // 每次都隨機選擇一個分支
            let mut rng = rand::thread_rng();
            // 隨機選擇一個 target id
            let all_targets = ["a1", "a2", "b1", "b2"];
            let first_choice = all_targets.choose(&mut rng).unwrap();
            let mut path = vec![start.clone()];
            let next = paragraphs.iter().find(|p| &p.id == *first_choice).unwrap().clone();
            path.push(next);
            // 呼叫主程式合併
            let result = merge_paragraphs_for_lang(&path, "en", true, false, &[]);
            info!(
                iteration = i,
                test_method = "Randomly pick one of [a1, a2, b1, b2] as next paragraph after start",
                picked = *first_choice,
                path_ids = ?path.iter().map(|p| p.id.clone()).collect::<Vec<_>>(),
                merged_result = %result,
                "Random path test: input path and merge result"
            );
            seen_paths.insert(result);
        }
        info!(?seen_paths, "All unique merge results seen in 100 iterations");
        // 至少要有多種不同的展開結果
        assert!(seen_paths.len() > 1, "Should see multiple random paths, got only one");
    }

    #[cfg_attr(target_arch = "wasm32", wasm_bindgen_test)]
    #[cfg_attr(not(target_arch = "wasm32"), test)]
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

    #[cfg_attr(target_arch = "wasm32", wasm_bindgen_test)]
    #[cfg_attr(not(target_arch = "wasm32"), test)]
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

    #[cfg_attr(target_arch = "wasm32", wasm_bindgen_test)]
    #[cfg_attr(not(target_arch = "wasm32"), test)]
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

    #[cfg_attr(target_arch = "wasm32", wasm_bindgen_test)]
    #[cfg_attr(not(target_arch = "wasm32"), test)]
    fn test_auto_expansion_random_path_wasm() {
        use tracing::info;
        #[cfg(not(target_arch = "wasm32"))]
        let _ = tracing_subscriber::fmt::try_init();
        use std::collections::HashSet;
        use ifecaro::pages::story::expand_story_path_with_random;
        // 測試說明：
        // 準備一個有多分支的段落結構，start 有兩個選項（Go A, Go B），
        // 每個選項都連到兩個不同的段落（a1, a2, b1, b2），
        // 每次呼叫 expand_story_path_with_random 都會隨機選一個分支。
        let start = create_test_paragraph_with_choices(
            "start", "chapter1", "en", "Start", vec!["Go A", "Go B"], vec![vec!["a1", "a2"], vec!["b1", "b2"]]
        );
        let a1 = create_test_paragraph_with_choices("a1", "chapter1", "en", "A1", vec![], vec![]);
        let a2 = create_test_paragraph_with_choices("a2", "chapter1", "en", "A2", vec![], vec![]);
        let b1 = create_test_paragraph_with_choices("b1", "chapter1", "en", "B1", vec![], vec![]);
        let b2 = create_test_paragraph_with_choices("b2", "chapter1", "en", "B2", vec![], vec![]);
        let paragraphs = vec![start.clone(), a1.clone(), a2.clone(), b1.clone(), b2.clone()];
        info!(?paragraphs, "Test setup: Paragraphs used for auto-expansion random path test");
        let mut seen_paths = HashSet::new();
        let iterations = 100;
        for i in 0..iterations {
            let path = expand_story_path_with_random(
                &paragraphs,
                "start",
                "en",
                |_pid, _idx, choices| {
                    use rand::seq::SliceRandom;
                    let mut rng = rand::thread_rng();
                    choices.choose(&mut rng).unwrap().clone()
                }
            );
            let path_ids: Vec<_> = path.iter().map(|p| p.id.clone()).collect();
            info!(iteration = i, ?path_ids, "Auto-expansion random path");
            seen_paths.insert(path_ids);
        }
        info!(?seen_paths, "All unique auto-expanded paths seen in 100 iterations");
        assert!(seen_paths.len() > 1, "Should see multiple random auto-expanded paths, got only one");
    }
} 