use ifecaro::components::story_content::should_hide_choices_after_same_page;

#[test]
fn hide_choices_when_same_page_and_countdown() {
    // same_page = true and countdown exists => should hide
    assert!(should_hide_choices_after_same_page(true, true));
}

#[test]
fn do_not_hide_when_same_page_without_countdown() {
    // same_page = true but no countdown => should not hide
    assert!(!should_hide_choices_after_same_page(true, false));
}

#[test]
fn do_not_hide_when_new_page_even_with_countdown() {
    // same_page = false should not hide regardless of countdown
    assert!(!should_hide_choices_after_same_page(false, true));
    assert!(!should_hide_choices_after_same_page(false, false));
} 