use std::time::{Duration, Instant};

// Import the Dashboard component and related structs
use ifecaro::contexts::chapter_context::{ChapterState, Chapter, ChapterTitle};
use ifecaro::contexts::paragraph_context::{
    ParagraphState, Paragraph, Text, ParagraphChoice
};

// Performance benchmarking tests
#[cfg(test)]
mod benchmark_tests {
    use super::*;
    
    // Create large-scale test data for performance testing
    pub fn create_large_scale_test_data() -> (String, ChapterState, ParagraphState) {
        let current_language = "zh-TW".to_string();
        
        // Create 50 chapters
        let chapters: Vec<Chapter> = (1..=50).map(|i| {
            Chapter {
                id: format!("chapter{}", i),
                titles: vec![
                    ChapterTitle {
                        lang: "zh-TW".to_string(),
                        title: format!("第{}章", i),
                    },
                    ChapterTitle {
                        lang: "en-US".to_string(),
                        title: format!("Chapter {}", i),
                    },
                    ChapterTitle {
                        lang: "ja-JP".to_string(),
                        title: format!("第{}章", i),
                    },
                ],
                order: i,
            }
        }).collect();
        
        let chapter_state = ChapterState {
            chapters,
            loaded: true,
        };
        
        // Create 2000 paragraphs (40 per chapter)
        let paragraphs: Vec<Paragraph> = (1..=2000).map(|i| {
            let chapter_id = format!("chapter{}", (i - 1) / 40 + 1);
            Paragraph {
                id: format!("para{}", i),
                chapter_id,
                texts: vec![
                    Text {
                        lang: "zh-TW".to_string(),
                        paragraphs: format!("這是第{}個段落的內容。這裡包含了詳細的故事情節和豐富的描述。", i),
                        choices: if i % 3 == 0 {
                            vec![]
                        } else {
                            vec![
                                format!("選項一 - 段落{}", i),
                                format!("選項二 - 段落{}", i),
                                format!("選項三 - 段落{}", i),
                            ]
                        },
                    },
                    Text {
                        lang: "en-US".to_string(),
                        paragraphs: format!("This is the content of paragraph {}. It contains detailed storyline and rich descriptions.", i),
                        choices: if i % 3 == 0 {
                            vec![]
                        } else {
                            vec![
                                format!("Option One - Paragraph {}", i),
                                format!("Option Two - Paragraph {}", i),
                                format!("Option Three - Paragraph {}", i),
                            ]
                        },
                    },
                    Text {
                        lang: "ja-JP".to_string(),
                        paragraphs: format!("これは段落{}の内容です。詳細なストーリーラインと豊富な説明が含まれています。", i),
                        choices: if i % 3 == 0 {
                            vec![]
                        } else {
                            vec![
                                format!("選択肢一 - 段落{}", i),
                                format!("選択肢二 - 段落{}", i),
                                format!("選択肢三 - 段落{}", i),
                            ]
                        },
                    },
                ],
                choices: if i % 3 == 0 {
                    vec![]
                } else {
                    vec![
                        ParagraphChoice::Simple(vec![format!("para{}", i + 1)]),
                        ParagraphChoice::Complex {
                            to: vec![format!("para{}", i + 2), format!("para{}", i + 3)],
                            type_: "conditional".to_string(),
                            key: Some(format!("condition_{}", i)),
                            value: Some(serde_json::json!({"value": i, "type": "number"})),
                            same_page: Some(i % 2 == 0),
                            time_limit: if i % 5 == 0 { Some(60) } else { None },
                            timeout_to: None,
                        },
                        ParagraphChoice::Simple(vec![format!("para{}", i + 4)]),
                    ]
                },
            }
        }).collect();
        
        let paragraph_state = ParagraphState {
            paragraphs,
            loaded: true,
        };
        
        (current_language, chapter_state, paragraph_state)
    }
    
    #[test]
    fn benchmark_large_dataset_creation() {
        let start = Instant::now();
        let (_current_language, _chapter_state, _paragraph_state) = create_large_scale_test_data();
        let duration = start.elapsed();
        
        println!("Large dataset creation took: {:?}", duration);
        
        // Should complete within 1 second
        assert!(duration < Duration::from_secs(1), 
            "Large dataset creation took too long: {:?}", duration);
    }
    
    #[test]
    fn benchmark_chapter_filtering() {
        let (_current_language, _chapter_state, paragraph_state) = create_large_scale_test_data();
        
        let start = Instant::now();
        
        // Test filtering multiple chapters
        for i in 1..=50 {
            let chapter_id = format!("chapter{}", i);
            let paragraphs = paragraph_state.get_by_chapter(&chapter_id);
            assert_eq!(paragraphs.len(), 40);
        }
        
        let duration = start.elapsed();
        
        println!("Chapter filtering (50 chapters) took: {:?}", duration);
        
        // Should complete within 100ms
        assert!(duration < Duration::from_millis(100),
            "Chapter filtering took too long: {:?}", duration);
    }
    
    #[test]
    fn benchmark_paragraph_lookup() {
        let (_current_language, _chapter_state, paragraph_state) = create_large_scale_test_data();
        
        let start = Instant::now();
        
        // Test looking up 1000 random paragraphs
        for i in 1..=1000 {
            let paragraph_id = format!("para{}", i);
            let paragraph = paragraph_state.get_by_id(&paragraph_id);
            assert!(paragraph.is_some());
        }
        
        let duration = start.elapsed();
        
        println!("Paragraph lookup (1000 lookups) took: {:?}", duration);
        
        // Should complete within 100ms (adjusted for container environment)
        assert!(duration < Duration::from_millis(100),
            "Paragraph lookup took too long: {:?}", duration);
    }
    
    #[test]
    fn benchmark_language_content_retrieval() {
        let (_current_language, _chapter_state, paragraph_state) = create_large_scale_test_data();
        
        let languages = vec!["zh-TW", "en-US", "ja-JP"];
        
        let start = Instant::now();
        
        // Test retrieving content in different languages for 500 paragraphs
        for i in 1..=500 {
            let paragraph_id = format!("para{}", i);
            if let Some(paragraph) = paragraph_state.get_by_id(&paragraph_id) {
                for lang in &languages {
                    let text = paragraph.texts.iter().find(|t| t.lang == *lang);
                    assert!(text.is_some());
                }
            }
        }
        
        let duration = start.elapsed();
        
        println!("Language content retrieval (500 paragraphs × 3 languages) took: {:?}", duration);
        
        // Should complete within 100ms
        assert!(duration < Duration::from_millis(100),
            "Language content retrieval took too long: {:?}", duration);
    }
    
    #[test]
    fn benchmark_choice_processing() {
        let (_current_language, _chapter_state, paragraph_state) = create_large_scale_test_data();
        
        let start = Instant::now();
        
        let mut total_choices = 0;
        
        // Process choices for all paragraphs
        for paragraph in &paragraph_state.paragraphs {
            for text in &paragraph.texts {
                total_choices += text.choices.len();
            }
            
            for choice in &paragraph.choices {
                match choice {
                    ParagraphChoice::Simple(targets) => {
                        assert!(!targets.is_empty());
                    },
                    ParagraphChoice::Complex { to, type_, .. } => {
                        assert!(!to.is_empty());
                        assert!(!type_.is_empty());
                        // Additional validation can be added here
                    },
                    _ => {}
                }
            }
        }
        
        let duration = start.elapsed();
        
        println!("Choice processing ({} choices) took: {:?}", total_choices, duration);
        
        // Should complete within 200ms
        assert!(duration < Duration::from_millis(200),
            "Choice processing took too long: {:?}", duration);
    }
    
    #[test]
    fn benchmark_concurrent_operations() {
        let (_current_language, _chapter_state, paragraph_state) = create_large_scale_test_data();
        
        let start = Instant::now();
        
        // Simulate concurrent operations
        let languages = vec!["zh-TW", "en-US", "ja-JP"];
        let mut operations_count = 0;
        
        for i in 1..=100 {
            // Language switching simulation
            let _current_lang = &languages[i % languages.len()];
            operations_count += 1;
            
            // Chapter selection simulation
            let chapter_id = format!("chapter{}", (i % 50) + 1);
            let _paragraphs = paragraph_state.get_by_chapter(&chapter_id);
            operations_count += 1;
            
            // Paragraph lookup simulation
            let paragraph_id = format!("para{}", i);
            let _paragraph = paragraph_state.get_by_id(&paragraph_id);
            operations_count += 1;
            
            // Content validation simulation
            if let Some(paragraph) = paragraph_state.get_by_id(&paragraph_id) {
                for text in &paragraph.texts {
                    let _is_valid = !text.paragraphs.trim().is_empty();
                    operations_count += 1;
                }
            }
        }
        
        let duration = start.elapsed();
        
        println!("Concurrent operations ({} operations) took: {:?}", operations_count, duration);
        
        // Should complete within 200ms (adjusted for container environment)
        assert!(duration < Duration::from_millis(200),
            "Concurrent operations took too long: {:?}", duration);
    }
    
    #[test]
    fn benchmark_memory_usage() {
        // This test checks if large datasets can be created without excessive memory usage
        let start = Instant::now();
        
        let mut datasets = Vec::new();
        
        // Create 10 large datasets
        for _i in 0..10 {
            let dataset = create_large_scale_test_data();
            datasets.push(dataset);
        }
        
        let duration = start.elapsed();
        
        println!("Creating 10 large datasets took: {:?}", duration);
        
        // Test accessing data from all datasets
        let access_start = Instant::now();
        
        for (i, (_current_lang, _chapter_state, paragraph_state)) in datasets.iter().enumerate() {
            let paragraph_id = format!("para{}", (i * 100) + 1);
            let _paragraph = paragraph_state.get_by_id(&paragraph_id);
        }
        
        let access_duration = access_start.elapsed();
        
        println!("Accessing data from 10 datasets took: {:?}", access_duration);
        
        // Should complete within reasonable time
        assert!(duration < Duration::from_secs(10),
            "Creating multiple datasets took too long: {:?}", duration);
        assert!(access_duration < Duration::from_millis(10),
            "Accessing multiple datasets took too long: {:?}", access_duration);
    }
    
    #[test]
    fn benchmark_form_validation_performance() {
        let (_current_language, _chapter_state, _paragraph_state) = create_large_scale_test_data();
        
        let start = Instant::now();
        
        // Create large number of form validation scenarios
        for i in 1..=1000 {
            let content = format!("Test paragraph content {}", i);
            let chapter = format!("chapter{}", (i % 50) + 1);
            let choices = vec![
                (format!("Choice 1 for {}", i), vec![format!("para{}", i + 1)], "goto".to_string(), None::<String>, None::<serde_json::Value>, chapter.clone(), false, None::<u32>),
                (format!("Choice 2 for {}", i), vec![format!("para{}", i + 2)], "conditional".to_string(), Some("key".to_string()), Some(serde_json::json!(i)), chapter.clone(), i % 2 == 0, Some(60)),
            ];
            
            // Validate form
            let main_fields_valid = !content.trim().is_empty() && !chapter.is_empty();
            let has_any_choices = !choices.is_empty();
            let choices_valid = choices.iter().all(|(_choice_text, to, _type, _key, _value, _target_chapter, _same_page, _time_limit)| {
                !to.is_empty()
            });
            
            let _is_form_valid = main_fields_valid && (!has_any_choices || choices_valid);
        }
        
        let duration = start.elapsed();
        
        println!("Form validation (1000 forms) took: {:?}", duration);
        
        // Should complete within 100ms (adjusted for container environment)
        assert!(duration < Duration::from_millis(100),
            "Form validation took too long: {:?}", duration);
    }
}

// Stress tests
#[cfg(test)]
mod stress_tests {
    use super::*;
    
    #[test]
    fn stress_test_massive_dataset() {
        // Create an extremely large dataset for stress testing
        let start = Instant::now();
        
        let mut paragraph_state = ParagraphState {
            paragraphs: Vec::new(),
            loaded: true,
        };
        
        // Create 10,000 paragraphs
        for i in 1..=10000 {
            paragraph_state.paragraphs.push(Paragraph {
                id: format!("stress_para{}", i),
                chapter_id: format!("stress_chapter{}", (i - 1) / 100 + 1),
                texts: vec![
                    Text {
                        lang: "zh-TW".to_string(),
                        paragraphs: format!("壓力測試段落{}的內容。這是一個包含大量文字的段落，用於測試系統在處理大型數據集時的表現。", i),
                        choices: vec![
                            format!("壓力測試選項1 - {}", i),
                            format!("壓力測試選項2 - {}", i),
                            format!("壓力測試選項3 - {}", i),
                        ],
                    },
                ],
                choices: vec![
                    ParagraphChoice::Simple(vec![format!("stress_para{}", i + 1)]),
                    ParagraphChoice::Complex {
                        to: vec![format!("stress_para{}", i + 2)],
                        type_: "stress_test".to_string(),
                        key: Some(format!("stress_key_{}", i)),
                        value: Some(serde_json::json!({"stress_value": i, "type": "stress"})),
                        same_page: Some(false),
                        time_limit: Some(30),
                        timeout_to: None,
                    },
                ],
            });
        }
        
        let creation_duration = start.elapsed();
        
        // Test operations on massive dataset
        let operation_start = Instant::now();
        
        // Test chapter filtering
        for i in 1..=100 {
            let chapter_id = format!("stress_chapter{}", i);
            let paragraphs = paragraph_state.get_by_chapter(&chapter_id);
            assert_eq!(paragraphs.len(), 100);
        }
        
        // Test paragraph lookup
        for i in 1..=1000 {
            let paragraph_id = format!("stress_para{}", i);
            let paragraph = paragraph_state.get_by_id(&paragraph_id);
            assert!(paragraph.is_some());
        }
        
        let operation_duration = operation_start.elapsed();
        
        println!("Stress test - Dataset creation: {:?}", creation_duration);
        println!("Stress test - Operations: {:?}", operation_duration);
        
        // Should complete within reasonable time even with massive dataset
        assert!(creation_duration < Duration::from_secs(5),
            "Massive dataset creation took too long: {:?}", creation_duration);
        assert!(operation_duration < Duration::from_secs(1),
            "Operations on massive dataset took too long: {:?}", operation_duration);
    }
    
    #[test]
    fn stress_test_rapid_language_switching() {
        let (_current_language, _chapter_state, paragraph_state) = benchmark_tests::create_large_scale_test_data();
        
        let languages = vec!["zh-TW", "en-US", "ja-JP", "ko-KR", "fr-FR", "de-DE", "es-ES"];
        let start = Instant::now();
        
        // Rapidly switch languages and access content
        for i in 0..10000 {
            let current_lang = &languages[i % languages.len()];
            let paragraph_id = format!("para{}", (i % 2000) + 1);
            
            if let Some(paragraph) = paragraph_state.get_by_id(&paragraph_id) {
                let _text = paragraph.texts.iter()
                    .find(|t| t.lang == *current_lang)
                    .or_else(|| paragraph.texts.first());
            }
        }
        
        let duration = start.elapsed();
        
        println!("Rapid language switching (10000 switches) took: {:?}", duration);
        
        // Should handle rapid switching efficiently
        assert!(duration < Duration::from_millis(500),
            "Rapid language switching took too long: {:?}", duration);
    }
    
    #[test]
    fn stress_test_concurrent_form_operations() {
        // Simulate concurrent form operations
        let start = Instant::now();
        
        let mut operations = 0;
        
        // Simulate 10000 concurrent form operations
        for i in 0..10000 {
            // Content editing
            let content = format!("Dynamic content {}", i);
            let _is_content_valid = !content.trim().is_empty();
            operations += 1;
            
            // Choice management
            let mut choices = Vec::new();
            for j in 0..5 {
                choices.push((
                    format!("Choice {} for operation {}", j, i),
                    vec![format!("target{}", i + j)],
                    "goto".to_string(),
                    None::<String>,
                    None::<serde_json::Value>,
                    format!("chapter{}", i % 10),
                    false,
                    None::<u32>,
                ));
            }
            operations += 5;
            
            // Form validation
            let _is_valid = choices.iter().all(|(_text, targets, _type, _key, _value, _chapter, _same_page, _time_limit)| {
                !targets.is_empty()
            });
            operations += 1;
        }
        
        let duration = start.elapsed();
        
        println!("Concurrent form operations ({} operations) took: {:?}", operations, duration);
        
        // Should handle high concurrent load
        assert!(duration < Duration::from_secs(2),
            "Concurrent form operations took too long: {:?}", duration);
    }
} 