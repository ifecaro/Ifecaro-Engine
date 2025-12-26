use dioxus::prelude::*;

pub struct StoryMergedContext {
    pub merged_paragraph: Signal<String>,
}

impl StoryMergedContext {
    pub fn new() -> Self {
        Self {
            merged_paragraph: Signal::new(String::new()),
        }
    }
}

pub fn use_story_merged_context() -> Signal<StoryMergedContext> {
    use_context::<Signal<StoryMergedContext>>()
}
