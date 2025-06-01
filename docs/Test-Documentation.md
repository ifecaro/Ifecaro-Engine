# Ifecaro Engine Test Documentation

## Overview

This document provides a comprehensive description of the Ifecaro Engine test suite. The test suite consists of four main test files covering everything from basic UI components to complex API integrations.

## Test Architecture

### Test File Structure
```
src/
├── components/
│   ├── story_content_tests.rs              # Basic UI Tests (27 tests)
│   ├── story_content_advanced_tests.rs     # Advanced Feature Tests (19 tests)
│   └── story_content_api_integration_tests.rs # API Integration Tests (6 tests)
└── services/
    └── api_tests.rs                         # API Mock Tests (10 tests)
```

### Execution Methods
- **Complete Test Suite**: `./test-all.sh`
- **Quick Test**: `./test-quick.sh`
- **Compilation Check**: `docker compose exec app cargo check`

### Quick Test Execution

```bash
# Using Rust test runner (recommended)
cargo run --bin test-runner full      # Complete test suite (all 62 tests)
cargo run --bin test-runner quick     # Quick tests (compile + basic UI + API mock)
cargo run --bin test-runner internal  # Container-optimized testing
cargo run --bin test-runner category compile  # Specific test category

# Using deployment tool
cargo run --bin deploy test           # Run complete test suite
cargo run --bin deploy dev            # Development mode (check + quick test)
```

---

## 1. Basic UI Tests (`story_content_tests.rs`)

### File Overview
- **Test Count**: 27 tests
- **Main Features**: Tests the basic rendering, choice states, responsive design, and accessibility features of the `StoryContentUI` component
- **Helper Functions**: 
  - `create_test_choice()` - Creates basic test choices
  - `create_test_choice_with_value()` - Creates test choices with values
  - `render_story_content_ui()` - Renders component and returns HTML

### Test Modules

#### 1.1 Basic UI Tests (`basic_ui_tests`) - 3 tests

##### `test_empty_story_content`
- **Purpose**: Tests basic rendering with empty content
- **Verification**: Ensures basic CSS classes exist
- **Key Assertions**: Contains `prose-sm` and `list-decimal` styles

##### `test_paragraph_display`
- **Purpose**: Tests display of multi-paragraph content
- **Test Data**: Three Chinese text paragraphs
- **Verification**: Paragraph content correctly displayed, including chapter title and styles

##### `test_chapter_title_styling`
- **Purpose**: Tests chapter title styles and typography
- **Verification**: Title font size, responsive design, letter spacing

#### 1.2 Choice Tests (`choice_tests`) - 6 tests

##### `test_single_choice_enabled`
- **Purpose**: Tests rendering of single enabled choice
- **Verification**: Choice text display, pointer styles, no opacity

##### `test_multiple_choices_mixed_states`
- **Purpose**: Tests mixed enabled/disabled states of multiple choices
- **Test Scenario**: 4 choices, 2 enabled, 2 disabled (1 disabled by countdown)
- **Verification**: Visual feedback for enabled and disabled states

##### `test_choice_with_complex_action`
- **Purpose**: Tests complex action choices (containing key-value data)
- **Test Data**: Setting difficulty, scene jump actions
- **Verification**: Correct handling of complex data structures

##### `test_all_choices_disabled`
- **Purpose**: Tests scenario where all choices are disabled
- **Verification**: Ensures consistency of disabled state

##### `test_countdown_disabled_choices`
- **Purpose**: Tests choices disabled by countdown timer
- **Verification**: Priority and display of countdown disabled state

##### `test_choice_display_format`
- **Purpose**: Tests choice display format and numbering
- **Verification**: Correct rendering of ordered lists

#### 1.3 Responsive Design Tests (`responsive_design_tests`) - 3 tests

##### `test_responsive_classes`
- **Purpose**: Tests responsive CSS classes
- **Verification**: `sm:`, `md:`, `lg:` prefixed styles

##### `test_dark_mode_classes`
- **Purpose**: Tests dark mode support
- **Verification**: `dark:` prefixed style classes

##### `test_layout_spacing`
- **Purpose**: Tests layout spacing and typography
- **Verification**: padding, margin, gap spacing styles

#### 1.4 Accessibility Tests (`accessibility_tests`) - 3 tests

##### `test_list_semantics`
- **Purpose**: Tests semantic list tags
- **Verification**: Correct use of `<ol>` and `<li>` tags

##### `test_focus_and_interaction_states`
- **Purpose**: Tests focus and interaction states
- **Verification**: focus, hover, active state styles

##### `test_disabled_state_accessibility`
- **Purpose**: Tests accessibility support for disabled states
- **Verification**: Visual and interaction feedback for disabled choices

#### 1.5 Edge Case Tests (`edge_case_tests`) - 6 tests

##### `test_empty_paragraph_with_choices`
- **Purpose**: Tests empty paragraph with choices scenario
- **Verification**: Component doesn't crash with empty content

##### `test_very_long_content`
- **Purpose**: Tests handling of very long text content
- **Test Data**: 500+ character long text
- **Verification**: Correct handling and display of long content

##### `test_special_characters_in_content`
- **Purpose**: Tests special character handling
- **Test Data**: HTML tags, quotes, symbols, etc.
- **Verification**: Special characters don't break HTML structure

##### `test_unicode_and_emoji_content`
- **Purpose**: Tests Unicode and emoji support
- **Test Data**: Chinese, Japanese, emojis
- **Verification**: Correct display of multilingual and emoji content

##### `test_mismatched_arrays`
- **Purpose**: Tests array length mismatch scenarios
- **Scenario**: Different lengths of choices and enabled_choices
- **Verification**: Component gracefully handles mismatched arrays

#### 1.6 Integration Style Tests (`integration_style_tests`) - 2 tests

##### `test_complete_story_ui_structure`
- **Purpose**: Tests integration of complete UI structure
- **Verification**: Coordinated work of all component parts

##### `test_component_css_classes_completeness`
- **Purpose**: Tests completeness of CSS classes
- **Verification**: All necessary Tailwind CSS classes exist

#### 1.7 Performance Tests (`performance_tests`) - 2 tests

##### `test_large_choice_list_rendering`
- **Purpose**: Tests rendering performance with large choice lists
- **Test Data**: 50 choices
- **Verification**: Large number of choices doesn't affect rendering

##### `test_complex_paragraph_structure`
- **Purpose**: Tests performance of complex paragraph structures
- **Test Data**: Multi-paragraph + multi-choice combinations
- **Verification**: Correct handling of complex structures

#### 1.8 Regression Tests (`regression_tests`) - 3 tests

##### `test_caption_vs_id_display_bug`
- **Purpose**: Prevents regression of displaying IDs instead of titles
- **Verification**: Ensures choice titles are displayed, not internal IDs

##### `test_enabled_choices_matching_logic`
- **Purpose**: Tests enabled state matching logic
- **Verification**: Correct determination of choice enabled states

##### `test_countdown_disabled_priority`
- **Purpose**: Tests countdown disabled priority
- **Verification**: Countdown disabled state has higher priority than general disabled

---

## 2. Advanced Feature Tests (`story_content_advanced_tests.rs`)

### File Overview
- **Test Count**: 19 tests
- **Main Features**: Tests choice data structures, action type validation, array operations, countdown logic, and other advanced features

### Test Modules

#### 2.1 Choice Data Structure Tests (`choice_data_structure_tests`) - 3 tests

##### `test_choice_creation_and_serialization`
- **Purpose**: Tests choice creation and basic properties
- **Verification**: Correct initialization of choice objects

##### `test_choice_with_complex_action_data`
- **Purpose**: Tests choices containing complex action data
- **Test Data**: Setting actions containing key-value data
- **Verification**: Correct handling of complex data structures

##### `test_choice_serialization_deserialization`
- **Purpose**: Tests JSON serialization and deserialization
- **Verification**: Data integrity and format correctness

#### 2.2 Action Type Validation Tests (`action_type_validation_tests`) - 4 tests

##### `test_goto_action_type`
- **Purpose**: Tests "goto" action type
- **Verification**: Correct handling of jump actions

##### `test_set_action_type`
- **Purpose**: Tests "set" action type
- **Verification**: Set actions require key and value

##### `test_add_action_type`
- **Purpose**: Tests "add" action type
- **Verification**: Numeric handling of add actions

##### `test_custom_action_type`
- **Purpose**: Tests custom action types
- **Verification**: System handling of unknown action types

#### 2.3 Choice Array Operations Tests (`choice_array_operations_tests`) - 4 tests

##### `test_empty_choice_array`
- **Purpose**: Tests empty choice arrays
- **Verification**: Correct handling of empty arrays

##### `test_single_choice_array`
- **Purpose**: Tests single choice arrays
- **Verification**: Correct handling of single choices

##### `test_multiple_choices_array`
- **Purpose**: Tests multiple choice arrays
- **Verification**: Array iteration and property access

##### `test_choice_filtering_by_type`
- **Purpose**: Tests filtering choices by action type
- **Verification**: Correctness of filtering functionality

#### 2.4 Enabled Choice Logic Tests (`enabled_choices_logic_tests`) - 3 tests

##### `test_enabled_choices_matching`
- **Purpose**: Tests matching logic for enabled choices
- **Verification**: Matching choice names with enabled list

##### `test_disabled_by_countdown_logic`
- **Purpose**: Tests countdown timer disabled logic
- **Verification**: Impact of countdown state on choice enabling

##### `test_all_choices_disabled_scenario`
- **Purpose**: Tests scenario where all choices are disabled
- **Verification**: Handling of fully disabled state

#### 2.5 Countdown State Tests (`countdown_state_tests`) - 3 tests

##### `test_countdown_array_initialization`
- **Purpose**: Tests countdown array initialization
- **Verification**: Correct setup of countdown state arrays

##### `test_countdown_time_setting`
- **Purpose**: Tests countdown time setting
- **Verification**: Correct application of time limits

##### `test_countdown_expiration_logic`
- **Purpose**: Tests countdown expiration logic
- **Verification**: State changes when time expires

#### 2.6 Keyboard Input Simulation Tests (`keyboard_input_simulation_tests`) - 3 tests

##### `test_keyboard_number_parsing`
- **Purpose**: Tests parsing of number keys
- **Verification**: Correct parsing of "1", "2", "3" keys

##### `test_keyboard_invalid_keys`
- **Purpose**: Tests invalid keyboard input
- **Verification**: Error handling for non-numeric keys

##### `test_keyboard_choice_index_calculation`
- **Purpose**: Tests index calculation for keyboard choices
- **Verification**: Correspondence between keyboard numbers and choice indices

---

## 3. API Mock Tests (`api_tests.rs`)

### File Overview
- **Test Count**: 10 tests
- **Main Features**: Tests Mock implementation of API client, including success and failure scenarios

### Test Features

#### 3.1 Mock API Basic Function Tests

##### `test_mock_api_get_paragraphs_success`
- **Purpose**: Tests successful retrieval of paragraph data
- **Test Data**: 3 test paragraphs
- **Verification**: Structure and content of returned data

##### `test_mock_api_get_paragraphs_failure`
- **Purpose**: Tests API failure scenarios
- **Verification**: Error types and error messages

##### `test_mock_api_get_chapters_success`
- **Purpose**: Tests successful retrieval of chapter data
- **Test Data**: 3 test chapters
- **Verification**: Chapter titles, order, and other attributes

#### 3.2 Specific API Operation Tests

##### `test_mock_api_get_paragraph_by_id_found`
- **Purpose**: Tests finding paragraph by ID (success)
- **Verification**: Correctly returns paragraph with specified ID

##### `test_mock_api_get_paragraph_by_id_not_found`
- **Purpose**: Tests finding paragraph by ID (failure)
- **Verification**: Correctly returns NotFound error

##### `test_mock_api_update_paragraph_success`
- **Purpose**: Tests successful paragraph update
- **Verification**: Successful execution of update operation

##### `test_mock_api_update_paragraph_failure`
- **Purpose**: Tests paragraph update failure
- **Verification**: Error handling in failure scenarios

#### 3.3 Complex Data Tests

##### `test_complex_paragraph_choice_data`
- **Purpose**: Tests complex choice data structures
- **Test Data**: Actions including settings, purchases, jumps
- **Verification**: Correct handling of complex JSON data

##### `test_multilingual_content`
- **Purpose**: Tests multilingual content support
- **Test Data**: Chinese, English, Japanese content
- **Verification**: Correct access to multilingual data

---

## 4. API Integration Tests (`story_content_api_integration_tests.rs`)

### File Overview
- **Test Count**: 6 tests
- **Main Features**: Tests integration of API data with UI components, simulating real-world usage scenarios

### Test Features

#### 4.1 End-to-End Integration Tests

##### `test_story_content_with_mock_api_data`
- **Purpose**: Tests complete flow from API data to UI component
- **Test Process**:
  1. Create Mock API data
  2. Simulate API calls
  3. Convert data format
  4. Render UI component
  5. Verify HTML output
- **Verification**: Complete data flow and UI rendering

##### `test_error_handling_with_mock_api`
- **Purpose**: Tests API error handling
- **Scenario**: Fallback handling when API calls fail
- **Verification**: Correct display of error messages

#### 4.2 Multilingual Integration Tests

##### `test_multilingual_content_with_mock_api`
- **Purpose**: Tests API integration for multilingual content
- **Test Data**: Chinese, English, Japanese versions
- **Verification**: Correctness of language switching

#### 4.3 Complex Scenario Tests

##### `test_complex_choice_data_with_time_limits`
- **Purpose**: Tests complex choices with time limits
- **Test Data**: Choice combinations with different time limits
- **Verification**: Impact of time limits on UI

##### `test_choice_conversion_edge_cases`
- **Purpose**: Tests edge cases in choice conversion
- **Scenario**: Incomplete or abnormal data formats
- **Verification**: Error handling and fallback mechanisms

---

## Test Coverage Summary

### Functional Coverage
- ✅ **Basic UI Rendering**: Text display, choice lists, chapter titles
- ✅ **Interactive Features**: Choice enable/disable, countdown timers, keyboard navigation
- ✅ **Responsive Design**: Multiple screen sizes, dark mode
- ✅ **Accessibility Features**: Semantic tags, focus management
- ✅ **Data Processing**: JSON serialization, multilingual support
- ✅ **API Integration**: Mock testing, error handling
- ✅ **Edge Cases**: Empty data, extremely long content, special characters
- ✅ **Performance Testing**: Large data rendering
- ✅ **Regression Testing**: Protection against known issues

### Test Type Distribution
- **Unit Tests**: 54 tests (85%)
- **Integration Tests**: 6 tests (9%)
- **End-to-End Tests**: 4 tests (6%)

### Code Quality Assurance
- **Compilation Check**: No warnings, no errors
- **Type Safety**: Guaranteed by Rust type system
- **Memory Safety**: No memory leak risks
- **Concurrency Safety**: Appropriate synchronization mechanisms

---

## Automated Testing Integration

- **Git Pre-commit Hooks**: Automatically execute complete test suite before each commit
- **Development Workflow**: Integrated with Rust deployment CLI
- **Continuous Integration**: All tests must pass before production builds
- **Performance Monitoring**: Test execution time tracking
- **Report Generation**: Detailed test reports with time statistics

---

*This document will be continuously updated as the project develops. For test-related questions, please refer to detailed comments in each test file.* 