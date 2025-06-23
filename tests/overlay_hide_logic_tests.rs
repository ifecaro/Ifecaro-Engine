use ifecaro::components::story_content::should_show_choices_on_overlay_hide;

#[test]
fn overlay_hide_logic_respects_non_scrollable_tolerance() {
    // Case 1: Overlay still visible — should never show
    assert!(!should_show_choices_on_overlay_hide(true, false, true, 400, 400));

    // Case 2: Already shown once — should always show regardless of heights
    assert!(should_show_choices_on_overlay_hide(false, true, true, 600, 800));

    // Case 3: Not scrollable (diff ≤ 10) — should show
    let not_scrollable = should_show_choices_on_overlay_hide(false, false, true, 505, 500);
    assert!(not_scrollable);

    // Case 4: Scrollable (scroll_height > client_height) — should NOT show
    let scrollable_case = should_show_choices_on_overlay_hide(false, false, true, 705, 500);
    assert!(!scrollable_case);
} 