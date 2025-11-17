use crate::models::impacts::{AttributeField, Impact, ImpactList, NumericOp, RelationshipField};
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

fn notify_impacts_changed(impacts: &Signal<Vec<Impact>>, on_save: &EventHandler<Vec<Impact>>) {
    on_save.call(impacts.read().clone());
}

fn impact_type(impact: &Impact) -> &'static str {
    match impact {
        Impact::CharacterAttribute { .. } => "character_attribute",
        Impact::Relationship { .. } => "relationship",
        Impact::Flag { .. } => "flag",
    }
}

fn update_impact_type(
    impact: &Impact,
    to: &str,
    characters: &[CharacterOption],
    relations: &[RelationshipOption],
) -> Impact {
    match to {
        "relationship" => {
            let (from_id, to_id) = first_relationship(relations);
            Impact::Relationship {
                from_id,
                to_id,
                field: RelationshipField::Trust,
                op: NumericOp::Add,
                value: 0,
            }
        }
        "flag" => Impact::Flag {
            character_id: first_character_id(characters),
            path: vec!["flag_name".to_string()],
            value: serde_json::Value::Bool(true),
        },
        _ => Impact::default_character(first_character_id(characters)),
    }
}

#[component]
pub fn ChoiceImpactsEditor(props: ChoiceImpactsEditorProps) -> Element {
    let initial_impacts = props
        .initial_impacts_json
        .as_deref()
        .and_then(|raw| ImpactList::from_json(raw).ok())
        .unwrap_or_default();

    let mut impacts = use_signal(|| initial_impacts.0);

    let on_add = {
        let characters = props.characters.clone();
        let mut impacts = impacts.clone();
        let on_save = props.on_save.clone();
        move |_| {
            impacts
                .write()
                .push(Impact::default_character(first_character_id(&characters)));
            notify_impacts_changed(&impacts, &on_save);
        }
    };

    rsx! {
        div { class: "choice-impacts-editor space-y-4",
            div { class: "flex items-center justify-between gap-3",
                h3 { class: "text-lg font-semibold text-gray-900 dark:text-gray-100", {t!("choice_impacts")} }
                button { class: "inline-flex items-center px-3 py-2 text-sm font-medium text-white bg-green-600 hover:bg-green-700 rounded-lg focus:outline-none focus:ring-2 focus:ring-green-500 focus:ring-offset-1 dark:focus:ring-offset-gray-800", onclick: on_add, {t!("add_impact")} }
            }
            div { class: "space-y-3",
                for (index, impact) in impacts.read().iter().cloned().enumerate() {
                    { render_impact_row(index, impact, impacts.clone(), &props.characters, &props.relationships, props.on_save.clone()) }
                }
            }
        }
    }
}

fn render_impact_row(
    index: usize,
    impact: Impact,
    mut impacts: Signal<Vec<Impact>>,
    characters: &[CharacterOption],
    relationships: &[RelationshipOption],
    on_save: EventHandler<Vec<Impact>>,
) -> Element {
    let impact_type_value = impact_type(&impact).to_string();
    let label_class = "block text-sm font-medium text-gray-700 dark:text-gray-200 mb-1";
    let input_class = "block w-full px-3 py-2 text-sm border border-gray-300 dark:border-gray-600 rounded-md shadow-sm focus:outline-none focus:ring-2 focus:ring-blue-500 dark:focus:ring-blue-400 focus:border-transparent bg-white dark:bg-gray-700 text-gray-900 dark:text-white";
    let row_class = "space-y-4 p-4 bg-gray-50 dark:bg-gray-800 rounded-lg border border-gray-200 dark:border-gray-700 shadow-sm";
    let column_class = "space-y-3";
    let grid_class = "grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-4";
    let impact_title = t!("impact_title", index: (index + 1).to_string());
    let characters_vec = characters.to_vec();
    let relationships_vec = relationships.to_vec();

    let on_type_change = {
        let characters = characters_vec.clone();
        let relationships = relationships_vec.clone();
        let mut impacts = impacts.clone();
        let on_save = on_save.clone();
        move |evt: FormEvent| {
            {
                let mut list = impacts.write();
                list[index] =
                    update_impact_type(&list[index], &evt.value(), &characters, &relationships);
            }
            notify_impacts_changed(&impacts, &on_save);
        }
    };

    let on_remove = {
        let mut impacts = impacts.clone();
        let on_save = on_save.clone();
        move |_| {
            impacts.write().remove(index);
            notify_impacts_changed(&impacts, &on_save);
        }
    };

    match impact {
        Impact::CharacterAttribute {
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
                    div { class: "flex items-center justify-between",
                        h4 { class: "text-sm font-semibold text-gray-800 dark:text-gray-100", "{impact_title}" }
                        button { class: "inline-flex items-center text-sm font-medium text-red-500 hover:text-red-400", onclick: on_remove.clone(), {t!("remove_impact") } }
                    }
                    div { class: grid_class,
                        div { class: column_class,
                            div { class: "space-y-2",
                                label { class: label_class, {t!("impact_type")} }
                                select { class: input_class, value: impact_type_value.clone(), onchange: on_type_change,
                                    option { value: "character_attribute", {t!("impact_type_attribute")} }
                                    option { value: "relationship", {t!("impact_type_relationship")} }
                                    option { value: "flag", {t!("impact_type_flag")} }
                                }
                            }
                            div { class: "space-y-2",
                                label { class: label_class, {t!("target_character")} }
                                select { class: input_class, value: character_id.clone(),
                                    oninput: {
                                        let on_save = on_save.clone();
                                        move |evt: FormEvent| {
                                            impacts.write()[index] = Impact::CharacterAttribute {
                                                character_id: evt.value(),
                                                field: field_for_character.clone(),
                                                op: op_for_character.clone(),
                                                value,
                                            };
                                            notify_impacts_changed(&impacts, &on_save);
                                        }
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
                                    oninput: {
                                        let on_save = on_save.clone();
                                        move |evt: FormEvent| {
                                            if let Some((new_field, _)) = attribute_field_options().into_iter().find(|(field_option, _)| format!("{:?}", field_option) == evt.value()) {
                                                impacts.write()[index] = Impact::CharacterAttribute {
                                                    character_id: character_id_for_field.clone(),
                                                    field: new_field,
                                                    op: op_for_field.clone(),
                                                    value,
                                                };
                                                notify_impacts_changed(&impacts, &on_save);
                                            }
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
                                    oninput: {
                                        let on_save = on_save.clone();
                                        move |evt: FormEvent| {
                                            if let Some((new_op, _)) = numeric_op_options().into_iter().find(|(op_option, _)| format!("{:?}", op_option) == evt.value()) {
                                                impacts.write()[index] = Impact::CharacterAttribute {
                                                    character_id: character_id_for_op.clone(),
                                                    field: field_for_field_select.clone(),
                                                    op: new_op,
                                                    value,
                                                };
                                                notify_impacts_changed(&impacts, &on_save);
                                            }
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
                                    oninput: {
                                        let on_save = on_save.clone();
                                        move |evt: FormEvent| {
                                            if let Ok(parsed) = evt.value().parse::<i32>() {
                                                impacts.write()[index] = Impact::CharacterAttribute {
                                                    character_id: character_id_for_value.clone(),
                                                    field: field_for_value.clone(),
                                                    op: op_for_value.clone(),
                                                    value: parsed,
                                                };
                                                notify_impacts_changed(&impacts, &on_save);
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
        Impact::Relationship {
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
                    div { class: "flex items-center justify-between",
                        h4 { class: "text-sm font-semibold text-gray-800 dark:text-gray-100", "{impact_title}" }
                        button { class: "inline-flex items-center text-sm font-medium text-red-500 hover:text-red-400", onclick: on_remove.clone(), {t!("remove_impact") } }
                    }
                    div { class: grid_class,
                        div { class: column_class,
                            div { class: "space-y-2",
                                label { class: label_class, {t!("impact_type")} }
                                select { class: input_class, value: impact_type_value.clone(), onchange: on_type_change,
                                    option { value: "character_attribute", {t!("impact_type_attribute")} }
                                    option { value: "relationship", {t!("impact_type_relationship")} }
                                    option { value: "flag", {t!("impact_type_flag")} }
                                }
                            }
                            div { class: "space-y-2",
                                label { class: label_class, {t!("relationship_from")} }
                                select { class: input_class, value: from_id.clone(),
                                    oninput: {
                                        let on_save = on_save.clone();
                                        move |evt: FormEvent| {
                                            impacts.write()[index] = Impact::Relationship {
                                                from_id: evt.value(),
                                                to_id: to_id_character_select.clone(),
                                                field: field_for_from.clone(),
                                                op: op_for_from.clone(),
                                                value,
                                            };
                                            notify_impacts_changed(&impacts, &on_save);
                                        }
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
                                    oninput: {
                                        let on_save = on_save.clone();
                                        move |evt: FormEvent| {
                                            impacts.write()[index] = Impact::Relationship {
                                                from_id: from_id_for_to.clone(),
                                                to_id: evt.value(),
                                                field: field_for_to.clone(),
                                                op: op_for_to.clone(),
                                                value,
                                            };
                                            notify_impacts_changed(&impacts, &on_save);
                                        }
                                    },
                                    for character in characters.iter() {
                                        option { value: character.id.clone(), "{character.char_id} ({character.role.clone().unwrap_or_default()})" }
                                    }
                                }
                            }
                            div { class: "space-y-2",
                                label { class: label_class, {t!("relationship_field")} }
                                select { class: input_class, value: format!("{:?}", field_for_field),
                                    oninput: {
                                        let on_save = on_save.clone();
                                        move |evt: FormEvent| {
                                            if let Some((new_field, _)) = relationship_field_options().into_iter().find(|(field_option, _)| format!("{:?}", field_option) == evt.value()) {
                                                impacts.write()[index] = Impact::Relationship {
                                                    from_id: from_id_for_field.clone(),
                                                    to_id: to_id_for_field.clone(),
                                                    field: new_field,
                                                    op: op_for_field.clone(),
                                                    value,
                                                };
                                                notify_impacts_changed(&impacts, &on_save);
                                            }
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
                                    oninput: {
                                        let on_save = on_save.clone();
                                        move |evt: FormEvent| {
                                            if let Some((new_op, _)) = numeric_op_options().into_iter().find(|(op_option, _)| format!("{:?}", op_option) == evt.value()) {
                                                impacts.write()[index] = Impact::Relationship {
                                                    from_id: from_id_for_op.clone(),
                                                    to_id: to_id_for_op.clone(),
                                                    field: field_for_op_select.clone(),
                                                    op: new_op,
                                                    value,
                                                };
                                                notify_impacts_changed(&impacts, &on_save);
                                            }
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
                                    oninput: {
                                        let on_save = on_save.clone();
                                        move |evt: FormEvent| {
                                            if let Ok(parsed) = evt.value().parse::<i32>() {
                                                impacts.write()[index] = Impact::Relationship {
                                                    from_id: from_id_for_value.clone(),
                                                    to_id: to_id_for_value.clone(),
                                                    field: field_for_value.clone(),
                                                    op: op_for_value.clone(),
                                                    value: parsed,
                                                };
                                                notify_impacts_changed(&impacts, &on_save);
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
        Impact::Flag {
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
                    div { class: "flex items-center justify-between",
                        h4 { class: "text-sm font-semibold text-gray-800 dark:text-gray-100", "{impact_title}" }
                        button { class: "inline-flex items-center text-sm font-medium text-red-500 hover:text-red-400", onclick: on_remove, {t!("remove_impact") } }
                    }
                    div { class: grid_class,
                        div { class: column_class,
                            div { class: "space-y-2",
                                label { class: label_class, {t!("impact_type")} }
                                select { class: input_class, value: impact_type_value, onchange: on_type_change,
                                    option { value: "character_attribute", {t!("impact_type_attribute")} }
                                    option { value: "relationship", {t!("impact_type_relationship")} }
                                    option { value: "flag", {t!("impact_type_flag")} }
                                }
                            }
                            div { class: "space-y-2",
                                label { class: label_class, {t!("target_character")} }
                                select { class: input_class, value: character_id.clone(),
                                    oninput: {
                                        let on_save = on_save.clone();
                                          move |evt: FormEvent| {
                                            impacts.write()[index] = Impact::Flag {
                                                character_id: evt.value(),
                                                path: path_for_character.clone(),
                                                value: value_for_character.clone(),
                                            };
                                            notify_impacts_changed(&impacts, &on_save);
                                        }
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
                                    oninput: {
                                        let on_save = on_save.clone();
                                          move |evt: FormEvent| {
                                            let segments = evt
                                                .value()
                                                .split('.')
                                                .map(|s| s.trim().to_string())
                                                .filter(|s| !s.is_empty())
                                                .collect::<Vec<_>>();
                                            impacts.write()[index] = Impact::Flag {
                                                character_id: character_id_for_path.clone(),
                                                path: segments,
                                                value: value_for_input.clone(),
                                            };
                                            notify_impacts_changed(&impacts, &on_save);
                                        }
                                    }
                                }
                            }
                        }
                        div { class: column_class,
                            div { class: "space-y-2",
                                label { class: label_class, {t!("flag_value")} }
                                div { class: "flex items-center space-x-3",
                                    input { class: "h-5 w-5", r#type: "checkbox", checked: is_checked,
                                        oninput: {
                                            let on_save = on_save.clone();
                                              move |evt: FormEvent| {
                                                let parsed = evt.value().parse::<bool>().unwrap_or(false);
                                                impacts.write()[index] = Impact::Flag {
                                                    character_id: character_id_for_checkbox.clone(),
                                                    path: path_for_checkbox.clone(),
                                                    value: serde_json::Value::Bool(parsed),
                                                };
                                                notify_impacts_changed(&impacts, &on_save);
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

#[derive(Props, Clone, PartialEq)]
pub struct ChoiceImpactsEditorProps {
    pub choice_id: String,
    pub initial_impacts_json: Option<String>,
    pub characters: Vec<CharacterOption>,
    pub relationships: Vec<RelationshipOption>,
    pub on_save: EventHandler<Vec<Impact>>,
}
