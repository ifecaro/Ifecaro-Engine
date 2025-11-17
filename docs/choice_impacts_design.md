# Choice Impacts Authoring

This page captures the impact schema for PocketBase plus the Rust-side shapes used by the Dioxus dashboard editor and the runtime engine.

## JSON schema for `choices.impacts`

Each choice stores an array of impact objects. The schema is intentionally explicit and tagged so Rust can deserialize with `serde(tag = "type")`.

```json
[
  {
    "type": "character_attribute",
    "character_id": "Spain",
    "field": "morality",
    "op": "add",
    "value": -20
  },
  {
    "type": "character_attribute",
    "character_id": "Spain",
    "field": "stress",
    "op": "add",
    "value": 25
  },
  {
    "type": "relationship",
    "from_id": "Spain",
    "to_id": "Father",
    "field": "trust",
    "op": "add",
    "value": -15
  },
  {
    "type": "flag",
    "character_id": "Spain",
    "path": ["traits_flags", "left_the_boy"],
    "value": true
  }
]
```

Impact types:

- `character_attribute`: Adjusts a numeric field on a single character. `field` matches a column in `attributes`. `op` can be `add` (delta), `set` (absolute), or `scale` (percentage multiplier such as `80` to shrink by 20%). Attribute values clamp to `[0, 100]`.
- `relationship`: Adjusts a numeric field for a `from_id` → `to_id` row. Values clamp to `[-100, 100]`.
- `flag`: Sets a boolean (or JSON) value inside `traits_flags`. `path` is an array of keys; nested objects are created on demand.

### Worked story examples

*“Leave the boy behind” choice*

```json
[
  {"type": "character_attribute", "character_id": "Spain", "field": "morality", "op": "add", "value": -30},
  {"type": "character_attribute", "character_id": "Spain", "field": "stress", "op": "add", "value": 15},
  {"type": "relationship", "from_id": "Spain", "to_id": "AhCheng", "field": "trust", "op": "add", "value": -10},
  {"type": "flag", "character_id": "Spain", "path": ["traits_flags", "left_the_boy"], "value": true}
]
```

*“Carry the boy with us” choice*

```json
[
  {"type": "character_attribute", "character_id": "Spain", "field": "morality", "op": "add", "value": 25},
  {"type": "character_attribute", "character_id": "Spain", "field": "fatigue", "op": "add", "value": 20},
  {"type": "relationship", "from_id": "Spain", "to_id": "AhCheng", "field": "trust", "op": "add", "value": 12},
  {"type": "relationship", "from_id": "Spain", "to_id": "Father", "field": "respect", "op": "add", "value": 10},
  {"type": "flag", "character_id": "Spain", "path": ["traits_flags", "saved_the_boy"], "value": true}
]
```

## Rust data model

`src/models/impacts.rs` defines the strongly typed schema, clamps for numeric fields, a helper to parse/serialize JSON, and a pure `apply_impacts_preview` used by the UI preview and the runtime engine. The same structs can be reused in gameplay to apply impacts to in-memory attributes and relationships.

Key types:

- `Impact` (`character_attribute | relationship | flag`) with `NumericOp` (`add | set | scale`).
- `CharacterAttributes` and `RelationshipMetrics` mirror PocketBase tables.
- `ImpactList::from_json` / `to_json` round-trip the `choices.impacts` field.
- `apply_impacts_preview` returns a `PreviewState` with updated characters/relationships after clamping.

## Dioxus authoring UI

`src/components/choice_impacts_editor.rs` implements a reusable `ChoiceImpactsEditor` component.

Props:

- `choice_id`: the choice being edited.
- `initial_impacts_json`: the raw JSON from PocketBase (nullable when creating a new choice).
- `characters`: list of available characters for dropdowns.
- `relationships`: list of relationship rows.
- `character_attributes` and `relationship_metrics`: current values for previews.
- `on_save`: callback invoked with the edited `Vec<Impact>`.

Behavior:

- Parses `initial_impacts_json` into `Vec<Impact>` using `ImpactList`.
- Renders rows with type, target character(s), field, operation, and value editors.
- Lets the author add/remove rows and switch impact type.
- Shows lightweight previews by calling `apply_impacts_preview` and comparing before/after values per character and relationship.

## PocketBase HTTP integration (Rust)

PocketBase REST calls are straightforward with `reqwest` and `serde_json` (already in `Cargo.toml`). Example snippets that fit into an async context:

```rust
use reqwest::Client;
use serde_json::json;
use crate::models::impacts::{CharacterAttributes, Impact, ImpactList, RelationshipMetrics};
use std::collections::HashMap;

pub async fn load_choice_impacts(client: &Client, base_url: &str, choice_id: &str) -> anyhow::Result<Vec<Impact>> {
    let choice_url = format!("{}/api/collections/choices/records/{}", base_url, choice_id);
    let resp = client.get(&choice_url).send().await?.error_for_status()?;
    let body = resp.json::<serde_json::Value>().await?;
    let impacts_json = body.get("impacts").and_then(|v| v.as_str()).unwrap_or("[]");
    Ok(ImpactList::from_json(impacts_json)?.0)
}

pub async fn save_choice_impacts(client: &Client, base_url: &str, choice_id: &str, impacts: &[Impact]) -> anyhow::Result<()> {
    let choice_url = format!("{}/api/collections/choices/records/{}", base_url, choice_id);
    let payload = json!({ "impacts": impacts });
    client.patch(&choice_url).json(&payload).send().await?.error_for_status()?;
    Ok(())
}

pub async fn load_attributes(client: &Client, base_url: &str) -> anyhow::Result<HashMap<String, CharacterAttributes>> {
    let url = format!("{}/api/collections/attributes/records", base_url);
    let resp = client.get(&url).send().await?.error_for_status()?;
    let json = resp.json::<serde_json::Value>().await?;
    let mut map = HashMap::new();
    if let Some(items) = json.get("items").and_then(|v| v.as_array()) {
        for item in items {
            if let Some(id) = item.get("character_id").and_then(|v| v.as_str()) {
                map.insert(id.to_string(), serde_json::from_value(item.clone())?);
            }
        }
    }
    Ok(map)
}

pub async fn load_relationships(client: &Client, base_url: &str) -> anyhow::Result<HashMap<(String, String), RelationshipMetrics>> {
    let url = format!("{}/api/collections/relationships/records", base_url);
    let resp = client.get(&url).send().await?.error_for_status()?;
    let json = resp.json::<serde_json::Value>().await?;
    let mut map = HashMap::new();
    if let Some(items) = json.get("items").and_then(|v| v.as_array()) {
        for item in items {
            if let (Some(from), Some(to)) = (item.get("from_id"), item.get("to_id")) {
                let key = (from.as_str().unwrap().to_string(), to.as_str().unwrap().to_string());
                map.insert(key, serde_json::from_value(item.clone())?);
            }
        }
    }
    Ok(map)
}
```

The dashboard would call `load_choice_impacts`, populate `ChoiceImpactsEditor` props, and pass `save_choice_impacts` as the `on_save` handler (serializing the returned `Vec<Impact>` back into JSON for PocketBase).

## Runtime consumption

The gameplay engine can reuse `Impact` and `NumericOp` directly. During a choice resolution:

1. Load the relevant `CharacterAttributes` and `RelationshipMetrics` for the characters referenced in the impacts.
2. Run `apply_impacts_preview` (or a similar mutating variant) to compute post-choice values with clamping.
3. Persist the deltas back to the PocketBase `attributes` and `relationships` tables and write any `traits_flags` changes.

Because the schema is tagged and every field name matches a column, the engine can switch over `Impact` and update the appropriate rows without fragile stringly-typed logic.
