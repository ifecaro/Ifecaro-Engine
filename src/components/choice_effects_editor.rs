use crate::models::effects::{AttributeField, Effect, EffectList, NumericOp, RelationshipField};
use dioxus::events::FormEvent;
use dioxus::prelude::*;
use dioxus_i18n::t;
use serde::{Deserialize, Serialize};

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

fn attribute_field_options() -> Vec<(AttributeField, String)> {
    vec![
        (AttributeField::Honesty, "Honesty".to_string()),
        (AttributeField::Empathy, "Empathy".to_string()),
        (AttributeField::Affability, "Affability".to_string()),
        (AttributeField::Intimidation, "Intimidation".to_string()),
        (AttributeField::Aggression, "Aggression".to_string()),
        (AttributeField::Discipline, "Discipline".to_string()),
        (AttributeField::Curiosity, "Curiosity".to_string()),
        (AttributeField::Courage, "Courage".to_string()),
        (AttributeField::Impulsivity, "Impulsivity".to_string()),
        (AttributeField::Idealism, "Idealism".to_string()),
        (AttributeField::Pragmatism, "Pragmatism".to_string()),
        (AttributeField::Loyalty, "Loyalty".to_string()),
        (AttributeField::Opportunism, "Opportunism".to_string()),
        (AttributeField::Stoicism, "Stoicism".to_string()),
        (AttributeField::Morality, "Morality".to_string()),
        (AttributeField::Health, "Health".to_string()),
        (AttributeField::Stress, "Stress".to_string()),
        (AttributeField::Fatigue, "Fatigue".to_string()),
        (AttributeField::Pain, "Pain".to_string()),
        (AttributeField::Morale, "Morale".to_string()),
        (AttributeField::Intox, "Intoxication".to_string()),
    ]
}

fn relationship_field_options() -> Vec<(RelationshipField, String)> {
    vec![
        (RelationshipField::Affinity, "Affinity".to_string()),
        (RelationshipField::Trust, "Trust".to_string()),
        (RelationshipField::Respect, "Respect".to_string()),
        (RelationshipField::Fear, "Fear".to_string()),
        (RelationshipField::Attraction, "Attraction".to_string()),
    ]
}

fn numeric_op_options() -> Vec<(NumericOp, String)> {
    vec![
        (NumericOp::Add, t!("numeric_op_add")),
        (NumericOp::Set, t!("numeric_op_set")),
        (NumericOp::Scale, t!("numeric_op_scale")),
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

    let on_add = {
        let characters = props.characters.clone();
        move |_| {
            effects
                .write()
                .push(Effect::default_character(first_character_id(&characters)));
        }
    };

    {
        let effects = effects.clone();
        let on_save = props.on_save.clone();
        use_effect(move || {
            on_save.call(effects.read().clone());
        });
    }

    rsx! {
        div { class: "choice-effects-editor space-y-4",
            h3 { class: "text-lg font-semibold text-gray-900 dark:text-gray-100", {t!("choice_effects")} }
            button { class: "inline-flex items-center px-3 py-2 text-sm font-medium text-white bg-green-600 hover:bg-green-700 rounded-lg focus:outline-none focus:ring-2 focus:ring-green-500 focus:ring-offset-1 dark:focus:ring-offset-gray-800", onclick: on_add, {t!("add_effect")} }
            div { class: "space-y-3",
                for (index, effect) in effects.read().iter().cloned().enumerate() {
                    { render_effect_row(index, effect, effects.clone(), &props.characters, &props.relationships) }
                }
            }
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
    let label_class = "block text-sm font-medium text-gray-700 dark:text-gray-300";
    let input_class = "block w-full px-4 py-2.5 text-sm border border-gray-300 dark:border-gray-600 rounded-lg shadow-sm focus:outline-none focus:ring-2 focus:ring-green-500 focus:border-green-500 bg-white dark:bg-gray-700 text-gray-900 dark:text-white";
    let row_class = "grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-4 items-start border border-gray-200 dark:border-gray-700 p-4 rounded-lg bg-white dark:bg-gray-800";
    let column_class = "space-y-3";
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
                div { class: row_class,
                    div { class: column_class,
                        div { class: "space-y-2",
                            label { class: label_class, {t!("effect_type")} }
                            select { class: input_class, value: effect_type_value.clone(), onchange: on_type_change,
                                option { value: "character_attribute", {t!("effect_type_attribute")} }
                                option { value: "relationship", {t!("effect_type_relationship")} }
                                option { value: "flag", {t!("effect_type_flag")} }
                            }
                        }
                        div { class: "space-y-2",
                            label { class: label_class, {t!("target_character")} }
                            select { class: input_class, value: character_id.clone(),
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
                        }
                    }
                    div { class: column_class,
                        div { class: "space-y-2",
                            label { class: label_class, {t!("attribute_field")} }
                            select { class: input_class, value: format!("{:?}", field_for_field_select),
                                oninput: move |evt| {
                                    if let Some((new_field, _)) = attribute_field_options().into_iter().find(|(field_option, _)| format!("{:?}", field_option) == evt.value()) {
                                        effects.write()[index] = Effect::CharacterAttribute {
                                            character_id: character_id_for_field.clone(),
                                            field: new_field,
                                            op: op_for_field.clone(),
                                            value,
                                        };
                                    }
                                },
                                for (field_option, label) in attribute_field_options() {
                                    option { value: format!("{:?}", field_option), "{label}" }
                                }
                            }
                        }
                        div { class: "space-y-2",
                            label { class: label_class, {t!("numeric_operation")} }
                            select { class: input_class, value: format!("{:?}", op_for_op_select),
                                oninput: move |evt| {
                                    if let Some((new_op, _)) = numeric_op_options().into_iter().find(|(op_option, _)| format!("{:?}", op_option) == evt.value()) {
                                        effects.write()[index] = Effect::CharacterAttribute {
                                            character_id: character_id_for_op.clone(),
                                            field: field_for_field_select.clone(),
                                            op: new_op,
                                            value,
                                        };
                                    }
                                },
                                for (op_option, label) in numeric_op_options() {
                                    option { value: format!("{:?}", op_option), "{label}" }
                                }
                            }
                        }
                    }
                    div { class: column_class,
                        div { class: "space-y-2",
                            label { class: label_class, {t!("numeric_value")} }
                            input { class: input_class, r#type: "number", value: value, min: "-100", max: "100",
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
                        }
                        div { class: "flex justify-end",
                            button { class: "inline-flex items-center justify-center px-3 py-2 text-sm font-medium text-red-600 hover:text-red-700 bg-red-50 dark:bg-red-900/20 rounded-lg", onclick: on_remove.clone(), {t!("remove_effect") } }
                        }
                    }
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
                div { class: row_class,
                    div { class: column_class,
                        div { class: "space-y-2",
                            label { class: label_class, {t!("effect_type")} }
                            select { class: input_class, value: effect_type_value.clone(), onchange: on_type_change,
                                option { value: "character_attribute", {t!("effect_type_attribute")} }
                                option { value: "relationship", {t!("effect_type_relationship")} }
                                option { value: "flag", {t!("effect_type_flag")} }
                            }
                        }
                        div { class: "space-y-2",
                            label { class: label_class, {t!("relationship_from")} }
                            select { class: input_class, value: from_id.clone(),
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
                        }
                    }
                    div { class: column_class,
                        div { class: "space-y-2",
                            label { class: label_class, {t!("relationship_to")} }
                            select { class: input_class, value: to_id.clone(),
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
                        }
                        div { class: "space-y-2",
                            label { class: label_class, {t!("relationship_field")} }
                            select { class: input_class, value: format!("{:?}", field_for_field),
                                oninput: move |evt| {
                                    if let Some((new_field, _)) = relationship_field_options().into_iter().find(|(field_option, _)| format!("{:?}", field_option) == evt.value()) {
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
                                    option { value: format!("{:?}", field_option), "{label}" }
                                }
                            }
                        }
                    }
                    div { class: column_class,
                        div { class: "space-y-2",
                            label { class: label_class, {t!("numeric_operation")} }
                            select { class: input_class, value: format!("{:?}", op_for_op_select),
                                oninput: move |evt| {
                                    if let Some((new_op, _)) = numeric_op_options().into_iter().find(|(op_option, _)| format!("{:?}", op_option) == evt.value()) {
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
                                    option { value: format!("{:?}", op_option), "{label}" }
                                }
                            }
                        }
                        div { class: "space-y-2",
                            label { class: label_class, {t!("numeric_value")} }
                            input { class: input_class, r#type: "number", value: value, min: "-100", max: "100",
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
                        }
                        div { class: "flex justify-end",
                            button { class: "inline-flex items-center justify-center px-3 py-2 text-sm font-medium text-red-600 hover:text-red-700 bg-red-50 dark:bg-red-900/20 rounded-lg", onclick: on_remove.clone(), {t!("remove_effect") } }
                        }
                    }
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
                div { class: row_class,
                    div { class: column_class,
                        div { class: "space-y-2",
                            label { class: label_class, {t!("effect_type")} }
                            select { class: input_class, value: effect_type_value, onchange: on_type_change,
                                option { value: "character_attribute", {t!("effect_type_attribute")} }
                                option { value: "relationship", {t!("effect_type_relationship")} }
                                option { value: "flag", {t!("effect_type_flag")} }
                            }
                        }
                        div { class: "space-y-2",
                            label { class: label_class, {t!("target_character")} }
                            select { class: input_class, value: character_id.clone(),
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
                        }
                    }
                    div { class: column_class,
                        div { class: "space-y-2",
                            label { class: label_class, {t!("flag_path")} }
                            input { class: input_class, value: path_for_input.join("."), placeholder: "flag.key.path",
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
                        }
                    }
                    div { class: column_class,
                        div { class: "space-y-2",
                            label { class: label_class, {t!("flag_value")} }
                            div { class: "flex items-center space-x-3",
                                input { class: "h-5 w-5", r#type: "checkbox", checked: is_checked,
                                    oninput: move |evt| {
                                        let parsed = evt.value().parse::<bool>().unwrap_or(false);
                                        effects.write()[index] = Effect::Flag {
                                            character_id: character_id_for_checkbox.clone(),
                                            path: path_for_checkbox.clone(),
                                            value: serde_json::Value::Bool(parsed),
                                        };
                                    }
                                }
                            }
                        }
                        div { class: "flex justify-end",
                            button { class: "inline-flex items-center justify-center px-3 py-2 text-sm font-medium text-red-600 hover:text-red-700 bg-red-50 dark:bg-red-900/20 rounded-lg", onclick: on_remove, {t!("remove_effect") } }
                        }
                    }
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
    pub on_save: EventHandler<Vec<Effect>>,
}
