use ifecaro::components::story_content::should_lock_page_scroll;

#[test]
fn test_should_lock_page_scroll_basic() {
    // When overlay (show_filter) is visible, we should lock page scroll.
    assert!(should_lock_page_scroll(true));

    // When overlay is hidden, we should not lock page scroll.
    assert!(!should_lock_page_scroll(false));
} 