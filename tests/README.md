# Test Architecture Documentation

This project now supports direct use of main program code for testing, and includes a comprehensive Dashboard functionality test suite, totaling 182+ tests.

## Test Overview

### Test Count Distribution
- **Dashboard Tests**: 83 tests (45.6%) - Complete Dashboard functionality testing including UI rendering
- **Basic UI Tests**: 27 tests (14.8%) - Component rendering and interaction testing
- **Advanced Feature Tests**: 28 tests (15.4%) - Complex UI logic and performance testing
- **Story Logic Tests**: 16 tests (8.8%) - Core story processing logic
- **Integration Tests**: 20 tests (11.0%) - End-to-end workflow testing
- **API Integration Tests**: 1 test (0.5%) - API data integration testing
- **Additional Library Tests**: 7 tests (3.8%) - Supporting functionality and utility testing
- **Total**: **182+ tests** providing comprehensive coverage

## Test Structure

### 1. Unit Tests
Located within modules in the `src/` directory:
- `src/pages/story_tests.rs` - Story page related tests (16 tests)
- `src/pages/dashboard_tests.rs` - Dashboard page tests (31 tests)
- `src/components/story_content_tests.rs` - Story content component tests (27 tests)
- `src/components/story_content_advanced_tests.rs` - Advanced feature tests (28 tests)
- `src/components/story_content_api_integration_tests.rs` - API integration tests (1 test)

### 2. Integration Tests
Located in the `tests/` directory:
- `tests/integration_tests.rs` - Core integration tests (4 tests)
- `tests/main_code_usage_example.rs` - Code usage examples (6 tests)
- `tests/story_flow_tests.rs` - Story flow tests (4 tests)
- `tests/dashboard_tests.rs` - Dashboard unit tests (31 tests)
- `tests/dashboard_interaction_tests.rs` - Dashboard interaction tests (17 tests)
- `tests/dashboard_ui_tests.rs` - Dashboard UI rendering tests (24 tests)
- `tests/dashboard_benchmark_tests.rs` - Dashboard performance tests (11 tests)

### 3. Dashboard Test Suite Key Features
#### Dashboard Unit Tests (31 tests)
- Data structure tests: Language state, chapter state, paragraph state
- Form validation tests: Content validation, error handling, real-time validation
- Button state tests: Dynamic submit button enable/disable logic
- Language consistency tests: Language state persistence in edit mode

#### Dashboard Interaction Tests (17 tests)
- Advanced language switching tests: UI/content language independence
- Form submit button tests: 10 test cases covering various validation states
- Real-time form validation tests: Real-time feedback during user input
- **Comprehensive Language Switching Tests**: Language updates for all interface elements
  - Labels: "段落內容" ↔ "Paragraph Content"
  - Buttons: "儲存" ↔ "Save", "取消" ↔ "Cancel"
  - Error messages: "此欄位為必填" ↔ "This field is required"
  - Placeholder text: "請輸入段落內容..." ↔ "Enter paragraph content..."
  - Content text: "這是測試段落一的內容" ↔ "This is test paragraph one"
  - Choice text: "走左邊的路" ↔ "Take the left path"
  - Chapter titles: "第一章節" ↔ "Chapter One"

#### Dashboard UI Tests (24 tests) ✨ **New**
- **UI Rendering Tests**: Basic structure, form layout, responsive design
- **Language Tests**: Multi-language rendering, language switching
- **State Tests**: Edit mode layout, form areas, selector grids
- **Accessibility Tests**: Semantic structure, color contrast, responsive accessibility
- **Error State Tests**: Toast notifications, validation structure
- **Performance Tests**: Render performance, multiple language renders
- **Edge Case Tests**: Empty/invalid languages, special characters

#### Dashboard Performance Tests (11 tests)
- Large dataset tests: 50 chapters, 2000 paragraphs
- Stress tests: Processing large amounts of data with 10,000 paragraphs
- Concurrent operation tests: Rapid language switching, form operations

### 4. Test Helper Tools
- `tests/common/mod.rs` - Provides test helper functions

## How to Use Main Program Code Directly

### 1. Import Main Program Modules
```rust
use ifecaro::*;  // Import all public modules
```

### 2. Use Specific Components or Functions
```rust
// Use main program components
use ifecaro::components::story_content::{StoryContentUI, StoryContentUIProps};

// Use main program business logic
use ifecaro::pages::story::merge_paragraphs_for_lang;

// Use main program Context
use ifecaro::contexts::settings_context::SettingsContext;

// Use main program routing
use ifecaro::enums::route::Route;
```

### 3. Use Test Helper Functions
```rust
mod common;
use common::*;

// Create test paragraphs
let paragraph = create_test_paragraph("id", "chapter", "zh", "Content");

// Create test choices
let choice = create_test_choice("Choice Title", "target_id");

// Render component to HTML
let html = render_component_to_html(MyComponent, props);

// Check HTML content
assert_html_contains_text(&html, "Expected text");
assert_html_contains_class(&html, "css-class");
```

## Test Examples

### Testing Main Program Business Logic
```rust
#[test]
fn test_main_business_logic() {
    use ifecaro::pages::story::merge_paragraphs_for_lang;
    
    let paragraphs = vec![
        create_test_paragraph("p1", "c1", "zh", "Paragraph 1"),
        create_test_paragraph("p2", "c1", "zh", "Paragraph 2"),
    ];
    
    let result = merge_paragraphs_for_lang(&paragraphs, "zh", false, false, &[]);
    assert_eq!(result, "Paragraph 1\n\nParagraph 2");
}
```

### Testing Main Program UI Components
```rust
#[test]
fn test_main_ui_component() {
    use ifecaro::components::story_content::{StoryContentUI, StoryContentUIProps};
    
    let props = StoryContentUIProps {
        paragraph: "Test paragraph".to_string(),
        choices: vec![],
        enabled_choices: vec![],
        disabled_by_countdown: vec![],
        chapter_title: "Test chapter".to_string(),
    };
    
    let html = render_component_to_html(StoryContentUI, props);
    assert_html_contains_text(&html, "Test paragraph");
}
```

### Testing Main Program Context
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

## Running Tests

```bash
# Run all tests
docker compose exec app cargo test

# Run specific test files
docker compose exec app cargo test integration_tests
docker compose exec app cargo test story_flow_tests
docker compose exec app cargo test main_code_usage_example
docker compose exec app cargo test dashboard_tests
docker compose exec app cargo test dashboard_interaction_tests
docker compose exec app cargo test dashboard_benchmark_tests

# Run specific test functions
docker compose exec app cargo test test_using_main_business_logic

# Use test runner (recommended)
docker compose exec app cargo run --bin test-runner full      # Complete test suite (all 182+ tests)
docker compose exec app cargo run --bin test-runner quick     # Quick tests (compile + basic tests)
docker compose exec app cargo run --bin test-runner category dashboard  # Dashboard tests (83 tests)
docker compose exec app cargo run --bin test-runner category ui         # UI tests
docker compose exec app cargo run --bin test-runner category integration # Integration tests
docker compose exec app cargo run --bin test-runner bench     # Performance benchmark tests
```

## Test Coverage

### ✅ Complete Functional Coverage
1. **UI Component Level**: All visual rendering, interactive features, responsive design, accessibility features
2. **Dashboard Management Level**: Content creation, editing, validation, multi-language support, form state management, UI rendering
3. **Business Logic Level**: Core algorithms, data processing, business rules, multi-language processing
4. **API Service Level**: CRUD operations, data transformation, error handling, mock integration
5. **System Integration Level**: Context integration, cross-component flows, complete user journeys

### 🎯 Featured Test Capabilities
- **Real Data Integration**: Direct use of main program data structures and functions
- **Multi-language Testing**: Chinese, English, Japanese content processing validation
- **Dashboard Specialized Testing**: 83 specialized Dashboard functionality tests including UI rendering
- **Performance Benchmark Testing**: Large dataset processing, concurrent operations, stress testing
- **Comprehensive Language Switching**: Independent language control testing for UI interface and content
- **UI Rendering Testing**: Complete Dashboard UI structure, responsive design, accessibility validation

## Advantages

1. **No Duplicate Implementation**: Direct use of main program code ensures testing of actual running logic
2. **Stay Synchronized**: Test automatically uses latest version when main program code updates
3. **Complete Coverage**: Can test all public functions, components, and Contexts
4. **Real Environment**: Test environment closer to actual runtime environment
5. **Easy Maintenance**: Reduces test code maintenance burden

## Notes

1. **WASM Limitations**: Some features requiring browser environment (like `window` object) cannot be used in test environment
2. **Context Dependencies**: Some components require specific Context, may need mocking in tests
3. **Asynchronous Operations**: Tests involving network requests or asynchronous operations need special handling