use crate::components::story_content::{Choice, Action};

/// 輔助函數：創建測試用的選項
fn create_test_choice(caption: &str, to: &str, type_: &str) -> Choice {
    Choice {
        caption: caption.to_string(),
        action: Action {
            type_: type_.to_string(),
            key: None,
            value: None,
            to: to.to_string(),
        },
    }
}

/// 輔助函數：創建有值的測試選項
fn create_test_choice_with_value(
    caption: &str, 
    to: &str, 
    type_: &str, 
    key: Option<String>, 
    value: Option<serde_json::Value>
) -> Choice {
    Choice {
        caption: caption.to_string(),
        action: Action {
            type_: type_.to_string(),
            key,
            value,
            to: to.to_string(),
        },
    }
}

#[cfg(test)]
mod choice_data_structure_tests {
    use super::*;

    #[test]
    fn test_choice_creation_and_serialization() {
        let choice = create_test_choice("測試選項", "test_target", "goto");
        
        assert_eq!(choice.caption, "測試選項");
        assert_eq!(choice.action.to, "test_target");
        assert_eq!(choice.action.type_, "goto");
        assert!(choice.action.key.is_none());
        assert!(choice.action.value.is_none());
    }

    #[test]
    fn test_choice_with_complex_action_data() {
        let choice = create_test_choice_with_value(
            "設定選項",
            "settings_page",
            "set",
            Some("difficulty".to_string()),
            Some(serde_json::Value::String("hard".to_string())),
        );
        
        assert_eq!(choice.caption, "設定選項");
        assert_eq!(choice.action.type_, "set");
        assert_eq!(choice.action.key, Some("difficulty".to_string()));
        assert!(choice.action.value.is_some());
        
        if let Some(serde_json::Value::String(value)) = &choice.action.value {
            assert_eq!(value, "hard");
        } else {
            panic!("Expected string value");
        }
    }

    #[test]
    fn test_choice_serialization_deserialization() {
        let original_choice = create_test_choice_with_value(
            "JSON 測試",
            "json_target",
            "add",
            Some("item".to_string()),
            Some(serde_json::json!({"name": "sword", "damage": 10})),
        );
        
        // 測試序列化
        let serialized = serde_json::to_string(&original_choice).expect("序列化失敗");
        assert!(serialized.contains("JSON 測試"));
        assert!(serialized.contains("json_target"));
        assert!(serialized.contains("add"));
        assert!(serialized.contains("item"));
        assert!(serialized.contains("sword"));
        
        // 測試反序列化
        let deserialized: Choice = serde_json::from_str(&serialized).expect("反序列化失敗");
        assert_eq!(deserialized.caption, original_choice.caption);
        assert_eq!(deserialized.action.type_, original_choice.action.type_);
        assert_eq!(deserialized.action.to, original_choice.action.to);
        assert_eq!(deserialized.action.key, original_choice.action.key);
    }
}

#[cfg(test)]
mod action_type_validation_tests {
    use super::*;

    #[test]
    fn test_goto_action_type() {
        let choice = create_test_choice("跳轉", "scene_1", "goto");
        assert_eq!(choice.action.type_, "goto");
        assert_eq!(choice.action.to, "scene_1");
    }

    #[test]
    fn test_set_action_type() {
        let choice = create_test_choice_with_value(
            "設定",
            "config",
            "set",
            Some("language".to_string()),
            Some(serde_json::Value::String("zh-TW".to_string())),
        );
        assert_eq!(choice.action.type_, "set");
        assert!(choice.action.key.is_some());
        assert!(choice.action.value.is_some());
    }

    #[test]
    fn test_add_action_type() {
        let choice = create_test_choice_with_value(
            "添加物品",
            "inventory",
            "add",
            Some("item_id".to_string()),
            Some(serde_json::Value::Number(serde_json::Number::from(42))),
        );
        assert_eq!(choice.action.type_, "add");
        assert_eq!(choice.action.key, Some("item_id".to_string()));
        
        if let Some(serde_json::Value::Number(num)) = &choice.action.value {
            assert_eq!(num.as_u64(), Some(42));
        } else {
            panic!("Expected number value");
        }
    }

    #[test]
    fn test_custom_action_type() {
        let choice = create_test_choice("自訂動作", "custom_target", "custom_action");
        assert_eq!(choice.action.type_, "custom_action");
        assert_eq!(choice.action.to, "custom_target");
    }
}

#[cfg(test)]
mod choice_array_operations_tests {
    use super::*;

    #[test]
    fn test_empty_choice_array() {
        let choices: Vec<Choice> = vec![];
        assert_eq!(choices.len(), 0);
        assert!(choices.is_empty());
    }

    #[test]
    fn test_single_choice_array() {
        let choices = vec![create_test_choice("單一選項", "single", "goto")];
        assert_eq!(choices.len(), 1);
        assert_eq!(choices[0].caption, "單一選項");
    }

    #[test]
    fn test_multiple_choices_array() {
        let choices = vec![
            create_test_choice("選項一", "choice1", "goto"),
            create_test_choice("選項二", "choice2", "goto"),
            create_test_choice("選項三", "choice3", "goto"),
        ];
        
        assert_eq!(choices.len(), 3);
        for (i, choice) in choices.iter().enumerate() {
            assert_eq!(choice.caption, format!("選項{}", ["一", "二", "三"][i]));
            assert_eq!(choice.action.to, format!("choice{}", i + 1));
            assert_eq!(choice.action.type_, "goto");
        }
    }

    #[test]
    fn test_choice_filtering_by_type() {
        let choices = vec![
            create_test_choice("跳轉選項", "scene1", "goto"),
            create_test_choice_with_value("設定選項", "config", "set", Some("key".to_string()), None),
            create_test_choice("另一個跳轉", "scene2", "goto"),
            create_test_choice_with_value("添加選項", "inventory", "add", Some("item".to_string()), None),
        ];
        
        let goto_choices: Vec<&Choice> = choices.iter()
            .filter(|c| c.action.type_ == "goto")
            .collect();
        assert_eq!(goto_choices.len(), 2);
        
        let set_choices: Vec<&Choice> = choices.iter()
            .filter(|c| c.action.type_ == "set")
            .collect();
        assert_eq!(set_choices.len(), 1);
        
        let add_choices: Vec<&Choice> = choices.iter()
            .filter(|c| c.action.type_ == "add")
            .collect();
        assert_eq!(add_choices.len(), 1);
    }
}

#[cfg(test)]
mod enabled_choices_logic_tests {
    use super::*;

    #[test]
    fn test_enabled_choices_matching() {
        let choices = vec![
            create_test_choice("啟用選項", "enabled", "goto"),
            create_test_choice("禁用選項", "disabled", "goto"),
            create_test_choice("另一個啟用", "enabled2", "goto"),
        ];
        
        let enabled_choices = vec!["啟用選項".to_string(), "另一個啟用".to_string()];
        
        // 測試啟用邏輯
        for (i, choice) in choices.iter().enumerate() {
            let is_enabled = enabled_choices.contains(&choice.caption);
            match i {
                0 => assert!(is_enabled, "第一個選項應該被啟用"),
                1 => assert!(!is_enabled, "第二個選項應該被禁用"),
                2 => assert!(is_enabled, "第三個選項應該被啟用"),
                _ => panic!("意外的選項索引"),
            }
        }
    }

    #[test]
    fn test_disabled_by_countdown_logic() {
        let choices = vec![
            create_test_choice("正常選項", "normal", "goto"),
            create_test_choice("倒數禁用", "countdown_disabled", "goto"),
        ];
        
        let enabled_choices = vec!["正常選項".to_string(), "倒數禁用".to_string()];
        let disabled_by_countdown = vec![false, true];
        
        // 測試組合邏輯
        for (i, choice) in choices.iter().enumerate() {
            let is_enabled = enabled_choices.contains(&choice.caption);
            let is_disabled_by_countdown = disabled_by_countdown.get(i).copied().unwrap_or(false);
            let final_enabled = is_enabled && !is_disabled_by_countdown;
            
            match i {
                0 => assert!(final_enabled, "第一個選項應該最終啟用"),
                1 => assert!(!final_enabled, "第二個選項應該被倒數禁用"),
                _ => panic!("意外的選項索引"),
            }
        }
    }

    #[test]
    fn test_all_choices_disabled_scenario() {
        let choices = vec![
            create_test_choice("選項一", "choice1", "goto"),
            create_test_choice("選項二", "choice2", "goto"),
        ];
        
        let enabled_choices: Vec<String> = vec![]; // 沒有啟用的選項
        
        for choice in &choices {
            let is_enabled = enabled_choices.contains(&choice.caption);
            assert!(!is_enabled, "所有選項都應該被禁用");
        }
    }
}

#[cfg(test)]
mod countdown_state_tests {
    use super::*;

    #[test]
    fn test_countdown_array_initialization() {
        let choices = vec![
            create_test_choice("選項一", "choice1", "goto"),
            create_test_choice("選項二", "choice2", "goto"),
            create_test_choice("選項三", "choice3", "goto"),
        ];
        
        // 模擬倒數計時陣列初始化
        let countdowns = vec![0u32; choices.len()];
        let max_times = vec![0u32; choices.len()];
        let disabled_by_countdown = vec![false; choices.len()];
        
        assert_eq!(countdowns.len(), choices.len());
        assert_eq!(max_times.len(), choices.len());
        assert_eq!(disabled_by_countdown.len(), choices.len());
        
        // 檢查初始值
        for &countdown in &countdowns {
            assert_eq!(countdown, 0);
        }
        for &disabled in &disabled_by_countdown {
            assert!(!disabled);
        }
    }

    #[test]
    fn test_countdown_time_setting() {
        let choices = vec![
            create_test_choice("快速選項", "quick", "goto"),
            create_test_choice("慢速選項", "slow", "goto"),
        ];
        
        let mut countdowns = vec![0u32; choices.len()];
        let mut max_times = vec![0u32; choices.len()];
        
        // 設定不同的倒數時間
        countdowns[0] = 5;
        countdowns[1] = 10;
        max_times[0] = 5;
        max_times[1] = 10;
        
        assert_eq!(countdowns[0], 5);
        assert_eq!(countdowns[1], 10);
        assert_eq!(max_times[0], 5);
        assert_eq!(max_times[1], 10);
    }

    #[test]
    fn test_countdown_expiration_logic() {
        let _choices = vec![
            create_test_choice("選項一", "choice1", "goto"),
            create_test_choice("選項二", "choice2", "goto"),
        ];
        
        let countdowns = vec![5u32, 0u32]; // 第一個還有時間，第二個已過期
        let mut disabled_by_countdown = vec![false, false];
        
        // 模擬倒數計時過期邏輯
        for (i, &countdown) in countdowns.iter().enumerate() {
            if countdown == 0 {
                disabled_by_countdown[i] = true;
            }
        }
        
        assert!(!disabled_by_countdown[0], "第一個選項不應該被禁用");
        assert!(disabled_by_countdown[1], "第二個選項應該被禁用");
    }
}

#[cfg(test)]
mod keyboard_input_simulation_tests {
    use super::*;

    #[test]
    fn test_keyboard_number_parsing() {
        let test_keys = vec!["1", "2", "3", "4", "5"];
        
        for (i, key) in test_keys.iter().enumerate() {
            if let Ok(num) = key.parse::<usize>() {
                assert_eq!(num, i + 1);
                assert!(num > 0);
                assert!(num <= 5);
            } else {
                panic!("無法解析按鍵: {}", key);
            }
        }
    }

    #[test]
    fn test_keyboard_invalid_keys() {
        let invalid_keys = vec!["0", "a", "Enter", "Space", "ArrowUp"];
        
        for key in invalid_keys {
            let parse_result = key.parse::<usize>();
            if key == "0" {
                // 0 可以解析但在選項選擇中無效（選項從 1 開始）
                assert_eq!(parse_result.unwrap(), 0);
            } else {
                // 其他按鍵無法解析為數字
                assert!(parse_result.is_err());
            }
        }
    }

    #[test]
    fn test_keyboard_choice_index_calculation() {
        let choices = vec![
            create_test_choice("第一選項", "choice1", "goto"),
            create_test_choice("第二選項", "choice2", "goto"),
            create_test_choice("第三選項", "choice3", "goto"),
        ];
        
        // 模擬按鍵 1, 2, 3
        for key_num in 1..=3 {
            let choice_index = key_num - 1; // 轉換為陣列索引
            assert!(choice_index < choices.len());
            assert_eq!(choices[choice_index].caption, format!("第{}選項", ["一", "二", "三"][choice_index]));
        }
        
        // 測試超出範圍的按鍵
        let invalid_key = 4;
        let invalid_index = invalid_key - 1;
        assert!(invalid_index >= choices.len());
    }
}

#[cfg(test)]
mod performance_simulation_tests {
    use super::*;

    #[test]
    fn test_large_choice_array_creation() {
        let mut choices = Vec::new();
        
        // 創建 1000 個選項
        for i in 1..=1000 {
            let choice = create_test_choice(
                &format!("選項 {}", i),
                &format!("target_{}", i),
                "goto"
            );
            choices.push(choice);
        }
        
        assert_eq!(choices.len(), 1000);
        assert_eq!(choices[0].caption, "選項 1");
        assert_eq!(choices[999].caption, "選項 1000");
        assert_eq!(choices[0].action.to, "target_1");
        assert_eq!(choices[999].action.to, "target_1000");
    }

    #[test]
    fn test_large_paragraph_content() {
        let base_text = "這是一個測試段落。";
        let large_paragraph = base_text.repeat(1000);
        
        assert!(large_paragraph.len() > 10000);
        assert!(large_paragraph.starts_with(base_text));
        assert!(large_paragraph.ends_with(base_text));
        
        // 測試段落分割
        let lines: Vec<&str> = large_paragraph.split('\n').collect();
        // 由於我們重複的是單行文字，所以應該只有一行
        assert_eq!(lines.len(), 1);
    }

    #[test]
    fn test_choice_search_performance() {
        let mut choices = Vec::new();
        let mut enabled_choices = Vec::new();
        
        // 創建大量選項
        for i in 1..=10000 {
            let caption = format!("選項 {}", i);
            let choice = create_test_choice(&caption, &format!("target_{}", i), "goto");
            choices.push(choice);
            
            // 每隔兩個啟用一個
            if i % 2 == 0 {
                enabled_choices.push(caption);
            }
        }
        
        assert_eq!(choices.len(), 10000);
        assert_eq!(enabled_choices.len(), 5000);
        
        // 測試搜尋性能（模擬啟用檢查）
        let mut enabled_count = 0;
        for choice in &choices {
            if enabled_choices.contains(&choice.caption) {
                enabled_count += 1;
            }
        }
        
        assert_eq!(enabled_count, 5000);
    }
}

#[cfg(test)]
mod edge_case_handling_tests {
    use super::*;

    #[test]
    fn test_empty_string_values() {
        let choice = Choice {
            caption: "".to_string(),
            action: Action {
                type_: "".to_string(),
                key: Some("".to_string()),
                value: Some(serde_json::Value::String("".to_string())),
                to: "".to_string(),
            },
        };
        
        assert_eq!(choice.caption, "");
        assert_eq!(choice.action.type_, "");
        assert_eq!(choice.action.to, "");
        assert!(choice.action.key.is_some());
        assert!(choice.action.value.is_some());
    }

    #[test]
    fn test_unicode_content() {
        let choice = create_test_choice(
            "🎮 遊戲選項 🎯",
            "unicode_target_測試",
            "goto"
        );
        
        assert_eq!(choice.caption, "🎮 遊戲選項 🎯");
        assert_eq!(choice.action.to, "unicode_target_測試");
        assert!(choice.caption.contains("🎮"));
        assert!(choice.caption.contains("🎯"));
    }

    #[test]
    fn test_special_characters() {
        let choice = create_test_choice(
            "特殊字符: !@#$%^&*()_+-=[]{}|;':\",./<>?",
            "special_chars",
            "goto"
        );
        
        assert!(choice.caption.contains("!@#$%^&*()"));
        assert!(choice.caption.contains("[]{}|"));
        assert!(choice.caption.contains("\",./<>?"));
    }

    #[test]
    fn test_very_long_strings() {
        let long_caption = "很長的選項標題".repeat(100);
        let long_target = "very_long_target_".repeat(50);
        
        let choice = create_test_choice(&long_caption, &long_target, "goto");
        
        assert!(choice.caption.len() > 1000);
        assert!(choice.action.to.len() > 500);
        assert!(choice.caption.starts_with("很長的選項標題"));
        assert!(choice.action.to.starts_with("very_long_target_"));
    }

    #[test]
    fn test_null_and_none_values() {
        let choice = Choice {
            caption: "測試 None 值".to_string(),
            action: Action {
                type_: "test".to_string(),
                key: None,
                value: Some(serde_json::Value::Null),
                to: "test_target".to_string(),
            },
        };
        
        assert!(choice.action.key.is_none());
        assert!(choice.action.value.is_some());
        
        if let Some(serde_json::Value::Null) = choice.action.value {
            // 正確處理 null 值
        } else {
            panic!("Expected null value");
        }
    }
} 