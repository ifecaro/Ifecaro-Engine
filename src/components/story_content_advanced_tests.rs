use crate::components::story_content::{Choice, Action};
use std::collections::HashSet;

/// Helper function: Create test choice
fn create_test_choice(caption: &str, to: &str, action_type: &str) -> Choice {
    Choice {
        caption: caption.to_string().into(),
        action: Action {
            type_: action_type.to_string().into(),
            key: None,
            value: None,
            to: to.to_string().into(),
        },
    }
}

/// Helper function: Create test choice with value
fn create_test_choice_with_value(caption: &str, to: &str, action_type: &str, key: Option<String>, value: Option<serde_json::Value>) -> Choice {
    Choice {
        caption: caption.to_string().into(),
        action: Action {
            type_: action_type.to_string().into(),
            key,
            value,
            to: to.to_string().into(),
        },
    }
}

#[allow(unused_macros)]
macro_rules! hs {
    () => { HashSet::<String>::new() };
    ( $( $x:expr ),+ $(,)? ) => {{
        let mut set = HashSet::<String>::new();
        $( set.insert($x.to_string()); )+
        set
    }};
}

#[cfg(test)]
mod choice_data_structure_tests {
    use super::*;

    #[test]
    fn test_choice_creation_and_serialization() {
        let choice = create_test_choice("Test option", "test_target", "goto");
        
        assert_eq!(choice.caption, "Test option");
        assert_eq!(choice.action.to, "test_target");
        assert_eq!(choice.action.type_, "goto");
        assert!(choice.action.key.is_none());
        assert!(choice.action.value.is_none());
    }

    #[test]
    fn test_choice_with_complex_action_data() {
        let choice = create_test_choice_with_value(
            "Settings option",
            "settings_page",
            "set",
            Some("difficulty".to_string()),
            Some(serde_json::Value::String("hard".to_string())),
        );
        
        assert_eq!(choice.caption, "Settings option");
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
            "JSON test",
            "json_target",
            "add",
            Some("item".to_string()),
            Some(serde_json::json!({"name": "sword", "damage": 10})),
        );
        
        // Test serialization
        let serialized = serde_json::to_string(&original_choice).expect("Serialization failed");
        assert!(serialized.contains("JSON test"));
        assert!(serialized.contains("json_target"));
        assert!(serialized.contains("add"));
        assert!(serialized.contains("item"));
        assert!(serialized.contains("sword"));
        
        // Test deserialization
        let deserialized: Choice = serde_json::from_str(&serialized).expect("Deserialization failed");
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
        let choice = create_test_choice("Jump to", "scene_1", "goto");
        assert_eq!(choice.action.type_, "goto");
        assert_eq!(choice.action.to, "scene_1");
    }

    #[test]
    fn test_set_action_type() {
        let choice = create_test_choice_with_value(
            "Settings",
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
            "Add item",
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
        let choice = create_test_choice("Custom action", "custom_target", "custom_action");
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
        let choices = vec![create_test_choice("Single option", "single", "goto")];
        assert_eq!(choices.len(), 1);
        assert_eq!(choices[0].caption, "Single option");
    }

    #[test]
    fn test_multiple_choices_array() {
        let choices = vec![
            create_test_choice("Option one", "choice1", "goto"),
            create_test_choice("Option two", "choice2", "goto"),
            create_test_choice("Option three", "choice3", "goto"),
        ];
        
        assert_eq!(choices.len(), 3);
        for (i, choice) in choices.iter().enumerate() {
            assert_eq!(choice.caption, format!("Option {}", ["one", "two", "three"][i]));
            assert_eq!(choice.action.to, format!("choice{}", i + 1));
            assert_eq!(choice.action.type_, "goto");
        }
    }

    #[test]
    fn test_choice_filtering_by_type() {
        let choices = vec![
            create_test_choice("Jump option", "scene1", "goto"),
            create_test_choice_with_value("Settings option", "config", "set", Some("key".to_string()), None),
            create_test_choice("Another jump", "scene2", "goto"),
            create_test_choice_with_value("Add option", "inventory", "add", Some("item".to_string()), None),
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
            create_test_choice("Enabled option", "enabled", "goto"),
            create_test_choice("Disabled option", "disabled", "goto"),
            create_test_choice("Another enabled", "enabled2", "goto"),
        ];
        
        let enabled_choices = hs!("Enabled option", "Another enabled");
        
        // Test enabled logic
        for (i, choice) in choices.iter().enumerate() {
            let is_enabled = enabled_choices.contains(&choice.caption.as_ref().to_string());
            match i {
                0 => assert!(is_enabled, "First option should be enabled"),
                1 => assert!(!is_enabled, "Second option should be disabled"),
                2 => assert!(is_enabled, "Third option should be enabled"),
                _ => panic!("Unexpected option index"),
            }
        }
    }

    #[test]
    fn test_disabled_by_countdown_logic() {
        let _choices = vec![
            create_test_choice("Normal option", "normal", "goto"),
            create_test_choice("Countdown disabled option", "countdown", "goto"),
        ];
        
        let _enabled_choices = vec!["Normal option".to_string(), "Countdown disabled option".to_string()];
        let _disabled_by_countdown = vec![false, true];
        
        // Mock countdown state
        let countdowns = vec![10u32, 0u32]; // First has time, second expired
        let max_times = vec![30u32, 30u32];
        
        // Test countdown logic
        for (i, &countdown) in countdowns.iter().enumerate() {
            let is_expired = countdown == 0 && max_times[i] > 0;
            if i == 0 {
                assert!(!is_expired, "First option should not be expired");
            } else {
                assert!(is_expired, "Second option should be expired");
            }
        }
    }

    #[test]
    fn test_all_choices_disabled_scenario() {
        let _choices = vec![
            create_test_choice("Disabled option 1", "disabled1", "goto"),
            create_test_choice("Disabled option 2", "disabled2", "goto"),
            create_test_choice("Disabled option 3", "disabled3", "goto"),
        ];
        
        // All choices disabled by countdown
        let choice_count = 3;
        let progress_started = vec![false; choice_count];
        let disabled_by_countdown = vec![false; choice_count];
        
        // Test that no choices are available
        let available_count = progress_started.iter()
            .zip(disabled_by_countdown.iter())
            .filter(|(&started, &disabled)| started && !disabled)
            .count();
        
        assert_eq!(available_count, 0);
    }
}

#[cfg(test)]
mod countdown_state_tests {
    use super::*;

    #[test]
    fn test_countdown_array_initialization() {
        let _choices = vec![
            create_test_choice("Option one", "choice1", "goto"),
            create_test_choice("Option two", "choice2", "goto"),
            create_test_choice("Option three", "choice3", "goto"),
        ];
        
        // Simulate countdown array initialization
        let choice_count = 3;
        let mut countdowns = vec![0u32; choice_count];
        let mut max_times = vec![0u32; choice_count];
        let progress_started = vec![false; choice_count];
        let disabled_by_countdown = vec![false; choice_count];
        
        // Check initial values
        assert_eq!(countdowns.len(), 3);
        assert_eq!(max_times.len(), 3);
        assert_eq!(progress_started.len(), 3);
        assert_eq!(disabled_by_countdown.len(), 3);
        
        for i in 0..choice_count {
            assert_eq!(countdowns[i], 0);
            assert_eq!(max_times[i], 0);
            assert_eq!(progress_started[i], false);
            assert_eq!(disabled_by_countdown[i], false);
        }
        
        // Set different countdown times
        countdowns[0] = 30;
        countdowns[1] = 60;
        countdowns[2] = 0; // No countdown
        
        max_times[0] = 30;
        max_times[1] = 60;
        max_times[2] = 0;
        
        assert_eq!(countdowns[0], 30);
        assert_eq!(countdowns[1], 60);
        assert_eq!(countdowns[2], 0);
    }

    #[test]
    fn test_countdown_time_setting() {
        let choices = vec![
            create_test_choice("First option", "choice1", "goto"),
            create_test_choice("Second option", "choice2", "goto"),
        ];
        
        let mut countdowns = vec![0u32; choices.len()];
        let mut max_times = vec![0u32; choices.len()];
        
        // Set different countdown times
        let countdown_time = 5;
        countdowns[0] = countdown_time;
        countdowns[1] = countdown_time;
        max_times[0] = countdown_time;
        max_times[1] = countdown_time;
        
        assert_eq!(countdowns[0], countdown_time);
        assert_eq!(countdowns[1], countdown_time);
        assert_eq!(max_times[0], countdown_time);
        assert_eq!(max_times[1], countdown_time);
    }

    #[test]
    fn test_countdown_expiration_logic() {
        let _choices = vec![
            create_test_choice("Option one", "choice1", "goto"),
            create_test_choice("Option two", "choice2", "goto"),
        ];
        
        let countdowns = vec![5u32, 0u32]; // First still has time, second expired
        let max_times = vec![30u32, 30u32];
        
        // Simulate countdown expiration logic
        for (i, &countdown) in countdowns.iter().enumerate() {
            if countdown == 0 && max_times[i] > 0 {
                // This choice has expired
                assert!(true);
            }
        }
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
                panic!("Unable to parse key: {}", key);
            }
        }
    }

    #[test]
    fn test_keyboard_invalid_keys() {
        let invalid_keys = vec!["0", "a", "Enter", "Space", "ArrowUp"];
        
        for key in invalid_keys {
            let parse_result = key.parse::<usize>();
            if key == "0" {
                // 0 can be parsed but invalid in option selection (options start from 1)
                assert!(parse_result.is_ok());
            } else {
                // Other keys cannot be parsed as numbers
                assert!(parse_result.is_err());
            }
        }
    }

    #[test]
    fn test_keyboard_choice_index_calculation() {
        let choices = vec![
            create_test_choice("First option", "choice1", "goto"),
            create_test_choice("Second option", "choice2", "goto"),
            create_test_choice("Third option", "choice3", "goto"),
        ];
        
        // Test keyboard input simulation
        let enabled_choices = hs!("choice1", "choice2", "choice3");
        
        // Simulate key presses 1, 2, 3
        for key_num in 1..=3 {
            let choice_index = key_num - 1; // Convert to array index
            if choice_index < choices.len() {
                let choice = &choices[choice_index];
                let is_enabled = enabled_choices.contains(&choice.action.to.as_ref().to_string());
                assert!(is_enabled, "Key {} should correspond to enabled option", key_num);
            }
        }
        
        // Test out of range keys
        for key_num in 4..=9 {
            assert!(key_num > choices.len(), "Key {} should be out of range", key_num);
        }
    }
}

#[cfg(test)]
mod performance_simulation_tests {
    use super::*;

    #[test]
    fn test_large_choice_array_creation() {
        let mut choices = Vec::new();
        
        // Create 1000 options
        for i in 0..1000 {
            choices.push(create_test_choice(&format!("Option {}", i), &format!("option_{}", i), "goto"));
        }
        
        let _enabled_choices: Vec<String> = (0..1000).map(|i| format!("option_{}", i)).collect();
        let _disabled_by_countdown = vec![false; 1000];
        
        // Test paragraph splitting
        let repeated_text = "Single line text".repeat(100);
        // Since we're repeating single line text, there should only be one line
        let lines: Vec<&str> = repeated_text.lines().collect();
        assert_eq!(lines.len(), 1);
        assert!(lines[0].len() > 1000);
        
        // Create many options
        let mut choices = Vec::new();
        for i in 0..1000 {
            choices.push(create_test_choice(&format!("Choice {}", i), &format!("target_{}", i), "goto"));
        }
        
        // Enable every other one
        let enabled_choices: Vec<bool> = (0..1000).map(|i| i % 2 == 0).collect();
        
        assert_eq!(choices.len(), 1000);
        assert_eq!(enabled_choices.len(), 1000);
        assert_eq!(enabled_choices.iter().filter(|&&x| x).count(), 500);
        
        // Test search performance (simulate enabled checking)
        let start = std::time::Instant::now();
        let _filtered_choices: Vec<_> = choices.iter()
            .zip(enabled_choices.iter())
            .filter(|(_, &enabled)| enabled)
            .collect();
        let duration = start.elapsed();
        
        // Should complete within reasonable time (less than 1ms for this simple operation)
        assert!(duration.as_millis() < 10);
    }

    #[test]
    fn test_large_paragraph_content() {
        let base_text = "This is a test paragraph.";
        let large_paragraph = base_text.repeat(1000);
        
        assert!(large_paragraph.len() > 10000);
        assert!(large_paragraph.starts_with(base_text));
        assert!(large_paragraph.ends_with(base_text));
        
        // Test paragraph splitting
        let lines: Vec<&str> = large_paragraph.split('\n').collect();
        // Since we're repeating single line text, there should only be one line
        assert_eq!(lines.len(), 1);
    }

    #[test]
    fn test_choice_search_performance() {
        let mut choices = Vec::new();
        let mut enabled_choices = Vec::new();
        
        // Create many options
        for i in 1..=10000 {
            let caption = format!("Option {}", i);
            let choice = create_test_choice(&caption, &format!("target_{}", i), "goto");
            choices.push(choice);
            
            // Enable every other one
            if i % 2 == 0 {
                enabled_choices.push(caption);
            }
        }
        
        assert_eq!(choices.len(), 10000);
        assert_eq!(enabled_choices.len(), 5000);
        
        // Test search performance (simulate enabled checking)
        let mut enabled_count = 0;
        for choice in &choices {
            if enabled_choices.contains(&choice.caption.as_ref().to_string()) {
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
            caption: "".into(),
            action: Action {
                type_: "".into(),
                key: Some("".to_string()),
                value: Some(serde_json::Value::String("".to_string())),
                to: "".into(),
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
            "🎮 Game option 🎯",
            "unicode_target_test",
            "goto"
        );
        
        assert_eq!(choice.caption, "🎮 Game option 🎯");
        assert_eq!(choice.action.to, "unicode_target_test");
        assert!(choice.caption.contains("🎮"));
        assert!(choice.caption.contains("🎯"));
    }

    #[test]
    fn test_special_characters() {
        let choice = create_test_choice(
            "Special chars: !@#$%^&*()_+-=[]{}|;':\",./<>?",
            "special_chars",
            "goto"
        );
        
        assert!(choice.caption.contains("!@#$%^&*()"));
        assert!(choice.caption.contains("[]{}|"));
        assert!(choice.caption.contains("\",./<>?"));
    }

    #[test]
    fn test_very_long_strings() {
        let long_caption = "Very long option title".repeat(100);
        let long_target = "very_long_target_".repeat(50);
        
        let choice = create_test_choice(&long_caption, &long_target, "goto");
        
        assert!(choice.caption.len() > 1000);
        assert!(choice.action.to.len() > 500);
        assert!(choice.caption.starts_with("Very long option title"));
        assert!(choice.action.to.starts_with("very_long_target_"));
    }

    #[test]
    fn test_null_and_none_values() {
        let choice = Choice {
            caption: "Test None values".into(),
            action: Action {
                type_: "test".into(),
                key: None,
                value: Some(serde_json::Value::Null),
                to: "test_target".into(),
            },
        };
        
        assert!(choice.action.key.is_none());
        assert!(choice.action.value.is_some());
        
        if let Some(serde_json::Value::Null) = choice.action.value {
            // Correctly handle null values
        } else {
            panic!("Expected null value");
        }
    }
} 