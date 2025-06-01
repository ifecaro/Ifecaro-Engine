use dioxus::prelude::*;
use dioxus_ssr::render;
use crate::components::story_content::{StoryContentUI, StoryContentUIProps, Choice, Action};
use dioxus_core::NoOpMutations;
use dioxus::prelude::VirtualDom;

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
fn create_test_choice_with_value(caption: &str, to: &str, type_: &str, key: Option<String>, value: Option<serde_json::Value>) -> Choice {
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

/// 輔助函數：渲染組件並返回 HTML
fn render_story_content_ui(props: StoryContentUIProps) -> String {
    let mut vdom = VirtualDom::new_with_props(StoryContentUI, props);
    let mut mutations = NoOpMutations;
    vdom.rebuild(&mut mutations);
    render(&vdom)
}

#[cfg(test)]
mod basic_ui_tests {
    use super::*;

    #[test]
    fn test_empty_story_content() {
        let props = StoryContentUIProps {
            paragraph: "".to_string(),
            choices: vec![],
            enabled_choices: vec![],
            disabled_by_countdown: vec![],
            chapter_title: "".to_string(),
        };
        
        let html = render_story_content_ui(props);
        assert!(html.contains("prose-sm"), "應包含基本的文章樣式");
        assert!(html.contains("list-decimal"), "應包含有序列表樣式");
    }

    #[test]
    fn test_paragraph_display() {
        let props = StoryContentUIProps {
            paragraph: "這是第一段\n\n這是第二段\n\n這是第三段".to_string(),
            choices: vec![],
            enabled_choices: vec![],
            disabled_by_countdown: vec![],
            chapter_title: "測試章節".to_string(),
        };
        
        let html = render_story_content_ui(props);
        assert!(html.contains("這是第一段"), "應顯示第一段內容");
        assert!(html.contains("這是第二段"), "應顯示第二段內容");
        assert!(html.contains("這是第三段"), "應顯示第三段內容");
        assert!(html.contains("測試章節"), "應顯示章節標題");
        assert!(html.contains("indent-10"), "段落應有縮排樣式");
        assert!(html.contains("tracking-wide"), "段落應有字間距樣式");
    }

    #[test]
    fn test_chapter_title_styling() {
        let props = StoryContentUIProps {
            paragraph: "段落內容".to_string(),
            choices: vec![],
            enabled_choices: vec![],
            disabled_by_countdown: vec![],
            chapter_title: "第一章：冒險的開始".to_string(),
        };
        
        let html = render_story_content_ui(props);
        assert!(html.contains("第一章：冒險的開始"), "應顯示完整章節標題");
        assert!(html.contains("text-3xl"), "標題應有大字體樣式");
        assert!(html.contains("md:text-4xl"), "標題應有響應式字體");
        assert!(html.contains("letter-spacing: 0.1em"), "標題應有字母間距");
        assert!(html.contains("min-h-[calc(100vh-56px)]"), "標題容器應有最小高度");
    }
}

#[cfg(test)]
mod choice_tests {
    use super::*;

    #[test]
    fn test_single_choice_enabled() {
        let choices = vec![create_test_choice("繼續", "next", "goto")];
        let props = StoryContentUIProps {
            paragraph: "故事內容".to_string(),
            choices: choices.clone(),
            enabled_choices: vec!["繼續".to_string()],
            disabled_by_countdown: vec![false],
            chapter_title: "".to_string(),
        };
        
        let html = render_story_content_ui(props);
        assert!(html.contains("繼續"), "應顯示選項文字");
        assert!(html.contains("cursor-pointer"), "啟用選項應有指標樣式");
        assert!(!html.contains("opacity-50"), "啟用選項不應有透明度");
}

#[test]
    fn test_multiple_choices_mixed_states() {
    let choices = vec![
            create_test_choice("選項一", "choice1", "goto"),
            create_test_choice("選項二", "choice2", "goto"),
            create_test_choice("選項三", "choice3", "goto"),
            create_test_choice("選項四", "choice4", "goto"),
        ];
    let props = StoryContentUIProps {
            paragraph: "選擇你的路".to_string(),
        choices: choices.clone(),
            enabled_choices: vec!["選項一".to_string(), "選項三".to_string()],
            disabled_by_countdown: vec![false, false, false, true], // choice4 被倒數禁用
            chapter_title: "重要抉擇".to_string(),
        };
        
        let html = render_story_content_ui(props);
        
        // 檢查所有選項都顯示
        for i in 1..=4 {
            assert!(html.contains(&format!("選項{}", ["一", "二", "三", "四"][i-1])), "應顯示選項{}", i);
        }
        
        // 檢查啟用狀態 (choice1, choice3 應啟用)
        let cursor_pointer_count = html.matches("cursor-pointer").count();
        assert!(cursor_pointer_count >= 2, "至少應有2個啟用選項");
        
        // 檢查禁用狀態
        let opacity_50_count = html.matches("opacity-50").count();
        assert!(opacity_50_count >= 2, "應有2個禁用選項");
        
        let cursor_not_allowed_count = html.matches("cursor-not-allowed").count();
        assert!(cursor_not_allowed_count >= 2, "應有2個不可點擊選項");
    }

    #[test]
    fn test_choice_with_complex_action() {
        let choices = vec![
            create_test_choice_with_value(
                "設定難度",
                "settings_difficulty",
                "set",
                Some("difficulty".to_string()),
                Some(serde_json::Value::String("hard".to_string())),
            ),
            create_test_choice_with_value(
                "跳轉場景",
                "scene_2",
                "goto",
                None,
                Some(serde_json::Value::Number(serde_json::Number::from(42))),
            ),
        ];
        
        let props = StoryContentUIProps {
            paragraph: "配置你的遊戲".to_string(),
            choices: choices.clone(),
            enabled_choices: vec!["設定難度".to_string(), "跳轉場景".to_string()],
            disabled_by_countdown: vec![false, false],
            chapter_title: "遊戲設定".to_string(),
        };
        
        let html = render_story_content_ui(props);
        assert!(html.contains("設定難度"), "應顯示設定選項");
        assert!(html.contains("跳轉場景"), "應顯示跳轉選項");
    }

    #[test]
    fn test_all_choices_disabled() {
        let choices = vec![
            create_test_choice("選項一", "choice1", "goto"),
            create_test_choice("選項二", "choice2", "goto"),
        ];
        let props = StoryContentUIProps {
            paragraph: "沒有可用選項".to_string(),
            choices: choices.clone(),
            enabled_choices: vec![], // 全部禁用
            disabled_by_countdown: vec![false, false],
            chapter_title: "死胡同".to_string(),
        };
        
        let html = render_story_content_ui(props);
        
        // 所有選項都應該是禁用狀態
        let opacity_50_count = html.matches("opacity-50").count();
        assert!(opacity_50_count >= 2, "所有選項都應被禁用");
        
        assert!(!html.contains("cursor-pointer"), "不應有可點擊選項");
    }

    #[test]
    fn test_countdown_disabled_choices() {
        let choices = vec![
            create_test_choice("時限選項", "timed_choice", "goto"),
            create_test_choice("普通選項", "normal_choice", "goto"),
        ];
        let props = StoryContentUIProps {
            paragraph: "時間緊迫的選擇".to_string(),
            choices: choices.clone(),
            enabled_choices: vec!["時限選項".to_string(), "普通選項".to_string()],
            disabled_by_countdown: vec![true, false], // 第一個被倒數禁用
            chapter_title: "時間壓力".to_string(),
        };
        
        let html = render_story_content_ui(props);
        assert!(html.contains("時限選項"), "應顯示時限選項");
        assert!(html.contains("普通選項"), "應顯示普通選項");
        
        // 檢查混合狀態
        assert!(html.contains("opacity-50"), "應有禁用選項");
        assert!(html.contains("cursor-pointer"), "應有啟用選項");
    }

    #[test]
    fn test_choice_display_format() {
        let choices = vec![
            create_test_choice("", "empty_caption", "goto"), // 空標題
            create_test_choice("很長的選項標題，包含中文、English和123數字", "long_caption", "goto"),
            create_test_choice("特殊符號!@#$%^&*()", "special_chars", "goto"),
        ];
        let props = StoryContentUIProps {
            paragraph: "測試各種標題格式".to_string(),
            choices: choices.clone(),
            enabled_choices: vec!["".to_string(), "很長的選項標題，包含中文、English和123數字".to_string(), "特殊符號!@#$%^&*()".to_string()],
            disabled_by_countdown: vec![false, false, false],
            chapter_title: "標題測試".to_string(),
        };
        
        let html = render_story_content_ui(props);
        
        assert!(html.contains("很長的選項標題，包含中文、English和123數字"), "應正確顯示長標題");
        // 特殊字符可能會被轉義，所以我們檢查轉義後的版本
        assert!(html.contains("特殊符號") || html.contains("!@#$%^&amp;*()") || html.contains("!@#$%^&*()"), "應正確顯示特殊字符");
        assert!(html.contains("mr-2"), "選項標題應有右邊距");
    }
}

#[cfg(test)]
mod responsive_design_tests {
    use super::*;

    #[test]
    fn test_responsive_classes() {
        let props = StoryContentUIProps {
            paragraph: "響應式設計測試".to_string(),
            choices: vec![create_test_choice("測試選項", "test", "goto")],
            enabled_choices: vec!["測試選項".to_string()],
            disabled_by_countdown: vec![false],
            chapter_title: "響應式章節".to_string(),
        };
        
        let html = render_story_content_ui(props);
        
        // 檢查響應式文字大小
        assert!(html.contains("text-3xl"), "應有基本文字大小");
        assert!(html.contains("md:text-4xl"), "應有中等螢幕文字大小");
        
        // 檢查響應式排版
        assert!(html.contains("prose-sm"), "應有小尺寸排版");
        assert!(html.contains("lg:prose-base"), "應有大尺寸排版");
        
        // 檢查響應式寬度
        assert!(html.contains("w-full"), "應有全寬度");
        assert!(html.contains("md:w-fit"), "應有中等螢幕適應寬度");
        
        // 檢查最大寬度
        assert!(html.contains("max-w-3xl"), "應有最大寬度限制");
    }

    #[test]
    fn test_dark_mode_classes() {
        let props = StoryContentUIProps {
            paragraph: "深色模式測試".to_string(),
            choices: vec![create_test_choice("深色選項", "dark_choice", "goto")],
            enabled_choices: vec!["深色選項".to_string()],
            disabled_by_countdown: vec![false],
            chapter_title: "深色模式".to_string(),
        };
        
        let html = render_story_content_ui(props);
        
        // 檢查深色模式文字顏色
        assert!(html.contains("dark:text-white"), "應有深色模式白色文字");
        assert!(html.contains("dark:prose-invert"), "應有深色模式反轉排版");
        assert!(html.contains("dark:bg-transparent"), "應有深色模式透明背景");
        
        // 檢查深色模式懸停效果
        assert!(html.contains("dark:hover:text-gray-300"), "應有深色模式懸停效果");
    }

    #[test]
    fn test_layout_spacing() {
        let props = StoryContentUIProps {
            paragraph: "版面間距測試\n\n第二段\n\n第三段".to_string(),
            choices: vec![
                create_test_choice("選項1", "c1", "goto"),
                create_test_choice("選項2", "c2", "goto"),
            ],
            enabled_choices: vec!["選項1".to_string(), "選項2".to_string()],
            disabled_by_countdown: vec![false, false],
            chapter_title: "間距測試".to_string(),
        };
        
        let html = render_story_content_ui(props);
        
        // 檢查間距類別
        assert!(html.contains("space-y-8"), "段落間應有垂直間距");
        assert!(html.contains("mt-10"), "選項列表應有上邊距");
        assert!(html.contains("p-4"), "選項應有內邊距");
        assert!(html.contains("p-8"), "文章應有內邊距");
        
        // 檢查縮排
        assert!(html.contains("indent-10"), "段落應有縮排");
    }
}

#[cfg(test)]
mod accessibility_tests {
    use super::*;

    #[test]
    fn test_list_semantics() {
        let choices = vec![
            create_test_choice("第一選項", "first", "goto"),
            create_test_choice("第二選項", "second", "goto"),
            create_test_choice("第三選項", "third", "goto"),
        ];
        let props = StoryContentUIProps {
            paragraph: "無障礙測試".to_string(),
            choices: choices.clone(),
            enabled_choices: vec!["第一選項".to_string(), "第二選項".to_string(), "第三選項".to_string()],
            disabled_by_countdown: vec![false, false, false],
            chapter_title: "無障礙".to_string(),
        };
        
        let html = render_story_content_ui(props);
        
        // 檢查語義化標籤
        assert!(html.contains("<ol"), "選項應使用有序列表");
        assert!(html.contains("<li"), "選項項目應使用列表項");
        assert!(html.contains("list-decimal"), "列表應有數字標記");
        assert!(html.contains("<article"), "內容應使用文章標籤");
    }

    #[test]
    fn test_focus_and_interaction_states() {
        let choices = vec![create_test_choice("可聚焦選項", "focusable", "goto")];
        let props = StoryContentUIProps {
            paragraph: "互動測試".to_string(),
            choices: choices.clone(),
            enabled_choices: vec!["可聚焦選項".to_string()],
            disabled_by_countdown: vec![false],
            chapter_title: "互動性".to_string(),
        };
        
        let html = render_story_content_ui(props);
        
        // 檢查互動狀態
        assert!(html.contains("cursor-pointer"), "啟用選項應可點擊");
        assert!(html.contains("transition"), "應有過渡效果");
        assert!(html.contains("duration-200"), "應有過渡持續時間");
        
        // 檢查懸停效果
        assert!(html.contains("hover:text-gray-700"), "應有懸停文字顏色變化");
    }

    #[test]
    fn test_disabled_state_accessibility() {
        let choices = vec![create_test_choice("禁用選項", "disabled", "goto")];
        let props = StoryContentUIProps {
            paragraph: "禁用狀態測試".to_string(),
            choices: choices.clone(),
            enabled_choices: vec![], // 不在啟用列表中
            disabled_by_countdown: vec![false],
            chapter_title: "禁用測試".to_string(),
        };
        
        let html = render_story_content_ui(props);
        
        // 檢查禁用狀態
        assert!(html.contains("opacity-50"), "禁用選項應有透明度");
        assert!(html.contains("cursor-not-allowed"), "禁用選項應有禁用游標");
        assert!(html.contains("text-gray-400"), "禁用選項應有淺色文字");
    }
}

#[cfg(test)]
mod edge_case_tests {
    use super::*;

    #[test]
    fn test_empty_paragraph_with_choices() {
        let choices = vec![create_test_choice("唯一選項", "only_choice", "goto")];
        let props = StoryContentUIProps {
            paragraph: "".to_string(), // 空段落
            choices: choices.clone(),
            enabled_choices: vec!["唯一選項".to_string()],
            disabled_by_countdown: vec![false],
            chapter_title: "空段落測試".to_string(),
        };
        
        let html = render_story_content_ui(props);
        assert!(html.contains("唯一選項"), "即使段落為空也應顯示選項");
        assert!(html.contains("空段落測試"), "應顯示章節標題");
    }

    #[test]
    fn test_very_long_content() {
        let long_paragraph = "這是一個很長的段落，".repeat(100);
        let long_title = "這是一個很長的標題，".repeat(20);
        let long_choice = "這是一個很長的選項，".repeat(15);
        
        let choices = vec![create_test_choice(&long_choice, "long_choice", "goto")];
        let props = StoryContentUIProps {
            paragraph: long_paragraph.clone(),
            choices: choices.clone(),
            enabled_choices: vec![long_choice.clone()],
            disabled_by_countdown: vec![false],
            chapter_title: long_title.clone(),
        };
        
        let html = render_story_content_ui(props);
        assert!(html.contains("這是一個很長的段落，"), "應包含長段落的開頭");
        assert!(html.contains("這是一個很長的標題，"), "應包含長標題的開頭");
        assert!(html.contains("這是一個很長的選項，"), "應包含長選項的開頭");
    }

    #[test]
    fn test_special_characters_in_content() {
        let special_paragraph = "包含特殊字符：\n\n\"引號\"、'單引號'、<標籤>、&符號、換行\n測試";
        let special_title = "特殊字符標題：<>&\"'";
        let special_choice = "特殊選項：<script>alert('test')</script>";
        
        let choices = vec![create_test_choice(&special_choice, "special", "goto")];
        let props = StoryContentUIProps {
            paragraph: special_paragraph.to_string(),
            choices: choices.clone(),
            enabled_choices: vec![special_choice.to_string()],
            disabled_by_countdown: vec![false],
            chapter_title: special_title.to_string(),
        };
        
        let html = render_story_content_ui(props);
        
        // HTML 應該正確轉義特殊字符
        assert!(!html.contains("<script>"), "不應包含未轉義的腳本標籤");
        assert!(html.contains("&lt;"), "應正確轉義小於號");
        assert!(html.contains("&gt;"), "應正確轉義大於號");
    }

    #[test]
    fn test_unicode_and_emoji_content() {
        let unicode_paragraph = "包含各種文字：中文、English、日本語、한국어、العربية\n\n還有表情符號：😀🎮🌟⭐💫";
        let emoji_title = "🎯 Unicode 測試 🚀";
        let emoji_choice = "🎪 選擇這個 🎨";
        
        let choices = vec![create_test_choice(&emoji_choice, "unicode_choice", "goto")];
        let props = StoryContentUIProps {
            paragraph: unicode_paragraph.to_string(),
            choices: choices.clone(),
            enabled_choices: vec![emoji_choice.to_string()],
            disabled_by_countdown: vec![false],
            chapter_title: emoji_title.to_string(),
        };
        
        let html = render_story_content_ui(props);
        assert!(html.contains("😀"), "應正確顯示表情符號");
        assert!(html.contains("🎯"), "標題應包含表情符號");
        assert!(html.contains("🎪"), "選項應包含表情符號");
        assert!(html.contains("日本語"), "應正確顯示日文");
        assert!(html.contains("العربية"), "應正確顯示阿拉伯文");
    }

    #[test]
    fn test_mismatched_arrays() {
        // 測試陣列長度不一致的情況
        let choices = vec![
            create_test_choice("選項1", "c1", "goto"),
            create_test_choice("選項2", "c2", "goto"),
            create_test_choice("選項3", "c3", "goto"),
        ];
        let props = StoryContentUIProps {
            paragraph: "陣列不匹配測試".to_string(),
            choices: choices.clone(),
            enabled_choices: vec!["選項1".to_string()], // 只有一個啟用，使用 caption
            disabled_by_countdown: vec![false, true], // 只有兩個狀態
            chapter_title: "不匹配測試".to_string(),
        };
        
        let html = render_story_content_ui(props);
        // 應該能夠安全處理，不會崩潰
        assert!(html.contains("選項1"), "應顯示第一個選項");
        assert!(html.contains("選項2"), "應顯示第二個選項");
        assert!(html.contains("選項3"), "應顯示第三個選項");
    }
}

#[cfg(test)]
mod integration_style_tests {
    use super::*;

    #[test]
    fn test_complete_story_ui_structure() {
        let choices = vec![
            create_test_choice("繼續冒險", "continue", "goto"),
            create_test_choice("返回村莊", "return", "goto"),
            create_test_choice("查看背包", "inventory", "goto"),
        ];
        
        let props = StoryContentUIProps {
            paragraph: "你站在十字路口前，夕陽西下。\n\n遠方傳來狼嚎聲，你必須做出選擇。\n\n時間不多了。".to_string(),
            choices: choices.clone(),
            enabled_choices: vec!["繼續冒險".to_string(), "返回村莊".to_string()],
            disabled_by_countdown: vec![false, false, true], // 背包被禁用
            chapter_title: "第三章：命運的十字路口".to_string(),
        };
        
        let html = render_story_content_ui(props);
        
        // 驗證完整結構
        assert!(html.contains("第三章：命運的十字路口"), "應顯示章節標題");
        assert!(html.contains("你站在十字路口前"), "應顯示故事內容");
        assert!(html.contains("繼續冒險"), "應顯示選項");
        assert!(html.contains("返回村莊"), "應顯示選項");
        assert!(html.contains("查看背包"), "應顯示選項");
        
        // 驗證樣式結構
        assert!(html.contains("min-h-[calc(100vh-56px)]"), "應有正確的標題容器高度");
        assert!(html.contains("prose-sm dark:prose-invert lg:prose-base"), "應有正確的文章樣式");
        assert!(html.contains("whitespace-pre-wrap space-y-8"), "應有正確的段落格式");
        assert!(html.contains("list-decimal"), "應有正確的列表樣式");
        
        // 驗證互動狀態
        let enabled_count = html.matches("cursor-pointer").count();
        let disabled_count = html.matches("opacity-50").count();
        assert!(enabled_count >= 2, "應有2個以上啟用選項");
        assert!(disabled_count >= 1, "應有1個以上禁用選項");
    }

    #[test]
    fn test_component_css_classes_completeness() {
        let props = StoryContentUIProps {
            paragraph: "CSS 類別完整性測試".to_string(),
            choices: vec![create_test_choice("測試選項", "test", "goto")],
            enabled_choices: vec!["測試選項".to_string()],
            disabled_by_countdown: vec![false],
            chapter_title: "樣式測試".to_string(),
        };
        
        let html = render_story_content_ui(props);
        
        // 檢查所有必要的 CSS 類別
        let expected_classes = vec![
            "w-full", "flex", "items-center", "justify-center",
            "text-3xl", "md:text-4xl", "text-gray-900", "dark:text-white",
            "prose-sm", "dark:prose-invert", "lg:prose-base", "mx-auto",
            "max-w-3xl", "p-8", "bg-white", "dark:bg-transparent",
            "whitespace-pre-wrap", "space-y-8", "indent-10", "tracking-wide",
            "leading-relaxed", "text-justify", "mt-10", "md:w-fit",
            "list-decimal", "rounded-lg", "transition", "duration-200",
            "relative", "cursor-pointer", "mr-2",
        ];
        
        for class in expected_classes {
            assert!(html.contains(class), "應包含 CSS 類別: {}", class);
        }
    }
}

#[cfg(test)]
mod performance_tests {
    use super::*;

#[test]
    fn test_large_choice_list_rendering() {
        // 測試大量選項的渲染性能
        let mut choices = Vec::new();
        let mut enabled_choices = Vec::new();
        let mut disabled_by_countdown = Vec::new();
        
        for i in 1..=50 {
            let caption = format!("選項 {}", i);
            choices.push(create_test_choice(&caption, &format!("choice_{}", i), "goto"));
            enabled_choices.push(caption);
            disabled_by_countdown.push(i % 3 == 0); // 每第三個被禁用
        }
        
        let props = StoryContentUIProps {
            paragraph: "大量選項測試".to_string(),
            choices,
            enabled_choices,
            disabled_by_countdown,
            chapter_title: "性能測試".to_string(),
        };
        
        let start = std::time::Instant::now();
        let html = render_story_content_ui(props);
        let duration = start.elapsed();
        
        // 驗證渲染成功且在合理時間內完成
        assert!(html.contains("選項 1"), "應包含第一個選項");
        assert!(html.contains("選項 50"), "應包含最後一個選項");
        assert!(duration.as_millis() < 1000, "渲染時間應少於1秒，實際：{:?}", duration);
    }

    #[test]
    fn test_complex_paragraph_structure() {
        // 測試複雜段落結構的渲染
        let mut complex_paragraph = String::new();
        for i in 1..=20 {
            complex_paragraph.push_str(&format!("這是第{}段，包含一些內容。", i));
            if i < 20 {
                complex_paragraph.push_str("\n\n");
            }
        }
        
        let props = StoryContentUIProps {
            paragraph: complex_paragraph,
            choices: vec![create_test_choice("完成", "finish", "goto")],
            enabled_choices: vec!["完成".to_string()],
            disabled_by_countdown: vec![false],
            chapter_title: "複雜結構測試".to_string(),
        };
        
        let start = std::time::Instant::now();
        let html = render_story_content_ui(props);
        let duration = start.elapsed();
        
        assert!(html.contains("這是第1段"), "應包含第一段");
        assert!(html.contains("這是第20段"), "應包含最後一段");
        assert!(duration.as_millis() < 500, "複雜段落渲染時間應少於500ms，實際：{:?}", duration);
        
        // 驗證段落數量
        let paragraph_count = html.matches("<p").count();
        assert!(paragraph_count >= 20, "應至少有20個段落標籤");
    }
}

#[cfg(test)]
mod regression_tests {
    use super::*;

    #[test]
    fn test_caption_vs_id_display_bug() {
        // 回歸測試：確保顯示的是 caption 而不是 action.to
        let choices = vec![
            Choice {
                caption: "友好的問候".to_string(),
                action: Action {
                    type_: "goto".to_string(),
                    key: None,
                    value: None,
                    to: "unfriendly_id_12345".to_string(),
                },
            },
        ];
        
        let props = StoryContentUIProps {
            paragraph: "測試標題顯示".to_string(),
            choices: choices.clone(),
            enabled_choices: vec!["友好的問候".to_string()],
            disabled_by_countdown: vec![false],
            chapter_title: "顯示測試".to_string(),
        };
        
        let html = render_story_content_ui(props);
        assert!(html.contains("友好的問候"), "應顯示友好的標題");
        assert!(!html.contains("unfriendly_id_12345"), "不應顯示內部ID");
    }

    #[test]
    fn test_enabled_choices_matching_logic() {
        // 回歸測試：確保啟用狀態的匹配邏輯正確
        let choices = vec![
            create_test_choice("選項A", "choice_a", "goto"),
            create_test_choice("選項B", "choice_b", "goto"),
        ];
        
        let props = StoryContentUIProps {
            paragraph: "匹配邏輯測試".to_string(),
            choices: choices.clone(),
            enabled_choices: vec!["選項A".to_string()],
            disabled_by_countdown: vec![false, false],
            chapter_title: "邏輯測試".to_string(),
        };
        
        let html = render_story_content_ui(props);
        
        // choice_a 應該啟用 (有 cursor-pointer)
        // choice_b 應該禁用 (有 opacity-50)
        assert!(html.contains("cursor-pointer"), "應有啟用的選項");
        assert!(html.contains("opacity-50"), "應有禁用的選項");
}

#[test]
    fn test_countdown_disabled_priority() {
        // 回歸測試：倒數禁用應該覆蓋啟用狀態
        let choices = vec![create_test_choice("倒數選項", "countdown_choice", "goto")];

    let props = StoryContentUIProps {
            paragraph: "倒數優先級測試".to_string(),
            choices: choices.clone(),
            enabled_choices: vec!["倒數選項".to_string()],
            disabled_by_countdown: vec![true],
            chapter_title: "優先級測試".to_string(),
        };
        
        let html = render_story_content_ui(props);
        
        // 應該顯示為禁用狀態，即使在啟用列表中
        assert!(html.contains("opacity-50"), "倒數禁用應該覆蓋啟用狀態");
        assert!(html.contains("cursor-not-allowed"), "應顯示不可點擊狀態");
    }
} 