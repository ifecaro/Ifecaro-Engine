#[cfg(not(target_arch = "wasm32"))]
mod tests {
    use super::super::api::*;
    use crate::contexts::paragraph_context::{Paragraph, Text, ParagraphChoice};

    /// Helper function: Create test paragraph
    fn create_test_paragraph(id: &str, chapter_id: &str) -> Paragraph {
        Paragraph {
            id: id.to_string(),
            chapter_id: chapter_id.to_string(),
            texts: vec![
                Text {
                    lang: "zh-TW".to_string(),
                    paragraphs: format!("This is test paragraph {}", id),
                    choices: vec![
                        "Continue Adventure".to_string(),
                        "Return to Village".to_string(),
                    ],
                },
                Text {
                    lang: "en".to_string(),
                    paragraphs: format!("This is test paragraph {}", id),
                    choices: vec![
                        "Continue Adventure".to_string(),
                        "Return to Village".to_string(),
                    ],
                },
            ],
            choices: vec![
                ParagraphChoice::Simple(vec!["next_scene".to_string()]),
                ParagraphChoice::Complex {
                    to: vec!["village".to_string()],
                    type_: "goto".to_string(),
                    key: None,
                    value: None,
                    same_page: None,
                    time_limit: Some(30),
                    timeout_to: None,
                },
            ],
        }
    }

    /// Helper function: Create test chapter
    fn create_test_chapter(id: &str, title: &str, order: i32) -> Chapter {
        Chapter {
            id: id.to_string(),
            title: title.to_string(),
            order,
        }
    }

    #[tokio::test]
    async fn test_get_paragraphs_success() {
        // Prepare test data
        let test_paragraphs = vec![
            create_test_paragraph("p1", "c1"),
            create_test_paragraph("p2", "c2"),
        ];

        // Create mock client
        let client = MockApiClient::new()
            .with_paragraphs(test_paragraphs.clone());

        // Execute test
        let result = client.get_paragraphs().await;

        // Verify result
        assert!(result.is_ok());
        let data = result.unwrap();
        assert_eq!(data.items.len(), 2);
    }

    #[tokio::test]
    async fn test_get_paragraphs_failure() {
        // Create mock client that will fail
        let client = MockApiClient::new().with_failure();

        // Execute test
        let result = client.get_paragraphs().await;

        // Verify error
        assert!(result.is_err());
        match result.unwrap_err() {
            ApiError::NetworkError(_) => {},
            _ => panic!("Expected NetworkError"),
        }
    }

    #[tokio::test]
    async fn test_get_chapters_success() {
        // Prepare test data
        let test_chapters = vec![
            create_test_chapter("c1", "Chapter 1", 1),
            create_test_chapter("c2", "Chapter 2", 2),
        ];

        // Create mock client
        let client = MockApiClient::new()
            .with_chapters(test_chapters.clone());

        // Execute test
        let result = client.get_chapters().await;

        // Verify result
        assert!(result.is_ok());
        let data = result.unwrap();
        assert_eq!(data.items.len(), 2);
    }

    #[tokio::test]
    async fn test_get_paragraph_by_id_found() {
        // Prepare test data
        let test_paragraphs = vec![
            create_test_paragraph("p1", "c1"),
            create_test_paragraph("p2", "c2"),
        ];

        // Test finding paragraph
        let client = MockApiClient::new()
            .with_paragraphs(test_paragraphs);

        let result = client.get_paragraph_by_id("p1").await;
        assert!(result.is_ok());
        assert_eq!(result.unwrap().id, "p1");
    }

    #[tokio::test]
    async fn test_get_paragraph_by_id_not_found() {
        // Prepare test data
        let test_paragraphs = vec![
            create_test_paragraph("p1", "c1"),
        ];

        // Test paragraph not found
        let client = MockApiClient::new()
            .with_paragraphs(test_paragraphs);

        let result = client.get_paragraph_by_id("p999").await;
        assert!(result.is_err());
        // Correct error type
        match result.unwrap_err() {
            ApiError::NotFound => {},
            _ => panic!("Expected NotFound error"),
        }
    }

    #[tokio::test]
    async fn test_complex_choice_serialization() {
        use serde_json::json;
        
        let complex_choice = ParagraphChoice::Complex {
            to: vec!["target1".to_string(), "target2".to_string()],
            type_: "conditional".to_string(),
            key: Some("player_level".to_string()),
            value: Some(json!({"min": 5, "max": 10})),
            same_page: Some(true),
            time_limit: Some(30),
            timeout_to: None,
        };

        // Test complex option data structure
        let json_str = serde_json::to_string(&complex_choice).unwrap();
        let deserialized: ParagraphChoice = serde_json::from_str(&json_str).unwrap();
        
        match deserialized {
            ParagraphChoice::Complex { to, type_, key, value, same_page, time_limit, timeout_to } => {
                assert_eq!(to, vec!["target1", "target2"]);
                assert_eq!(type_, "conditional");
                assert_eq!(key, Some("player_level".to_string()));
                assert!(value.is_some());
                assert_eq!(same_page, Some(true));
                assert_eq!(time_limit, Some(30));
                assert_eq!(timeout_to, None);
            },
            _ => panic!("Expected Complex variant"),
        }

        // Test first complex option
        assert_eq!(complex_choice.get_to(), vec!["target1", "target2"]);
        assert_eq!(complex_choice.get_type(), "conditional");
        assert_eq!(complex_choice.get_key(), Some("player_level".to_string()));
        assert!(complex_choice.get_value().is_some());
        assert_eq!(complex_choice.get_same_page(), Some(true));
        assert_eq!(complex_choice.get_time_limit(), Some(30));
        // timeout_to is validated above; no getter implemented

        // Test second complex option
        let simple_choice = ParagraphChoice::Simple(vec!["simple_target".to_string()]);
        assert_eq!(simple_choice.get_to(), vec!["simple_target"]);
        assert_eq!(simple_choice.get_type(), "goto");
        assert_eq!(simple_choice.get_key(), None);
        assert_eq!(simple_choice.get_value(), None);
        assert_eq!(simple_choice.get_same_page(), None);
        assert_eq!(simple_choice.get_time_limit(), None);
        // simple choice timeout_to none verified implicitly
    }

    #[tokio::test]
    async fn test_multilingual_content() {
        // Test multilingual content
        let paragraph = Paragraph {
            id: "multi_lang_test".to_string(),
            chapter_id: "c1".to_string(),
            texts: vec![
                Text {
                    lang: "zh-TW".to_string(),
                    paragraphs: "This is test paragraph 1".to_string(),
                    choices: vec!["Continue Adventure".to_string(), "Return to Village".to_string()],
                },
                Text {
                    lang: "zh-CN".to_string(),
                    paragraphs: "This is simplified Chinese content".to_string(),
                    choices: vec!["Option one".to_string(), "Option two".to_string()],
                },
                Text {
                    lang: "en".to_string(),
                    paragraphs: "This is English content".to_string(),
                    choices: vec!["Option One".to_string(), "Option Two".to_string()],
                },
            ],
            choices: vec![
                ParagraphChoice::Simple(vec!["next_scene".to_string()]),
                ParagraphChoice::Complex {
                    to: vec!["village".to_string()],
                    type_: "goto".to_string(),
                    key: None,
                    value: None,
                    same_page: None,
                    time_limit: Some(30),
                    timeout_to: None,
                },
            ],
        };

        // Verify each language version
        assert_eq!(paragraph.texts.len(), 3);
        assert!(paragraph.texts.iter().any(|t| t.lang == "zh-TW"));
        assert!(paragraph.texts.iter().any(|t| t.lang == "zh-CN"));
        assert!(paragraph.texts.iter().any(|t| t.lang == "en"));
    }
} 