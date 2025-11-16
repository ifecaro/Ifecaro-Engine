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

// Potential extension hooks:
// - Track "critical success" or "critical failure" when successes far exceed or fall below
//   required thresholds.
// - Allow resist influences to also roll dice to potentially reduce successes directly.
// - Emit probabilities for UI previews by simulating or calculating expected successes.
