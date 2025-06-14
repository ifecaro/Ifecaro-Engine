use ifecaro::components::story_content::should_show_choices_on_overlay_hide;

#[test]
fn should_not_show_when_overlay_still_visible() {
    assert_eq!(
        should_show_choices_on_overlay_hide(true, false, false, 1000, 800),
        false
    );
}

#[test]
fn should_show_when_already_shown_once() {
    assert_eq!(
        should_show_choices_on_overlay_hide(false, true, false, 1000, 800),
        true
    );
}

#[test]
fn should_show_when_scroll_not_needed_and_has_countdown() {
    // scroll_height <= client_height indicates not scrollable
    assert_eq!(
        should_show_choices_on_overlay_hide(false, false, true, 800, 800),
        true
    );
}

#[test]
fn should_not_show_in_other_cases() {
    // Overlay hidden, not shown before, no countdown, but scrollable (> client height)
    assert_eq!(
        should_show_choices_on_overlay_hide(false, false, false, 1200, 800),
        false
    );
} 