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
        command: YazelixRuntimeCommand::StatusCacheWidget("cursor"),
        format: "{stdout}",
        render_mode: Some("dynamic"),
        interval: "10",
    },
    YazelixRuntimeCommandWidget {
        name: "claude_usage",
        command: YazelixRuntimeCommand::StatusCacheWidget("claude_usage"),
        format: "#[fg=#bb88ff,bold]{stdout}",
        render_mode: None,
        interval: "10",
    },
    YazelixRuntimeCommandWidget {
        name: "codex_usage",
        command: YazelixRuntimeCommand::StatusCacheWidget("codex_usage"),
        format: "#[fg=#bb88ff,bold]{stdout}",
        render_mode: None,
        interval: "10",
    },
    YazelixRuntimeCommandWidget {
        name: "opencode_go_usage",
        command: YazelixRuntimeCommand::StatusCacheWidget("opencode_go_usage"),
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
        });

        assert!(rendered.contains(
            r#"command_workspace_command "/runtime/bin/yzx_control zellij status-cache-widget workspace""#
        ));
        assert!(rendered.contains(r##"command_workspace_format "#[fg=#00ff88,bold]{stdout}""##));
        assert!(rendered.contains(r#"command_workspace_interval "1""#));
        assert!(rendered.contains(
            r#"command_cursor_command "/runtime/bin/yzx_control zellij status-cache-widget cursor""#
        ));
        assert!(rendered.contains(r#"command_cursor_rendermode "dynamic""#));
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
