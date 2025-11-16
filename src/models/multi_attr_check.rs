use rand::Rng;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

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

// Potential extension hooks:
// - Track "critical success" or "critical failure" when successes far exceed or fall below
//   required thresholds.
// - Allow resist influences to also roll dice to potentially reduce successes directly.
// - Emit probabilities for UI previews by simulating or calculating expected successes.
