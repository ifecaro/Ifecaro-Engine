#[cfg(target_arch = "wasm32")]
use console_error_panic_hook;
#[cfg(target_arch = "wasm32")]
use tracing_wasm;
#[cfg(target_arch = "wasm32")]
use {log::Level, log::LevelFilter, log::Log, log::Metadata, log::Record, web_sys::console};

mod components;
mod constants;
mod contexts;
mod enums;
mod hooks;
mod i18n;
mod layout;
mod models;
mod pages;
mod services;
mod utils;

use crate::{
    contexts::{
        chapter_context::ChapterProvider, language_context::LanguageProvider,
        paragraph_context::ParagraphProvider, settings_context::SettingsContext,
        story_context::StoryContext,
    },
    enums::route::Route,
};
use dioxus::prelude::*;
use dioxus::web;
use dioxus::web::launch::launch_cfg;
use dioxus_router::components::HistoryProvider;
use dioxus_toastr::ToastProvider;
use std::collections::HashMap;
use std::rc::Rc;

#[cfg(target_arch = "wasm32")]
use wasm_bindgen::JsValue;
#[cfg(target_arch = "wasm32")]
use web_sys::UrlSearchParams;

use crate::models::multi_attr_check::{
    run_event_resolution, AttrInfluence, AttrUpdateRule, AttrUpdateRuleMap, EventCheckConfig,
    EventOutcomeTier, InfluenceKind,
};

#[cfg(target_arch = "wasm32")]
fn append_log_line(msg: &str) {
    use web_sys::window;

    let Some(window) = window() else { return };
    let Some(document) = window.document() else {
        return;
    };
    let Some(el) = document.get_element_by_id("debug-log") else {
        return;
    };

    // 很土法，但簡單：舊內容 + <br> + 新的一行文字
    let current = el.inner_html();
    let new_html = if current.is_empty() || current == "(DOM log)" {
        format!("{msg}")
    } else {
        format!("{current}<br>{msg}")
    };

    el.set_inner_html(&new_html);
}

// 小小 macro，方便在任何地方呼叫
macro_rules! log_dom {
    ($($t:tt)*) => {{
        #[cfg(target_arch = "wasm32")]
        {
            append_log_line(&format!($($t)*));
        }
    }};
}

#[cfg(target_arch = "wasm32")]
struct DomLogger;

#[cfg(target_arch = "wasm32")]
static DOM_LOGGER: DomLogger = DomLogger;

#[cfg(target_arch = "wasm32")]
impl Log for DomLogger {
    fn enabled(&self, _metadata: &Metadata) -> bool {
        true
    }

    fn log(&self, record: &Record) {
        if !self.enabled(record.metadata()) {
            return;
        }

        let message = format!("[{}] {}", record.level(), record.args());

        log_dom!("{message}");

        match record.level() {
            Level::Error => console::error_1(&message.into()),
            Level::Warn => console::warn_1(&message.into()),
            Level::Info => console::info_1(&message.into()),
            _ => console::log_1(&message.into()),
        }
    }

    fn flush(&self) {}
}

#[cfg(target_arch = "wasm32")]
fn init_dom_logger() {
    let _ = log::set_logger(&DOM_LOGGER).map(|()| log::set_max_level(LevelFilter::Trace));
}

fn main() {
    #[cfg(target_arch = "wasm32")]
    {
        console_error_panic_hook::set_once();
        tracing_wasm::set_as_global_default();
        init_dom_logger();
        cache_initial_query_and_hash();
    }

    // 這裡一定要指定 root id = "app-root"
    let cfg = web::Config::new().rootname("app-root");
    launch_cfg(App, cfg);
}

#[component]
fn App() -> Element {
    let _settings_context = use_context_provider(|| Signal::new(SettingsContext::default()));

    #[cfg(target_arch = "wasm32")]
    {
        use_effect(restore_initial_query_and_hash_if_stripped);

        let node_runtime = use_signal(GameRuntimeState::default);
        let story_nodes = use_signal(sample_story_nodes);
        let show_node_runtime_demo = should_show_node_runtime_demo();

        rsx! {
            ToastProvider {
                LanguageProvider {
                    ChapterProvider {
                        ParagraphProvider {
                            StoryProvider {
                                HistoryProvider {
                                    history: || -> Rc<dyn History> {
                                        Rc::new(web::WebHistory::new(staging_prefix(), true))
                                    },
                                    Router::<Route> {}
                                }
                            }
                        }
                    }
                }
            }

            if should_show_node_runtime_demo() {
                NodeRuntimePanel {
                    runtime: node_runtime,
                    nodes: story_nodes,
                }
            }
        }
    }

    #[cfg(not(target_arch = "wasm32"))]
    {
        rsx! {
            div {
                class: "min-h-screen flex items-center justify-center bg-gray-100 text-gray-900 dark:bg-gray-900 dark:text-gray-100",
                "Ifecaro is intended to run in a WebAssembly target."
            }
        }
    }
}

#[cfg(target_arch = "wasm32")]
fn cache_initial_query_and_hash() {
    let Some(win) = web_sys::window() else { return };
    let Ok(Some(storage)) = win.session_storage() else {
        return;
    };
    let location = win.location();
    let initial_search = location.search().unwrap_or_default();
    let initial_hash = location.hash().unwrap_or_default();

    let _ = storage.set_item("ifecaro_initial_search", &initial_search);
    let _ = storage.set_item("ifecaro_initial_hash", &initial_hash);
}

#[cfg(target_arch = "wasm32")]
fn restore_initial_query_and_hash_if_stripped() {
    let Some(win) = web_sys::window() else { return };
    let Ok(Some(storage)) = win.session_storage() else {
        return;
    };

    let initial_search = storage
        .get_item("ifecaro_initial_search")
        .ok()
        .flatten()
        .unwrap_or_default();
    let initial_hash = storage
        .get_item("ifecaro_initial_hash")
        .ok()
        .flatten()
        .unwrap_or_default();

    if initial_search.is_empty() && initial_hash.is_empty() {
        return;
    }

    let location = win.location();
    let current_path = location.pathname().unwrap_or_default();

    let current_search = location.search().unwrap_or_default();
    let current_hash = location.hash().unwrap_or_default();
    if current_search == initial_search && current_hash == initial_hash {
        let _ = storage.remove_item("ifecaro_initial_search");
        let _ = storage.remove_item("ifecaro_initial_hash");
        return;
    }

    let new_url = format!("{current_path}{initial_search}{initial_hash}");
    let _ = win
        .history()
        .unwrap()
        .replace_state_with_url(&JsValue::NULL, "", Some(&new_url));

    let _ = storage.remove_item("ifecaro_initial_search");
    let _ = storage.remove_item("ifecaro_initial_hash");
}

#[cfg(target_arch = "wasm32")]
fn should_show_node_runtime_demo() -> bool {
    let search = web_sys::window()
        .and_then(|win| win.location().search().ok())
        .unwrap_or_default();
    let query = search.trim_start_matches('?');
    let params = UrlSearchParams::new_with_str(query)
        .unwrap_or_else(|_| UrlSearchParams::new().expect("failed to build URLSearchParams"));

    params
        .get("node_runtime_demo")
        .is_some_and(|value| value == "1")
}

#[cfg(not(target_arch = "wasm32"))]
fn should_show_node_runtime_demo() -> bool {
    false
}

#[cfg(target_arch = "wasm32")]
fn staging_prefix() -> Option<String> {
    let path = web_sys::window()
        .and_then(|win| win.location().pathname().ok())
        .unwrap_or_default();

    if path == "/staging" || path.starts_with("/staging/") {
        Some("staging".to_string())
    } else {
        None
    }
}

#[cfg(not(target_arch = "wasm32"))]
fn staging_prefix() -> Option<String> {
    None
}

#[derive(Props, Clone, PartialEq)]
struct StoryProviderProps {
    children: Element,
}

#[component]
fn StoryProvider(props: StoryProviderProps) -> Element {
    use_context_provider(|| Signal::new(StoryContext::new()));
    rsx! {
        {props.children}
    }
}

#[derive(Debug, Clone, PartialEq)]
struct StoryNode {
    id: String,
    text: Option<String>,
    text_key: Option<String>,
    next_node_id: Option<String>,
    actor_id: Option<String>,
    check: Option<EventCheckConfig>,
    update_rules: AttrUpdateRuleMap,
    outcomes: HashMap<String, NodeOutcome>,
}

#[derive(Debug, Clone, PartialEq)]
struct NodeOutcome {
    next_node_id: String,
    text: Option<String>,
    text_key: Option<String>,
}

#[derive(Debug, Clone, PartialEq)]
struct NodeResolveResult {
    next_node_id: String,
    outcome_tier: String,
    text: Option<String>,
    text_key: Option<String>,
}

#[derive(Debug, Clone, PartialEq)]
struct GameRuntimeState {
    current_node_id: String,
    current_text: String,
    current_text_key: Option<String>,
    debug_message: Option<String>,
    last_outcome_tier: Option<String>,
}

impl Default for GameRuntimeState {
    fn default() -> Self {
        Self {
            current_node_id: "intro".to_string(),
            current_text: "按下「下一步」開始節點流程。".to_string(),
            current_text_key: None,
            debug_message: None,
            last_outcome_tier: None,
        }
    }
}

#[component]
fn NodeRuntimePanel(
    runtime: Signal<GameRuntimeState>,
    nodes: Signal<HashMap<String, StoryNode>>,
) -> Element {
    let state = runtime.read().clone();
    let outcome_tier = state
        .last_outcome_tier
        .clone()
        .unwrap_or_else(|| "-".to_string());

    rsx! {
        div {
            class: "fixed bottom-4 right-4 z-50 max-w-md rounded bg-white/90 p-4 text-sm shadow-lg dark:bg-black/70",
            h3 { class: "mb-2 font-bold", "Node Runtime Demo" }
            p { "Node: {state.current_node_id}" }
            p { "Outcome tier: {outcome_tier}" }
            p { class: "mt-2 whitespace-pre-wrap", "{state.current_text}" }
            if let Some(text_key) = state.current_text_key.clone() {
                p { class: "text-xs text-gray-600 dark:text-gray-300", "textKey: {text_key}" }
            }
            if let Some(debug_message) = state.debug_message.clone() {
                p { class: "mt-2 text-xs text-red-700 dark:text-red-300", "{debug_message}" }
            }
            button {
                class: "mt-3 rounded bg-blue-600 px-3 py-2 text-white",
                onclick: move |_| {
                    spawn(async move {
                        run_current_node(runtime, nodes).await;
                    });
                },
                "下一步（runCurrentNode）"
            }
        }
    }
}

fn goto_node(next_node_id: String, mut runtime: Signal<GameRuntimeState>) {
    runtime.with_mut(|state| {
        state.current_node_id = next_node_id;
    });
}

async fn run_current_node(
    mut runtime: Signal<GameRuntimeState>,
    nodes: Signal<HashMap<String, StoryNode>>,
) {
    let current_node_id = runtime.read().current_node_id.clone();
    let node = nodes.read().get(&current_node_id).cloned();

    let Some(node) = node else {
        runtime.with_mut(|state| {
            state.debug_message = Some(format!("找不到 node: {current_node_id}"));
        });
        return;
    };

    if node.check.is_none() {
        runtime.with_mut(|state| {
            state.current_text = resolve_node_text(node.text.clone(), node.text_key.clone());
            state.current_text_key = node.text_key.clone();
            state.last_outcome_tier = None;
            state.debug_message = None;
        });

        if let Some(next_node_id) = node.next_node_id.clone() {
            goto_node(next_node_id, runtime);
        }

        return;
    }

    match resolve_node_check_and_jump(&node).await {
        Ok(result) => {
            runtime.with_mut(|state| {
                state.current_text =
                    resolve_node_text(result.text.clone(), result.text_key.clone());
                state.current_text_key = result.text_key.clone();
                state.last_outcome_tier = Some(result.outcome_tier);
                state.debug_message = None;
            });
            goto_node(result.next_node_id, runtime);
        }
        Err(err) => {
            runtime.with_mut(|state| {
                state.debug_message = Some(err);
            });
        }
    }
}

async fn resolve_node_check_and_jump(node: &StoryNode) -> Result<NodeResolveResult, String> {
    let mut check = node
        .check
        .clone()
        .ok_or_else(|| format!("node {} 缺少 check", node.id))?;

    if let Some(actor_id) = node.actor_id.clone() {
        check.actor_id = actor_id;
    }

    let event_run_result = run_event_resolution(&check, &node.update_rules).await?;
    let tier_key = tier_to_key(&event_run_result.resolution.outcome_tier).to_string();
    let outcome = pick_outcome_with_fallback(node, &tier_key)?;

    log::info!(
        "[node-check] node={} actor={} successes={} required={} tier={} next={}",
        node.id,
        check.actor_id,
        event_run_result.resolution.check.successes,
        event_run_result.resolution.check.required_successes,
        tier_key,
        outcome.next_node_id
    );
    log::info!(
        "[node-check] updated attrs persisted to IndexedDB: {:?}",
        event_run_result.updated_attrs
    );

    Ok(NodeResolveResult {
        next_node_id: outcome.next_node_id.clone(),
        outcome_tier: tier_key,
        text: outcome.text.clone(),
        text_key: outcome.text_key.clone(),
    })
}

fn resolve_node_text(text: Option<String>, text_key: Option<String>) -> String {
    if let Some(text) = text {
        return text;
    }

    if let Some(key) = text_key {
        if let Some(resolved) = load_text_by_key(&key) {
            return resolved;
        }

        return format!("[textKey:{key}]");
    }

    "(missing narrative text)".to_string()
}

fn load_text_by_key(text_key: &str) -> Option<String> {
    match text_key {
        "demo.node.intro" => Some("夜幕降臨，前線傳來緊急命令。".to_string()),
        "demo.outcome.success" => Some("你穩住局勢，隊伍順利前進。".to_string()),
        "demo.outcome.failure" => Some("你失去節奏，隊伍陷入混亂。".to_string()),
        _ => None,
    }
}

fn pick_outcome_with_fallback<'a>(
    node: &'a StoryNode,
    tier_key: &str,
) -> Result<&'a NodeOutcome, String> {
    let outcome = match tier_key {
        "great_success" => node
            .outcomes
            .get("great_success")
            .or_else(|| node.outcomes.get("success")),
        "disaster" => node
            .outcomes
            .get("disaster")
            .or_else(|| node.outcomes.get("failure")),
        "mixed" => node
            .outcomes
            .get("mixed")
            .or_else(|| node.outcomes.get("success")),
        other => node.outcomes.get(other),
    }
    .or_else(|| node.outcomes.get("success"))
    .or_else(|| node.outcomes.get("failure"));

    outcome.ok_or_else(|| format!("node {} 找不到可用 outcome（tier={tier_key}）", node.id))
}

fn tier_to_key(tier: &EventOutcomeTier) -> &'static str {
    match tier {
        EventOutcomeTier::GreatSuccess => "great_success",
        EventOutcomeTier::Success => "success",
        EventOutcomeTier::Mixed => "mixed",
        EventOutcomeTier::Failure => "failure",
        EventOutcomeTier::Disaster => "disaster",
    }
}

fn sample_story_nodes() -> HashMap<String, StoryNode> {
    let mut nodes = HashMap::new();

    let mut outcomes = HashMap::new();
    outcomes.insert(
        "success".to_string(),
        NodeOutcome {
            next_node_id: "camp_success".to_string(),
            text: None,
            text_key: Some("demo.outcome.success".to_string()),
        },
    );
    outcomes.insert(
        "failure".to_string(),
        NodeOutcome {
            next_node_id: "camp_failure".to_string(),
            text: None,
            text_key: Some("demo.outcome.failure".to_string()),
        },
    );

    let mut update_rules: AttrUpdateRuleMap = HashMap::new();
    update_rules.insert(
        "courage".to_string(),
        AttrUpdateRule {
            key: "courage".to_string(),
            base_scale: 0.4,
            success_sign: Some(1.0),
            failure_sign: Some(-1.0),
        },
    );
    update_rules.insert(
        "stress".to_string(),
        AttrUpdateRule {
            key: "stress".to_string(),
            base_scale: 0.5,
            success_sign: Some(-1.0),
            failure_sign: Some(1.0),
        },
    );

    nodes.insert(
        "intro".to_string(),
        StoryNode {
            id: "intro".to_string(),
            text: None,
            text_key: Some("demo.node.intro".to_string()),
            next_node_id: None,
            actor_id: Some("demo_actor".to_string()),
            check: Some(EventCheckConfig {
                actor_id: "demo_actor".to_string(),
                influences: vec![
                    AttrInfluence {
                        key: "courage".to_string(),
                        kind: InfluenceKind::Support,
                        die_sides: 6,
                        count_factor: 0.6,
                        weight: Some(1.0),
                    },
                    AttrInfluence {
                        key: "stress".to_string(),
                        kind: InfluenceKind::Resist,
                        die_sides: 6,
                        count_factor: 0.4,
                        weight: Some(1.0),
                    },
                ],
                base_required: 1,
                resist_to_extra_required: 0.5,
                success_threshold: 5,
            }),
            update_rules,
            outcomes,
        },
    );

    nodes.insert(
        "camp_success".to_string(),
        StoryNode {
            id: "camp_success".to_string(),
            text: Some("營地士氣上升，你獲得了新的信任。".to_string()),
            text_key: None,
            next_node_id: None,
            actor_id: None,
            check: None,
            update_rules: HashMap::new(),
            outcomes: HashMap::new(),
        },
    );

    nodes.insert(
        "camp_failure".to_string(),
        StoryNode {
            id: "camp_failure".to_string(),
            text: Some("營地瀰漫著不安，大家對你產生懷疑。".to_string()),
            text_key: None,
            next_node_id: None,
            actor_id: None,
            check: None,
            update_rules: HashMap::new(),
            outcomes: HashMap::new(),
        },
    );

    nodes
}
