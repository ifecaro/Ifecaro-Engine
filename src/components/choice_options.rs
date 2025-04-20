#[component]
pub fn ChoiceOptions(
    available_chapters: Vec<Chapter>,
    selected_language: Language,
    on_target_chapter_change: Action<String, Result<(), ServerFnError>>,
    target_chapter: String,
    // ... existing code ...
) -> impl IntoView {
    // ... existing code ...
    let target_chapter_options = create_memo(move |_| {
        available_chapters
            .iter()
            .filter(|chapter| chapter.language == selected_language)
            .map(|chapter| {
                view! {
                    <option value={chapter.id.clone()}>
                        {chapter.title.clone()}
                    </option>
                }
            })
            .collect::<Vec<_>>()
    });

    view! {
        <div class="choice-options">
            // ... existing code ...
            <div class="target-chapter">
                <label for="target-chapter">"目標章節："</label>
                <select
                    id="target-chapter"
                    on:change=move |ev| {
                        let value = event_target_value(&ev);
                        on_target_chapter_change.dispatch(value);
                    }
                    prop:value=target_chapter
                >
                    <option value="">"選擇目標章節"</option>
                    {target_chapter_options}
                </select>
            </div>
            // ... existing code ...
        </div>
    }
} 