use crate::components::story_content::Choice;
use crate::pages::story::Paragraph;
use dioxus::prelude::*;

#[derive(Debug)]
#[allow(dead_code)]
pub struct StoryContext {
    pub current_choices: Vec<Choice>,
    pub target_paragraph_id: Option<String>,
    pub countdowns: Signal<Vec<u32>>,
    pub paragraphs: Signal<Vec<Paragraph>>,
    pub chapters: Signal<Vec<crate::pages::story::Chapter>>,
    pub is_settings_chapter: Signal<bool>,
    pub choice_ids: Signal<Vec<String>>,
}

impl StoryContext {
    pub fn new() -> Self {
        Self {
            current_choices: Vec::new(),
            target_paragraph_id: None,
            countdowns: Signal::new(Vec::new()),
            paragraphs: Signal::new(Vec::new()),
            chapters: Signal::new(Vec::new()),
            is_settings_chapter: Signal::new(false),
            choice_ids: Signal::new(Vec::new()),
        }
    }

    #[allow(dead_code)]
    pub fn set_is_settings_chapter(&mut self, value: bool) {
        self.is_settings_chapter.set(value);
    }

    pub fn is_settings_chapter(&self) -> bool {
        *self.is_settings_chapter.read()
    }
}

pub fn use_story_context() -> Signal<StoryContext> {
    use_context::<Signal<StoryContext>>()
}

pub fn provide_story_context() -> Signal<StoryContext> {
    use_context_provider(|| Signal::new(StoryContext::new()))
}
