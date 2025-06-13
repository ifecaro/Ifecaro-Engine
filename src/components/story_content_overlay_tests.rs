use crate::components::story_content::should_show_choices_on_overlay_hide;

#[cfg(test)]
mod overlay_logic_tests {
    use super::*;

    #[test]
    fn overlay_hide_with_short_content_and_countdown_shows_choices() {
        let show_filter = false;
        let has_shown_choices = false;
        let has_countdown = true;
        let scroll_height = 500; // content height less than viewport height
        let client_height = 800;
        assert!(should_show_choices_on_overlay_hide(
            show_filter,
            has_shown_choices,
            has_countdown,
            scroll_height,
            client_height
        ));
    }

    #[test]
    fn overlay_hide_with_scrollable_content_and_countdown_keeps_choices_hidden() {
        let show_filter = false;
        let has_shown_choices = false;
        let has_countdown = true;
        let scroll_height = 1500; // content height greater than viewport height
        let client_height = 800;
        assert!(!should_show_choices_on_overlay_hide(
            show_filter,
            has_shown_choices,
            has_countdown,
            scroll_height,
            client_height
        ));
    }
} 