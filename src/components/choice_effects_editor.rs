use crate::models::effects::{
    apply_effects_preview, AttributeField, CharacterAttributes, Effect, EffectList, NumericOp,
    RelationshipField, RelationshipMetrics,
};
use dioxus::events::FormEvent;
use dioxus::prelude::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CharacterOption {
    pub id: String,
    pub char_id: String,
    pub role: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RelationshipOption {
    pub id: String,
    pub from_id: String,
    pub to_id: String,
}

fn first_character_id(characters: &[CharacterOption]) -> String {
    characters
        .first()
        .map(|c| c.id.clone())
        .unwrap_or_else(|| "".to_string())
}

fn first_relationship(relations: &[RelationshipOption]) -> (String, String) {
    relations
        .first()
        .map(|r| (r.from_id.clone(), r.to_id.clone()))
        .unwrap_or_else(|| (String::new(), String::new()))
}

fn attribute_field_options() -> Vec<(AttributeField, &'static str)> {
    vec![
        (AttributeField::Honesty, "Honesty"),
        (AttributeField::Empathy, "Empathy"),
        (AttributeField::Affability, "Affability"),
        (AttributeField::Intimidation, "Intimidation"),
        (AttributeField::Aggression, "Aggression"),
        (AttributeField::Discipline, "Discipline"),
        (AttributeField::Curiosity, "Curiosity"),
        (AttributeField::Courage, "Courage"),
        (AttributeField::Impulsivity, "Impulsivity"),
        (AttributeField::Idealism, "Idealism"),
        (AttributeField::Pragmatism, "Pragmatism"),
        (AttributeField::Loyalty, "Loyalty"),
        (AttributeField::Opportunism, "Opportunism"),
        (AttributeField::Stoicism, "Stoicism"),
        (AttributeField::Morality, "Morality"),
        (AttributeField::Health, "Health"),
        (AttributeField::Stress, "Stress"),
        (AttributeField::Fatigue, "Fatigue"),
        (AttributeField::Pain, "Pain"),
        (AttributeField::Morale, "Morale"),
        (AttributeField::Intox, "Intoxication"),
    ]
}

fn relationship_field_options() -> Vec<(RelationshipField, &'static str)> {
    vec![
        (RelationshipField::Affinity, "Affinity"),
        (RelationshipField::Trust, "Trust"),
        (RelationshipField::Respect, "Respect"),
        (RelationshipField::Fear, "Fear"),
        (RelationshipField::Attraction, "Attraction"),
    ]
}

fn numeric_op_options() -> Vec<(NumericOp, &'static str)> {
    vec![
        (NumericOp::Add, "Add"),
        (NumericOp::Set, "Set"),
        (NumericOp::Scale, "Scale (%)"),
    ]
}

fn effect_type(effect: &Effect) -> &'static str {
    match effect {
        Effect::CharacterAttribute { .. } => "character_attribute",
        Effect::Relationship { .. } => "relationship",
        Effect::Flag { .. } => "flag",
    }
}

fn update_effect_type(
    effect: &Effect,
    to: &str,
    characters: &[CharacterOption],
    relations: &[RelationshipOption],
) -> Effect {
    match to {
        "relationship" => {
            let (from_id, to_id) = first_relationship(relations);
            Effect::Relationship {
                from_id,
                to_id,
                field: RelationshipField::Trust,
                op: NumericOp::Add,
                value: 0,
            }
        }
        "flag" => Effect::Flag {
            character_id: first_character_id(characters),
            path: vec!["flag_name".to_string()],
            value: serde_json::Value::Bool(true),
        },
        _ => Effect::default_character(first_character_id(characters)),
    }
}

#[component]
pub fn ChoiceEffectsEditor(props: ChoiceEffectsEditorProps) -> Element {
    let initial_effects = props
        .initial_effects_json
        .as_deref()
        .and_then(|raw| EffectList::from_json(raw).ok())
        .unwrap_or_default();

    let mut effects = use_signal(|| initial_effects.0);

    let preview = apply_effects_preview(
        &props.character_attributes,
        &props.relationship_metrics,
        &effects.read(),
    );

    let on_add = {
        let characters = props.characters.clone();
        move |_| {
            effects
                .write()
                .push(Effect::default_character(first_character_id(&characters)));
        }
    };

    let on_save_click = {
        let on_save = props.on_save.clone();
        move |_| {
            on_save.call(effects.read().clone());
        }
    };

    rsx! {
        div { class: "choice-effects-editor space-y-4",
            h3 { class: "text-lg font-semibold", "Choice Effects" }
            button { class: "px-3 py-1 bg-blue-600 text-white rounded", onclick: on_add, "Add effect" }
            div { class: "space-y-3",
                for (index, effect) in effects.read().iter().cloned().enumerate() {
                    { render_effect_row(index, effect, effects.clone(), &props.characters, &props.relationships) }
                }
            }
            div { class: "border-t pt-3 space-y-2",
                h4 { class: "font-semibold", "Preview" }
                for (id, after) in preview.characters.iter() {
                    if let Some(before) = props.character_attributes.get(id) {
                        div { class: "text-sm text-gray-700",
                            strong { "{id}: " }
                            span { "Morale {before.morale} -> {after.morale}, Stress {before.stress} -> {after.stress}, Morality {before.morality} -> {after.morality}" }
                        }
                    }
                }
                for ((from, to), after) in preview.relationships.iter() {
                    if let Some(before) = props.relationship_metrics.get(&(from.clone(), to.clone())) {
                        div { class: "text-sm text-gray-700",
                            strong { "{from} → {to}: " }
                            span { "Trust {before.trust} -> {after.trust}, Affinity {before.affinity} -> {after.affinity}" }
                        }
                    }
                }
            }
            button { class: "px-4 py-2 bg-green-600 text-white rounded", onclick: on_save_click, "Save" }
        }
    }
}

fn render_effect_row(
    index: usize,
    effect: Effect,
    mut effects: Signal<Vec<Effect>>,
    characters: &[CharacterOption],
    relationships: &[RelationshipOption],
) -> Element {
    let effect_type_value = effect_type(&effect).to_string();
    let characters_vec = characters.to_vec();
    let relationships_vec = relationships.to_vec();

    let on_type_change = {
        let characters = characters_vec.clone();
        let relationships = relationships_vec.clone();
        let mut effects = effects.clone();
        move |evt: FormEvent| {
            let mut list = effects.write();
            list[index] =
                update_effect_type(&list[index], &evt.value(), &characters, &relationships);
        }
    };

    let on_remove = {
        let mut effects = effects.clone();
        move |_| {
            effects.write().remove(index);
        }
    };

    match effect {
        Effect::CharacterAttribute {
            character_id,
            field,
            op,
            value,
        } => {
            let characters = characters_vec.clone();
            let character_id_for_select = character_id.clone();
            let character_id_for_field = character_id.clone();
            let character_id_for_op = character_id.clone();
            let character_id_for_value = character_id.clone();
            let field_for_character = field.clone();
            let field_for_field_select = field.clone();
            let op_for_character = op.clone();
            let op_for_field = op.clone();
            let op_for_op_select = op.clone();
            let field_for_value = field.clone();
            let op_for_value = op.clone();
            rsx! {
                div { class: "grid grid-cols-6 gap-2 items-center border p-3 rounded",
                    select { class: "col-span-1 border p-1", value: effect_type_value.clone(), onchange: on_type_change,
                        option { value: "character_attribute", "Attribute" }
                        option { value: "relationship", "Relationship" }
                        option { value: "flag", "Flag" }
                    }
                    select { class: "col-span-1 border p-1", value: character_id.clone(),
                        oninput: move |evt| {
                            effects.write()[index] = Effect::CharacterAttribute {
                                character_id: evt.value(),
                                field: field_for_character.clone(),
                                op: op_for_character.clone(),
                                value,
                            };
                        },
                        for character in characters.iter() {
                            option { value: character.id.clone(), "{character.char_id} ({character.role.clone().unwrap_or_default()})" }
                        }
                    }
                    select { class: "col-span-1 border p-1", value: format!("{:?}", field_for_field_select),
                        oninput: move |evt| {
                            if let Some((new_field, _)) = attribute_field_options().into_iter().find(|(_, label)| label == &evt.value()) {
                                effects.write()[index] = Effect::CharacterAttribute {
                                    character_id: character_id_for_field.clone(),
                                    field: new_field,
                                    op: op_for_field.clone(),
                                    value,
                                };
                            }
                        },
                        for (field_option, label) in attribute_field_options() {
                            option { value: label, label }
                        }
                    }
                    select { class: "col-span-1 border p-1", value: format!("{:?}", op_for_op_select),
                        oninput: move |evt| {
                            if let Some((new_op, _)) = numeric_op_options().into_iter().find(|(_, label)| label == &evt.value()) {
                                effects.write()[index] = Effect::CharacterAttribute {
                                    character_id: character_id_for_op.clone(),
                                    field: field_for_field_select.clone(),
                                    op: new_op,
                                    value,
                                };
                            }
                        },
                        for (op_option, label) in numeric_op_options() {
                            option { value: label, label }
                        }
                    }
                    input { class: "col-span-1 border p-1", r#type: "number", value: value, min: "-100", max: "100",
                        oninput: move |evt| {
                            if let Ok(parsed) = evt.value().parse::<i32>() {
                                effects.write()[index] = Effect::CharacterAttribute {
                                    character_id: character_id_for_value.clone(),
                                    field: field_for_value.clone(),
                                    op: op_for_value.clone(),
                                    value: parsed,
                                };
                            }
                        }
                    }
                    button { class: "col-span-1 text-red-600", onclick: on_remove.clone(), "Remove" }
                }
            }
        }
        Effect::Relationship {
            from_id,
            to_id,
            field,
            op,
            value,
        } => {
            let characters = characters_vec.clone();
            let from_id_character_select = from_id.clone();
            let to_id_character_select = to_id.clone();
            let from_id_for_to = from_id.clone();
            let from_id_for_field = from_id.clone();
            let from_id_for_op = from_id.clone();
            let from_id_for_value = from_id.clone();
            let to_id_for_from = to_id.clone();
            let to_id_for_field = to_id.clone();
            let to_id_for_op = to_id.clone();
            let to_id_for_value = to_id.clone();
            let field_for_from = field.clone();
            let field_for_to = field.clone();
            let field_for_field = field.clone();
            let field_for_op_select = field.clone();
            let field_for_value = field.clone();
            let op_for_from = op.clone();
            let op_for_to = op.clone();
            let op_for_field = op.clone();
            let op_for_op_select = op.clone();
            let op_for_value = op.clone();
            rsx! {
                div { class: "grid grid-cols-6 gap-2 items-center border p-3 rounded",
                    select { class: "col-span-1 border p-1", value: effect_type_value.clone(), onchange: on_type_change,
                        option { value: "character_attribute", "Attribute" }
                        option { value: "relationship", "Relationship" }
                        option { value: "flag", "Flag" }
                    }
                    select { class: "col-span-1 border p-1", value: from_id.clone(),
                        oninput: move |evt| {
                            effects.write()[index] = Effect::Relationship {
                                from_id: evt.value(),
                                to_id: to_id_character_select.clone(),
                                field: field_for_from.clone(),
                                op: op_for_from.clone(),
                                value,
                            };
                        },
                        for character in characters.iter() {
                            option { value: character.id.clone(), "{character.char_id} ({character.role.clone().unwrap_or_default()})" }
                        }
                    }
                    select { class: "col-span-1 border p-1", value: to_id.clone(),
                        oninput: move |evt| {
                            effects.write()[index] = Effect::Relationship {
                                from_id: from_id_for_to.clone(),
                                to_id: evt.value(),
                                field: field_for_to.clone(),
                                op: op_for_to.clone(),
                                value,
                            };
                        },
                        for character in characters.iter() {
                            option { value: character.id.clone(), "{character.char_id} ({character.role.clone().unwrap_or_default()})" }
                        }
                    }
                    select { class: "col-span-1 border p-1", value: format!("{:?}", field_for_field),
                        oninput: move |evt| {
                            if let Some((new_field, _)) = relationship_field_options().into_iter().find(|(_, label)| label == &evt.value()) {
                                effects.write()[index] = Effect::Relationship {
                                    from_id: from_id_for_field.clone(),
                                    to_id: to_id_for_field.clone(),
                                    field: new_field,
                                    op: op_for_field.clone(),
                                    value,
                                };
                            }
                        },
                        for (field_option, label) in relationship_field_options() {
                            option { value: label, label }
                        }
                    }
                    select { class: "col-span-1 border p-1", value: format!("{:?}", op_for_op_select),
                        oninput: move |evt| {
                            if let Some((new_op, _)) = numeric_op_options().into_iter().find(|(_, label)| label == &evt.value()) {
                                effects.write()[index] = Effect::Relationship {
                                    from_id: from_id_for_op.clone(),
                                    to_id: to_id_for_op.clone(),
                                    field: field_for_op_select.clone(),
                                    op: new_op,
                                    value,
                                };
                            }
                        },
                        for (op_option, label) in numeric_op_options() {
                            option { value: label, label }
                        }
                    }
                    input { class: "col-span-1 border p-1", r#type: "number", value: value, min: "-100", max: "100",
                        oninput: move |evt| {
                            if let Ok(parsed) = evt.value().parse::<i32>() {
                                effects.write()[index] = Effect::Relationship {
                                    from_id: from_id_for_value.clone(),
                                    to_id: to_id_for_value.clone(),
                                    field: field_for_value.clone(),
                                    op: op_for_value.clone(),
                                    value: parsed,
                                };
                            }
                        }
                    }
                    button { class: "col-span-1 text-red-600", onclick: on_remove.clone(), "Remove" }
                }
            }
        }
        Effect::Flag {
            character_id,
            path,
            value,
        } => {
            let characters = characters_vec.clone();
            let is_checked = value.as_bool().unwrap_or(false);
            let character_id_for_path = character_id.clone();
            let character_id_for_checkbox = character_id.clone();
            let path_for_character = path.clone();
            let path_for_input = path.clone();
            let path_for_checkbox = path.clone();
            let value_for_character = value.clone();
            let value_for_input = value.clone();
            rsx! {
                div { class: "grid grid-cols-6 gap-2 items-center border p-3 rounded",
                    select { class: "col-span-1 border p-1", value: effect_type_value, onchange: on_type_change,
                        option { value: "character_attribute", "Attribute" }
                        option { value: "relationship", "Relationship" }
                        option { value: "flag", "Flag" }
                    }
                    select { class: "col-span-1 border p-1", value: character_id.clone(),
                        oninput: move |evt| {
                            effects.write()[index] = Effect::Flag {
                                character_id: evt.value(),
                                path: path_for_character.clone(),
                                value: value_for_character.clone(),
                            };
                        },
                        for character in characters.iter() {
                            option { value: character.id.clone(), "{character.char_id} ({character.role.clone().unwrap_or_default()})" }
                        }
                    }
                    input { class: "col-span-3 border p-1", value: path_for_input.join("."), placeholder: "flag.key.path",
                        oninput: move |evt| {
                            let segments = evt
                                .value()
                                .split('.')
                                .map(|s| s.trim().to_string())
                                .filter(|s| !s.is_empty())
                                .collect::<Vec<_>>();
                            effects.write()[index] = Effect::Flag {
                                character_id: character_id_for_path.clone(),
                                path: segments,
                                value: value_for_input.clone(),
                            };
                        }
                    }
                    input { class: "col-span-1 border p-1", r#type: "checkbox", checked: is_checked,
                        oninput: move |evt| {
                            let parsed = evt.value().parse::<bool>().unwrap_or(false);
                            effects.write()[index] = Effect::Flag {
                                character_id: character_id_for_checkbox.clone(),
                                path: path_for_checkbox.clone(),
                                value: serde_json::Value::Bool(parsed),
                            };
                        }
                    }
                    div { class: "col-span-1" }
                    button { class: "col-span-1 text-red-600", onclick: on_remove, "Remove" }
                }
            }
        }
    }
}

#[derive(Props, Clone, PartialEq)]
pub struct ChoiceEffectsEditorProps {
    pub choice_id: String,
    pub initial_effects_json: Option<String>,
    pub characters: Vec<CharacterOption>,
    pub relationships: Vec<RelationshipOption>,
    pub character_attributes: HashMap<String, CharacterAttributes>,
    pub relationship_metrics: HashMap<(String, String), RelationshipMetrics>,
    pub on_save: EventHandler<Vec<Effect>>,
}
