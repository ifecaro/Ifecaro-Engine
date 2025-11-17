use dioxus_core::NoOpMutations;
use ifecaro::*;

#[cfg(test)]
mod story_context_persistence_tests {
    use super::*;
    use dioxus::prelude::*;
    use ifecaro::contexts::story_context::{use_story_context, StoryContext};

    #[component]
    fn TestComponent() -> Element {
        let mut story_ctx = use_story_context();
        // Local signal to ensure the impact runs on every render
        let mut has_set_value = use_signal(|| false);

        use_effect(move || {
            // On the first render, write a value into the context.
            if !*has_set_value.peek() {
                story_ctx.write().target_paragraph_id = Some("test_value".to_string());
                has_set_value.set(true);
            } else {
                // On subsequent renders, verify the value is still present.
                assert_eq!(
                    story_ctx.read().target_paragraph_id,
                    Some("test_value".to_string()),
                    "StoryContext should persist across renders"
                );
            }
            ()
        });

        rsx! {
            div { "Testing StoryContext persistence" }
        }
    }

    #[component]
    fn TestApp() -> Element {
        // Provide a StoryContext that should remain stable for the lifetime of the app.
        use_context_provider(|| Signal::new(StoryContext::new()));

        // Local state to trigger a re-render.
        let mut counter = use_signal(|| 0);
        if *counter.read() == 0 {
            // Trigger a state change to force a second render.
            counter.set(1);
        }

        rsx! { TestComponent {} }
    }

    #[test]
    fn test_story_context_is_persistent() {
        let mut dom = VirtualDom::new(TestApp);
        let mut mutations = NoOpMutations;
        dom.rebuild(&mut mutations); // First build
        dom.rebuild(&mut mutations); // Second build after state change
    }
}
