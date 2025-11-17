use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::collections::HashMap;

#[allow(dead_code)]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum NumericOp {
    Add,
    Set,
    Scale,
}

#[allow(dead_code)]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum AttributeField {
    Honesty,
    Empathy,
    Affability,
    Intimidation,
    Aggression,
    Discipline,
    Curiosity,
    Courage,
    Impulsivity,
    Idealism,
    Pragmatism,
    Loyalty,
    Opportunism,
    Stoicism,
    Morality,
    Health,
    Stress,
    Fatigue,
    Pain,
    Morale,
    Intox,
}

#[allow(dead_code)]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum RelationshipField {
    Affinity,
    Trust,
    Respect,
    Fear,
    Attraction,
}

#[allow(dead_code)]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum Impact {
    CharacterAttribute {
        character_id: String,
        field: AttributeField,
        op: NumericOp,
        value: i32,
    },
    Relationship {
        from_id: String,
        to_id: String,
        field: RelationshipField,
        op: NumericOp,
        value: i32,
    },
    Flag {
        character_id: String,
        /// Path inside traits_flags (e.g. ["saved_the_boy"]).
        path: Vec<String>,
        value: Value,
    },
}

impl Impact {
    #[allow(dead_code)]
    pub fn default_character(character_id: String) -> Self {
        Impact::CharacterAttribute {
            character_id,
            field: AttributeField::Morality,
            op: NumericOp::Add,
            value: 0,
        }
    }
}

#[allow(dead_code)]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ImpactList(pub Vec<Impact>);

impl Default for ImpactList {
    fn default() -> Self {
        ImpactList(Vec::new())
    }
}

impl ImpactList {
    #[allow(dead_code)]
    pub fn from_json(raw: &str) -> serde_json::Result<Self> {
        serde_json::from_str(raw)
    }

    #[allow(dead_code)]
    pub fn to_json(&self) -> serde_json::Result<String> {
        serde_json::to_string_pretty(&self.0)
    }
}

#[allow(dead_code)]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CharacterAttributes {
    pub honesty: i32,
    pub empathy: i32,
    pub affability: i32,
    pub intimidation: i32,
    pub aggression: i32,
    pub discipline: i32,
    pub curiosity: i32,
    pub courage: i32,
    pub impulsivity: i32,
    pub idealism: i32,
    pub pragmatism: i32,
    pub loyalty: i32,
    pub opportunism: i32,
    pub stoicism: i32,
    pub morality: i32,
    pub health: i32,
    pub stress: i32,
    pub fatigue: i32,
    pub pain: i32,
    pub morale: i32,
    pub intox: i32,
    #[serde(default = "default_traits_flags")]
    pub traits_flags: Value,
}

impl Default for CharacterAttributes {
    fn default() -> Self {
        Self {
            honesty: 0,
            empathy: 0,
            affability: 0,
            intimidation: 0,
            aggression: 0,
            discipline: 0,
            curiosity: 0,
            courage: 0,
            impulsivity: 0,
            idealism: 0,
            pragmatism: 0,
            loyalty: 0,
            opportunism: 0,
            stoicism: 0,
            morality: 0,
            health: 0,
            stress: 0,
            fatigue: 0,
            pain: 0,
            morale: 0,
            intox: 0,
            traits_flags: default_traits_flags(),
        }
    }
}

fn default_traits_flags() -> Value {
    Value::Object(serde_json::Map::new())
}

#[allow(dead_code)]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct RelationshipMetrics {
    pub affinity: i32,
    pub trust: i32,
    pub respect: i32,
    pub fear: i32,
    pub attraction: i32,
}

#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct PreviewState {
    pub characters: HashMap<String, CharacterAttributes>,
    pub relationships: HashMap<(String, String), RelationshipMetrics>,
}

#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct RelationshipState {
    pub from_id: String,
    pub to_id: String,
    pub metrics: RelationshipMetrics,
}

#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct CharacterStateSnapshot {
    pub characters: HashMap<String, CharacterAttributes>,
    pub relationships: Vec<RelationshipState>,
}

impl CharacterStateSnapshot {
    pub fn to_preview_state(&self) -> PreviewState {
        let mut relationships: HashMap<(String, String), RelationshipMetrics> = HashMap::new();
        for rel in &self.relationships {
            relationships.insert(
                (rel.from_id.clone(), rel.to_id.clone()),
                rel.metrics.clone(),
            );
        }

        PreviewState {
            characters: self.characters.clone(),
            relationships,
        }
    }

    pub fn from_preview_state(state: PreviewState) -> Self {
        let relationships = state
            .relationships
            .into_iter()
            .map(|((from_id, to_id), metrics)| RelationshipState {
                from_id,
                to_id,
                metrics,
            })
            .collect();

        Self {
            characters: state.characters,
            relationships,
        }
    }

    pub fn apply_impacts(&self, impacts: &[Impact]) -> Self {
        let base = self.to_preview_state();
        let updated = apply_impacts_preview(&base.characters, &base.relationships, impacts);
        CharacterStateSnapshot::from_preview_state(updated)
    }
}

fn clamp_0_100(value: i32) -> i32 {
    value.clamp(0, 100)
}

fn clamp_relationship(value: i32) -> i32 {
    value.clamp(-100, 100)
}

fn apply_numeric_op(
    current: i32,
    op: &NumericOp,
    value: i32,
    clamp_fn: impl Fn(i32) -> i32,
) -> i32 {
    let result = match op {
        NumericOp::Add => current + value,
        NumericOp::Set => value,
        NumericOp::Scale => ((current as f32) * (value as f32 / 100.0)).round() as i32,
    };

    clamp_fn(result)
}

fn set_flag_path(root: &mut Value, path: &[String], value: Value) {
    let mut cursor = root;
    for (index, key) in path.iter().enumerate() {
        if index == path.len() - 1 {
            if let Some(map) = cursor.as_object_mut() {
                map.insert(key.clone(), value);
            }
            return;
        }

        if !cursor.get(key).is_some() {
            if let Some(map) = cursor.as_object_mut() {
                map.insert(key.clone(), json!({}));
            }
        }

        cursor = cursor.get_mut(key).unwrap();
        if !cursor.is_object() {
            *cursor = json!({});
        }
    }
}

#[allow(dead_code)]
pub fn apply_impacts_preview(
    attributes: &HashMap<String, CharacterAttributes>,
    relationships: &HashMap<(String, String), RelationshipMetrics>,
    impacts: &[Impact],
) -> PreviewState {
    let mut characters = attributes.clone();
    let mut relationships = relationships.clone();

    for impact in impacts {
        match impact {
            Impact::CharacterAttribute {
                character_id,
                field,
                op,
                value,
            } => {
                let attr = characters
                    .entry(character_id.clone())
                    .or_insert_with(CharacterAttributes::default);

                match field {
                    AttributeField::Honesty => {
                        attr.honesty = apply_numeric_op(attr.honesty, op, *value, clamp_0_100)
                    }
                    AttributeField::Empathy => {
                        attr.empathy = apply_numeric_op(attr.empathy, op, *value, clamp_0_100)
                    }
                    AttributeField::Affability => {
                        attr.affability = apply_numeric_op(attr.affability, op, *value, clamp_0_100)
                    }
                    AttributeField::Intimidation => {
                        attr.intimidation =
                            apply_numeric_op(attr.intimidation, op, *value, clamp_0_100)
                    }
                    AttributeField::Aggression => {
                        attr.aggression = apply_numeric_op(attr.aggression, op, *value, clamp_0_100)
                    }
                    AttributeField::Discipline => {
                        attr.discipline = apply_numeric_op(attr.discipline, op, *value, clamp_0_100)
                    }
                    AttributeField::Curiosity => {
                        attr.curiosity = apply_numeric_op(attr.curiosity, op, *value, clamp_0_100)
                    }
                    AttributeField::Courage => {
                        attr.courage = apply_numeric_op(attr.courage, op, *value, clamp_0_100)
                    }
                    AttributeField::Impulsivity => {
                        attr.impulsivity =
                            apply_numeric_op(attr.impulsivity, op, *value, clamp_0_100)
                    }
                    AttributeField::Idealism => {
                        attr.idealism = apply_numeric_op(attr.idealism, op, *value, clamp_0_100)
                    }
                    AttributeField::Pragmatism => {
                        attr.pragmatism = apply_numeric_op(attr.pragmatism, op, *value, clamp_0_100)
                    }
                    AttributeField::Loyalty => {
                        attr.loyalty = apply_numeric_op(attr.loyalty, op, *value, clamp_0_100)
                    }
                    AttributeField::Opportunism => {
                        attr.opportunism =
                            apply_numeric_op(attr.opportunism, op, *value, clamp_0_100)
                    }
                    AttributeField::Stoicism => {
                        attr.stoicism = apply_numeric_op(attr.stoicism, op, *value, clamp_0_100)
                    }
                    AttributeField::Morality => {
                        attr.morality = apply_numeric_op(attr.morality, op, *value, clamp_0_100)
                    }
                    AttributeField::Health => {
                        attr.health = apply_numeric_op(attr.health, op, *value, clamp_0_100)
                    }
                    AttributeField::Stress => {
                        attr.stress = apply_numeric_op(attr.stress, op, *value, clamp_0_100)
                    }
                    AttributeField::Fatigue => {
                        attr.fatigue = apply_numeric_op(attr.fatigue, op, *value, clamp_0_100)
                    }
                    AttributeField::Pain => {
                        attr.pain = apply_numeric_op(attr.pain, op, *value, clamp_0_100)
                    }
                    AttributeField::Morale => {
                        attr.morale = apply_numeric_op(attr.morale, op, *value, clamp_0_100)
                    }
                    AttributeField::Intox => {
                        attr.intox = apply_numeric_op(attr.intox, op, *value, clamp_0_100)
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
                let rel = relationships
                    .entry((from_id.clone(), to_id.clone()))
                    .or_insert_with(RelationshipMetrics::default);

                match field {
                    RelationshipField::Affinity => {
                        rel.affinity =
                            apply_numeric_op(rel.affinity, op, *value, clamp_relationship)
                    }
                    RelationshipField::Trust => {
                        rel.trust = apply_numeric_op(rel.trust, op, *value, clamp_relationship)
                    }
                    RelationshipField::Respect => {
                        rel.respect = apply_numeric_op(rel.respect, op, *value, clamp_relationship)
                    }
                    RelationshipField::Fear => {
                        rel.fear = apply_numeric_op(rel.fear, op, *value, clamp_relationship)
                    }
                    RelationshipField::Attraction => {
                        rel.attraction =
                            apply_numeric_op(rel.attraction, op, *value, clamp_relationship)
                    }
                }
            }
            Impact::Flag {
                character_id,
                path,
                value,
            } => {
                if let Some(attr) = characters.get_mut(character_id) {
                    set_flag_path(&mut attr.traits_flags, path, value.clone());
                }
            }
        }
    }

    PreviewState {
        characters,
        relationships,
    }
}
