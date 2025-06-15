use crate::components::story_content::should_show_filter_on_blur;

#[cfg(test)]
mod show_filter_blur_tests {
    use super::*;

    #[test]
    fn desktop_blur_shows_overlay() {
        assert!(should_show_filter_on_blur(false));
    }

    #[test]
    fn mobile_blur_does_not_show_overlay() {
        assert!(!should_show_filter_on_blur(true));
    }
} 