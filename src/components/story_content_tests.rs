use dioxus::prelude::*;
use dioxus_ssr::render;
use crate::components::story_content::{StoryContent, StoryContentProps, StoryContentUI, StoryContentUIProps, Choice, Action};
use dioxus_core::NoOpMutations;
use dioxus::prelude::VirtualDom;
use dioxus::prelude::Signal;
use dioxus::prelude::EventHandler;

#[test]
fn test_story_content_ui_enabled_choices_by_id() {
    let choices = vec![
        Choice { caption: "選項一".to_string(), action: Action { type_: "goto".to_string(), key: None, value: None, to: "id1".to_string() } },
        Choice { caption: "選項二".to_string(), action: Action { type_: "goto".to_string(), key: None, value: None, to: "id2".to_string() } },
        Choice { caption: "選項三".to_string(), action: Action { type_: "goto".to_string(), key: None, value: None, to: "id3".to_string() } },
    ];
    let enabled_choices = vec!["id1".to_string(), "id3".to_string()];
    let disabled_by_countdown = vec![false, false, false];
    let props = StoryContentUIProps {
        paragraph: "這是段落內容".to_string(),
        choices: choices.clone(),
        enabled_choices: enabled_choices.clone(),
        disabled_by_countdown: disabled_by_countdown.clone(),
        chapter_title: "第一章".to_string(),
    };
    // 渲染 StoryContentUI
    let mut vdom = VirtualDom::new_with_props(StoryContentUI, props);
    let mut mutations = NoOpMutations;
    vdom.rebuild(&mut mutations);
    let html = render(&vdom);
    println!("{html}");
    // 驗證啟用/禁用狀態
    // id1, id3 應啟用，id2 應禁用
    assert!(html.contains("選項一"));
    assert!(html.contains("選項二"));
    assert!(html.contains("選項三"));
    // 啟用選項應該有 cursor-pointer，禁用選項應該有 opacity-50
    assert!(html.contains("選項一"), "選項一應出現在 HTML");
    assert!(html.contains("選項三"), "選項三應出現在 HTML");
    assert!(html.contains("選項二"), "選項二應出現在 HTML");
    // 禁用 class
    assert!(html.contains("opacity-50"), "禁用選項應有 opacity-50 class");
}

#[test]
fn test_story_content_ui_display_caption() {
    let choices = vec![
        Choice { caption: "標題一".to_string(), action: Action { type_: "goto".to_string(), key: None, value: None, to: "id1".to_string() } },
        Choice { caption: "標題二".to_string(), action: Action { type_: "goto".to_string(), key: None, value: None, to: "id2".to_string() } },
        Choice { caption: "標題三".to_string(), action: Action { type_: "goto".to_string(), key: None, value: None, to: "id3".to_string() } },
    ];
    let enabled_choices = vec!["id1".to_string(), "id2".to_string(), "id3".to_string()];
    let disabled_by_countdown = vec![false, false, false];
    let props = StoryContentUIProps {
        paragraph: "這是段落內容".to_string(),
        choices: choices.clone(),
        enabled_choices: enabled_choices.clone(),
        disabled_by_countdown: disabled_by_countdown.clone(),
        chapter_title: "第一章".to_string(),
    };
    let mut vdom = VirtualDom::new_with_props(StoryContentUI, props);
    let mut mutations = NoOpMutations;
    vdom.rebuild(&mut mutations);
    let html = render(&vdom);
    println!("{html}");
    // 驗證每個選項都正確顯示 caption（標題），而不是 id
    assert!(html.contains("標題一"), "應顯示標題一");
    assert!(html.contains("標題二"), "應顯示標題二");
    assert!(html.contains("標題三"), "應顯示標題三");
    assert!(!html.contains(">id1<"), "不應直接顯示 id1");
    assert!(!html.contains(">id2<"), "不應直接顯示 id2");
    assert!(!html.contains(">id3<"), "不應直接顯示 id3");
}

#[test]
fn test_chapter_title_renders_at_top() {
    fn app() -> dioxus::prelude::Element {
        let chapter_title = "章節標題測試";
        rsx! {
            div {
                class: "w-full flex items-center justify-center min-h-[calc(100vh-56px)]",
                div {
                    class: "text-3xl md:text-4xl font-extrabold text-gray-900 dark:text-white text-center w-full select-none flex items-center justify-center",
                    style: "letter-spacing: 0.1em;",
                    {chapter_title}
                }
            }
        }
    }
    let mut vdom = dioxus::prelude::VirtualDom::new(app);
    let mut mutations = dioxus_core::NoOpMutations;
    vdom.rebuild(&mut mutations);
    let html = dioxus_ssr::render(&vdom);
    println!("{html}");
    assert!(html.contains("章節標題測試"), "HTML 應包含章節標題");
    assert!(html.contains("min-h-[calc(100vh-56px)]"), "HTML 應包含正確的高度 class");
    assert!(html.contains("text-3xl"), "HTML 應包含標題字體 class");
}

#[test]
fn test_chapter_title_with_ui() {
    use dioxus::prelude::*;
    use dioxus_ssr::render;
    use crate::components::story_content::{StoryContentUI, StoryContentUIProps};
    use dioxus_core::NoOpMutations;

    let props = StoryContentUIProps {
        paragraph: "這是段落內容".to_string(),
        choices: vec![],
        enabled_choices: vec![],
        disabled_by_countdown: vec![],
        chapter_title: "章節標題測試UI".to_string(),
    };
    let mut vdom = VirtualDom::new_with_props(StoryContentUI, props);
    let mut mutations = NoOpMutations;
    vdom.rebuild(&mut mutations);
    let html = render(&vdom);
    println!("{html}");
    assert!(html.contains("章節標題測試UI"), "HTML 應包含章節標題");
    assert!(html.contains("min-h-[calc(100vh-56px)]"), "HTML 應包含正確的高度 class");
    assert!(html.contains("text-3xl"), "HTML 應包含標題字體 class");
} 