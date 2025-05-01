use dioxus::prelude::*;
use crate::components::story_content::Choice;

#[allow(dead_code)]
pub struct StoryContext {
    pub current_choices: Vec<Choice>,
    pub target_paragraph_id: Option<String>,
}

impl StoryContext {
    pub fn new() -> Self {
        Self {
            current_choices: Vec::new(),
            target_paragraph_id: None,
        }
    }
}

pub fn use_story_context() -> Signal<StoryContext> {
    use_context::<Signal<StoryContext>>()
}

pub fn provide_story_context() {
    provide_context(Signal::new(StoryContext::new()));
} 