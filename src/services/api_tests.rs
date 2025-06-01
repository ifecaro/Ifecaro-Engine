#[cfg(test)]
mod tests {
    use super::super::api::*;
    use crate::contexts::paragraph_context::{Paragraph, Text, ParagraphChoice};

    /// 輔助函數：創建測試段落
    fn create_test_paragraph(id: &str, chapter_id: &str) -> Paragraph {
        Paragraph {
            id: id.to_string(),
            chapter_id: chapter_id.to_string(),
            texts: vec![
                Text {
                    lang: "zh-TW".to_string(),
                    paragraphs: format!("這是測試段落 {}", id),
                    choices: vec![
                        "繼續冒險".to_string(),
                        "返回村莊".to_string(),
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
                },
            ],
        }
    }

    /// 輔助函數：創建測試章節
    fn create_test_chapter(id: &str, title: &str, order: i32) -> Chapter {
        Chapter {
            id: id.to_string(),
            title: title.to_string(),
            order,
        }
    }

    #[tokio::test]
    async fn test_mock_api_get_paragraphs_success() {
        // 準備測試資料
        let test_paragraphs = vec![
            create_test_paragraph("p1", "ch1"),
            create_test_paragraph("p2", "ch1"),
            create_test_paragraph("p3", "ch2"),
        ];

        // 創建 mock 客戶端
        let mock_client = MockApiClient::new()
            .with_paragraphs(test_paragraphs.clone());

        // 執行測試
        let result = mock_client.get_paragraphs().await;

        // 驗證結果
        assert!(result.is_ok());
        let data = result.unwrap();
        assert_eq!(data.items.len(), 3);
        assert_eq!(data.items[0].id, "p1");
        assert_eq!(data.items[0].chapter_id, "ch1");
        assert_eq!(data.items[2].id, "p3");
        assert_eq!(data.items[2].chapter_id, "ch2");
    }

    #[tokio::test]
    async fn test_mock_api_get_paragraphs_failure() {
        // 創建會失敗的 mock 客戶端
        let mock_client = MockApiClient::new().with_failure();

        // 執行測試
        let result = mock_client.get_paragraphs().await;

        // 驗證錯誤
        assert!(result.is_err());
        match result.unwrap_err() {
            ApiError::NetworkError(msg) => {
                assert_eq!(msg, "Mock network error");
            }
            _ => panic!("Expected NetworkError"),
        }
    }

    #[tokio::test]
    async fn test_mock_api_get_chapters_success() {
        // 準備測試資料
        let test_chapters = vec![
            create_test_chapter("ch1", "第一章：開始", 1),
            create_test_chapter("ch2", "第二章：冒險", 2),
            create_test_chapter("ch3", "第三章：結局", 3),
        ];

        // 創建 mock 客戶端
        let mock_client = MockApiClient::new()
            .with_chapters(test_chapters.clone());

        // 執行測試
        let result = mock_client.get_chapters().await;

        // 驗證結果
        assert!(result.is_ok());
        let data = result.unwrap();
        assert_eq!(data.items.len(), 3);
        assert_eq!(data.items[0].title, "第一章：開始");
        assert_eq!(data.items[1].order, 2);
        assert_eq!(data.items[2].id, "ch3");
    }

    #[tokio::test]
    async fn test_mock_api_get_paragraph_by_id_found() {
        // 準備測試資料
        let test_paragraphs = vec![
            create_test_paragraph("p1", "ch1"),
            create_test_paragraph("p2", "ch1"),
        ];

        let mock_client = MockApiClient::new()
            .with_paragraphs(test_paragraphs);

        // 測試找到段落
        let result = mock_client.get_paragraph_by_id("p2").await;
        assert!(result.is_ok());
        let paragraph = result.unwrap();
        assert_eq!(paragraph.id, "p2");
        assert_eq!(paragraph.chapter_id, "ch1");
    }

    #[tokio::test]
    async fn test_mock_api_get_paragraph_by_id_not_found() {
        // 準備測試資料
        let test_paragraphs = vec![
            create_test_paragraph("p1", "ch1"),
        ];

        let mock_client = MockApiClient::new()
            .with_paragraphs(test_paragraphs);

        // 測試找不到段落
        let result = mock_client.get_paragraph_by_id("p999").await;
        assert!(result.is_err());
        match result.unwrap_err() {
            ApiError::NotFound => {
                // 正確的錯誤類型
            }
            _ => panic!("Expected NotFound error"),
        }
    }

    #[tokio::test]
    async fn test_mock_api_update_paragraph_success() {
        let mock_client = MockApiClient::new();
        let test_paragraph = create_test_paragraph("p1", "ch1");

        let result = mock_client.update_paragraph(&test_paragraph).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_mock_api_update_paragraph_failure() {
        let mock_client = MockApiClient::new().with_failure();
        let test_paragraph = create_test_paragraph("p1", "ch1");

        let result = mock_client.update_paragraph(&test_paragraph).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_complex_paragraph_choice_data() {
        // 測試複雜的選項資料結構
        let complex_paragraph = Paragraph {
            id: "complex_p1".to_string(),
            chapter_id: "ch1".to_string(),
            texts: vec![
                Text {
                    lang: "zh-TW".to_string(),
                    paragraphs: "複雜選項測試".to_string(),
                    choices: vec![
                        "設定難度".to_string(),
                        "購買道具".to_string(),
                        "跳轉場景".to_string(),
                    ],
                },
            ],
            choices: vec![
                ParagraphChoice::Complex {
                    to: vec!["settings".to_string()],
                    type_: "set".to_string(),
                    key: Some("difficulty".to_string()),
                    value: Some(serde_json::Value::String("hard".to_string())),
                    same_page: Some(true),
                    time_limit: None,
                },
                ParagraphChoice::Complex {
                    to: vec!["shop".to_string()],
                    type_: "add".to_string(),
                    key: Some("item_id".to_string()),
                    value: Some(serde_json::Value::Number(serde_json::Number::from(42))),
                    same_page: Some(false),
                    time_limit: Some(60),
                },
                ParagraphChoice::Simple(vec!["next_scene".to_string()]),
            ],
        };

        let mock_client = MockApiClient::new()
            .with_paragraphs(vec![complex_paragraph.clone()]);

        let result = mock_client.get_paragraph_by_id("complex_p1").await;
        assert!(result.is_ok());
        
        let retrieved = result.unwrap();
        assert_eq!(retrieved.choices.len(), 3);
        
        // 測試第一個複雜選項
        if let ParagraphChoice::Complex { type_, key, value, time_limit, .. } = &retrieved.choices[0] {
            assert_eq!(type_, "set");
            assert_eq!(key.as_ref().unwrap(), "difficulty");
            assert_eq!(value.as_ref().unwrap(), &serde_json::Value::String("hard".to_string()));
            assert_eq!(*time_limit, None);
        } else {
            panic!("Expected Complex choice");
        }
        
        // 測試第二個複雜選項
        if let ParagraphChoice::Complex { type_, time_limit, .. } = &retrieved.choices[1] {
            assert_eq!(type_, "add");
            assert_eq!(*time_limit, Some(60));
        } else {
            panic!("Expected Complex choice");
        }
    }

    #[tokio::test]
    async fn test_multilingual_content() {
        // 測試多語言內容
        let multilingual_paragraph = Paragraph {
            id: "multi_p1".to_string(),
            chapter_id: "ch1".to_string(),
            texts: vec![
                Text {
                    lang: "zh-TW".to_string(),
                    paragraphs: "繁體中文內容\n\n第二段落".to_string(),
                    choices: vec!["繼續".to_string(), "返回".to_string()],
                },
                Text {
                    lang: "en".to_string(),
                    paragraphs: "English content\n\nSecond paragraph".to_string(),
                    choices: vec!["Continue".to_string(), "Return".to_string()],
                },
                Text {
                    lang: "ja".to_string(),
                    paragraphs: "日本語の内容\n\n第二段落".to_string(),
                    choices: vec!["続ける".to_string(), "戻る".to_string()],
                },
            ],
            choices: vec![
                ParagraphChoice::Simple(vec!["next".to_string()]),
                ParagraphChoice::Simple(vec!["back".to_string()]),
            ],
        };

        let mock_client = MockApiClient::new()
            .with_paragraphs(vec![multilingual_paragraph]);

        let result = mock_client.get_paragraph_by_id("multi_p1").await;
        assert!(result.is_ok());
        
        let retrieved = result.unwrap();
        assert_eq!(retrieved.texts.len(), 3);
        
        // 驗證各語言版本
        let zh_text = retrieved.texts.iter().find(|t| t.lang == "zh-TW").unwrap();
        assert!(zh_text.paragraphs.contains("繁體中文"));
        assert_eq!(zh_text.choices[0], "繼續");
        
        let en_text = retrieved.texts.iter().find(|t| t.lang == "en").unwrap();
        assert!(en_text.paragraphs.contains("English"));
        assert_eq!(en_text.choices[0], "Continue");
        
        let ja_text = retrieved.texts.iter().find(|t| t.lang == "ja").unwrap();
        assert!(ja_text.paragraphs.contains("日本語"));
        assert_eq!(ja_text.choices[0], "続ける");
    }
} 