# 測試架構說明

本專案現在支援直接使用主程式程式碼進行測試，無需重複實作邏輯。

## 測試結構

### 1. 單元測試（Unit Tests）
位於 `src/` 目錄中的各個模組內：
- `src/pages/story_tests.rs` - 故事頁面相關測試
- `src/components/story_content_tests.rs` - 故事內容組件測試

### 2. 整合測試（Integration Tests）
位於 `tests/` 目錄：
- `tests/integration_tests.rs` - 基本整合測試
- `tests/story_flow_tests.rs` - 故事流程測試
- `tests/main_code_usage_example.rs` - 主程式程式碼使用範例

### 3. 測試輔助工具
- `tests/common/mod.rs` - 提供測試輔助函數

## 如何直接使用主程式程式碼

### 1. 引入主程式模組
```rust
use ifecaro::*;  // 引入所有公開的模組
```

### 2. 使用特定的組件或函數
```rust
// 使用主程式的組件
use ifecaro::components::story_content::{StoryContentUI, StoryContentUIProps};

// 使用主程式的業務邏輯
use ifecaro::pages::story::merge_paragraphs_for_lang;

// 使用主程式的 Context
use ifecaro::contexts::settings_context::SettingsContext;

// 使用主程式的路由
use ifecaro::enums::route::Route;
```

### 3. 使用測試輔助函數
```rust
mod common;
use common::*;

// 建立測試用的段落
let paragraph = create_test_paragraph("id", "chapter", "zh", "內容");

// 建立測試用的選擇
let choice = create_test_choice("選項標題", "target_id");

// 渲染組件為 HTML
let html = render_component_to_html(MyComponent, props);

// 檢查 HTML 內容
assert_html_contains_text(&html, "預期文字");
assert_html_contains_class(&html, "css-class");
```

## 測試範例

### 測試主程式的業務邏輯
```rust
#[test]
fn test_main_business_logic() {
    use ifecaro::pages::story::merge_paragraphs_for_lang;
    
    let paragraphs = vec![
        create_test_paragraph("p1", "c1", "zh", "段落1"),
        create_test_paragraph("p2", "c1", "zh", "段落2"),
    ];
    
    let result = merge_paragraphs_for_lang(&paragraphs, "zh", false, false, &[]);
    assert_eq!(result, "段落1\n\n段落2");
}
```

### 測試主程式的 UI 組件
```rust
#[test]
fn test_main_ui_component() {
    use ifecaro::components::story_content::{StoryContentUI, StoryContentUIProps};
    
    let props = StoryContentUIProps {
        paragraph: "測試段落".to_string(),
        choices: vec![],
        enabled_choices: vec![],
        disabled_by_countdown: vec![],
        chapter_title: "測試章節".to_string(),
    };
    
    let html = render_component_to_html(StoryContentUI, props);
    assert_html_contains_text(&html, "測試段落");
}
```

### 測試主程式的 Context
```rust
#[test]
fn test_main_context() {
    use ifecaro::contexts::settings_context::SettingsContext;
    use ifecaro::layout::KeyboardState;
    
    let settings = SettingsContext::default();
    let keyboard_state = KeyboardState::default();
    
    assert_eq!(keyboard_state.selected_index, 0);
}
```

## 執行測試

```bash
# 執行所有測試
docker compose exec app cargo test

# 執行特定測試檔案
docker compose exec app cargo test integration_tests
docker compose exec app cargo test story_flow_tests
docker compose exec app cargo test main_code_usage_example

# 執行特定測試函數
docker compose exec app cargo test test_using_main_business_logic
```

## 優點

1. **無需重複實作**：直接使用主程式的程式碼，確保測試的是實際運行的邏輯
2. **保持同步**：主程式程式碼更新時，測試自動使用最新版本
3. **完整覆蓋**：可以測試所有公開的函數、組件和 Context
4. **真實環境**：測試環境更接近實際運行環境
5. **易於維護**：減少測試程式碼的維護負擔

## 注意事項

1. **WASM 限制**：某些需要瀏覽器環境的功能（如 `window` 物件）在測試環境中無法使用
2. **Context 依賴**：某些組件需要特定的 Context，在測試中可能需要模擬
3. **非同步操作**：涉及網路請求或非同步操作的測試需要特別處理 