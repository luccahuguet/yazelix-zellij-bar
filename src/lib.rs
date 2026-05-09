pub const WIDGET_TRAY_PLACEHOLDER: &str = "__YAZELIX_WIDGET_TRAY__";
pub const CUSTOM_TEXT_PLACEHOLDER: &str = "__YAZELIX_CUSTOM_TEXT_SEGMENT__";
pub const TAB_NORMAL_PLACEHOLDER: &str = "__YAZELIX_ZJSTATUS_TAB_NORMAL__";
pub const TAB_NORMAL_FULLSCREEN_PLACEHOLDER: &str = "__YAZELIX_ZJSTATUS_TAB_NORMAL_FULLSCREEN__";
pub const TAB_NORMAL_SYNC_PLACEHOLDER: &str = "__YAZELIX_ZJSTATUS_TAB_NORMAL_SYNC__";
pub const TAB_ACTIVE_PLACEHOLDER: &str = "__YAZELIX_ZJSTATUS_TAB_ACTIVE__";
pub const TAB_ACTIVE_FULLSCREEN_PLACEHOLDER: &str = "__YAZELIX_ZJSTATUS_TAB_ACTIVE_FULLSCREEN__";
pub const TAB_ACTIVE_SYNC_PLACEHOLDER: &str = "__YAZELIX_ZJSTATUS_TAB_ACTIVE_SYNC__";
pub const TAB_RENAME_PLACEHOLDER: &str = "__YAZELIX_ZJSTATUS_TAB_RENAME__";
pub const ZJSTATUS_PLUGIN_URL_PLACEHOLDER: &str = "__YAZELIX_ZJSTATUS_PLUGIN_URL__";
pub const ZJSTATUS_RUNTIME_DIR_PLACEHOLDER: &str = "__YAZELIX_RUNTIME_DIR__";
pub const ZJSTATUS_NU_BIN_PLACEHOLDER: &str = "__YAZELIX_NU_BIN__";
pub const ZJSTATUS_YZX_CONTROL_BIN_PLACEHOLDER: &str = "__YAZELIX_YZX_CONTROL_BIN__";

pub const WIDGET_EDITOR: &str = "editor";
pub const WIDGET_SHELL: &str = "shell";
pub const WIDGET_TERM: &str = "term";
pub const WIDGET_WORKSPACE: &str = "workspace";
pub const WIDGET_CURSOR: &str = "cursor";
pub const WIDGET_CLAUDE_USAGE: &str = "claude_usage";
pub const WIDGET_CODEX_USAGE: &str = "codex_usage";
pub const WIDGET_OPENCODE_GO_USAGE: &str = "opencode_go_usage";
pub const WIDGET_CPU: &str = "cpu";
pub const WIDGET_RAM: &str = "ram";

pub const COMMAND_WORKSPACE: &str = "{command_workspace}";
pub const COMMAND_CURSOR: &str = "{command_cursor}";
pub const COMMAND_CLAUDE_USAGE: &str = "{command_claude_usage}";
pub const COMMAND_CODEX_USAGE: &str = "{command_codex_usage}";
pub const COMMAND_OPENCODE_GO_USAGE: &str = "{command_opencode_go_usage}";
pub const COMMAND_CPU: &str = "{command_cpu}";
pub const COMMAND_RAM: &str = "{command_ram}";
pub const COMMAND_VERSION: &str = "{command_version}";
pub const TAB_LABEL_MODE_FULL: &str = "full";
pub const TAB_LABEL_MODE_COMPACT: &str = "compact";
pub const DEFAULT_STANDALONE_WASM_URL: &str = "__YAZELIX_BAR_ZJSTATUS_WASM__";

pub const DEFAULT_WIDGET_TRAY: &[&str] = &[
    WIDGET_EDITOR,
    WIDGET_SHELL,
    WIDGET_TERM,
    WIDGET_CURSOR,
    WIDGET_CODEX_USAGE,
    WIDGET_CPU,
    WIDGET_RAM,
];

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BarRenderRequest {
    pub widget_tray: Vec<String>,
    pub editor_label: String,
    pub shell_label: String,
    pub terminal_label: String,
    pub custom_text: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BarRenderData {
    pub widget_tray_segment: String,
    pub custom_text_segment: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct YazelixRuntimeCommandPaths {
    pub nu_bin: String,
    pub yzx_control_bin: String,
    pub yazelix_bar_widget_bin: String,
    pub runtime_dir: String,
    pub claude_usage_display: String,
    pub codex_usage_display: String,
    pub opencode_go_usage_display: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TabLabelFormats {
    pub tab_normal: &'static str,
    pub tab_normal_fullscreen: &'static str,
    pub tab_normal_sync: &'static str,
    pub tab_active: &'static str,
    pub tab_active_fullscreen: &'static str,
    pub tab_active_sync: &'static str,
    pub tab_rename: &'static str,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BarRenderError {
    InvalidWidgetTrayEntry { entry: String },
    InvalidTabLabelMode { mode: String },
}

impl BarRenderError {
    pub fn code(&self) -> &'static str {
        match self {
            Self::InvalidWidgetTrayEntry { .. } => "invalid_widget_tray_entry",
            Self::InvalidTabLabelMode { .. } => "invalid_tab_label_mode",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StandalonePresetOptions {
    pub wasm_url: String,
    pub brand_label: String,
    pub brand_color: String,
    pub session_color: String,
    pub datetime_color: String,
    pub datetime_format: String,
    pub tab_label_mode: String,
    pub format_left: Vec<StandalonePresetPart>,
    pub format_center: Vec<StandalonePresetPart>,
    pub format_right: Vec<StandalonePresetPart>,
    pub command_widgets: Vec<StandaloneCommandWidget>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum StandalonePresetPart {
    Mode,
    Tabs,
    Session,
    Datetime,
    Brand,
    Command(String),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StandaloneCommandWidget {
    pub name: String,
    pub command: String,
    pub format: String,
    pub interval: String,
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct CursorWidgetFacts {
    pub name: String,
    pub color: Option<String>,
    pub family: Option<String>,
    pub divider: Option<String>,
    pub primary_color: Option<String>,
    pub secondary_color: Option<String>,
}

const CURSOR_NAME_FACT: &str = "YAZELIX_CURSOR_NAME";
const CURSOR_COLOR_FACT: &str = "YAZELIX_CURSOR_COLOR";
const CURSOR_FAMILY_FACT: &str = "YAZELIX_CURSOR_FAMILY";
const CURSOR_DIVIDER_FACT: &str = "YAZELIX_CURSOR_DIVIDER";
const CURSOR_PRIMARY_COLOR_FACT: &str = "YAZELIX_CURSOR_PRIMARY_COLOR";
const CURSOR_SECONDARY_COLOR_FACT: &str = "YAZELIX_CURSOR_SECONDARY_COLOR";

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AgentUsageDisplay {
    Both,
    Token,
    Quota,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AgentUsagePeriod {
    FiveHour,
    Weekly,
    Monthly,
}

impl AgentUsagePeriod {
    pub fn short_label(self) -> &'static str {
        match self {
            Self::FiveHour => "5h",
            Self::Weekly => "wk",
            Self::Monthly => "mo",
        }
    }
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct WindowedAgentUsageFacts {
    pub updated_at_unix_seconds: Option<u64>,
    pub five_hour_tokens: Option<u64>,
    pub weekly_tokens: Option<u64>,
    pub monthly_tokens: Option<u64>,
    pub five_hour_remaining_percent: Option<u64>,
    pub weekly_remaining_percent: Option<u64>,
    pub monthly_remaining_percent: Option<u64>,
    pub five_hour_reset_at_unix_seconds: Option<u64>,
    pub weekly_reset_at_unix_seconds: Option<u64>,
    pub five_hour_window_seconds: Option<u64>,
    pub weekly_window_seconds: Option<u64>,
    pub error: Option<String>,
}

impl WindowedAgentUsageFacts {
    fn has_tokens(&self) -> bool {
        self.five_hour_tokens.is_some()
            || self.weekly_tokens.is_some()
            || self.monthly_tokens.is_some()
    }

    fn has_quota(&self) -> bool {
        self.five_hour_remaining_percent.is_some()
            || self.weekly_remaining_percent.is_some()
            || self.monthly_remaining_percent.is_some()
    }

    fn is_empty(&self) -> bool {
        !self.has_tokens() && !self.has_quota()
    }
}

pub const CODEX_USAGE_CACHE_SCHEMA_VERSION: i64 = 2;
pub const CLAUDE_USAGE_CACHE_SCHEMA_VERSION: i64 = 1;
pub const OPENCODE_GO_USAGE_CACHE_SCHEMA_VERSION: i64 = 1;
pub const CODEX_USAGE_LOCK_STALE_AFTER_SECONDS: u64 = 300;
pub const CLAUDE_USAGE_LOCK_STALE_AFTER_SECONDS: u64 = 300;
pub const OPENCODE_GO_USAGE_LOCK_STALE_AFTER_SECONDS: u64 = 300;
pub const OPENCODE_GO_PROVIDER_ID: &str = "opencode-go";
pub const OPENCODE_GO_FIVE_HOUR_SECONDS: u64 = 5 * 60 * 60;
pub const OPENCODE_GO_WEEK_SECONDS: u64 = 7 * 24 * 60 * 60;
pub const OPENCODE_GO_MONTH_SECONDS: u64 = 30 * 24 * 60 * 60;
pub const OPENCODE_GO_FIVE_HOUR_LIMIT_USD: f64 = 12.0;
pub const OPENCODE_GO_WEEKLY_LIMIT_USD: f64 = 30.0;
pub const OPENCODE_GO_MONTHLY_LIMIT_USD: f64 = 60.0;
const AGENT_USAGE_MINUTE_SECONDS: u64 = 60;

#[derive(Debug, Clone, Copy)]
pub struct CodexUsageWidgetOptions<'a> {
    pub cache_path: &'a std::path::Path,
    pub path_var: Option<&'a std::ffi::OsStr>,
    pub now_unix_seconds: u64,
    pub max_age_seconds: u64,
    pub error_backoff_seconds: u64,
    pub timeout: std::time::Duration,
    pub display: AgentUsageDisplay,
}

#[derive(Debug, Clone, Copy)]
pub struct ClaudeUsageWidgetOptions<'a> {
    pub cache_path: &'a std::path::Path,
    pub path_var: Option<&'a std::ffi::OsStr>,
    pub now_unix_seconds: u64,
    pub max_age_seconds: u64,
    pub error_backoff_seconds: u64,
    pub timeout: std::time::Duration,
    pub display: AgentUsageDisplay,
}

#[derive(Debug, Clone, Copy)]
pub struct OpenCodeGoUsageWidgetOptions<'a> {
    pub cache_path: &'a std::path::Path,
    pub db_paths: &'a [std::path::PathBuf],
    pub now_unix_seconds: u64,
    pub max_age_seconds: u64,
    pub error_backoff_seconds: u64,
    pub display: AgentUsageDisplay,
}

pub fn codex_usage_widget_text(options: CodexUsageWidgetOptions<'_>) -> Result<String, String> {
    refresh_codex_usage_shared_cache(
        options.cache_path,
        options.path_var,
        options.now_unix_seconds,
        options.max_age_seconds,
        options.error_backoff_seconds,
        options.timeout,
    )?;
    Ok(render_codex_usage_widget_from_cache(
        options.cache_path,
        options.display,
    ))
}

pub fn claude_usage_widget_text(options: ClaudeUsageWidgetOptions<'_>) -> Result<String, String> {
    refresh_claude_usage_shared_cache(
        options.cache_path,
        options.path_var,
        options.now_unix_seconds,
        options.max_age_seconds,
        options.error_backoff_seconds,
        options.timeout,
    )?;
    Ok(render_claude_usage_widget_from_cache(
        options.cache_path,
        options.display,
    ))
}

#[cfg(feature = "providers")]
pub fn opencode_go_usage_widget_text(
    options: OpenCodeGoUsageWidgetOptions<'_>,
) -> Result<String, String> {
    refresh_opencode_go_usage_shared_cache(
        options.cache_path,
        options.db_paths,
        options.now_unix_seconds,
        options.max_age_seconds,
        options.error_backoff_seconds,
    )?;
    Ok(render_opencode_go_usage_widget_from_cache(
        options.cache_path,
        options.display,
    ))
}

pub fn render_opencode_go_usage_widget_from_cache(
    path: &std::path::Path,
    display: AgentUsageDisplay,
) -> String {
    read_opencode_go_usage_shared_cache_value(path)
        .and_then(|cache| cache.get("opencode_go").cloned())
        .map(|entry| {
            render_windowed_agent_usage_status_widget(
                "go",
                &windowed_usage_facts_from_cache_entry(&entry),
                &[
                    AgentUsagePeriod::FiveHour,
                    AgentUsagePeriod::Weekly,
                    AgentUsagePeriod::Monthly,
                ],
                display,
            )
        })
        .unwrap_or_default()
}

pub fn render_claude_usage_widget_from_cache(
    path: &std::path::Path,
    display: AgentUsageDisplay,
) -> String {
    read_claude_usage_shared_cache_value(path)
        .and_then(|cache| cache.get("claude").cloned())
        .map(|entry| {
            render_windowed_agent_usage_status_widget(
                "claude",
                &windowed_usage_facts_from_cache_entry(&entry),
                &[AgentUsagePeriod::FiveHour, AgentUsagePeriod::Weekly],
                display,
            )
        })
        .unwrap_or_default()
}

pub fn render_codex_usage_widget_from_cache(
    path: &std::path::Path,
    display: AgentUsageDisplay,
) -> String {
    read_codex_usage_shared_cache_value(path)
        .and_then(|cache| cache.get("codex").cloned())
        .map(|entry| {
            render_codex_usage_status_widget(
                &windowed_usage_facts_from_cache_entry(&entry),
                display,
            )
        })
        .unwrap_or_default()
}

pub fn read_codex_usage_shared_cache_value(path: &std::path::Path) -> Option<serde_json::Value> {
    let raw = std::fs::read_to_string(path).ok()?;
    let cache: serde_json::Value = serde_json::from_str(&raw).ok()?;
    if cache
        .get("schema_version")
        .and_then(serde_json::Value::as_i64)
        != Some(CODEX_USAGE_CACHE_SCHEMA_VERSION)
    {
        return None;
    }
    Some(cache)
}

pub fn read_claude_usage_shared_cache_value(path: &std::path::Path) -> Option<serde_json::Value> {
    let raw = std::fs::read_to_string(path).ok()?;
    let cache: serde_json::Value = serde_json::from_str(&raw).ok()?;
    if cache
        .get("schema_version")
        .and_then(serde_json::Value::as_i64)
        != Some(CLAUDE_USAGE_CACHE_SCHEMA_VERSION)
    {
        return None;
    }
    Some(cache)
}

pub fn read_opencode_go_usage_shared_cache_value(
    path: &std::path::Path,
) -> Option<serde_json::Value> {
    let raw = std::fs::read_to_string(path).ok()?;
    let cache: serde_json::Value = serde_json::from_str(&raw).ok()?;
    if cache
        .get("schema_version")
        .and_then(serde_json::Value::as_i64)
        != Some(OPENCODE_GO_USAGE_CACHE_SCHEMA_VERSION)
    {
        return None;
    }
    Some(cache)
}

pub fn claude_usage_cache_path_from_env() -> Option<std::path::PathBuf> {
    status_bar_cache_path_from_env().and_then(|path| {
        agent_usage_cache_path_from_status_cache_path(
            &path,
            "claude_usage_cache",
            CLAUDE_USAGE_CACHE_SCHEMA_VERSION,
        )
    })
}

pub fn codex_usage_cache_path_from_env() -> Option<std::path::PathBuf> {
    status_bar_cache_path_from_env().and_then(|path| {
        agent_usage_cache_path_from_status_cache_path(
            &path,
            "codex_usage_cache",
            CODEX_USAGE_CACHE_SCHEMA_VERSION,
        )
    })
}

pub fn opencode_go_usage_cache_path_from_env() -> Option<std::path::PathBuf> {
    status_bar_cache_path_from_env().and_then(|path| {
        agent_usage_cache_path_from_status_cache_path(
            &path,
            "opencode_go_usage_cache",
            OPENCODE_GO_USAGE_CACHE_SCHEMA_VERSION,
        )
    })
}

pub fn agent_usage_cache_path_from_status_cache_path(
    status_cache_path: &std::path::Path,
    file_stem: &str,
    schema_version: i64,
) -> Option<std::path::PathBuf> {
    let state_dir = status_cache_path.parent()?.parent()?.parent()?;
    Some(
        state_dir
            .join("agent_usage")
            .join(format!("{file_stem}_v{schema_version}.json")),
    )
}

fn status_bar_cache_path_from_env() -> Option<std::path::PathBuf> {
    std::env::var_os("YAZELIX_STATUS_BAR_CACHE_PATH")
        .map(std::path::PathBuf::from)
        .filter(|path| !path.as_os_str().is_empty())
}

pub fn refresh_codex_usage_shared_cache(
    shared_path: &std::path::Path,
    path_var: Option<&std::ffi::OsStr>,
    now: u64,
    max_age_seconds: u64,
    error_backoff_seconds: u64,
    timeout: std::time::Duration,
) -> Result<bool, String> {
    if codex_usage_shared_cache_is_fresh(shared_path, now, max_age_seconds)
        || codex_usage_shared_cache_is_backing_off(shared_path, now)
    {
        return Ok(false);
    }
    let Some(_lock) = try_acquire_codex_usage_cache_lock(shared_path, now)? else {
        return Ok(false);
    };
    if codex_usage_shared_cache_is_fresh(shared_path, now, max_age_seconds)
        || codex_usage_shared_cache_is_backing_off(shared_path, now)
    {
        return Ok(false);
    }

    let quota_backoff_until = codex_usage_quota_backoff_until(shared_path, now);
    let previous_facts = read_codex_usage_shared_cache_value(shared_path)
        .and_then(|cache| cache.get("codex").cloned())
        .map(|entry| windowed_usage_facts_from_cache_entry(&entry));
    let mut facts = collect_codex_usage_facts(path_var, timeout, quota_backoff_until.is_none());
    let quota_probe_failed = quota_backoff_until.is_none() && !facts.has_quota();
    preserve_previous_codex_window_tokens(&mut facts, previous_facts.as_ref());
    preserve_previous_codex_window_quota(&mut facts, previous_facts.as_ref(), now);

    let mut entry = serde_json::Map::new();
    entry.insert(
        "updated_at_unix_seconds".to_string(),
        serde_json::json!(now),
    );
    insert_u64(&mut entry, "five_hour_tokens", facts.five_hour_tokens);
    insert_u64(&mut entry, "weekly_tokens", facts.weekly_tokens);
    insert_u64(
        &mut entry,
        "five_hour_remaining_percent",
        facts.five_hour_remaining_percent,
    );
    insert_u64(
        &mut entry,
        "weekly_remaining_percent",
        facts.weekly_remaining_percent,
    );
    insert_u64(
        &mut entry,
        "five_hour_reset_at_unix_seconds",
        facts.five_hour_reset_at_unix_seconds,
    );
    insert_u64(
        &mut entry,
        "weekly_reset_at_unix_seconds",
        facts.weekly_reset_at_unix_seconds,
    );
    insert_u64(
        &mut entry,
        "five_hour_window_seconds",
        facts.five_hour_window_seconds,
    );
    insert_u64(
        &mut entry,
        "weekly_window_seconds",
        facts.weekly_window_seconds,
    );
    if let Some(error) = facts.error.as_deref().filter(|value| !value.is_empty()) {
        entry.insert("error".to_string(), serde_json::json!(error));
        if facts.is_empty() {
            entry.insert(
                "backoff_until_unix_seconds".to_string(),
                serde_json::json!(now.saturating_add(error_backoff_seconds)),
            );
        }
    }
    if let Some(backoff_until) = quota_backoff_until {
        entry.insert(
            "quota_backoff_until_unix_seconds".to_string(),
            serde_json::json!(backoff_until),
        );
    } else if facts.has_tokens() && (quota_probe_failed || !facts.has_quota()) {
        entry.insert(
            "quota_backoff_until_unix_seconds".to_string(),
            serde_json::json!(now.saturating_add(error_backoff_seconds)),
        );
    }
    let status = if facts.is_empty() {
        "error"
    } else if facts.has_tokens()
        && facts.has_quota()
        && !quota_probe_failed
        && quota_backoff_until.is_none()
    {
        "ok"
    } else {
        "partial"
    };
    entry.insert("status".to_string(), serde_json::json!(status));

    write_json_value_atomic(
        shared_path,
        &serde_json::json!({
            "schema_version": CODEX_USAGE_CACHE_SCHEMA_VERSION,
            "codex": serde_json::Value::Object(entry),
        }),
    )?;
    Ok(true)
}

pub fn refresh_claude_usage_shared_cache(
    shared_path: &std::path::Path,
    path_var: Option<&std::ffi::OsStr>,
    now: u64,
    max_age_seconds: u64,
    error_backoff_seconds: u64,
    timeout: std::time::Duration,
) -> Result<bool, String> {
    if claude_usage_shared_cache_is_fresh(shared_path, now, max_age_seconds)
        || claude_usage_shared_cache_is_backing_off(shared_path, now)
    {
        return Ok(false);
    }
    let Some(_lock) = try_acquire_claude_usage_cache_lock(shared_path, now)? else {
        return Ok(false);
    };
    if claude_usage_shared_cache_is_fresh(shared_path, now, max_age_seconds)
        || claude_usage_shared_cache_is_backing_off(shared_path, now)
    {
        return Ok(false);
    }

    let quota_backoff_until = claude_usage_quota_backoff_until(shared_path, now);
    let previous_facts = read_claude_usage_shared_cache_value(shared_path)
        .and_then(|cache| cache.get("claude").cloned())
        .map(|entry| windowed_usage_facts_from_cache_entry(&entry));
    let mut facts = collect_claude_usage_facts(path_var, timeout, quota_backoff_until.is_none());
    let quota_probe_failed = quota_backoff_until.is_none() && !facts.has_quota();
    preserve_previous_claude_window_quota(&mut facts, previous_facts.as_ref());

    let mut entry = serde_json::Map::new();
    entry.insert(
        "updated_at_unix_seconds".to_string(),
        serde_json::json!(now),
    );
    insert_u64(&mut entry, "five_hour_tokens", facts.five_hour_tokens);
    insert_u64(&mut entry, "weekly_tokens", facts.weekly_tokens);
    insert_u64(
        &mut entry,
        "five_hour_remaining_percent",
        facts.five_hour_remaining_percent,
    );
    insert_u64(
        &mut entry,
        "weekly_remaining_percent",
        facts.weekly_remaining_percent,
    );
    if let Some(error) = facts.error.as_deref().filter(|value| !value.is_empty()) {
        entry.insert("error".to_string(), serde_json::json!(error));
        if facts.is_empty() {
            entry.insert(
                "backoff_until_unix_seconds".to_string(),
                serde_json::json!(now.saturating_add(error_backoff_seconds)),
            );
        }
    }
    if let Some(backoff_until) = quota_backoff_until {
        entry.insert(
            "quota_backoff_until_unix_seconds".to_string(),
            serde_json::json!(backoff_until),
        );
    } else if facts.has_tokens() && (quota_probe_failed || !facts.has_quota()) {
        entry.insert(
            "quota_backoff_until_unix_seconds".to_string(),
            serde_json::json!(now.saturating_add(error_backoff_seconds)),
        );
    }
    let status = if facts.is_empty() {
        "error"
    } else if facts.has_tokens()
        && facts.has_quota()
        && !quota_probe_failed
        && quota_backoff_until.is_none()
    {
        "ok"
    } else {
        "partial"
    };
    entry.insert("status".to_string(), serde_json::json!(status));

    write_json_value_atomic(
        shared_path,
        &serde_json::json!({
            "schema_version": CLAUDE_USAGE_CACHE_SCHEMA_VERSION,
            "claude": serde_json::Value::Object(entry),
        }),
    )?;
    Ok(true)
}

#[cfg(feature = "providers")]
pub fn refresh_opencode_go_usage_shared_cache(
    shared_path: &std::path::Path,
    db_paths: &[std::path::PathBuf],
    now: u64,
    max_age_seconds: u64,
    error_backoff_seconds: u64,
) -> Result<bool, String> {
    if opencode_go_usage_shared_cache_is_fresh(shared_path, now, max_age_seconds)
        || opencode_go_usage_shared_cache_is_backing_off(shared_path, now)
    {
        return Ok(false);
    }
    let Some(_lock) = try_acquire_opencode_go_usage_cache_lock(shared_path, now)? else {
        return Ok(false);
    };
    if opencode_go_usage_shared_cache_is_fresh(shared_path, now, max_age_seconds)
        || opencode_go_usage_shared_cache_is_backing_off(shared_path, now)
    {
        return Ok(false);
    }

    let facts = collect_opencode_go_usage_facts_from_dbs(db_paths, now);
    let mut entry = serde_json::Map::new();
    entry.insert(
        "updated_at_unix_seconds".to_string(),
        serde_json::json!(now),
    );
    insert_u64(&mut entry, "five_hour_tokens", facts.five_hour_tokens);
    insert_u64(&mut entry, "weekly_tokens", facts.weekly_tokens);
    insert_u64(&mut entry, "monthly_tokens", facts.monthly_tokens);
    insert_u64(
        &mut entry,
        "five_hour_remaining_percent",
        facts.five_hour_remaining_percent,
    );
    insert_u64(
        &mut entry,
        "weekly_remaining_percent",
        facts.weekly_remaining_percent,
    );
    insert_u64(
        &mut entry,
        "monthly_remaining_percent",
        facts.monthly_remaining_percent,
    );
    if let Some(error) = facts.error.as_deref().filter(|value| !value.is_empty()) {
        entry.insert("error".to_string(), serde_json::json!(error));
        entry.insert(
            "backoff_until_unix_seconds".to_string(),
            serde_json::json!(now.saturating_add(error_backoff_seconds)),
        );
    }
    let status = if facts.is_empty() {
        "error"
    } else if facts.has_tokens() && facts.has_quota() {
        "ok"
    } else {
        "partial"
    };
    entry.insert("status".to_string(), serde_json::json!(status));

    write_json_value_atomic(
        shared_path,
        &serde_json::json!({
            "schema_version": OPENCODE_GO_USAGE_CACHE_SCHEMA_VERSION,
            "opencode_go": serde_json::Value::Object(entry),
        }),
    )?;
    Ok(true)
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum StandalonePresetError {
    InvalidTabLabelMode { mode: String },
    InvalidColor { field: String, value: String },
    InvalidCommandName { name: String },
    EmptyCommand { name: String },
    DuplicateCommandName { name: String },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct YazelixRuntimeCommandWidget {
    name: &'static str,
    command: YazelixRuntimeCommand,
    format: &'static str,
    render_mode: Option<&'static str>,
    interval: &'static str,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum YazelixRuntimeCommand {
    StatusCacheWidget(&'static str),
    BarWidget(&'static str),
    ClaudeUsage,
    CodexUsage,
    OpenCodeGoUsage,
    RuntimeNuConstantsVersion,
}

const YAZELIX_RUNTIME_COMMAND_WIDGETS: &[YazelixRuntimeCommandWidget] = &[
    YazelixRuntimeCommandWidget {
        name: "workspace",
        command: YazelixRuntimeCommand::StatusCacheWidget("workspace"),
        format: "#[fg=#00ff88,bold]{stdout}",
        render_mode: None,
        interval: "1",
    },
    YazelixRuntimeCommandWidget {
        name: "cursor",
        command: YazelixRuntimeCommand::BarWidget("cursor"),
        format: "{stdout}",
        render_mode: Some("dynamic"),
        interval: "10",
    },
    YazelixRuntimeCommandWidget {
        name: "claude_usage",
        command: YazelixRuntimeCommand::ClaudeUsage,
        format: "#[fg=#bb88ff,bold]{stdout}",
        render_mode: None,
        interval: "10",
    },
    YazelixRuntimeCommandWidget {
        name: "codex_usage",
        command: YazelixRuntimeCommand::CodexUsage,
        format: "#[fg=#bb88ff,bold]{stdout}",
        render_mode: None,
        interval: "10",
    },
    YazelixRuntimeCommandWidget {
        name: "opencode_go_usage",
        command: YazelixRuntimeCommand::OpenCodeGoUsage,
        format: "#[fg=#bb88ff,bold]{stdout}",
        render_mode: None,
        interval: "10",
    },
    YazelixRuntimeCommandWidget {
        name: "cpu",
        command: YazelixRuntimeCommand::BarWidget("cpu"),
        format: " #[fg=#ff6600][cpu {stdout}]",
        render_mode: None,
        interval: "5",
    },
    YazelixRuntimeCommandWidget {
        name: "ram",
        command: YazelixRuntimeCommand::BarWidget("ram"),
        format: " #[fg=#ff6600][ram {stdout}]",
        render_mode: None,
        interval: "5",
    },
    YazelixRuntimeCommandWidget {
        name: "version",
        command: YazelixRuntimeCommand::RuntimeNuConstantsVersion,
        format: "{stdout}",
        render_mode: None,
        interval: "3600",
    },
];

impl Default for StandalonePresetOptions {
    fn default() -> Self {
        Self {
            wasm_url: DEFAULT_STANDALONE_WASM_URL.to_string(),
            brand_label: "YAZELIX BAR".to_string(),
            brand_color: "#00ccff".to_string(),
            session_color: "#ff0088".to_string(),
            datetime_color: "#bb88ff".to_string(),
            datetime_format: "%H:%M %d/%m".to_string(),
            tab_label_mode: TAB_LABEL_MODE_FULL.to_string(),
            format_left: vec![StandalonePresetPart::Mode, StandalonePresetPart::Tabs],
            format_center: Vec::new(),
            format_right: vec![
                StandalonePresetPart::Session,
                StandalonePresetPart::Datetime,
                StandalonePresetPart::Brand,
            ],
            command_widgets: Vec::new(),
        }
    }
}

impl StandaloneCommandWidget {
    pub fn new(name: impl Into<String>, command: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            command: command.into(),
            format: " #[fg=#00ff88,bold][{stdout}]".to_string(),
            interval: "30".to_string(),
        }
    }
}

pub fn render_yazelix_runtime_command_definitions(paths: &YazelixRuntimeCommandPaths) -> String {
    YAZELIX_RUNTIME_COMMAND_WIDGETS
        .iter()
        .map(|widget| render_yazelix_runtime_command_definition(widget, paths))
        .collect::<Vec<_>>()
        .join("\n\n")
}

fn render_yazelix_runtime_command_definition(
    widget: &YazelixRuntimeCommandWidget,
    paths: &YazelixRuntimeCommandPaths,
) -> String {
    let command = widget.command.render(paths);
    let mut lines = vec![
        format!(
            "    command_{}_command \"{}\"",
            widget.name,
            escape_kdl_string(&command)
        ),
        format!(
            "    command_{}_format \"{}\"",
            widget.name,
            escape_kdl_string(widget.format)
        ),
    ];
    if let Some(render_mode) = widget.render_mode {
        lines.push(format!(
            "    command_{}_rendermode \"{}\"",
            widget.name,
            escape_kdl_string(render_mode)
        ));
    }
    lines.push(format!(
        "    command_{}_interval \"{}\"",
        widget.name,
        escape_kdl_string(widget.interval)
    ));
    lines.join("\n")
}

impl YazelixRuntimeCommand {
    fn render(self, paths: &YazelixRuntimeCommandPaths) -> String {
        match self {
            Self::StatusCacheWidget(widget) => {
                format!(
                    "{} zellij status-cache-widget {widget}",
                    paths.yzx_control_bin
                )
            }
            Self::BarWidget(widget) => format!("{} {widget}", paths.yazelix_bar_widget_bin),
            Self::ClaudeUsage => format!(
                "{} claude_usage --display {}",
                paths.yazelix_bar_widget_bin, paths.claude_usage_display
            ),
            Self::CodexUsage => format!(
                "{} codex_usage --display {}",
                paths.yazelix_bar_widget_bin, paths.codex_usage_display
            ),
            Self::OpenCodeGoUsage => format!(
                "{} opencode_go_usage --display {}",
                paths.yazelix_bar_widget_bin, paths.opencode_go_usage_display
            ),
            Self::RuntimeNuConstantsVersion => format!(
                "{} -c 'use {}/nushell/scripts/utils/constants.nu YAZELIX_VERSION; $YAZELIX_VERSION'",
                paths.nu_bin, paths.runtime_dir
            ),
        }
    }
}

pub fn generate_standalone_preset(
    options: &StandalonePresetOptions,
) -> Result<String, StandalonePresetError> {
    validate_hex_color("brand_color", &options.brand_color)?;
    validate_hex_color("session_color", &options.session_color)?;
    validate_hex_color("datetime_color", &options.datetime_color)?;
    let tab_labels = render_zjstatus_tab_label_formats(&options.tab_label_mode).map_err(|_| {
        StandalonePresetError::InvalidTabLabelMode {
            mode: options.tab_label_mode.clone(),
        }
    })?;
    validate_commands(&options.command_widgets)?;

    let mut lines = vec![
        format!(
            "plugin location=\"{}\" {{",
            escape_kdl_string(&options.wasm_url)
        ),
        "    // Generated by yazelix_bar_generate. Edit the options or raw KDL.".to_string(),
        format!(
            "    format_left   \"{}\"",
            escape_kdl_string(&render_standalone_format(&options.format_left, options))
        ),
        format!(
            "    format_center \"{}\"",
            escape_kdl_string(&render_standalone_format(&options.format_center, options))
        ),
        format!(
            "    format_right  \"{}\"",
            escape_kdl_string(&render_standalone_format(&options.format_right, options))
        ),
        "    format_hide_on_overlength \"true\"".to_string(),
        "    format_precedence \"lrc\"".to_string(),
        "    format_space  \"\"".to_string(),
        String::new(),
        "    border_enabled  \"false\"".to_string(),
        String::new(),
        "    mode_normal  \"#[bg=#00ff88,fg=#000000,bold] NORMAL \"".to_string(),
        "    mode_tmux    \"#[bg=#ffff00,fg=#000000,bold] TMUX \"".to_string(),
        "    mode_session \"#[bg=#ff6600,fg=#000000,bold] SESSION \"".to_string(),
        "    mode_scroll  \"#[bg=#ff0088,fg=#ffffff,bold] SCROLL \"".to_string(),
        String::new(),
        format!("    {}", tab_labels.tab_normal),
        format!("    {}", tab_labels.tab_normal_fullscreen),
        format!("    {}", tab_labels.tab_normal_sync),
        format!("    {}", tab_labels.tab_active),
        format!("    {}", tab_labels.tab_active_fullscreen),
        format!("    {}", tab_labels.tab_active_sync),
        "    tab_separator \"\"".to_string(),
        format!("    {}", tab_labels.tab_rename),
        "    tab_sync_indicator       \"<> \"".to_string(),
        "    tab_fullscreen_indicator \"[] \"".to_string(),
        "    tab_floating_indicator   \"o \"".to_string(),
        "    tab_display_count \"6\"".to_string(),
        "    tab_truncate_start_format \"#[fg=#ff6600,bold]< +{count} ... \"".to_string(),
        "    tab_truncate_end_format   \"#[fg=#ff6600,bold]... +{count} > \"".to_string(),
        String::new(),
        format!(
            "    datetime        \"#[fg={},bold] {{format}} \"",
            options.datetime_color
        ),
        format!(
            "    datetime_format \"{}\"",
            escape_kdl_string(&options.datetime_format)
        ),
    ];

    if !options.command_widgets.is_empty() {
        lines.push(String::new());
        for command in &options.command_widgets {
            lines.push(format!(
                "    command_{}_command \"{}\"",
                command.name,
                escape_kdl_string(&command.command)
            ));
            lines.push(format!(
                "    command_{}_format \"{}\"",
                command.name,
                escape_kdl_string(&command.format)
            ));
            lines.push(format!(
                "    command_{}_interval \"{}\"",
                command.name,
                escape_kdl_string(&command.interval)
            ));
            lines.push(String::new());
        }
        if lines.last().is_some_and(String::is_empty) {
            lines.pop();
        }
    }

    lines.push("}".to_string());
    lines.push(String::new());
    Ok(lines.join("\n"))
}

pub fn standalone_part_from_token(token: &str) -> Option<StandalonePresetPart> {
    let token = token.trim();
    match token {
        "mode" => Some(StandalonePresetPart::Mode),
        "tabs" => Some(StandalonePresetPart::Tabs),
        "session" => Some(StandalonePresetPart::Session),
        "datetime" => Some(StandalonePresetPart::Datetime),
        "brand" => Some(StandalonePresetPart::Brand),
        _ => token
            .strip_prefix("command:")
            .map(str::trim)
            .filter(|name| !name.is_empty())
            .map(|name| StandalonePresetPart::Command(name.to_string())),
    }
}

fn render_standalone_format(
    parts: &[StandalonePresetPart],
    options: &StandalonePresetOptions,
) -> String {
    let rendered = parts
        .iter()
        .map(|part| match part {
            StandalonePresetPart::Mode => "{mode}".to_string(),
            StandalonePresetPart::Tabs => "{tabs}".to_string(),
            StandalonePresetPart::Session => {
                format!("#[fg={},bold]{{session}}", options.session_color)
            }
            StandalonePresetPart::Datetime => "{datetime}".to_string(),
            StandalonePresetPart::Brand => {
                format!("#[fg={},bold]{}", options.brand_color, options.brand_label)
            }
            StandalonePresetPart::Command(name) => format!("{{command_{name}}}"),
        })
        .collect::<Vec<_>>()
        .join(" ");
    if rendered.is_empty() {
        rendered
    } else {
        format!("{rendered} ")
    }
}

fn validate_commands(commands: &[StandaloneCommandWidget]) -> Result<(), StandalonePresetError> {
    let mut names = std::collections::BTreeSet::new();
    for command in commands {
        validate_command_name(&command.name)?;
        if command.command.trim().is_empty() {
            return Err(StandalonePresetError::EmptyCommand {
                name: command.name.clone(),
            });
        }
        if !names.insert(command.name.clone()) {
            return Err(StandalonePresetError::DuplicateCommandName {
                name: command.name.clone(),
            });
        }
    }
    Ok(())
}

fn validate_command_name(name: &str) -> Result<(), StandalonePresetError> {
    let valid = !name.is_empty()
        && name
            .bytes()
            .all(|byte| byte.is_ascii_lowercase() || byte.is_ascii_digit() || byte == b'_')
        && !name.starts_with("command_");
    if valid {
        Ok(())
    } else {
        Err(StandalonePresetError::InvalidCommandName {
            name: name.to_string(),
        })
    }
}

fn validate_hex_color(field: &str, color: &str) -> Result<(), StandalonePresetError> {
    let valid = color.len() == 7
        && color.starts_with('#')
        && color[1..].bytes().all(|byte| byte.is_ascii_hexdigit());
    if valid {
        Ok(())
    } else {
        Err(StandalonePresetError::InvalidColor {
            field: field.to_string(),
            value: color.to_string(),
        })
    }
}

fn escape_kdl_string(value: &str) -> String {
    value
        .replace('\\', "\\\\")
        .replace('"', "\\\"")
        .replace('\n', "\\n")
        .replace('\r', "\\r")
        .replace('\t', "\\t")
}

pub fn render_zjstatus_bar_segments(
    request: &BarRenderRequest,
) -> Result<BarRenderData, BarRenderError> {
    Ok(BarRenderData {
        widget_tray_segment: render_widget_tray_segment(request)?,
        custom_text_segment: render_custom_text_segment(&request.custom_text),
    })
}

pub fn render_widget_tray_segment(request: &BarRenderRequest) -> Result<String, BarRenderError> {
    request
        .widget_tray
        .iter()
        .map(|widget| render_widget(widget, request))
        .collect::<Result<Vec<_>, _>>()
        .map(|parts| {
            parts
                .into_iter()
                .filter(|part| !part.is_empty())
                .collect::<Vec<_>>()
                .join("")
        })
}

pub fn render_custom_text_segment(custom_text: &str) -> String {
    let trimmed = custom_text.trim();
    if trimmed.is_empty() {
        String::new()
    } else {
        format!("#[fg=#ffff00,bold][{trimmed}] ")
    }
}

pub fn render_cursor_status_widget(facts: &CursorWidgetFacts) -> String {
    let name = sanitize_cursor_name(&facts.name);
    if name.is_empty() {
        return String::new();
    }

    let color = facts
        .color
        .as_deref()
        .and_then(normalize_hex_color)
        .unwrap_or_else(|| "#00ff88".to_string());

    if let Some((glyph, primary_color, secondary_color)) = cursor_split_preview(facts, &color) {
        let glyph_segment = format!("#[fg={primary_color},bg={secondary_color},bold]{glyph}");
        return render_cursor_status_widget_frame(&color, &glyph_segment, &name);
    }

    let glyph_segment = format!("#[fg={color},bold]█");
    render_cursor_status_widget_frame(&color, &glyph_segment, &name)
}

pub fn cursor_widget_text_from_env() -> String {
    render_cursor_status_widget(&cursor_widget_facts_from_pairs(std::env::vars()))
}

pub fn cursor_widget_text_from_fact_file(
    path: impl AsRef<std::path::Path>,
) -> std::io::Result<String> {
    let raw = std::fs::read_to_string(path)?;
    Ok(render_cursor_status_widget(
        &cursor_widget_facts_from_pairs(cursor_fact_file_pairs(&raw)),
    ))
}

pub fn cursor_widget_facts_from_pairs(
    pairs: impl IntoIterator<Item = (String, String)>,
) -> CursorWidgetFacts {
    let mut facts = CursorWidgetFacts::default();
    for (key, value) in pairs {
        let value = value.trim();
        if value.is_empty() {
            continue;
        }
        match key.trim() {
            CURSOR_NAME_FACT => facts.name = value.to_string(),
            CURSOR_COLOR_FACT => facts.color = Some(value.to_string()),
            CURSOR_FAMILY_FACT => facts.family = Some(value.to_string()),
            CURSOR_DIVIDER_FACT => facts.divider = Some(value.to_string()),
            CURSOR_PRIMARY_COLOR_FACT => facts.primary_color = Some(value.to_string()),
            CURSOR_SECONDARY_COLOR_FACT => facts.secondary_color = Some(value.to_string()),
            _ => {}
        }
    }
    facts
}

pub fn current_cpu_usage_widget_text() -> String {
    let Some(before) = std::fs::read_to_string("/proc/stat")
        .ok()
        .and_then(|raw| parse_proc_stat_cpu_totals(&raw))
    else {
        return "??%".to_string();
    };
    std::thread::sleep(std::time::Duration::from_millis(100));
    let Some(after) = std::fs::read_to_string("/proc/stat")
        .ok()
        .and_then(|raw| parse_proc_stat_cpu_totals(&raw))
    else {
        return "??%".to_string();
    };
    format_percent(cpu_usage_percent_from_totals(before, after))
}

pub fn current_ram_usage_widget_text() -> String {
    format_percent(
        std::fs::read_to_string("/proc/meminfo")
            .ok()
            .and_then(|raw| ram_usage_percent_from_meminfo(&raw)),
    )
}

pub fn cpu_usage_percent_from_proc_stat(before: &str, after: &str) -> Option<u64> {
    cpu_usage_percent_from_totals(
        parse_proc_stat_cpu_totals(before)?,
        parse_proc_stat_cpu_totals(after)?,
    )
}

pub fn ram_usage_percent_from_meminfo(raw: &str) -> Option<u64> {
    let total = meminfo_kib_value(raw, "MemTotal:")?;
    let available = meminfo_kib_value(raw, "MemAvailable:")?;
    if total == 0 || available > total {
        return None;
    }
    Some(round_percent(total - available, total))
}

fn parse_proc_stat_cpu_totals(raw: &str) -> Option<(u64, u64)> {
    let line = raw.lines().find(|line| line.starts_with("cpu "))?;
    let values = line
        .split_whitespace()
        .skip(1)
        .map(str::parse::<u64>)
        .collect::<Result<Vec<_>, _>>()
        .ok()?;
    if values.len() < 7 {
        return None;
    }
    let idle = values[3].checked_add(*values.get(4).unwrap_or(&0))?;
    let total = values
        .iter()
        .try_fold(0u64, |sum, value| sum.checked_add(*value))?;
    Some((idle, total))
}

fn cpu_usage_percent_from_totals(before: (u64, u64), after: (u64, u64)) -> Option<u64> {
    let idle_delta = after.0.checked_sub(before.0)?;
    let total_delta = after.1.checked_sub(before.1)?;
    if total_delta == 0 || idle_delta > total_delta {
        return None;
    }
    Some(round_percent(total_delta - idle_delta, total_delta))
}

fn meminfo_kib_value(raw: &str, key: &str) -> Option<u64> {
    raw.lines()
        .find_map(|line| line.strip_prefix(key))
        .and_then(|value| value.split_whitespace().next())
        .and_then(|value| value.parse::<u64>().ok())
}

fn round_percent(numerator: u64, denominator: u64) -> u64 {
    ((numerator.saturating_mul(100) + denominator / 2) / denominator).min(100)
}

fn format_percent(percent: Option<u64>) -> String {
    percent
        .map(|percent| format!("{percent}%"))
        .unwrap_or_else(|| "??%".to_string())
}

fn cursor_fact_file_pairs(raw: &str) -> impl Iterator<Item = (String, String)> + '_ {
    raw.lines().filter_map(|line| {
        let line = line.trim();
        if line.is_empty() || line.starts_with('#') {
            return None;
        }
        let (key, value) = line.split_once('=')?;
        Some((key.trim().to_string(), value.trim().to_string()))
    })
}

fn insert_u64(
    entry: &mut serde_json::Map<String, serde_json::Value>,
    key: &str,
    value: Option<u64>,
) {
    if let Some(value) = value {
        entry.insert(key.to_string(), serde_json::json!(value));
    }
}

fn windowed_usage_facts_from_cache_entry(entry: &serde_json::Value) -> WindowedAgentUsageFacts {
    WindowedAgentUsageFacts {
        updated_at_unix_seconds: entry
            .get("updated_at_unix_seconds")
            .and_then(serde_json::Value::as_u64),
        five_hour_tokens: entry
            .get("five_hour_tokens")
            .and_then(serde_json::Value::as_u64),
        weekly_tokens: entry
            .get("weekly_tokens")
            .and_then(serde_json::Value::as_u64),
        monthly_tokens: entry
            .get("monthly_tokens")
            .and_then(serde_json::Value::as_u64),
        five_hour_remaining_percent: entry
            .get("five_hour_remaining_percent")
            .and_then(serde_json::Value::as_u64),
        weekly_remaining_percent: entry
            .get("weekly_remaining_percent")
            .and_then(serde_json::Value::as_u64),
        monthly_remaining_percent: entry
            .get("monthly_remaining_percent")
            .and_then(serde_json::Value::as_u64),
        five_hour_reset_at_unix_seconds: entry
            .get("five_hour_reset_at_unix_seconds")
            .and_then(serde_json::Value::as_u64),
        weekly_reset_at_unix_seconds: entry
            .get("weekly_reset_at_unix_seconds")
            .and_then(serde_json::Value::as_u64),
        five_hour_window_seconds: entry
            .get("five_hour_window_seconds")
            .and_then(serde_json::Value::as_u64),
        weekly_window_seconds: entry
            .get("weekly_window_seconds")
            .and_then(serde_json::Value::as_u64),
        error: entry
            .get("error")
            .and_then(serde_json::Value::as_str)
            .map(str::trim)
            .filter(|value| !value.is_empty())
            .map(str::to_string),
    }
}

pub fn codex_usage_shared_cache_is_fresh(
    path: &std::path::Path,
    now: u64,
    max_age_seconds: u64,
) -> bool {
    let Some(cache) = read_codex_usage_shared_cache_value(path) else {
        return false;
    };
    cache
        .get("codex")
        .and_then(|entry| entry.get("updated_at_unix_seconds"))
        .and_then(serde_json::Value::as_u64)
        .is_some_and(|updated_at| {
            now.saturating_sub(updated_at) < max_age_seconds
                && cache
                    .get("codex")
                    .map(windowed_usage_facts_from_cache_entry)
                    .is_some_and(|facts| codex_usage_facts_are_complete(&facts))
        })
}

pub fn codex_usage_shared_cache_is_backing_off(path: &std::path::Path, now: u64) -> bool {
    read_codex_usage_shared_cache_value(path)
        .and_then(|cache| {
            let entry = cache.get("codex")?;
            let facts = windowed_usage_facts_from_cache_entry(entry);
            if !facts.is_empty() && !codex_usage_facts_are_complete(&facts) {
                return None;
            }
            entry.get("backoff_until_unix_seconds")?.as_u64()
        })
        .is_some_and(|backoff_until| now < backoff_until)
}

pub fn claude_usage_shared_cache_is_fresh(
    path: &std::path::Path,
    now: u64,
    max_age_seconds: u64,
) -> bool {
    let Some(cache) = read_claude_usage_shared_cache_value(path) else {
        return false;
    };
    cache
        .get("claude")
        .and_then(|entry| entry.get("updated_at_unix_seconds"))
        .and_then(serde_json::Value::as_u64)
        .is_some_and(|updated_at| {
            now.saturating_sub(updated_at) < max_age_seconds
                && cache
                    .get("claude")
                    .map(windowed_usage_facts_from_cache_entry)
                    .is_some_and(|facts| claude_usage_facts_are_complete(&facts))
        })
}

pub fn claude_usage_shared_cache_is_backing_off(path: &std::path::Path, now: u64) -> bool {
    read_claude_usage_shared_cache_value(path)
        .and_then(|cache| {
            let entry = cache.get("claude")?;
            let facts = windowed_usage_facts_from_cache_entry(entry);
            if !facts.is_empty() && !claude_usage_facts_are_complete(&facts) {
                return None;
            }
            entry.get("backoff_until_unix_seconds")?.as_u64()
        })
        .is_some_and(|backoff_until| now < backoff_until)
}

pub fn opencode_go_usage_shared_cache_is_fresh(
    path: &std::path::Path,
    now: u64,
    max_age_seconds: u64,
) -> bool {
    let Some(cache) = read_opencode_go_usage_shared_cache_value(path) else {
        return false;
    };
    cache
        .get("opencode_go")
        .and_then(|entry| entry.get("updated_at_unix_seconds"))
        .and_then(serde_json::Value::as_u64)
        .is_some_and(|updated_at| {
            now.saturating_sub(updated_at) < max_age_seconds
                && cache
                    .get("opencode_go")
                    .map(windowed_usage_facts_from_cache_entry)
                    .is_some_and(|facts| opencode_go_usage_facts_are_complete(&facts))
        })
}

pub fn opencode_go_usage_shared_cache_is_backing_off(path: &std::path::Path, now: u64) -> bool {
    read_opencode_go_usage_shared_cache_value(path)
        .and_then(|cache| {
            cache
                .get("opencode_go")?
                .get("backoff_until_unix_seconds")?
                .as_u64()
        })
        .is_some_and(|backoff_until| now < backoff_until)
}

pub fn codex_usage_quota_backoff_until(path: &std::path::Path, now: u64) -> Option<u64> {
    read_codex_usage_shared_cache_value(path)
        .and_then(|cache| {
            cache
                .get("codex")?
                .get("quota_backoff_until_unix_seconds")?
                .as_u64()
        })
        .filter(|backoff_until| now < *backoff_until)
}

pub fn claude_usage_quota_backoff_until(path: &std::path::Path, now: u64) -> Option<u64> {
    read_claude_usage_shared_cache_value(path)
        .and_then(|cache| {
            cache
                .get("claude")?
                .get("quota_backoff_until_unix_seconds")?
                .as_u64()
        })
        .filter(|backoff_until| now < *backoff_until)
}

fn codex_usage_facts_are_complete(facts: &WindowedAgentUsageFacts) -> bool {
    facts.five_hour_tokens.is_some()
        && facts.weekly_tokens.is_some()
        && facts.five_hour_remaining_percent.is_some()
        && facts.weekly_remaining_percent.is_some()
        && facts.five_hour_reset_at_unix_seconds.is_some()
        && facts.weekly_reset_at_unix_seconds.is_some()
        && facts.five_hour_window_seconds.is_some()
        && facts.weekly_window_seconds.is_some()
}

fn claude_usage_facts_are_complete(facts: &WindowedAgentUsageFacts) -> bool {
    facts.five_hour_tokens.is_some()
        && facts.weekly_tokens.is_some()
        && facts.five_hour_remaining_percent.is_some()
        && facts.weekly_remaining_percent.is_some()
}

fn opencode_go_usage_facts_are_complete(facts: &WindowedAgentUsageFacts) -> bool {
    facts.five_hour_tokens.is_some()
        && facts.weekly_tokens.is_some()
        && facts.monthly_tokens.is_some()
        && facts.five_hour_remaining_percent.is_some()
        && facts.weekly_remaining_percent.is_some()
        && facts.monthly_remaining_percent.is_some()
}

struct AgentUsageCacheLock {
    path: std::path::PathBuf,
}

impl Drop for AgentUsageCacheLock {
    fn drop(&mut self) {
        let _ = std::fs::remove_dir(&self.path);
    }
}

fn try_acquire_codex_usage_cache_lock(
    shared_path: &std::path::Path,
    now: u64,
) -> Result<Option<AgentUsageCacheLock>, String> {
    try_acquire_agent_usage_cache_lock(
        shared_path.with_file_name(format!(
            ".codex_usage_cache_v{CODEX_USAGE_CACHE_SCHEMA_VERSION}.lock"
        )),
        now,
        CODEX_USAGE_LOCK_STALE_AFTER_SECONDS,
    )
}

fn try_acquire_claude_usage_cache_lock(
    shared_path: &std::path::Path,
    now: u64,
) -> Result<Option<AgentUsageCacheLock>, String> {
    try_acquire_agent_usage_cache_lock(
        shared_path.with_file_name(format!(
            ".claude_usage_cache_v{CLAUDE_USAGE_CACHE_SCHEMA_VERSION}.lock"
        )),
        now,
        CLAUDE_USAGE_LOCK_STALE_AFTER_SECONDS,
    )
}

fn try_acquire_opencode_go_usage_cache_lock(
    shared_path: &std::path::Path,
    now: u64,
) -> Result<Option<AgentUsageCacheLock>, String> {
    try_acquire_agent_usage_cache_lock(
        shared_path.with_file_name(format!(
            ".opencode_go_usage_cache_v{OPENCODE_GO_USAGE_CACHE_SCHEMA_VERSION}.lock"
        )),
        now,
        OPENCODE_GO_USAGE_LOCK_STALE_AFTER_SECONDS,
    )
}

fn try_acquire_agent_usage_cache_lock(
    lock_path: std::path::PathBuf,
    now: u64,
    stale_after_seconds: u64,
) -> Result<Option<AgentUsageCacheLock>, String> {
    if let Some(parent) = lock_path.parent() {
        std::fs::create_dir_all(parent)
            .map_err(|error| format!("failed to create usage cache lock parent: {error}"))?;
    }
    match std::fs::create_dir(&lock_path) {
        Ok(()) => Ok(Some(AgentUsageCacheLock { path: lock_path })),
        Err(error) if error.kind() == std::io::ErrorKind::AlreadyExists => {
            if agent_usage_cache_lock_is_stale(&lock_path, now, stale_after_seconds) {
                let _ = std::fs::remove_dir(&lock_path);
                return match std::fs::create_dir(&lock_path) {
                    Ok(()) => Ok(Some(AgentUsageCacheLock { path: lock_path })),
                    Err(error) if error.kind() == std::io::ErrorKind::AlreadyExists => Ok(None),
                    Err(error) => Err(format!("failed to acquire usage cache lock: {error}")),
                };
            }
            Ok(None)
        }
        Err(error) => Err(format!("failed to acquire usage cache lock: {error}")),
    }
}

fn agent_usage_cache_lock_is_stale(
    lock_path: &std::path::Path,
    now: u64,
    stale_after_seconds: u64,
) -> bool {
    std::fs::metadata(lock_path)
        .ok()
        .and_then(|metadata| metadata.modified().ok())
        .and_then(|modified| modified.duration_since(std::time::UNIX_EPOCH).ok())
        .map(|duration| now.saturating_sub(duration.as_secs()) > stale_after_seconds)
        .unwrap_or(false)
}

fn preserve_previous_codex_window_tokens(
    facts: &mut WindowedAgentUsageFacts,
    previous: Option<&WindowedAgentUsageFacts>,
) {
    let Some(previous) = previous else {
        return;
    };
    if !codex_usage_facts_are_complete(previous) {
        return;
    }
    if facts.five_hour_tokens.is_none()
        && tokenusage_window_identity_matches(
            facts.five_hour_reset_at_unix_seconds,
            facts.five_hour_window_seconds,
            previous.five_hour_reset_at_unix_seconds,
            previous.five_hour_window_seconds,
        )
    {
        facts.five_hour_tokens = previous.five_hour_tokens;
    }
    if facts.weekly_tokens.is_none()
        && tokenusage_window_identity_matches(
            facts.weekly_reset_at_unix_seconds,
            facts.weekly_window_seconds,
            previous.weekly_reset_at_unix_seconds,
            previous.weekly_window_seconds,
        )
    {
        facts.weekly_tokens = previous.weekly_tokens;
    }
}

fn preserve_previous_codex_window_quota(
    facts: &mut WindowedAgentUsageFacts,
    previous: Option<&WindowedAgentUsageFacts>,
    now: u64,
) {
    let Some(previous) = previous else {
        return;
    };
    if facts.five_hour_remaining_percent.is_none()
        && previous_codex_quota_window_is_still_valid(
            previous.five_hour_reset_at_unix_seconds,
            previous.five_hour_window_seconds,
            now,
        )
    {
        facts.five_hour_remaining_percent = previous.five_hour_remaining_percent;
        facts.five_hour_reset_at_unix_seconds = previous.five_hour_reset_at_unix_seconds;
        facts.five_hour_window_seconds = previous.five_hour_window_seconds;
    }
    if facts.weekly_remaining_percent.is_none()
        && previous_codex_quota_window_is_still_valid(
            previous.weekly_reset_at_unix_seconds,
            previous.weekly_window_seconds,
            now,
        )
    {
        facts.weekly_remaining_percent = previous.weekly_remaining_percent;
        facts.weekly_reset_at_unix_seconds = previous.weekly_reset_at_unix_seconds;
        facts.weekly_window_seconds = previous.weekly_window_seconds;
    }
}

fn preserve_previous_claude_window_quota(
    facts: &mut WindowedAgentUsageFacts,
    previous: Option<&WindowedAgentUsageFacts>,
) {
    let Some(previous) = previous else {
        return;
    };
    if facts.five_hour_remaining_percent.is_none() {
        facts.five_hour_remaining_percent = previous.five_hour_remaining_percent;
    }
    if facts.weekly_remaining_percent.is_none() {
        facts.weekly_remaining_percent = previous.weekly_remaining_percent;
    }
}

fn previous_codex_quota_window_is_still_valid(
    reset_at_unix_seconds: Option<u64>,
    window_seconds: Option<u64>,
    now: u64,
) -> bool {
    reset_at_unix_seconds.is_some_and(|reset_at| now < reset_at)
        && window_seconds.is_some_and(|seconds| seconds > 0)
}

fn tokenusage_window_identity_matches(
    reset_at: Option<u64>,
    window_seconds: Option<u64>,
    previous_reset_at: Option<u64>,
    previous_window_seconds: Option<u64>,
) -> bool {
    reset_at.is_some()
        && window_seconds.is_some()
        && reset_at == previous_reset_at
        && window_seconds == previous_window_seconds
}

fn collect_codex_usage_facts(
    path_var: Option<&std::ffi::OsStr>,
    timeout: std::time::Duration,
    include_quota: bool,
) -> WindowedAgentUsageFacts {
    let Some(path_var) = path_var else {
        return WindowedAgentUsageFacts {
            error: Some("missing PATH".to_string()),
            ..WindowedAgentUsageFacts::default()
        };
    };
    let Some(binary_path) = find_command_in_path_var(path_var, "tu") else {
        return WindowedAgentUsageFacts {
            error: Some("missing tu".to_string()),
            ..WindowedAgentUsageFacts::default()
        };
    };

    let mut facts = WindowedAgentUsageFacts::default();
    match run_tokenusage_json_command(&binary_path, codex_active_block_args().as_slice(), timeout) {
        Ok(Some(value)) => {
            facts.five_hour_tokens = tokenusage_active_block_tokens_from_json(&value)
        }
        Ok(None) => facts.error = Some("active block unavailable".to_string()),
        Err(error) => facts.error = Some(error),
    }
    match run_tokenusage_json_command(&binary_path, codex_weekly_args().as_slice(), timeout) {
        Ok(Some(value)) => facts.weekly_tokens = tokenusage_weekly_tokens_from_json(&value),
        Ok(None) => {
            if facts.error.is_none() {
                facts.error = Some("weekly usage unavailable".to_string());
            }
        }
        Err(error) => {
            if facts.error.is_none() {
                facts.error = Some(error);
            }
        }
    }
    if include_quota {
        match run_tokenusage_json_command(
            &binary_path,
            codex_official_limits_args().as_slice(),
            timeout,
        ) {
            Ok(Some(value)) => {
                let quota = codex_quota_from_official_json(&value);
                facts.five_hour_remaining_percent = quota.five_hour_remaining_percent;
                facts.weekly_remaining_percent = quota.weekly_remaining_percent;
                facts.five_hour_reset_at_unix_seconds = quota.five_hour_reset_at_unix_seconds;
                facts.weekly_reset_at_unix_seconds = quota.weekly_reset_at_unix_seconds;
                facts.five_hour_window_seconds = quota.five_hour_window_seconds;
                facts.weekly_window_seconds = quota.weekly_window_seconds;
                if !quota.has_quota() && facts.error.is_none() {
                    facts.error = Some("quota unavailable".to_string());
                }
            }
            Ok(None) => {
                if facts.error.is_none() {
                    facts.error = Some("quota unavailable".to_string());
                }
            }
            Err(error) => {
                if facts.error.is_none() {
                    facts.error = Some(error);
                }
            }
        }
    }
    facts
}

fn collect_claude_usage_facts(
    path_var: Option<&std::ffi::OsStr>,
    timeout: std::time::Duration,
    include_quota: bool,
) -> WindowedAgentUsageFacts {
    let Some(path_var) = path_var else {
        return WindowedAgentUsageFacts {
            error: Some("missing PATH".to_string()),
            ..WindowedAgentUsageFacts::default()
        };
    };
    let Some(binary_path) = find_command_in_path_var(path_var, "tu") else {
        return WindowedAgentUsageFacts {
            error: Some("missing tu".to_string()),
            ..WindowedAgentUsageFacts::default()
        };
    };

    let mut facts = WindowedAgentUsageFacts::default();
    match run_tokenusage_json_command(&binary_path, claude_active_block_args().as_slice(), timeout)
    {
        Ok(Some(value)) => {
            facts.five_hour_tokens = tokenusage_active_block_tokens_from_json(&value)
        }
        Ok(None) => facts.error = Some("active block unavailable".to_string()),
        Err(error) => facts.error = Some(error),
    }
    match run_tokenusage_json_command(&binary_path, claude_weekly_args().as_slice(), timeout) {
        Ok(Some(value)) => facts.weekly_tokens = tokenusage_weekly_tokens_from_json(&value),
        Ok(None) => {
            if facts.error.is_none() {
                facts.error = Some("weekly usage unavailable".to_string());
            }
        }
        Err(error) => {
            if facts.error.is_none() {
                facts.error = Some(error);
            }
        }
    }
    if include_quota {
        match run_tokenusage_json_command(
            &binary_path,
            claude_official_limits_args().as_slice(),
            timeout,
        ) {
            Ok(Some(value)) => {
                let quota = claude_quota_from_official_json(&value);
                facts.five_hour_remaining_percent = quota.five_hour_remaining_percent;
                facts.weekly_remaining_percent = quota.weekly_remaining_percent;
                if !quota.has_quota() && facts.error.is_none() {
                    facts.error = Some("quota unavailable".to_string());
                }
            }
            Ok(None) => {
                if facts.error.is_none() {
                    facts.error = Some("quota unavailable".to_string());
                }
            }
            Err(error) => {
                if facts.error.is_none() {
                    facts.error = Some(error);
                }
            }
        }
    }
    facts
}

#[cfg(feature = "providers")]
pub fn collect_opencode_go_usage_facts_from_dbs(
    db_paths: &[std::path::PathBuf],
    now: u64,
) -> WindowedAgentUsageFacts {
    if db_paths.is_empty() {
        return WindowedAgentUsageFacts {
            error: Some("missing OpenCode DB".to_string()),
            ..WindowedAgentUsageFacts::default()
        };
    }

    let mut five_hour = OpenCodeGoUsageWindow::default();
    let mut weekly = OpenCodeGoUsageWindow::default();
    let mut monthly = OpenCodeGoUsageWindow::default();
    let mut opened_any = false;
    let mut first_error = None;

    for path in db_paths {
        match collect_opencode_go_usage_windows_from_db(path, now) {
            Ok(db_windows) => {
                opened_any = true;
                five_hour.tokens = five_hour.tokens.saturating_add(db_windows.five_hour.tokens);
                five_hour.cost_usd += db_windows.five_hour.cost_usd;
                weekly.tokens = weekly.tokens.saturating_add(db_windows.weekly.tokens);
                weekly.cost_usd += db_windows.weekly.cost_usd;
                monthly.tokens = monthly.tokens.saturating_add(db_windows.monthly.tokens);
                monthly.cost_usd += db_windows.monthly.cost_usd;
            }
            Err(error) => {
                if first_error.is_none() {
                    first_error = Some(format!("{}: {error}", path.display()));
                }
            }
        }
    }

    if !opened_any {
        return WindowedAgentUsageFacts {
            error: first_error.or_else(|| Some("OpenCode DB unavailable".to_string())),
            ..WindowedAgentUsageFacts::default()
        };
    }

    WindowedAgentUsageFacts {
        five_hour_tokens: Some(five_hour.tokens),
        five_hour_remaining_percent: Some(remaining_percent_from_cost_limit(
            five_hour.cost_usd,
            OPENCODE_GO_FIVE_HOUR_LIMIT_USD,
        )),
        weekly_tokens: Some(weekly.tokens),
        weekly_remaining_percent: Some(remaining_percent_from_cost_limit(
            weekly.cost_usd,
            OPENCODE_GO_WEEKLY_LIMIT_USD,
        )),
        monthly_tokens: Some(monthly.tokens),
        monthly_remaining_percent: Some(remaining_percent_from_cost_limit(
            monthly.cost_usd,
            OPENCODE_GO_MONTHLY_LIMIT_USD,
        )),
        ..WindowedAgentUsageFacts::default()
    }
}

#[derive(Debug, Clone, Copy, Default, PartialEq)]
#[cfg(feature = "providers")]
pub struct OpenCodeGoUsageWindow {
    pub tokens: u64,
    pub cost_usd: f64,
}

#[derive(Debug, Clone, Copy, Default, PartialEq)]
#[cfg(feature = "providers")]
pub struct OpenCodeGoUsageWindows {
    pub five_hour: OpenCodeGoUsageWindow,
    pub weekly: OpenCodeGoUsageWindow,
    pub monthly: OpenCodeGoUsageWindow,
}

#[cfg(feature = "providers")]
pub fn collect_opencode_go_usage_windows_from_db(
    path: &std::path::Path,
    now: u64,
) -> Result<OpenCodeGoUsageWindows, String> {
    let connection =
        rusqlite::Connection::open_with_flags(path, rusqlite::OpenFlags::SQLITE_OPEN_READ_ONLY)
            .map_err(|error| format!("failed to open OpenCode DB read-only: {error}"))?;
    connection
        .busy_timeout(std::time::Duration::from_millis(250))
        .map_err(|error| format!("failed to configure OpenCode DB read timeout: {error}"))?;
    let five_hour = query_opencode_go_usage_window(
        &connection,
        now.saturating_sub(OPENCODE_GO_FIVE_HOUR_SECONDS),
    )?;
    let weekly =
        query_opencode_go_usage_window(&connection, now.saturating_sub(OPENCODE_GO_WEEK_SECONDS))?;
    let monthly =
        query_opencode_go_usage_window(&connection, now.saturating_sub(OPENCODE_GO_MONTH_SECONDS))?;
    Ok(OpenCodeGoUsageWindows {
        five_hour,
        weekly,
        monthly,
    })
}

#[cfg(feature = "providers")]
pub fn query_opencode_go_usage_window(
    connection: &rusqlite::Connection,
    since_unix_seconds: u64,
) -> Result<OpenCodeGoUsageWindow, String> {
    connection
        .query_row(
            r#"
            SELECT
              COALESCE(SUM(
                COALESCE(
                  json_extract(data, '$.tokens.total'),
                  COALESCE(json_extract(data, '$.tokens.input'), 0) +
                  COALESCE(json_extract(data, '$.tokens.output'), 0) +
                  COALESCE(json_extract(data, '$.tokens.reasoning'), 0) +
                  COALESCE(json_extract(data, '$.tokens.cache.read'), 0) +
                  COALESCE(json_extract(data, '$.tokens.cache.write'), 0)
                )
              ), 0),
              COALESCE(SUM(COALESCE(json_extract(data, '$.cost'), 0.0)), 0.0)
            FROM message
            WHERE time_created >= ?1
              AND json_extract(data, '$.role') = 'assistant'
              AND json_extract(data, '$.providerID') = ?2
            "#,
            rusqlite::params![
                unix_millis_from_seconds_saturating(since_unix_seconds),
                OPENCODE_GO_PROVIDER_ID
            ],
            |row| {
                Ok(OpenCodeGoUsageWindow {
                    tokens: row.get::<_, i64>(0)?.max(0) as u64,
                    cost_usd: row.get::<_, f64>(1)?.max(0.0),
                })
            },
        )
        .map_err(|error| format!("failed to read OpenCode Go usage window: {error}"))
}

#[cfg(feature = "providers")]
pub fn unix_millis_from_seconds_saturating(seconds: u64) -> i64 {
    seconds.saturating_mul(1000).min(i64::MAX as u64) as i64
}

#[cfg(feature = "providers")]
pub fn remaining_percent_from_cost_limit(cost_usd: f64, limit_usd: f64) -> u64 {
    if limit_usd <= 0.0 {
        return 0;
    }
    (100.0 - (cost_usd / limit_usd * 100.0))
        .clamp(0.0, 100.0)
        .round() as u64
}

pub fn opencode_db_candidates_from_env() -> Vec<std::path::PathBuf> {
    opencode_db_candidates_from_values(
        std::env::var_os("OPENCODE_DB").map(std::path::PathBuf::from),
        std::env::var_os("OPENCODE_DATA_DIR").map(std::path::PathBuf::from),
        std::env::var_os("XDG_DATA_HOME").map(std::path::PathBuf::from),
        std::env::var_os("HOME").map(std::path::PathBuf::from),
    )
}

pub fn opencode_db_candidates_from_values(
    opencode_db: Option<std::path::PathBuf>,
    opencode_data_dir: Option<std::path::PathBuf>,
    xdg_data_home: Option<std::path::PathBuf>,
    home: Option<std::path::PathBuf>,
) -> Vec<std::path::PathBuf> {
    let data_dir = opencode_data_dir
        .filter(|path| !path.as_os_str().is_empty())
        .or_else(|| {
            xdg_data_home
                .filter(|path| !path.as_os_str().is_empty())
                .map(|path| path.join("opencode"))
        })
        .or_else(|| {
            home.filter(|path| !path.as_os_str().is_empty())
                .map(|path| path.join(".local").join("share").join("opencode"))
        });

    let mut candidates = Vec::new();
    if let Some(path) = opencode_db.filter(|path| !path.as_os_str().is_empty()) {
        if path.is_absolute() {
            push_unique_path(&mut candidates, path);
        } else if let Some(data_dir) = data_dir.as_ref() {
            push_unique_path(&mut candidates, data_dir.join(path));
        } else {
            push_unique_path(&mut candidates, path);
        }
    }

    if let Some(data_dir) = data_dir {
        push_unique_path(&mut candidates, data_dir.join("opencode.db"));
        if let Ok(entries) = std::fs::read_dir(data_dir) {
            let mut channel_dbs = entries
                .filter_map(Result::ok)
                .map(|entry| entry.path())
                .filter(|path| {
                    path.file_name()
                        .and_then(|name| name.to_str())
                        .is_some_and(|name| name.starts_with("opencode-") && name.ends_with(".db"))
                })
                .collect::<Vec<_>>();
            channel_dbs.sort();
            for path in channel_dbs {
                push_unique_path(&mut candidates, path);
            }
        }
    }
    candidates
}

fn push_unique_path(paths: &mut Vec<std::path::PathBuf>, path: std::path::PathBuf) {
    if !paths.contains(&path) {
        paths.push(path);
    }
}

fn codex_active_block_args() -> Vec<&'static str> {
    vec![
        "blocks",
        "--active",
        "--json",
        "--offline",
        "--no-claude",
        "--no-antigravity",
    ]
}

fn codex_weekly_args() -> Vec<&'static str> {
    vec![
        "weekly",
        "--json",
        "--offline",
        "--no-claude",
        "--no-antigravity",
        "--order",
        "desc",
    ]
}

fn codex_official_limits_args() -> Vec<&'static str> {
    vec![
        "blocks",
        "--active",
        "--json",
        "--official-limits",
        "--no-claude",
        "--no-antigravity",
    ]
}

fn claude_active_block_args() -> Vec<&'static str> {
    vec![
        "blocks",
        "--active",
        "--json",
        "--offline",
        "--no-codex",
        "--no-antigravity",
    ]
}

fn claude_weekly_args() -> Vec<&'static str> {
    vec![
        "weekly",
        "--json",
        "--offline",
        "--no-codex",
        "--no-antigravity",
        "--order",
        "desc",
    ]
}

fn claude_official_limits_args() -> Vec<&'static str> {
    vec![
        "blocks",
        "--active",
        "--json",
        "--official-limits",
        "--no-codex",
        "--no-antigravity",
    ]
}

fn run_tokenusage_json_command(
    binary_path: &std::path::Path,
    args: &[&str],
    timeout: std::time::Duration,
) -> Result<Option<serde_json::Value>, String> {
    let output =
        run_agent_usage_command_with_timeout(binary_path, args, timeout).map_err(|error| {
            format!(
                "failed to run tokenusage command {}: {error}",
                binary_path.display()
            )
        })?;
    let Some(output) = output else {
        return Ok(None);
    };
    if !output.status.success() {
        return Ok(None);
    }
    let stdout = String::from_utf8_lossy(&output.stdout);
    let Some(json_raw) = extract_json_object(&stdout) else {
        return Ok(None);
    };
    serde_json::from_str::<serde_json::Value>(json_raw)
        .map(Some)
        .map_err(|error| error.to_string())
}

fn run_agent_usage_command_with_timeout(
    binary_path: &std::path::Path,
    args: &[&str],
    timeout: std::time::Duration,
) -> std::io::Result<Option<std::process::Output>> {
    let mut child = std::process::Command::new(binary_path)
        .args(args)
        .stdout(std::process::Stdio::piped())
        .stderr(std::process::Stdio::null())
        .spawn()?;
    let started = std::time::Instant::now();
    loop {
        if child.try_wait()?.is_some() {
            return child.wait_with_output().map(Some);
        }
        if started.elapsed() >= timeout {
            let _ = child.kill();
            let _ = child.wait();
            return Ok(None);
        }
        std::thread::sleep(std::time::Duration::from_millis(25));
    }
}

fn find_command_in_path_var(
    path_var: &std::ffi::OsStr,
    command_name: &str,
) -> Option<std::path::PathBuf> {
    std::env::split_paths(path_var)
        .map(|entry| entry.join(command_name))
        .find(|candidate| candidate.is_file())
}

fn tokenusage_active_block_tokens_from_json(value: &serde_json::Value) -> Option<u64> {
    value
        .get("blocks")
        .and_then(serde_json::Value::as_array)
        .and_then(|blocks| {
            blocks
                .iter()
                .find(|block| {
                    block
                        .get("isActive")
                        .or_else(|| block.get("is_active"))
                        .and_then(serde_json::Value::as_bool)
                        == Some(true)
                })
                .or_else(|| blocks.first())
        })
        .and_then(|block| {
            first_u64_at(
                block,
                &[
                    &["totals", "total_tokens"],
                    &["totals", "totalTokens"],
                    &["total_tokens"],
                    &["totalTokens"],
                ],
            )
        })
}

fn tokenusage_weekly_tokens_from_json(value: &serde_json::Value) -> Option<u64> {
    value
        .get("weekly")
        .and_then(serde_json::Value::as_array)
        .and_then(|rows| rows.first())
        .and_then(|row| {
            first_u64_at(
                row,
                &[
                    &["totals", "total_tokens"],
                    &["totals", "totalTokens"],
                    &["total_tokens"],
                    &["totalTokens"],
                ],
            )
        })
}

fn codex_quota_from_official_json(value: &serde_json::Value) -> WindowedAgentUsageFacts {
    let Some(official) = value.get("official_codex") else {
        return WindowedAgentUsageFacts::default();
    };
    WindowedAgentUsageFacts {
        five_hour_remaining_percent: official
            .get("primary_used_percent")
            .and_then(serde_json::Value::as_f64)
            .map(remaining_percent_from_used),
        weekly_remaining_percent: official
            .get("secondary_used_percent")
            .and_then(serde_json::Value::as_f64)
            .map(remaining_percent_from_used),
        five_hour_reset_at_unix_seconds: official
            .get("primary_resets_at")
            .and_then(serde_json::Value::as_u64),
        weekly_reset_at_unix_seconds: official
            .get("secondary_resets_at")
            .and_then(serde_json::Value::as_u64),
        five_hour_window_seconds: official
            .get("primary_window_mins")
            .and_then(serde_json::Value::as_u64)
            .and_then(window_minutes_to_seconds),
        weekly_window_seconds: official
            .get("secondary_window_mins")
            .and_then(serde_json::Value::as_u64)
            .and_then(window_minutes_to_seconds),
        ..WindowedAgentUsageFacts::default()
    }
}

fn claude_quota_from_official_json(value: &serde_json::Value) -> WindowedAgentUsageFacts {
    let Some(official) = value.get("official_claude") else {
        return WindowedAgentUsageFacts::default();
    };
    WindowedAgentUsageFacts {
        five_hour_remaining_percent: official
            .get("primary_used_percent")
            .and_then(serde_json::Value::as_f64)
            .map(remaining_percent_from_used),
        weekly_remaining_percent: official
            .get("secondary_used_percent")
            .and_then(serde_json::Value::as_f64)
            .map(remaining_percent_from_used),
        ..WindowedAgentUsageFacts::default()
    }
}

fn window_minutes_to_seconds(minutes: u64) -> Option<u64> {
    minutes
        .checked_mul(AGENT_USAGE_MINUTE_SECONDS)
        .filter(|seconds| *seconds > 0)
}

fn remaining_percent_from_used(used_percent: f64) -> u64 {
    (100.0 - used_percent).clamp(0.0, 100.0).round() as u64
}

fn extract_json_object(raw: &str) -> Option<&str> {
    let start = raw.find('{')?;
    let end = raw.rfind('}')?;
    (start <= end).then_some(&raw[start..=end])
}

fn first_u64_at(value: &serde_json::Value, paths: &[&[&str]]) -> Option<u64> {
    paths
        .iter()
        .find_map(|path| nested_value(value, path)?.as_u64())
}

fn nested_value<'a>(value: &'a serde_json::Value, path: &[&str]) -> Option<&'a serde_json::Value> {
    let mut current = value;
    for segment in path {
        current = current.get(*segment)?;
    }
    Some(current)
}

fn write_json_value_atomic(
    path: &std::path::Path,
    value: &serde_json::Value,
) -> Result<(), String> {
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent)
            .map_err(|error| format!("failed to create usage cache parent: {error}"))?;
    }
    let file_name = path
        .file_name()
        .and_then(|name| name.to_str())
        .unwrap_or("agent_usage_cache");
    let tmp_path = path.with_file_name(format!(".{file_name}.tmp-{}", std::process::id()));
    let raw = serde_json::to_vec(value).map_err(|error| error.to_string())?;
    std::fs::write(&tmp_path, raw)
        .map_err(|error| format!("failed to write temporary usage cache: {error}"))?;
    std::fs::rename(&tmp_path, path)
        .map_err(|error| format!("failed to install usage cache: {error}"))?;
    Ok(())
}

pub fn render_codex_usage_status_widget(
    facts: &WindowedAgentUsageFacts,
    display: AgentUsageDisplay,
) -> String {
    render_agent_usage_status_widget("codex", &render_codex_usage_summary(facts, display))
}

pub fn render_codex_usage_summary(
    facts: &WindowedAgentUsageFacts,
    display: AgentUsageDisplay,
) -> String {
    let mut parts = Vec::new();
    for period in [AgentUsagePeriod::FiveHour, AgentUsagePeriod::Weekly] {
        let (tokens, remaining_percent) = usage_period_values(facts, period);
        let label = codex_reset_window_label(facts, period)
            .unwrap_or_else(|| period.short_label().to_string());
        if let Some(part) = render_codex_usage_window(&label, tokens, remaining_percent, display) {
            parts.push(part);
        }
    }
    parts.join(" · ")
}

pub fn render_windowed_agent_usage_status_widget(
    label: &str,
    facts: &WindowedAgentUsageFacts,
    periods: &[AgentUsagePeriod],
    display: AgentUsageDisplay,
) -> String {
    render_agent_usage_status_widget(
        label,
        &render_windowed_agent_usage_summary(facts, periods, display),
    )
}

pub fn render_windowed_agent_usage_summary(
    facts: &WindowedAgentUsageFacts,
    periods: &[AgentUsagePeriod],
    display: AgentUsageDisplay,
) -> String {
    let mut parts = Vec::new();
    for period in periods {
        let (tokens, remaining_percent) = usage_period_values(facts, *period);
        if let Some(part) = render_windowed_agent_usage_window(
            period.short_label(),
            tokens,
            remaining_percent,
            display,
        ) {
            parts.push(part);
        }
    }
    parts.join(" ")
}

fn render_agent_usage_status_widget(label: &str, summary: &str) -> String {
    if summary.is_empty() {
        String::new()
    } else {
        format!(" [{label} {summary}]")
    }
}

fn render_codex_usage_window(
    label: &str,
    tokens: Option<u64>,
    remaining_percent: Option<u64>,
    display: AgentUsageDisplay,
) -> Option<String> {
    let mut pieces = vec![label.to_string()];
    match display {
        AgentUsageDisplay::Token => {
            pieces.push(format_agent_usage_token_count(tokens?));
        }
        AgentUsageDisplay::Quota => {
            pieces.push(match remaining_percent {
                Some(percent) => format_quota_percent(percent),
                None if tokens.is_some() => "n/a".to_string(),
                None => return None,
            });
        }
        AgentUsageDisplay::Both => {
            if let Some(tokens) = tokens {
                pieces.push(format_agent_usage_token_count(tokens));
            }
            if let Some(remaining_percent) = remaining_percent {
                pieces.push(format_quota_percent(remaining_percent));
            }
            if pieces.len() == 1 {
                return None;
            }
        }
    }
    Some(pieces.join(" "))
}

fn render_windowed_agent_usage_window(
    label: &str,
    tokens: Option<u64>,
    remaining_percent: Option<u64>,
    display: AgentUsageDisplay,
) -> Option<String> {
    let mut pieces = vec![label.to_string()];
    match display {
        AgentUsageDisplay::Token => {
            pieces.push(format_agent_usage_token_count(tokens?));
        }
        AgentUsageDisplay::Quota => {
            pieces.push(match remaining_percent {
                Some(percent) => format_quota_percent(percent),
                None if tokens.is_some() => "n/a".to_string(),
                None => return None,
            });
        }
        AgentUsageDisplay::Both => {
            if let Some(tokens) = tokens {
                pieces.push(format_agent_usage_token_count(tokens));
            }
            if let Some(remaining_percent) = remaining_percent {
                pieces.push(format_quota_percent(remaining_percent));
            }
            if pieces.len() == 1 {
                return None;
            }
        }
    }
    Some(pieces.join("|"))
}

fn usage_period_values(
    facts: &WindowedAgentUsageFacts,
    period: AgentUsagePeriod,
) -> (Option<u64>, Option<u64>) {
    match period {
        AgentUsagePeriod::FiveHour => (facts.five_hour_tokens, facts.five_hour_remaining_percent),
        AgentUsagePeriod::Weekly => (facts.weekly_tokens, facts.weekly_remaining_percent),
        AgentUsagePeriod::Monthly => (facts.monthly_tokens, facts.monthly_remaining_percent),
    }
}

fn codex_reset_window_label(
    facts: &WindowedAgentUsageFacts,
    period: AgentUsagePeriod,
) -> Option<String> {
    let now = facts.updated_at_unix_seconds?;
    let (reset_at, window_seconds) = match period {
        AgentUsagePeriod::FiveHour => (
            facts.five_hour_reset_at_unix_seconds?,
            facts.five_hour_window_seconds?,
        ),
        AgentUsagePeriod::Weekly => (
            facts.weekly_reset_at_unix_seconds?,
            facts.weekly_window_seconds?,
        ),
        AgentUsagePeriod::Monthly => return None,
    };
    format_reset_window_label(reset_at, window_seconds, now)
}

fn format_agent_usage_token_count(tokens: u64) -> String {
    if tokens >= 1_000_000_000 {
        format_scaled_agent_usage_count(tokens as f64 / 1_000_000_000.0, "B")
    } else if tokens >= 1_000_000 {
        format_scaled_agent_usage_count(tokens as f64 / 1_000_000.0, "M")
    } else if tokens >= 1_000 {
        format!("{}k", tokens / 1_000)
    } else {
        tokens.to_string()
    }
}

fn format_scaled_agent_usage_count(value: f64, suffix: &str) -> String {
    let raw = if value >= 100.0 {
        format!("{value:.0}")
    } else if value >= 10.0 {
        format!("{value:.1}")
    } else {
        format!("{value:.2}")
    };
    let trimmed = if raw.contains('.') {
        raw.trim_end_matches('0').trim_end_matches('.')
    } else {
        raw.as_str()
    };
    format!("{trimmed}{suffix}")
}

fn format_reset_window_label(
    reset_at_unix_seconds: u64,
    window_seconds: u64,
    now_unix_seconds: u64,
) -> Option<String> {
    if window_seconds == 0 {
        return None;
    }
    let remaining_seconds = reset_at_unix_seconds
        .saturating_sub(now_unix_seconds)
        .min(window_seconds);
    let elapsed_seconds = window_seconds.saturating_sub(remaining_seconds);
    Some(format!(
        "{}/{}",
        format_reset_window_position_duration(elapsed_seconds, window_seconds),
        format_reset_window_total_duration(window_seconds)
    ))
}

fn format_reset_window_position_duration(seconds: u64, window_seconds: u64) -> String {
    const MINUTE_SECONDS: u64 = 60;
    const HOUR_SECONDS: u64 = 60 * MINUTE_SECONDS;
    const DAY_SECONDS: u64 = 24 * HOUR_SECONDS;

    if window_seconds >= DAY_SECONDS {
        let days = seconds / DAY_SECONDS;
        let hours = (seconds % DAY_SECONDS) / HOUR_SECONDS;
        if days > 0 && hours > 0 {
            format!("{days}d{hours}h")
        } else if days > 0 {
            format!("{days}d")
        } else if hours > 0 {
            format!("{hours}h")
        } else {
            "0h".to_string()
        }
    } else if window_seconds >= HOUR_SECONDS {
        let hours = seconds / HOUR_SECONDS;
        let minutes = elapsed_minutes_after_hour(seconds);
        if hours > 0 && minutes > 0 {
            format!("{hours}h{minutes}m")
        } else if hours > 0 {
            format!("{hours}h")
        } else {
            format!("{minutes}m")
        }
    } else if window_seconds >= MINUTE_SECONDS {
        if seconds > 0 {
            format!("{}m", seconds.div_ceil(MINUTE_SECONDS))
        } else {
            "0m".to_string()
        }
    } else if seconds > 0 {
        format!("{seconds}s")
    } else {
        "0s".to_string()
    }
}

fn elapsed_minutes_after_hour(seconds: u64) -> u64 {
    const HOUR_SECONDS: u64 = 60 * 60;
    const MINUTE_SECONDS: u64 = 60;

    let minutes = (seconds % HOUR_SECONDS) / MINUTE_SECONDS;
    if seconds > 0 && seconds < HOUR_SECONDS && minutes == 0 {
        1
    } else {
        minutes
    }
}

fn format_reset_window_total_duration(seconds: u64) -> String {
    const MINUTE_SECONDS: u64 = 60;
    const HOUR_SECONDS: u64 = 60 * MINUTE_SECONDS;
    const DAY_SECONDS: u64 = 24 * HOUR_SECONDS;

    if seconds % DAY_SECONDS == 0 {
        format!("{}d", seconds / DAY_SECONDS)
    } else if seconds % HOUR_SECONDS == 0 {
        format!("{}h", seconds / HOUR_SECONDS)
    } else if seconds % MINUTE_SECONDS == 0 {
        format!("{}m", seconds / MINUTE_SECONDS)
    } else {
        format!("{seconds}s")
    }
}

fn format_quota_percent(percent: u64) -> String {
    format!("{}%", percent.min(100))
}

fn cursor_split_preview(
    facts: &CursorWidgetFacts,
    fallback_primary_color: &str,
) -> Option<(&'static str, String, String)> {
    if facts.family.as_deref().map(str::trim) != Some("split") {
        return None;
    }

    let glyph = match facts.divider.as_deref().map(str::trim)? {
        "vertical" => "▌",
        "horizontal" => "▀",
        _ => return None,
    };
    let primary_color = facts
        .primary_color
        .as_deref()
        .and_then(normalize_hex_color)
        .unwrap_or_else(|| fallback_primary_color.to_string());
    let secondary_color = facts
        .secondary_color
        .as_deref()
        .and_then(normalize_hex_color)?;

    Some((glyph, primary_color, secondary_color))
}

fn render_cursor_status_widget_frame(
    accent_color: &str,
    glyph_segment: &str,
    name: &str,
) -> String {
    format!(
        " #[fg={accent_color},bg=default,bold][{glyph_segment}#[fg={accent_color},bg=default,bold] {name}]"
    )
}

fn normalize_hex_color(raw: &str) -> Option<String> {
    let normalized = raw.trim().to_ascii_lowercase();
    let valid = normalized.len() == 7
        && normalized.starts_with('#')
        && normalized[1..].bytes().all(|byte| byte.is_ascii_hexdigit());
    valid.then_some(normalized)
}

fn sanitize_cursor_name(name: &str) -> String {
    name.chars()
        .filter(|ch| ch.is_ascii_alphanumeric() || matches!(ch, '_' | '-' | '/' | '.'))
        .collect()
}

pub fn render_zjstatus_tab_label_formats(mode: &str) -> Result<TabLabelFormats, BarRenderError> {
    match mode {
        TAB_LABEL_MODE_FULL => Ok(TabLabelFormats {
            tab_normal: r##"tab_normal   "#[fg=#ffff00] [{index}] {name} ""##,
            tab_normal_fullscreen: r##"tab_normal_fullscreen "#[fg=#ffff00] [{index}] {name} [] ""##,
            tab_normal_sync: r##"tab_normal_sync       "#[fg=#ffff00] [{index}] {name} <> ""##,
            tab_active: r##"tab_active   "#[bg=#ff6600,fg=#000000,bold] [{index}] {name} {floating_indicator}""##,
            tab_active_fullscreen: r##"tab_active_fullscreen "#[bg=#ff6600,fg=#000000,bold] [{index}] {name} {fullscreen_indicator}""##,
            tab_active_sync: r##"tab_active_sync       "#[bg=#ff6600,fg=#000000,bold] [{index}] {name} {sync_indicator}""##,
            tab_rename: r##"tab_rename    "#[bg=#ff6600,fg=#000000,bold] {index} {name} {floating_indicator} ""##,
        }),
        TAB_LABEL_MODE_COMPACT => Ok(TabLabelFormats {
            tab_normal: r##"tab_normal   "#[fg=#ffff00] [{index}] ""##,
            tab_normal_fullscreen: r##"tab_normal_fullscreen "#[fg=#ffff00] [{index}] [] ""##,
            tab_normal_sync: r##"tab_normal_sync       "#[fg=#ffff00] [{index}] <> ""##,
            tab_active: r##"tab_active   "#[bg=#ff6600,fg=#000000,bold] [{index}] {floating_indicator}""##,
            tab_active_fullscreen: r##"tab_active_fullscreen "#[bg=#ff6600,fg=#000000,bold] [{index}] {fullscreen_indicator}""##,
            tab_active_sync: r##"tab_active_sync       "#[bg=#ff6600,fg=#000000,bold] [{index}] {sync_indicator}""##,
            tab_rename: r##"tab_rename    "#[bg=#ff6600,fg=#000000,bold] {index} {name} {floating_indicator} ""##,
        }),
        _ => Err(BarRenderError::InvalidTabLabelMode {
            mode: mode.to_string(),
        }),
    }
}

fn render_widget(widget: &str, request: &BarRenderRequest) -> Result<String, BarRenderError> {
    match widget {
        WIDGET_EDITOR => Ok(format!(
            " #[fg=#00ff88,bold][editor: {}]",
            request.editor_label
        )),
        WIDGET_SHELL => Ok(format!(
            " #[fg=#00ff88,bold][shell: {}]",
            request.shell_label
        )),
        WIDGET_TERM => Ok(format!(
            " #[fg=#00ff88,bold][term: {}]",
            request.terminal_label
        )),
        WIDGET_WORKSPACE => Ok(COMMAND_WORKSPACE.to_string()),
        WIDGET_CURSOR => Ok(COMMAND_CURSOR.to_string()),
        WIDGET_CLAUDE_USAGE => Ok(COMMAND_CLAUDE_USAGE.to_string()),
        WIDGET_CODEX_USAGE => Ok(COMMAND_CODEX_USAGE.to_string()),
        WIDGET_OPENCODE_GO_USAGE => Ok(COMMAND_OPENCODE_GO_USAGE.to_string()),
        WIDGET_CPU => Ok(COMMAND_CPU.to_string()),
        WIDGET_RAM => Ok(COMMAND_RAM.to_string()),
        _ => Err(BarRenderError::InvalidWidgetTrayEntry {
            entry: widget.to_string(),
        }),
    }
}

// Test lane: default
#[cfg(test)]
mod tests {
    use super::*;

    fn render_request(widget_tray: &[&str]) -> BarRenderRequest {
        BarRenderRequest {
            widget_tray: widget_tray
                .iter()
                .map(|widget| widget.to_string())
                .collect(),
            editor_label: "hx".to_string(),
            shell_label: "nu".to_string(),
            terminal_label: "ghostty".to_string(),
            custom_text: String::new(),
        }
    }

    fn temp_test_dir(name: &str) -> std::path::PathBuf {
        let unique = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_nanos();
        let path = std::env::temp_dir().join(format!(
            "yazelix_bar_{name}_{}_{}",
            std::process::id(),
            unique
        ));
        let _ = std::fs::remove_dir_all(&path);
        std::fs::create_dir_all(&path).unwrap();
        path
    }

    #[cfg(unix)]
    fn write_tokenusage_provider_script(bin_dir: &std::path::Path, body: &str) {
        use std::os::unix::fs::PermissionsExt;

        std::fs::create_dir_all(bin_dir).unwrap();
        let path = bin_dir.join("tu");
        let shell = std::env::var_os("PATH")
            .and_then(|path| {
                std::env::split_paths(&path)
                    .map(|entry| entry.join("sh"))
                    .find(|candidate| candidate.is_file())
            })
            .unwrap_or_else(|| std::path::PathBuf::from("/bin/sh"));
        let script = body.replacen("#!/usr/bin/env sh", &format!("#!{}", shell.display()), 1);
        std::fs::write(&path, script).unwrap();
        let mut permissions = std::fs::metadata(&path).unwrap().permissions();
        permissions.set_mode(0o755);
        std::fs::set_permissions(path, permissions).unwrap();
    }

    fn write_opencode_go_test_db(path: &std::path::Path, now: u64) {
        let connection = rusqlite::Connection::open(path).unwrap();
        connection
            .execute(
                "CREATE TABLE message (time_created INTEGER NOT NULL, data TEXT NOT NULL)",
                [],
            )
            .unwrap();
        let rows = [
            (
                now.saturating_sub(60),
                r#"{"role":"assistant","providerID":"opencode-go","tokens":{"total":1000},"cost":3.0}"#,
            ),
            (
                now.saturating_sub(6 * 60 * 60),
                r#"{"role":"assistant","providerID":"opencode-go","tokens":{"input":1200,"output":800},"cost":6.0}"#,
            ),
            (
                now.saturating_sub(8 * 24 * 60 * 60),
                r#"{"role":"assistant","providerID":"opencode-go","tokens":{"total":30000},"cost":30.0}"#,
            ),
            (
                now.saturating_sub(60),
                r#"{"role":"user","providerID":"opencode-go","tokens":{"total":900000},"cost":99.0}"#,
            ),
            (
                now.saturating_sub(60),
                r#"{"role":"assistant","providerID":"other","tokens":{"total":900000},"cost":99.0}"#,
            ),
        ];
        for (created_at, data) in rows {
            connection
                .execute(
                    "INSERT INTO message (time_created, data) VALUES (?1, ?2)",
                    rusqlite::params![unix_millis_from_seconds_saturating(created_at), data],
                )
                .unwrap();
        }
    }

    // Defends: the bar registry preserves the existing zjstatus widgets and exact segment syntax.
    // Strength: defect=2 behavior=2 resilience=1 cost=1 uniqueness=2 total=8/10
    #[test]
    fn renders_existing_widget_tray_segments() {
        let rendered = render_widget_tray_segment(&render_request(DEFAULT_WIDGET_TRAY)).unwrap();

        assert_eq!(
            rendered,
            " #[fg=#00ff88,bold][editor: hx] #[fg=#00ff88,bold][shell: nu] #[fg=#00ff88,bold][term: ghostty]{command_cursor}{command_codex_usage}{command_cpu}{command_ram}"
        );
    }

    // Defends: a deliberately empty tray stays empty rather than introducing stray spacing.
    // Strength: defect=1 behavior=2 resilience=1 cost=2 uniqueness=2 total=8/10
    #[test]
    fn renders_empty_widget_tray_without_padding() {
        let rendered = render_widget_tray_segment(&render_request(&[])).unwrap();

        assert_eq!(rendered, "");
    }

    // Regression: dynamic status-bus widgets render through cached zjstatus command placeholders instead of being silently hidden.
    // Strength: defect=2 behavior=2 resilience=2 cost=1 uniqueness=2 total=9/10
    #[test]
    fn renders_status_bus_widgets_as_cached_command_placeholders() {
        let rendered =
            render_widget_tray_segment(&render_request(&["workspace", "cursor"])).unwrap();

        assert_eq!(rendered, "{command_workspace}{command_cursor}");
    }

    // Regression: agent usage widgets render through cache readers so expensive providers are never polled by zjstatus.
    // Strength: defect=2 behavior=2 resilience=2 cost=1 uniqueness=2 total=9/10
    #[test]
    fn renders_agent_usage_widgets_as_cached_command_placeholders() {
        let rendered = render_widget_tray_segment(&render_request(&[
            "claude_usage",
            "codex_usage",
            "opencode_go_usage",
        ]))
        .unwrap();

        assert_eq!(
            rendered,
            "{command_claude_usage}{command_codex_usage}{command_opencode_go_usage}"
        );
    }

    // Regression: the full Yazelix integration renderer owns generic zjstatus command-definition KDL while Yazelix core owns only resolved helper paths.
    // Strength: defect=2 behavior=2 resilience=2 cost=1 uniqueness=2 total=9/10
    #[test]
    fn renders_yazelix_runtime_command_definitions() {
        let rendered = render_yazelix_runtime_command_definitions(&YazelixRuntimeCommandPaths {
            nu_bin: "/runtime/bin/nu".to_string(),
            yzx_control_bin: "/runtime/bin/yzx_control".to_string(),
            yazelix_bar_widget_bin: "/runtime/bin/yazelix_bar_widget".to_string(),
            runtime_dir: "/runtime".to_string(),
            claude_usage_display: "both".to_string(),
            codex_usage_display: "quota".to_string(),
            opencode_go_usage_display: "both".to_string(),
        });

        assert!(rendered.contains(
            r#"command_workspace_command "/runtime/bin/yzx_control zellij status-cache-widget workspace""#
        ));
        assert!(rendered.contains(r##"command_workspace_format "#[fg=#00ff88,bold]{stdout}""##));
        assert!(rendered.contains(r#"command_workspace_interval "1""#));
        assert!(
            rendered.contains(r#"command_cursor_command "/runtime/bin/yazelix_bar_widget cursor""#)
        );
        assert!(rendered.contains(r#"command_cursor_rendermode "dynamic""#));
        assert!(rendered.contains(
            r#"command_claude_usage_command "/runtime/bin/yazelix_bar_widget claude_usage --display both""#
        ));
        assert!(rendered.contains(
            r#"command_codex_usage_command "/runtime/bin/yazelix_bar_widget codex_usage --display quota""#
        ));
        assert!(rendered.contains(
            r#"command_opencode_go_usage_command "/runtime/bin/yazelix_bar_widget opencode_go_usage --display both""#
        ));
        assert!(rendered.contains(r#"command_cpu_command "/runtime/bin/yazelix_bar_widget cpu""#));
        assert!(rendered.contains(r#"command_ram_command "/runtime/bin/yazelix_bar_widget ram""#));
        assert!(rendered.contains(
            r#"command_version_command "/runtime/bin/nu -c 'use /runtime/nushell/scripts/utils/constants.nu YAZELIX_VERSION; $YAZELIX_VERSION'""#
        ));
    }

    // Defends: CPU widget math uses /proc/stat deltas and hides malformed samples.
    // Strength: defect=2 behavior=2 resilience=2 cost=1 uniqueness=2 total=9/10
    #[test]
    fn calculates_cpu_widget_percent_from_proc_stat_snapshots() {
        let before = "cpu  100 0 50 850 0 0 0 0\n";
        let after = "cpu  130 0 70 900 0 0 0 0\n";

        assert_eq!(cpu_usage_percent_from_proc_stat(before, after), Some(50));
        assert_eq!(cpu_usage_percent_from_proc_stat("cpu  1 2\n", after), None);
        assert_eq!(cpu_usage_percent_from_proc_stat(after, before), None);
    }

    // Defends: RAM widget math uses MemAvailable rather than Yazelix/Nushell runtime state.
    // Strength: defect=2 behavior=2 resilience=2 cost=1 uniqueness=2 total=9/10
    #[test]
    fn calculates_ram_widget_percent_from_meminfo() {
        let meminfo = "\
MemTotal:       1000000 kB
MemFree:         100000 kB
MemAvailable:   250000 kB
";

        assert_eq!(ram_usage_percent_from_meminfo(meminfo), Some(75));
        assert_eq!(
            ram_usage_percent_from_meminfo("MemTotal: 0 kB\nMemAvailable: 0 kB\n"),
            None
        );
        assert_eq!(
            ram_usage_percent_from_meminfo("MemTotal: 10 kB\nMemAvailable: 20 kB\n"),
            None
        );
    }

    // Defends: standalone cursor rendering accepts Yazelix-compatible launch facts without importing Yazelix runtime code.
    // Strength: defect=2 behavior=2 resilience=2 cost=1 uniqueness=2 total=9/10
    #[test]
    fn cursor_widget_facts_render_from_env_pairs() {
        let rendered = render_cursor_status_widget(&cursor_widget_facts_from_pairs([
            (CURSOR_NAME_FACT.to_string(), "reef".to_string()),
            (CURSOR_COLOR_FACT.to_string(), "#00ff66".to_string()),
            (CURSOR_FAMILY_FACT.to_string(), "split".to_string()),
            (CURSOR_DIVIDER_FACT.to_string(), "vertical".to_string()),
            (CURSOR_PRIMARY_COLOR_FACT.to_string(), "#00e6ff".to_string()),
            (
                CURSOR_SECONDARY_COLOR_FACT.to_string(),
                "#00ff66".to_string(),
            ),
        ]));

        assert_eq!(
            rendered,
            " #[fg=#00ff66,bg=default,bold][#[fg=#00e6ff,bg=#00ff66,bold]▌#[fg=#00ff66,bg=default,bold] reef]"
        );
    }

    // Defends: standalone users can feed cursor facts from a small path-based fact file instead of Yazelix session env.
    // Strength: defect=2 behavior=2 resilience=2 cost=1 uniqueness=2 total=9/10
    #[test]
    fn cursor_widget_facts_parse_key_value_fact_file() {
        let facts = cursor_widget_facts_from_pairs(cursor_fact_file_pairs(
            "\n# cursor facts\nYAZELIX_CURSOR_NAME = ember\nYAZELIX_CURSOR_COLOR = #ff8800\nignored=value\n",
        ));

        assert_eq!(
            render_cursor_status_widget(&facts),
            " #[fg=#ff8800,bg=default,bold][#[fg=#ff8800,bold]█#[fg=#ff8800,bg=default,bold] ember]"
        );
    }

    // Regression: dynamic command placeholders must preserve stable spacing around safe widgets.
    // Strength: defect=2 behavior=2 resilience=2 cost=1 uniqueness=2 total=9/10
    #[test]
    fn dynamic_widgets_do_not_leave_spacing_artifacts() {
        let rendered =
            render_widget_tray_segment(&render_request(&["editor", "workspace", "shell"])).unwrap();

        assert_eq!(
            rendered,
            " #[fg=#00ff88,bold][editor: hx]{command_workspace} #[fg=#00ff88,bold][shell: nu]"
        );
    }

    // Defends: custom text remains trim-aware and does not reserve bar space when absent.
    // Strength: defect=1 behavior=2 resilience=1 cost=2 uniqueness=2 total=8/10
    #[test]
    fn renders_custom_text_segment_only_when_present() {
        assert_eq!(
            render_custom_text_segment("  verdant-lake  "),
            "#[fg=#ffff00,bold][verdant-lake] "
        );
        assert_eq!(render_custom_text_segment("   "), "");
    }

    // Defends: standalone cursor facts render the same mono and split previews Yazelix feeds into the status cache.
    // Strength: defect=2 behavior=2 resilience=2 cost=1 uniqueness=2 total=9/10
    #[test]
    fn renders_cursor_status_widget_from_explicit_facts() {
        assert_eq!(
            render_cursor_status_widget(&CursorWidgetFacts {
                name: "reef".into(),
                color: Some("#14D9A0".into()),
                family: Some("mono".into()),
                ..CursorWidgetFacts::default()
            }),
            " #[fg=#14d9a0,bg=default,bold][#[fg=#14d9a0,bold]█#[fg=#14d9a0,bg=default,bold] reef]"
        );
        assert_eq!(
            render_cursor_status_widget(&CursorWidgetFacts {
                name: "reef".into(),
                color: Some("#00e6ff".into()),
                family: Some("split".into()),
                divider: Some("vertical".into()),
                primary_color: Some("#00e6ff".into()),
                secondary_color: Some("#00ff66".into()),
            }),
            " #[fg=#00e6ff,bg=default,bold][#[fg=#00e6ff,bg=#00ff66,bold]▌#[fg=#00e6ff,bg=default,bold] reef]"
        );
        assert_eq!(
            render_cursor_status_widget(&CursorWidgetFacts {
                name: "magma".into(),
                color: Some("#ff1600".into()),
                family: Some("split".into()),
                divider: Some("horizontal".into()),
                primary_color: Some("#ff1600".into()),
                secondary_color: Some("#2a3340".into()),
            }),
            " #[fg=#ff1600,bg=default,bold][#[fg=#ff1600,bg=#2a3340,bold]▀#[fg=#ff1600,bg=default,bold] magma]"
        );
    }

    // Regression: missing or unsafe cursor facts hide the widget or fall back without leaking markup-breaking text.
    // Strength: defect=2 behavior=2 resilience=2 cost=1 uniqueness=2 total=9/10
    #[test]
    fn cursor_status_widget_handles_missing_and_partial_facts() {
        assert_eq!(
            render_cursor_status_widget(&CursorWidgetFacts {
                name: "n/a".into(),
                ..CursorWidgetFacts::default()
            }),
            " #[fg=#00ff88,bg=default,bold][#[fg=#00ff88,bold]█#[fg=#00ff88,bg=default,bold] n/a]"
        );
        assert_eq!(
            render_cursor_status_widget(&CursorWidgetFacts {
                name: "bad #[fg=#fff] name".into(),
                ..CursorWidgetFacts::default()
            }),
            " #[fg=#00ff88,bg=default,bold][#[fg=#00ff88,bold]█#[fg=#00ff88,bg=default,bold] badfgfffname]"
        );
        assert_eq!(
            render_cursor_status_widget(&CursorWidgetFacts::default()),
            ""
        );
        assert_eq!(
            render_cursor_status_widget(&CursorWidgetFacts {
                name: "magma".into(),
                color: Some("#ff1600".into()),
                family: Some("split".into()),
                divider: Some("horizontal".into()),
                primary_color: Some("#ff1600".into()),
                secondary_color: Some("hot".into()),
            }),
            " #[fg=#ff1600,bg=default,bold][#[fg=#ff1600,bold]█#[fg=#ff1600,bg=default,bold] magma]"
        );
    }

    // Defends: Codex cached usage facts support quota, token, both, partial quota, and elapsed reset-window labels outside Yazelix.
    // Strength: defect=2 behavior=2 resilience=2 cost=1 uniqueness=2 total=9/10
    #[test]
    fn renders_codex_usage_widget_from_cached_facts() {
        let facts = WindowedAgentUsageFacts {
            updated_at_unix_seconds: Some(1_700_001_800),
            five_hour_tokens: Some(1_234_567),
            weekly_tokens: Some(345_000_000),
            five_hour_remaining_percent: Some(88),
            weekly_remaining_percent: Some(101),
            five_hour_reset_at_unix_seconds: Some(1_700_018_000),
            weekly_reset_at_unix_seconds: Some(1_700_604_800),
            five_hour_window_seconds: Some(18_000),
            weekly_window_seconds: Some(604_800),
            ..WindowedAgentUsageFacts::default()
        };

        assert_eq!(
            render_codex_usage_status_widget(&facts, AgentUsageDisplay::Quota),
            " [codex 30m/5h 88% · 0h/7d 100%]"
        );
        assert_eq!(
            render_codex_usage_status_widget(&facts, AgentUsageDisplay::Token),
            " [codex 30m/5h 1.23M · 0h/7d 345M]"
        );
        assert_eq!(
            render_codex_usage_status_widget(&facts, AgentUsageDisplay::Both),
            " [codex 30m/5h 1.23M 88% · 0h/7d 345M 100%]"
        );
    }

    // Regression: Codex quota mode renders n/a for token-only windows but hides fully missing windows.
    // Strength: defect=2 behavior=2 resilience=2 cost=1 uniqueness=2 total=9/10
    #[test]
    fn codex_usage_widget_handles_partial_quota_facts() {
        let facts = WindowedAgentUsageFacts {
            five_hour_tokens: Some(42_000),
            ..WindowedAgentUsageFacts::default()
        };

        assert_eq!(
            render_codex_usage_status_widget(&facts, AgentUsageDisplay::Quota),
            " [codex 5h n/a]"
        );
        assert_eq!(
            render_codex_usage_status_widget(
                &WindowedAgentUsageFacts::default(),
                AgentUsageDisplay::Both
            ),
            ""
        );
    }

    // Defends: the standalone Codex command owns tokenusage probing, cache writes, reset-window labels, and rendering without Yazelix runtime paths.
    // Strength: defect=2 behavior=2 resilience=2 cost=1 uniqueness=2 total=9/10
    #[cfg(unix)]
    #[test]
    fn codex_usage_widget_refreshes_shared_cache_from_tokenusage_provider() {
        let temp = temp_test_dir("codex_usage_refresh");
        let bin_dir = temp.join("bin");
        write_tokenusage_provider_script(
            &bin_dir,
            r#"#!/usr/bin/env sh
if [ "$1" = "blocks" ] && [ "$4" = "--official-limits" ]; then
  printf '%s\n' '{"official_codex":{"primary_used_percent":51.0,"secondary_used_percent":20.0,"primary_resets_at":8200,"primary_window_mins":300,"secondary_resets_at":260200,"secondary_window_mins":10080}}'
elif [ "$1" = "blocks" ]; then
  printf '%s\n' '{"blocks":[{"isActive":true,"totals":{"total_tokens":138456789}}]}'
elif [ "$1" = "weekly" ]; then
  printf '%s\n' '{"weekly":[{"totals":{"total_tokens":1337000000}}]}'
else
  exit 1
fi
"#,
        );
        let cache_path = temp.join("agent_usage").join("codex_usage_cache_v2.json");

        let text = codex_usage_widget_text(CodexUsageWidgetOptions {
            cache_path: &cache_path,
            path_var: Some(bin_dir.as_os_str()),
            now_unix_seconds: 1_000,
            max_age_seconds: 600,
            error_backoff_seconds: 1_800,
            timeout: std::time::Duration::from_secs(1),
            display: AgentUsageDisplay::Both,
        })
        .unwrap();

        assert_eq!(text, " [codex 3h/5h 138M 49% · 4d/7d 1.34B 80%]");
        assert_eq!(
            read_codex_usage_shared_cache_value(&cache_path)
                .unwrap()
                .pointer("/codex/status")
                .and_then(serde_json::Value::as_str),
            Some("ok")
        );
        let _ = std::fs::remove_dir_all(temp);
    }

    // Defends: Codex cache freshness and error backoff are enforced by yazelix-bar, not by Yazelix wrappers.
    // Strength: defect=2 behavior=2 resilience=2 cost=1 uniqueness=2 total=9/10
    #[test]
    fn codex_usage_shared_cache_respects_freshness_and_backoff() {
        let temp = temp_test_dir("codex_usage_backoff");
        let cache_path = temp.join("codex_usage_cache_v2.json");
        write_json_value_atomic(
            &cache_path,
            &serde_json::json!({
                "schema_version": CODEX_USAGE_CACHE_SCHEMA_VERSION,
                "codex": {
                    "updated_at_unix_seconds": 1_000u64,
                    "five_hour_tokens": 138_456_789u64,
                    "weekly_tokens": 1_337_000_000u64,
                    "five_hour_remaining_percent": 49u64,
                    "weekly_remaining_percent": 80u64,
                    "five_hour_reset_at_unix_seconds": 8_200u64,
                    "weekly_reset_at_unix_seconds": 260_200u64,
                    "five_hour_window_seconds": 18_000u64,
                    "weekly_window_seconds": 604_800u64,
                    "status": "ok"
                }
            }),
        )
        .unwrap();

        assert!(codex_usage_shared_cache_is_fresh(&cache_path, 1_100, 600));
        assert!(!codex_usage_shared_cache_is_fresh(&cache_path, 1_700, 600));

        write_json_value_atomic(
            &cache_path,
            &serde_json::json!({
                "schema_version": CODEX_USAGE_CACHE_SCHEMA_VERSION,
                "codex": {
                    "updated_at_unix_seconds": 1_700u64,
                    "error": "missing tu",
                    "backoff_until_unix_seconds": 2_000u64,
                    "status": "error"
                }
            }),
        )
        .unwrap();

        assert!(codex_usage_shared_cache_is_backing_off(&cache_path, 1_999));
        assert!(!codex_usage_shared_cache_is_backing_off(&cache_path, 2_000));
        let _ = std::fs::remove_dir_all(temp);
    }

    // Defends: Yazelix runtime commands can derive provider cache files from the window status-cache env without calling yzx_control.
    // Strength: defect=2 behavior=2 resilience=2 cost=1 uniqueness=2 total=9/10
    #[test]
    fn agent_usage_cache_paths_derive_from_status_cache_path() {
        let status_cache =
            std::path::Path::new("/state/sessions/window_a/status/status_bar_cache.json");

        assert_eq!(
            agent_usage_cache_path_from_status_cache_path(
                status_cache,
                "codex_usage_cache",
                CODEX_USAGE_CACHE_SCHEMA_VERSION,
            ),
            Some(std::path::PathBuf::from(
                "/state/sessions/agent_usage/codex_usage_cache_v2.json"
            ))
        );
    }

    // Defends: the standalone Claude command owns tokenusage probing, cache writes, and rendering without Yazelix runtime paths.
    // Strength: defect=2 behavior=2 resilience=2 cost=1 uniqueness=2 total=9/10
    #[cfg(unix)]
    #[test]
    fn claude_usage_widget_refreshes_shared_cache_from_tokenusage_provider() {
        let temp = temp_test_dir("claude_usage_refresh");
        let bin_dir = temp.join("bin");
        write_tokenusage_provider_script(
            &bin_dir,
            r#"#!/usr/bin/env sh
if [ "$1" = "blocks" ] && [ "$4" = "--official-limits" ]; then
  printf '%s\n' '{"official_claude":{"primary_used_percent":51.0,"secondary_used_percent":20.0}}'
elif [ "$1" = "blocks" ]; then
  printf '%s\n' '{"blocks":[{"isActive":true,"totals":{"total_tokens":15456373}}]}'
elif [ "$1" = "weekly" ]; then
  printf '%s\n' '{"weekly":[{"totals":{"total_tokens":66610005}}]}'
else
  exit 1
fi
"#,
        );
        let cache_path = temp.join("agent_usage").join("claude_usage_cache_v1.json");

        let text = claude_usage_widget_text(ClaudeUsageWidgetOptions {
            cache_path: &cache_path,
            path_var: Some(bin_dir.as_os_str()),
            now_unix_seconds: 1_000,
            max_age_seconds: 600,
            error_backoff_seconds: 1_800,
            timeout: std::time::Duration::from_secs(1),
            display: AgentUsageDisplay::Both,
        })
        .unwrap();

        assert_eq!(text, " [claude 5h|15.5M|49% wk|66.6M|80%]");
        assert_eq!(
            read_claude_usage_shared_cache_value(&cache_path)
                .unwrap()
                .pointer("/claude/status")
                .and_then(serde_json::Value::as_str),
            Some("ok")
        );
        let _ = std::fs::remove_dir_all(temp);
    }

    // Defends: Claude cache freshness and error backoff are enforced by yazelix-bar, not by Yazelix wrappers.
    // Strength: defect=2 behavior=2 resilience=2 cost=1 uniqueness=2 total=9/10
    #[test]
    fn claude_usage_shared_cache_respects_freshness_and_backoff() {
        let temp = temp_test_dir("claude_usage_backoff");
        let cache_path = temp.join("claude_usage_cache_v1.json");
        write_json_value_atomic(
            &cache_path,
            &serde_json::json!({
                "schema_version": CLAUDE_USAGE_CACHE_SCHEMA_VERSION,
                "claude": {
                    "updated_at_unix_seconds": 1_000u64,
                    "five_hour_tokens": 15_456_373u64,
                    "weekly_tokens": 66_610_005u64,
                    "five_hour_remaining_percent": 49u64,
                    "weekly_remaining_percent": 80u64,
                    "status": "ok"
                }
            }),
        )
        .unwrap();

        assert!(claude_usage_shared_cache_is_fresh(&cache_path, 1_100, 600));
        assert!(!claude_usage_shared_cache_is_fresh(&cache_path, 1_700, 600));

        write_json_value_atomic(
            &cache_path,
            &serde_json::json!({
                "schema_version": CLAUDE_USAGE_CACHE_SCHEMA_VERSION,
                "claude": {
                    "updated_at_unix_seconds": 1_700u64,
                    "error": "missing tu",
                    "backoff_until_unix_seconds": 2_000u64,
                    "status": "error"
                }
            }),
        )
        .unwrap();

        assert!(claude_usage_shared_cache_is_backing_off(&cache_path, 1_999));
        assert!(!claude_usage_shared_cache_is_backing_off(
            &cache_path,
            2_000
        ));
        let _ = std::fs::remove_dir_all(temp);
    }

    // Defends: generic provider usage rendering covers Claude-style configured windows without Yazelix cache paths.
    // Strength: defect=2 behavior=2 resilience=2 cost=1 uniqueness=2 total=9/10
    #[test]
    fn renders_windowed_agent_usage_widget_for_claude_facts() {
        let facts = WindowedAgentUsageFacts {
            five_hour_tokens: Some(15_456_373),
            weekly_tokens: Some(66_610_005),
            five_hour_remaining_percent: Some(49),
            weekly_remaining_percent: Some(80),
            ..WindowedAgentUsageFacts::default()
        };
        let periods = [AgentUsagePeriod::FiveHour, AgentUsagePeriod::Weekly];

        assert_eq!(
            render_windowed_agent_usage_status_widget(
                "claude",
                &facts,
                &periods,
                AgentUsageDisplay::Both
            ),
            " [claude 5h|15.5M|49% wk|66.6M|80%]"
        );
        assert_eq!(
            render_windowed_agent_usage_status_widget(
                "claude",
                &facts,
                &periods,
                AgentUsageDisplay::Token
            ),
            " [claude 5h|15.5M wk|66.6M]"
        );
        assert_eq!(
            render_windowed_agent_usage_status_widget(
                "claude",
                &facts,
                &periods,
                AgentUsageDisplay::Quota
            ),
            " [claude 5h|49% wk|80%]"
        );
    }

    // Defends: OpenCode Go standalone rendering supports configured 5h/week/month windows and missing-cache hiding.
    // Strength: defect=2 behavior=2 resilience=2 cost=1 uniqueness=2 total=9/10
    #[test]
    fn renders_windowed_agent_usage_widget_for_opencode_go_facts() {
        let facts = WindowedAgentUsageFacts {
            five_hour_tokens: Some(138_424_632),
            weekly_tokens: Some(1_335_519_960),
            monthly_tokens: Some(2_220_000_000),
            five_hour_remaining_percent: Some(49),
            weekly_remaining_percent: Some(80),
            monthly_remaining_percent: Some(70),
            ..WindowedAgentUsageFacts::default()
        };
        let periods = [
            AgentUsagePeriod::FiveHour,
            AgentUsagePeriod::Weekly,
            AgentUsagePeriod::Monthly,
        ];

        assert_eq!(
            render_windowed_agent_usage_status_widget(
                "go",
                &facts,
                &periods,
                AgentUsageDisplay::Both
            ),
            " [go 5h|138M|49% wk|1.34B|80% mo|2.22B|70%]"
        );
        assert_eq!(
            render_windowed_agent_usage_status_widget(
                "go",
                &facts,
                &periods,
                AgentUsageDisplay::Token
            ),
            " [go 5h|138M wk|1.34B mo|2.22B]"
        );
        assert_eq!(
            render_windowed_agent_usage_status_widget(
                "go",
                &WindowedAgentUsageFacts::default(),
                &periods,
                AgentUsageDisplay::Both
            ),
            ""
        );
    }

    // Defends: the standalone OpenCode Go command owns DB probing, cost-limit windows, cache writes, and rendering without Yazelix runtime paths.
    // Strength: defect=2 behavior=2 resilience=2 cost=1 uniqueness=2 total=9/10
    #[test]
    fn opencode_go_usage_widget_refreshes_shared_cache_from_db() {
        let temp = temp_test_dir("opencode_go_usage_refresh");
        let now = 10_000_000;
        let db_path = temp.join("opencode.db");
        write_opencode_go_test_db(&db_path, now);
        let cache_path = temp
            .join("agent_usage")
            .join("opencode_go_usage_cache_v1.json");

        let text = opencode_go_usage_widget_text(OpenCodeGoUsageWidgetOptions {
            cache_path: &cache_path,
            db_paths: &[db_path],
            now_unix_seconds: now,
            max_age_seconds: 600,
            error_backoff_seconds: 1_800,
            display: AgentUsageDisplay::Both,
        })
        .unwrap();

        assert_eq!(text, " [go 5h|1k|75% wk|3k|70% mo|33k|35%]");
        assert_eq!(
            read_opencode_go_usage_shared_cache_value(&cache_path)
                .unwrap()
                .pointer("/opencode_go/status")
                .and_then(serde_json::Value::as_str),
            Some("ok")
        );
        let _ = std::fs::remove_dir_all(temp);
    }

    // Defends: OpenCode Go cache freshness and error backoff are enforced by yazelix-bar, not by Yazelix wrappers.
    // Strength: defect=2 behavior=2 resilience=2 cost=1 uniqueness=2 total=9/10
    #[test]
    fn opencode_go_usage_shared_cache_respects_freshness_and_backoff() {
        let temp = temp_test_dir("opencode_go_usage_backoff");
        let cache_path = temp.join("opencode_go_usage_cache_v1.json");
        write_json_value_atomic(
            &cache_path,
            &serde_json::json!({
                "schema_version": OPENCODE_GO_USAGE_CACHE_SCHEMA_VERSION,
                "opencode_go": {
                    "updated_at_unix_seconds": 1_000u64,
                    "five_hour_tokens": 1_000u64,
                    "weekly_tokens": 3_000u64,
                    "monthly_tokens": 33_000u64,
                    "five_hour_remaining_percent": 75u64,
                    "weekly_remaining_percent": 70u64,
                    "monthly_remaining_percent": 35u64,
                    "status": "ok"
                }
            }),
        )
        .unwrap();

        assert!(opencode_go_usage_shared_cache_is_fresh(
            &cache_path,
            1_100,
            600
        ));
        assert!(!opencode_go_usage_shared_cache_is_fresh(
            &cache_path,
            1_700,
            600
        ));

        write_json_value_atomic(
            &cache_path,
            &serde_json::json!({
                "schema_version": OPENCODE_GO_USAGE_CACHE_SCHEMA_VERSION,
                "opencode_go": {
                    "updated_at_unix_seconds": 1_700u64,
                    "error": "missing OpenCode DB",
                    "backoff_until_unix_seconds": 2_000u64,
                    "status": "error"
                }
            }),
        )
        .unwrap();

        assert!(opencode_go_usage_shared_cache_is_backing_off(
            &cache_path,
            1_999
        ));
        assert!(!opencode_go_usage_shared_cache_is_backing_off(
            &cache_path,
            2_000
        ));
        let _ = std::fs::remove_dir_all(temp);
    }

    // Regression: unsupported widget names must fail fast instead of leaving broken zjstatus placeholders.
    // Strength: defect=2 behavior=2 resilience=2 cost=1 uniqueness=2 total=9/10
    #[test]
    fn rejects_unknown_widget_tray_entries() {
        let error =
            render_widget_tray_segment(&render_request(&["editor", "weather"])).unwrap_err();

        assert_eq!(
            error,
            BarRenderError::InvalidWidgetTrayEntry {
                entry: "weather".to_string()
            }
        );
        assert_eq!(error.code(), "invalid_widget_tray_entry");
    }

    // Defends: full tab labels keep the existing index plus name format unless compact mode is explicitly enabled.
    // Strength: defect=2 behavior=2 resilience=1 cost=1 uniqueness=2 total=8/10
    #[test]
    fn renders_full_tab_label_formats_by_default_contract() {
        let formats = render_zjstatus_tab_label_formats(TAB_LABEL_MODE_FULL).unwrap();

        assert!(formats.tab_normal.contains("[{index}] {name}"));
        assert!(formats.tab_active.contains("[{index}] {name}"));
        assert!(formats.tab_rename.contains("{index} {name}"));
    }

    // Defends: compact tab labels remove tab names from normal rendering while preserving index and state indicators.
    // Strength: defect=2 behavior=2 resilience=1 cost=1 uniqueness=2 total=8/10
    #[test]
    fn renders_compact_tab_label_formats_without_names() {
        let formats = render_zjstatus_tab_label_formats(TAB_LABEL_MODE_COMPACT).unwrap();

        assert_eq!(
            formats.tab_normal,
            r##"tab_normal   "#[fg=#ffff00] [{index}] ""##
        );
        assert!(formats.tab_normal_fullscreen.contains("{index}] []"));
        assert!(formats.tab_active_sync.contains("{sync_indicator}"));
        assert!(!formats.tab_active.contains("{name}"));
        assert!(formats.tab_rename.contains("{name}"));
    }

    // Regression: unsupported tab label modes fail fast instead of emitting broken zjstatus KDL.
    // Strength: defect=2 behavior=2 resilience=2 cost=1 uniqueness=2 total=9/10
    #[test]
    fn rejects_unknown_tab_label_mode() {
        let error = render_zjstatus_tab_label_formats("tiny").unwrap_err();

        assert_eq!(
            error,
            BarRenderError::InvalidTabLabelMode {
                mode: "tiny".to_string()
            }
        );
        assert_eq!(error.code(), "invalid_tab_label_mode");
    }

    // Defends: the standalone generator emits a complete generic zjstatus preset without Yazelix runtime helper commands.
    // Strength: defect=2 behavior=2 resilience=1 cost=1 uniqueness=2 total=8/10
    #[test]
    fn standalone_generator_emits_generic_default_preset() {
        let rendered = generate_standalone_preset(&StandalonePresetOptions::default()).unwrap();

        assert!(rendered.contains("plugin location=\"__YAZELIX_BAR_ZJSTATUS_WASM__\""));
        assert!(rendered.contains("format_left   \"{mode} {tabs} \""));
        assert!(rendered.contains("format_right  \"#[fg=#ff0088,bold]{session} {datetime} #[fg=#00ccff,bold]YAZELIX BAR \""));
        assert!(rendered.contains("datetime_format \"%H:%M %d/%m\""));
        assert!(!rendered.contains("yzx_control"));
        assert!(!rendered.contains("__YAZELIX_RUNTIME_DIR__"));
    }

    // Defends: structured command-widget options generate zjstatus command keys without requiring users to copy a whole preset.
    // Strength: defect=2 behavior=2 resilience=1 cost=1 uniqueness=2 total=8/10
    #[test]
    fn standalone_generator_emits_command_widgets_from_structured_options() {
        let mut options = StandalonePresetOptions {
            brand_label: "DEV BAR".into(),
            format_right: vec![
                StandalonePresetPart::Session,
                StandalonePresetPart::Command("host".into()),
                StandalonePresetPart::Brand,
            ],
            ..StandalonePresetOptions::default()
        };
        options.command_widgets.push(StandaloneCommandWidget {
            name: "host".into(),
            command: "hostname -s".into(),
            format: " #[fg=#00ff88,bold][{stdout}]".into(),
            interval: "30".into(),
        });

        let rendered = generate_standalone_preset(&options).unwrap();

        assert!(rendered.contains(
            "format_right  \"#[fg=#ff0088,bold]{session} {command_host} #[fg=#00ccff,bold]DEV BAR \""
        ));
        assert!(rendered.contains("command_host_command \"hostname -s\""));
        assert!(rendered.contains("command_host_format \" #[fg=#00ff88,bold][{stdout}]\""));
        assert!(rendered.contains("command_host_interval \"30\""));
    }

    // Regression: generator command names must fail fast before producing invalid zjstatus KDL keys.
    // Strength: defect=2 behavior=2 resilience=2 cost=1 uniqueness=2 total=9/10
    #[test]
    fn standalone_generator_rejects_invalid_command_names() {
        let mut options = StandalonePresetOptions::default();
        options
            .command_widgets
            .push(StandaloneCommandWidget::new("bad-name", "hostname -s"));

        let error = generate_standalone_preset(&options).unwrap_err();

        assert_eq!(
            error,
            StandalonePresetError::InvalidCommandName {
                name: "bad-name".into()
            }
        );
    }
}
