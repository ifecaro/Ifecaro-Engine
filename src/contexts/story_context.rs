use dioxus::prelude::*;
use crate::components::story_content::Choice;
use crate::pages::story::Paragraph;

#[allow(dead_code)]
pub struct StoryContext {
    pub current_choices: Vec<Choice>,
    pub target_paragraph_id: Option<String>,
    pub countdowns: Signal<Vec<u32>>,
    pub paragraphs: Signal<Vec<Paragraph>>,
}

impl StoryContext {
    pub fn new() -> Self {
        Self {
            current_choices: Vec::new(),
            target_paragraph_id: None,
            countdowns: Signal::new(Vec::new()),
            paragraphs: Signal::new(Vec::new()),
        }
    }
}

pub fn use_story_context() -> Signal<StoryContext> {
    use_context::<Signal<StoryContext>>()
}

pub fn provide_story_context() {
    provide_context(Signal::new(StoryContext::new()));
} 