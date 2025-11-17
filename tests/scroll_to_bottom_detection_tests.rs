use ifecaro::components::story_content::is_scrolled_to_bottom;

#[test]
fn is_scrolled_to_bottom_desktop() {
    // Desktop tolerance is 10px.
    // diff = 6px  → should be considered bottom.
    assert!(is_scrolled_to_bottom(1000, 600, 394, false));

    // diff = 20px → should NOT be considered bottom.
    assert!(!is_scrolled_to_bottom(1000, 600, 380, false));
}

#[test]
fn is_scrolled_to_bottom_mobile() {
    // Mobile tolerance is 100px.
    // Simulate Firefox for Android with bottom navigation bar (~90px overlay).
    // diff = 90px → should be considered bottom.
    assert!(is_scrolled_to_bottom(2000, 800, 1110, true));

    // diff = 150px → should NOT be considered bottom.
    assert!(!is_scrolled_to_bottom(2000, 800, 1050, true));
}
