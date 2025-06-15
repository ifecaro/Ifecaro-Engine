use crate::components::story_content::should_lock_page_scroll;

#[cfg(test)]
mod scroll_lock_tests {
    use super::*;

    #[test]
    fn overlay_shown_disables_scroll() {
        assert!(should_lock_page_scroll(true));
    }

    #[test]
    fn overlay_hidden_enables_scroll() {
        assert!(!should_lock_page_scroll(false));
    }
} 