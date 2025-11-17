use crate::models::effects::{CharacterAttributes, CharacterStateSnapshot};
use crate::services::indexeddb::{
    get_latest_character_state_from_indexeddb, set_latest_character_state_to_indexeddb,
};
use rand::Rng;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use wasm_bindgen::JsValue;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum InfluenceKind {
    Support,
    Resist,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct AttrInfluence {
    /// Attribute name, e.g. "courage" or "empathy".
    pub key: String,
    /// Role in this check: supports success or raises difficulty.
    pub kind: InfluenceKind,
    /// The die size to roll for this attribute (e.g. 4, 6, 8, 10).
    pub die_sides: u32,
    /// Multiplier from attribute value to effective amount (commonly 1.0).
    pub count_factor: f32,
    /// Optional weight; defaults to 1.0 when unspecified.
    #[serde(default)]
    pub weight: Option<f32>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct EventCheckConfig {
    /// Actor identifier (for reference only in this pure layer).
    pub actor_id: String,
    /// Attribute influences participating in this check.
    pub influences: Vec<AttrInfluence>,
    /// Base successes required before resist modifiers.
    pub base_required: u32,
    /// Conversion from resist points to additional required successes.
    pub resist_to_extra_required: f32,
    /// Threshold for a single die to count as a success.
    pub success_threshold: u32,
}

pub type ActorAttrs = HashMap<String, i32>;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct AttrUpdateRule {
    /// Attribute name, should match AttrInfluence.key.
    pub key: String,
    /// Baseline magnitude applied each event, commonly between 0.1 and 1.0.
    pub base_scale: f32,
    /// Direction when the check succeeds; defaults to +1.
    #[serde(default)]
    pub success_sign: Option<f32>,
    /// Direction when the check fails; defaults to -1.
    #[serde(default)]
    pub failure_sign: Option<f32>,
}

pub type AttrUpdateRuleMap = HashMap<String, AttrUpdateRule>;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct MultiAttrRollLog {
    pub key: String,
    pub kind: InfluenceKind,
    pub die_sides: u32,
    /// Rolls recorded for this attribute (empty for resist entries).
    pub rolled: Vec<u32>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct MultiAttrCheckResult {
    pub success: bool,
    pub successes: u32,
    pub required_successes: u32,
    pub rolls: Vec<MultiAttrRollLog>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct AttrDelta {
    pub key: String,
    /// Signed change to apply to this attribute after the event.
    pub delta: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum EventOutcomeTier {
    GreatSuccess,
    Success,
    Mixed,
    Failure,
    Disaster,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct EventResolutionResult {
    /// Raw dice resolution outcome for this event.
    pub check: MultiAttrCheckResult,
    /// Continuous attribute adjustments derived from the check outcome.
    pub deltas: Vec<AttrDelta>,
    /// UI-only outcome tier for selecting narrative snippets; does not affect deltas.
    pub outcome_tier: EventOutcomeTier,
}

struct SupportDiceSpec {
    log_index: usize,
    die_sides: u32,
    count: u32,
}

fn weight_or_default(weight: Option<f32>) -> f32 {
    weight.unwrap_or(1.0)
}

/// Resolve a multi-attribute check by building a mixed dice pool from support influences,
/// translating resist influences into extra difficulty, and rolling to determine success.
///
/// This function is pure: it only consumes the provided config and attribute map and
/// does not mutate external state.
///
/// Example usage:
/// ```rust
/// use std::collections::HashMap;
/// use ifecaro::models::multi_attr_check::{
///     resolve_multi_attr_check, AttrInfluence, EventCheckConfig, InfluenceKind, ActorAttrs,
/// };
///
/// let actor_attrs: ActorAttrs = HashMap::from([
///     ("courage".to_string(), 7),
///     ("empathy".to_string(), 4),
///     ("fear".to_string(), 3),
/// ]);
///
/// let config = EventCheckConfig {
///     actor_id: "spain".to_string(),
///     influences: vec![
///         AttrInfluence {
///             key: "courage".to_string(),
///             kind: InfluenceKind::Support,
///             die_sides: 6,
///             count_factor: 1.0,
///             weight: None,
///         },
///         AttrInfluence {
///             key: "empathy".to_string(),
///             kind: InfluenceKind::Support,
///             die_sides: 8,
///             count_factor: 0.5,
///             weight: Some(1.2),
///         },
///         AttrInfluence {
///             key: "fear".to_string(),
///             kind: InfluenceKind::Resist,
///             die_sides: 6,
///             count_factor: 0.5,
///             weight: None,
///         },
///     ],
///     base_required: 2,
///     resist_to_extra_required: 0.5,
///     success_threshold: 5,
/// };
///
/// let result = resolve_multi_attr_check(&config, &actor_attrs);
/// println!("Check success? {} ({} / {} successes)", result.success, result.successes, result.required_successes);
/// ```
pub fn resolve_multi_attr_check(
    config: &EventCheckConfig,
    actor_attrs: &ActorAttrs,
) -> MultiAttrCheckResult {
    let mut support_specs: Vec<SupportDiceSpec> = Vec::new();
    let mut resist_points: f32 = 0.0;
    let mut roll_logs: Vec<MultiAttrRollLog> = Vec::new();

    // Step 2: parse influences
    for influence in &config.influences {
        let value = *actor_attrs.get(&influence.key).unwrap_or(&0) as f32;
        let effective = value * weight_or_default(influence.weight);
        let dice_count_raw = effective * influence.count_factor;
        let dice_count = (dice_count_raw.round()).max(0.0) as u32;

        let log_index = roll_logs.len();
        roll_logs.push(MultiAttrRollLog {
            key: influence.key.clone(),
            kind: influence.kind.clone(),
            die_sides: influence.die_sides,
            rolled: Vec::new(),
        });

        match influence.kind {
            InfluenceKind::Support => {
                support_specs.push(SupportDiceSpec {
                    log_index,
                    die_sides: influence.die_sides,
                    count: dice_count,
                });
            }
            InfluenceKind::Resist => {
                resist_points += effective * influence.count_factor;
            }
        }
    }

    // Step 3: compute required successes
    let extra_required = (resist_points * config.resist_to_extra_required).round();
    let required_successes = config.base_required + extra_required.max(0.0) as u32;

    // Step 4: roll support dice pool
    let mut rng = rand::thread_rng();
    let mut total_successes: u32 = 0;

    for spec in support_specs {
        for _ in 0..spec.count {
            let roll = rng.gen_range(1..=spec.die_sides);
            if let Some(log) = roll_logs.get_mut(spec.log_index) {
                log.rolled.push(roll);
            }
            if roll >= config.success_threshold {
                total_successes += 1;
            }
        }
    }

    // Step 5: determine outcome
    let success = total_successes >= required_successes;

    MultiAttrCheckResult {
        success,
        successes: total_successes,
        required_successes,
        rolls: roll_logs,
    }
}

/// Compute continuous attribute deltas based on the result of a multi-attribute check and
/// per-attribute update rules.
///
/// This function stays pure: it only returns deltas and does not mutate any external state.
///
/// Example usage:
/// ```rust
/// use std::collections::HashMap;
/// use ifecaro::models::multi_attr_check::{
///     compute_attribute_deltas, resolve_multi_attr_check, ActorAttrs, AttrInfluence,
///     AttrUpdateRule, AttrUpdateRuleMap, EventCheckConfig, InfluenceKind,
/// };
///
/// let actor_attrs: ActorAttrs = HashMap::from([
///     ("courage".to_string(), 7),
///     ("empathy".to_string(), 4),
///     ("fear".to_string(), 3),
/// ]);
///
/// let config = EventCheckConfig {
///     actor_id: "spain".to_string(),
///     influences: vec![
///         AttrInfluence {
///             key: "courage".to_string(),
///             kind: InfluenceKind::Support,
///             die_sides: 6,
///             count_factor: 1.0,
///             weight: None,
///         },
///         AttrInfluence {
///             key: "empathy".to_string(),
///             kind: InfluenceKind::Support,
///             die_sides: 8,
///             count_factor: 0.5,
///             weight: Some(1.2),
///         },
///         AttrInfluence {
///             key: "fear".to_string(),
///             kind: InfluenceKind::Resist,
///             die_sides: 6,
///             count_factor: 0.5,
///             weight: None,
///         },
///     ],
///     base_required: 2,
///     resist_to_extra_required: 0.5,
///     success_threshold: 5,
/// };
///
/// let check_result = resolve_multi_attr_check(&config, &actor_attrs);
///
/// let update_rules: AttrUpdateRuleMap = HashMap::from([
///     (
///         "courage".to_string(),
///         AttrUpdateRule {
///             key: "courage".to_string(),
///             base_scale: 0.4,
///             success_sign: Some(1.0),
///             failure_sign: Some(-1.0),
///         },
///     ),
///     (
///         "empathy".to_string(),
///         AttrUpdateRule {
///             key: "empathy".to_string(),
///             base_scale: 0.3,
///             success_sign: Some(1.0),
///             failure_sign: Some(-0.5),
///         },
///     ),
/// ]);
///
/// let deltas = compute_attribute_deltas(&config, &actor_attrs, &check_result, &update_rules);
/// // deltas will contain continuous signed adjustments (e.g., courage +0.1, empathy +0.05)
/// ```
pub fn compute_attribute_deltas(
    config: &EventCheckConfig,
    actor_attrs: &ActorAttrs,
    check_result: &MultiAttrCheckResult,
    update_rules: &AttrUpdateRuleMap,
) -> Vec<AttrDelta> {
    // Step 1: compute outcomeFactor as a continuous measure of how strongly the event went
    // for or against the actor. Roughly bounded within [-1.5, +1.5], where positive values
    // indicate a positive experience and negative values indicate a negative one.
    let successes = check_result.successes as f32;
    let required = check_result.required_successes as f32;
    let margin = successes - required;
    let norm = required.max(1.0);
    let mut outcome_factor = if successes >= required {
        (margin + 1.0) / (norm + 1.0)
    } else {
        -((margin.abs() + 1.0) / (norm + 1.0))
    };

    const MAX_IMPACT: f32 = 1.5;
    outcome_factor = outcome_factor.clamp(-MAX_IMPACT, MAX_IMPACT);

    // Step 2: compute support contribution ratios.
    let mut support_effective: Vec<(String, f32)> = Vec::new();
    let mut support_total: f32 = 0.0;

    for influence in config
        .influences
        .iter()
        .filter(|inf| matches!(inf.kind, InfluenceKind::Support))
    {
        let value = *actor_attrs.get(&influence.key).unwrap_or(&0) as f32;
        let effective = value * weight_or_default(influence.weight) * influence.count_factor;
        support_total += effective;
        support_effective.push((influence.key.clone(), effective));
    }

    // Step 3: compute deltas for support attributes.
    let mut deltas: Vec<AttrDelta> = Vec::new();
    for influence in config
        .influences
        .iter()
        .filter(|inf| matches!(inf.kind, InfluenceKind::Support))
    {
        let rule = match update_rules.get(&influence.key) {
            Some(rule) => rule,
            None => continue, // skip attributes without rules
        };

        let contrib = support_effective
            .iter()
            .find(|(key, _)| key == &influence.key)
            .map(|(_, eff)| *eff)
            .unwrap_or(0.0);
        let contrib_ratio = if support_total > 0.0 {
            contrib / support_total
        } else {
            0.0
        };

        let is_success = check_result.success;
        let sign = if is_success {
            rule.success_sign.unwrap_or(1.0)
        } else {
            rule.failure_sign.unwrap_or(-1.0)
        };

        let magnitude = rule.base_scale * outcome_factor.abs() * contrib_ratio;
        let delta = sign * magnitude;
        deltas.push(AttrDelta {
            key: influence.key.clone(),
            delta,
        });
    }

    // TODO: If resist attributes should also drift based on outcomes, extend this function to
    // compute their deltas using dedicated rules rather than leaving them unchanged.

    deltas
}

/// Classify the UI-facing outcome tier from a multi-attribute check. This does not affect
/// numeric deltas; it only helps the UI pick narrative snippets.
pub fn classify_outcome_tier(result: &MultiAttrCheckResult) -> EventOutcomeTier {
    let successes = result.successes as i32;
    let required = result.required_successes as i32;
    let margin = successes - required;

    if successes >= required {
        if margin >= 3 {
            EventOutcomeTier::GreatSuccess
        } else {
            EventOutcomeTier::Success
        }
    } else if margin >= -1 {
        EventOutcomeTier::Mixed
    } else if margin <= -3 {
        EventOutcomeTier::Disaster
    } else {
        EventOutcomeTier::Failure
    }
}

/// High-level helper that rolls the check, computes attribute deltas, and derives a UI tier.
///
/// This function is pure: it does not mutate actor attributes or persist any data. Callers
/// should apply the returned deltas and store updated values (e.g., in IndexedDB) themselves,
/// and use the outcome tier to choose which narrative text to display.
pub fn resolve_event_with_attribute_updates(
    config: &EventCheckConfig,
    actor_attrs: &ActorAttrs,
    update_rules: &AttrUpdateRuleMap,
) -> EventResolutionResult {
    let check_result = resolve_multi_attr_check(config, actor_attrs);
    let deltas = compute_attribute_deltas(config, actor_attrs, &check_result, update_rules);
    let outcome_tier = classify_outcome_tier(&check_result);

    EventResolutionResult {
        check: check_result,
        deltas,
        outcome_tier,
    }
}

// Example usage:
//
// use std::collections::HashMap;
// use ifecaro::models::multi_attr_check::{
//     resolve_event_with_attribute_updates, AttrInfluence, AttrUpdateRule, AttrUpdateRuleMap,
//     ActorAttrs, EventCheckConfig, InfluenceKind,
// };
//
// let actor_attrs: ActorAttrs = HashMap::from([
//     ("courage".to_string(), 7),
//     ("empathy".to_string(), 4),
//     ("obedience".to_string(), 8),
//     ("fear".to_string(), 3),
// ]);
//
// let config = EventCheckConfig {
//     actor_id: "spain".to_string(),
//     influences: vec![
//         AttrInfluence {
//             key: "courage".to_string(),
//             kind: InfluenceKind::Support,
//             die_sides: 6,
//             count_factor: 1.0,
//             weight: None,
//         },
//         AttrInfluence {
//             key: "empathy".to_string(),
//             kind: InfluenceKind::Support,
//             die_sides: 8,
//             count_factor: 0.5,
//             weight: Some(1.2),
//         },
//         AttrInfluence {
//             key: "fear".to_string(),
//             kind: InfluenceKind::Resist,
//             die_sides: 6,
//             count_factor: 0.5,
//             weight: None,
//         },
//     ],
//     base_required: 2,
//     resist_to_extra_required: 0.5,
//     success_threshold: 5,
// };
//
// let update_rules: AttrUpdateRuleMap = HashMap::from([
//     (
//         "courage".to_string(),
//         AttrUpdateRule {
//             key: "courage".to_string(),
//             base_scale: 0.4,
//             success_sign: Some(1.0),
//             failure_sign: Some(-1.0),
//         },
//     ),
//     (
//         "empathy".to_string(),
//         AttrUpdateRule {
//             key: "empathy".to_string(),
//             base_scale: 0.3,
//             success_sign: Some(1.0),
//             failure_sign: Some(-0.5),
//         },
//     ),
// ]);
//
// let result = resolve_event_with_attribute_updates(&config, &actor_attrs, &update_rules);
//
// // result.check       -> dice outcome (successes, required, roll logs)
// // result.deltas      -> floating deltas to apply to each influenced attribute
// // result.outcome_tier-> UI-facing tier like "success" for choosing narrative text
//
// // Caller can now:
// // 1. Apply deltas to actor_attrs and persist them externally.
// // 2. Select narrative content based on outcome_tier.

// Potential extension hooks:
// - Track "critical success" or "critical failure" when successes far exceed or fall below
//   required thresholds.
// - Allow resist influences to also roll dice to potentially reduce successes directly.
// - Emit probabilities for UI previews by simulating or calculating expected successes.

fn character_attributes_to_actor_attrs(attrs: &CharacterAttributes) -> ActorAttrs {
    let mut map = ActorAttrs::new();
    map.insert("honesty".to_string(), attrs.honesty);
    map.insert("empathy".to_string(), attrs.empathy);
    map.insert("affability".to_string(), attrs.affability);
    map.insert("intimidation".to_string(), attrs.intimidation);
    map.insert("aggression".to_string(), attrs.aggression);
    map.insert("discipline".to_string(), attrs.discipline);
    map.insert("curiosity".to_string(), attrs.curiosity);
    map.insert("courage".to_string(), attrs.courage);
    map.insert("impulsivity".to_string(), attrs.impulsivity);
    map.insert("idealism".to_string(), attrs.idealism);
    map.insert("pragmatism".to_string(), attrs.pragmatism);
    map.insert("loyalty".to_string(), attrs.loyalty);
    map.insert("opportunism".to_string(), attrs.opportunism);
    map.insert("stoicism".to_string(), attrs.stoicism);
    map.insert("morality".to_string(), attrs.morality);
    map.insert("health".to_string(), attrs.health);
    map.insert("stress".to_string(), attrs.stress);
    map.insert("fatigue".to_string(), attrs.fatigue);
    map.insert("pain".to_string(), attrs.pain);
    map.insert("morale".to_string(), attrs.morale);
    map.insert("intox".to_string(), attrs.intox);
    map
}

fn read_attr_or_current(attrs: &ActorAttrs, key: &str, current: i32) -> i32 {
    attrs.get(key).copied().unwrap_or(current)
}

fn apply_actor_attrs_to_character(
    mut base: CharacterAttributes,
    attrs: &ActorAttrs,
) -> CharacterAttributes {
    base.honesty = read_attr_or_current(attrs, "honesty", base.honesty);
    base.empathy = read_attr_or_current(attrs, "empathy", base.empathy);
    base.affability = read_attr_or_current(attrs, "affability", base.affability);
    base.intimidation = read_attr_or_current(attrs, "intimidation", base.intimidation);
    base.aggression = read_attr_or_current(attrs, "aggression", base.aggression);
    base.discipline = read_attr_or_current(attrs, "discipline", base.discipline);
    base.curiosity = read_attr_or_current(attrs, "curiosity", base.curiosity);
    base.courage = read_attr_or_current(attrs, "courage", base.courage);
    base.impulsivity = read_attr_or_current(attrs, "impulsivity", base.impulsivity);
    base.idealism = read_attr_or_current(attrs, "idealism", base.idealism);
    base.pragmatism = read_attr_or_current(attrs, "pragmatism", base.pragmatism);
    base.loyalty = read_attr_or_current(attrs, "loyalty", base.loyalty);
    base.opportunism = read_attr_or_current(attrs, "opportunism", base.opportunism);
    base.stoicism = read_attr_or_current(attrs, "stoicism", base.stoicism);
    base.morality = read_attr_or_current(attrs, "morality", base.morality);
    base.health = read_attr_or_current(attrs, "health", base.health);
    base.stress = read_attr_or_current(attrs, "stress", base.stress);
    base.fatigue = read_attr_or_current(attrs, "fatigue", base.fatigue);
    base.pain = read_attr_or_current(attrs, "pain", base.pain);
    base.morale = read_attr_or_current(attrs, "morale", base.morale);
    base.intox = read_attr_or_current(attrs, "intox", base.intox);
    base
}

fn clamp_personality(value: i32) -> i32 {
    value.clamp(0, 100)
}

fn apply_deltas_to_attrs(actor_attrs: &ActorAttrs, deltas: &[AttrDelta]) -> ActorAttrs {
    let mut updated = actor_attrs.clone();
    for delta in deltas {
        let entry = updated.entry(delta.key.clone()).or_insert(0);
        let new_value = (*entry as f32 + delta.delta).round() as i32;
        *entry = clamp_personality(new_value);
    }
    updated
}

fn js_error_to_string(err: JsValue) -> String {
    format!("{:?}", err)
}

async fn fetch_latest_snapshot() -> Result<CharacterStateSnapshot, String> {
    let raw = get_latest_character_state_from_indexeddb()
        .await
        .map_err(js_error_to_string)?;

    let snapshot = raw
        .as_string()
        .and_then(|s| serde_json::from_str::<CharacterStateSnapshot>(&s).ok())
        .unwrap_or_default();

    Ok(snapshot)
}

async fn persist_snapshot(snapshot: &CharacterStateSnapshot) -> Result<(), String> {
    let serialized = serde_json::to_string(snapshot).map_err(|e| e.to_string())?;
    set_latest_character_state_to_indexeddb(&serialized)
        .await
        .map_err(js_error_to_string)?;
    Ok(())
}

/// Load the actor's current attributes from IndexedDB, defaulting missing actors to zeros.
pub async fn load_actor_attrs_from_db(actor_id: &str) -> Result<ActorAttrs, String> {
    let snapshot = fetch_latest_snapshot().await?;
    let attrs = snapshot
        .characters
        .get(actor_id)
        .cloned()
        .unwrap_or_default();
    Ok(character_attributes_to_actor_attrs(&attrs))
}

/// Save updated actor attributes back into IndexedDB using the existing character snapshot store.
pub async fn save_actor_attrs_to_db(actor_id: &str, attrs: &ActorAttrs) -> Result<(), String> {
    let mut snapshot = fetch_latest_snapshot().await?;
    let base = snapshot
        .characters
        .get(actor_id)
        .cloned()
        .unwrap_or_default();
    let updated_character = apply_actor_attrs_to_character(base, attrs);
    snapshot
        .characters
        .insert(actor_id.to_string(), updated_character);

    persist_snapshot(&snapshot).await
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct EventRunResult {
    pub resolution: EventResolutionResult,
    pub updated_attrs: ActorAttrs,
}

/// Full event flow: load actor attributes, resolve the check, compute deltas, apply them with
/// clamping, persist the new attributes to IndexedDB, and return the detailed outcome.
///
/// This function orchestrates pure logic plus the IndexedDB IO layer; it does not otherwise
/// mutate global state. Callers can use the returned updated attributes for UI refresh and rely on
/// the persisted snapshot for later retrieval.
pub async fn run_event_resolution(
    config: &EventCheckConfig,
    update_rules: &AttrUpdateRuleMap,
) -> Result<EventRunResult, String> {
    let actor_attrs = load_actor_attrs_from_db(&config.actor_id).await?;
    let resolution = resolve_event_with_attribute_updates(config, &actor_attrs, update_rules);
    let updated_attrs = apply_deltas_to_attrs(&actor_attrs, &resolution.deltas);
    save_actor_attrs_to_db(&config.actor_id, &updated_attrs).await?;

    Ok(EventRunResult {
        resolution,
        updated_attrs,
    })
}

// Example usage with IndexedDB round-tripping:
//
// use std::collections::HashMap;
// use ifecaro::models::multi_attr_check::{
//     run_event_resolution, AttrInfluence, AttrUpdateRule, AttrUpdateRuleMap, EventCheckConfig,
//     InfluenceKind,
// };
//
// let config = EventCheckConfig {
//     actor_id: "spain".to_string(),
//     influences: vec![
//         AttrInfluence {
//             key: "courage".to_string(),
//             kind: InfluenceKind::Support,
//             die_sides: 6,
//             count_factor: 1.0,
//             weight: None,
//         },
//         AttrInfluence {
//             key: "fear".to_string(),
//             kind: InfluenceKind::Resist,
//             die_sides: 6,
//             count_factor: 0.5,
//             weight: None,
//         },
//     ],
//     base_required: 2,
//     resist_to_extra_required: 0.5,
//     success_threshold: 5,
// };
//
// let update_rules: AttrUpdateRuleMap = HashMap::from([
//     (
//         "courage".to_string(),
//         AttrUpdateRule {
//             key: "courage".to_string(),
//             base_scale: 0.4,
//             success_sign: Some(1.0),
//             failure_sign: Some(-1.0),
//         },
//     ),
// ]);
//
// let outcome = run_event_resolution(&config, &update_rules).await?;
//
// // outcome.resolution.check   -> dice result (success count, required, logs)
// // outcome.resolution.deltas  -> per-attribute deltas that were applied
// // outcome.resolution.outcome_tier -> UI tier for narrative selection
// // outcome.updated_attrs      -> clamped attributes after applying deltas, already persisted
