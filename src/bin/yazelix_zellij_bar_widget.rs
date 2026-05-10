use std::env;
use std::process;
use std::process::Command;

fn main() {
    match run(env::args().skip(1).collect()) {
        Ok(output) => println!("{output}"),
        Err(error) => {
            eprintln!("{error}");
            process::exit(1);
        }
    }
}

fn run(args: Vec<String>) -> Result<String, String> {
    let Some((command, rest)) = args.split_first() else {
        return Err(
            "expected command: claude, codex, cursor, cpu, opencode_go, ram, or render-yazelix-runtime"
                .to_string(),
        );
    };
    match command.as_str() {
        "cursor" => run_cursor(rest),
        "claude" => run_claude_usage(rest),
        "codex" => run_codex_usage(rest),
        "opencode_go" => run_opencode_go_usage(rest),
        "render-yazelix-runtime" => run_render_yazelix_runtime(rest),
        "cpu" if rest.is_empty() => Ok(yazelix_zellij_bar::current_cpu_usage_widget_text()),
        "ram" if rest.is_empty() => Ok(yazelix_zellij_bar::current_ram_usage_widget_text()),
        "cpu" | "ram" => Err(format!("{command} accepts no arguments")),
        _ => Err(format!("unknown widget command: {command}")),
    }
}

fn run_render_yazelix_runtime(args: &[String]) -> Result<String, String> {
    let [flag, raw_request] = args else {
        return Err(
            "render-yazelix-runtime usage: yazelix_zellij_bar_widget render-yazelix-runtime --json <request-json>"
                .to_string(),
        );
    };
    if flag != "--json" {
        return Err("render-yazelix-runtime expects --json <request-json>".to_string());
    }

    let config: yazelix_zellij_bar::YazelixRuntimeBarConfig = serde_json::from_str(raw_request)
        .map_err(|error| format!("invalid runtime bar config json: {error}"))?;
    let plugin_block = yazelix_zellij_bar::render_yazelix_runtime_plugin_block(&config)
        .map_err(|error| format!("failed to render Yazelix runtime bar: {}", error.code()))?;

    serde_json::to_string(&yazelix_zellij_bar::YazelixRuntimeBarRender {
        schema_version: yazelix_zellij_bar::YAZELIX_RUNTIME_BAR_RENDER_SCHEMA_VERSION,
        plugin_block,
    })
    .map_err(|error| format!("failed to encode Yazelix runtime bar render: {error}"))
}

fn run_cursor(args: &[String]) -> Result<String, String> {
    match args {
        [] => {
            let rendered = yazelix_zellij_bar::cursor_widget_text_from_env();
            if !rendered.is_empty() {
                return Ok(rendered);
            }
            Ok(cursor_widget_text_from_yzc())
        }
        [flag, path] if flag == "--facts" => {
            yazelix_zellij_bar::cursor_widget_text_from_fact_file(path)
                .map_err(|error| format!("failed to read cursor fact file {path}: {error}"))
        }
        _ => Err("cursor usage: yazelix_zellij_bar_widget cursor [--facts <path>]".to_string()),
    }
}

fn run_claude_usage(args: &[String]) -> Result<String, String> {
    let mut cache_path = None;
    let mut display = yazelix_zellij_bar::AgentUsageDisplay::Both;
    let mut max_age_seconds = 600;
    let mut error_backoff_seconds = 1_800;
    let mut timeout_ms = 5_000;
    let mut iter = args.iter();
    while let Some(arg) = iter.next() {
        match arg.as_str() {
            "--cache" => {
                cache_path = Some(std::path::PathBuf::from(
                    iter.next()
                        .ok_or_else(|| "--cache requires a value".to_string())?,
                ));
            }
            "--display" => {
                display = parse_agent_usage_display(
                    iter.next()
                        .ok_or_else(|| "--display requires a value".to_string())?,
                )?;
            }
            "--max-age-seconds" => {
                max_age_seconds = parse_u64_arg("--max-age-seconds", iter.next())?;
            }
            "--error-backoff-seconds" => {
                error_backoff_seconds = parse_u64_arg("--error-backoff-seconds", iter.next())?;
            }
            "--timeout-ms" => {
                timeout_ms = parse_u64_arg("--timeout-ms", iter.next())?.max(1);
            }
            _ => {
                return Err(format!("unknown claude_usage argument: {arg}"));
            }
        }
    }
    let cache_path = cache_path
        .or_else(yazelix_zellij_bar::claude_usage_cache_path_from_env)
        .or_else(|| default_cache_path("claude_usage_cache_v1.json"))
        .ok_or_else(|| {
            "claude usage: yazelix_zellij_bar_widget claude [--cache <path>] [--display quota|token|both]".to_string()
        })?;
    yazelix_zellij_bar::claude_usage_widget_text(yazelix_zellij_bar::ClaudeUsageWidgetOptions {
        cache_path: &cache_path,
        path_var: env::var_os("PATH").as_deref(),
        now_unix_seconds: unix_time_seconds(),
        max_age_seconds,
        error_backoff_seconds,
        timeout: std::time::Duration::from_millis(timeout_ms),
        display,
    })
}

fn run_codex_usage(args: &[String]) -> Result<String, String> {
    let mut cache_path = None;
    let mut display = yazelix_zellij_bar::AgentUsageDisplay::Quota;
    let mut max_age_seconds = 600;
    let mut error_backoff_seconds = 1_800;
    let mut timeout_ms = 5_000;
    let mut iter = args.iter();
    while let Some(arg) = iter.next() {
        match arg.as_str() {
            "--cache" => {
                cache_path = Some(std::path::PathBuf::from(
                    iter.next()
                        .ok_or_else(|| "--cache requires a value".to_string())?,
                ));
            }
            "--display" => {
                display = parse_agent_usage_display(
                    iter.next()
                        .ok_or_else(|| "--display requires a value".to_string())?,
                )?;
            }
            "--max-age-seconds" => {
                max_age_seconds = parse_u64_arg("--max-age-seconds", iter.next())?;
            }
            "--error-backoff-seconds" => {
                error_backoff_seconds = parse_u64_arg("--error-backoff-seconds", iter.next())?;
            }
            "--timeout-ms" => {
                timeout_ms = parse_u64_arg("--timeout-ms", iter.next())?.max(1);
            }
            _ => {
                return Err(format!("unknown codex_usage argument: {arg}"));
            }
        }
    }
    let cache_path = cache_path
        .or_else(yazelix_zellij_bar::codex_usage_cache_path_from_env)
        .or_else(|| default_cache_path("codex_usage_cache_v2.json"))
        .ok_or_else(|| {
            "codex usage: yazelix_zellij_bar_widget codex [--cache <path>] [--display quota|token|both]".to_string()
        })?;
    yazelix_zellij_bar::codex_usage_widget_text(yazelix_zellij_bar::CodexUsageWidgetOptions {
        cache_path: &cache_path,
        path_var: env::var_os("PATH").as_deref(),
        now_unix_seconds: unix_time_seconds(),
        max_age_seconds,
        error_backoff_seconds,
        timeout: std::time::Duration::from_millis(timeout_ms),
        display,
    })
}

fn run_opencode_go_usage(args: &[String]) -> Result<String, String> {
    let mut cache_path = None;
    let mut db_paths = Vec::new();
    let mut display = yazelix_zellij_bar::AgentUsageDisplay::Both;
    let mut max_age_seconds = 600;
    let mut error_backoff_seconds = 1_800;
    let mut iter = args.iter();
    while let Some(arg) = iter.next() {
        match arg.as_str() {
            "--cache" => {
                cache_path = Some(std::path::PathBuf::from(
                    iter.next()
                        .ok_or_else(|| "--cache requires a value".to_string())?,
                ));
            }
            "--db" => {
                db_paths.push(std::path::PathBuf::from(
                    iter.next()
                        .ok_or_else(|| "--db requires a value".to_string())?,
                ));
            }
            "--display" => {
                display = parse_agent_usage_display(
                    iter.next()
                        .ok_or_else(|| "--display requires a value".to_string())?,
                )?;
            }
            "--max-age-seconds" => {
                max_age_seconds = parse_u64_arg("--max-age-seconds", iter.next())?;
            }
            "--error-backoff-seconds" => {
                error_backoff_seconds = parse_u64_arg("--error-backoff-seconds", iter.next())?;
            }
            _ => {
                return Err(format!("unknown opencode_go_usage argument: {arg}"));
            }
        }
    }
    let cache_path = cache_path
        .or_else(yazelix_zellij_bar::opencode_go_usage_cache_path_from_env)
        .or_else(|| default_cache_path("opencode_go_usage_cache_v1.json"))
        .ok_or_else(|| {
            "opencode_go usage: yazelix_zellij_bar_widget opencode_go [--cache <path>] [--db <path>] [--display quota|token|both]".to_string()
        })?;
    if db_paths.is_empty() {
        db_paths = yazelix_zellij_bar::opencode_db_candidates_from_env();
    }
    yazelix_zellij_bar::opencode_go_usage_widget_text(
        yazelix_zellij_bar::OpenCodeGoUsageWidgetOptions {
            cache_path: &cache_path,
            db_paths: &db_paths,
            now_unix_seconds: unix_time_seconds(),
            max_age_seconds,
            error_backoff_seconds,
            display,
        },
    )
}

fn parse_agent_usage_display(raw: &str) -> Result<yazelix_zellij_bar::AgentUsageDisplay, String> {
    match raw {
        "both" => Ok(yazelix_zellij_bar::AgentUsageDisplay::Both),
        "token" | "tokens" => Ok(yazelix_zellij_bar::AgentUsageDisplay::Token),
        "quota" => Ok(yazelix_zellij_bar::AgentUsageDisplay::Quota),
        _ => Err(format!("invalid display mode: {raw}")),
    }
}

fn parse_u64_arg(name: &str, value: Option<&String>) -> Result<u64, String> {
    value
        .ok_or_else(|| format!("{name} requires a value"))?
        .parse::<u64>()
        .map_err(|_| format!("{name} must be an integer"))
}

fn unix_time_seconds() -> u64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|duration| duration.as_secs())
        .unwrap_or(0)
}

fn default_cache_path(file_name: &str) -> Option<std::path::PathBuf> {
    xdg_base_path("XDG_CACHE_HOME", ".cache")
        .map(|base| base.join("yazelix_zellij_bar").join(file_name))
}

fn cursor_widget_text_from_yzc() -> String {
    let Ok(output) = Command::new("yzc")
        .args(["current", "--format", "env"])
        .output()
    else {
        return String::new();
    };
    if !output.status.success() {
        return String::new();
    }
    yazelix_zellij_bar::cursor_widget_text_from_fact_text(&String::from_utf8_lossy(&output.stdout))
}

fn xdg_base_path(env_name: &str, home_fallback: &str) -> Option<std::path::PathBuf> {
    env::var_os(env_name)
        .filter(|value| !value.is_empty())
        .map(std::path::PathBuf::from)
        .or_else(|| {
            env::var_os("HOME")
                .filter(|value| !value.is_empty())
                .map(|home| std::path::PathBuf::from(home).join(home_fallback))
        })
}

// Test lane: default
#[cfg(test)]
mod tests {
    use super::*;

    // Defends: integrated Yazelix status-bar rendering is owned by this child command surface, not copied into the main repo.
    #[test]
    fn render_yazelix_runtime_outputs_layout_fragments() {
        let output = run(vec![
            "render-yazelix-runtime".into(),
            "--json".into(),
            serde_json::json!({
                "widget_tray": ["editor", "workspace", "cpu"],
                "zjstatus_plugin_url": "file:/runtime/configs/zellij/plugins/zjstatus.wasm",
                "editor_label": "hx",
                "shell_label": "nu",
                "terminal_label": "ghostty",
                "custom_text": "demo",
                "tab_label_mode": "compact",
                "nu_bin": "/runtime/libexec/nu",
                "yzx_control_bin": "/runtime/libexec/yzx_control",
                "yazelix_zellij_bar_widget_bin": "/runtime/libexec/yazelix_zellij_bar_widget",
                "runtime_dir": "/runtime",
                "claude_usage_display": "both",
                "codex_usage_display": "quota",
                "opencode_go_usage_display": "both"
            })
            .to_string(),
        ])
        .unwrap();
        let rendered: serde_json::Value = serde_json::from_str(&output).unwrap();

        assert_eq!(rendered["schema_version"], 2);
        let plugin_block = rendered["plugin_block"].as_str().unwrap();
        assert!(
            plugin_block.contains(
                r#"plugin location="file:/runtime/configs/zellij/plugins/zjstatus.wasm" {"#
            )
        );
        assert!(plugin_block.contains(
            "format_right  \"#[fg=#ff0088,bold]{session} #[fg=#00ff88,bold][editor: hx]{pipe_workspace}{command_cpu} #[fg=#ffff00,bold][demo] #[fg=#00ccff,bold]YAZELIX {command_version} \" // {datetime}"
        ));
        assert!(plugin_block.contains(r##"tab_normal   "#[fg=#ffff00] [{index}] ""##));
        assert!(plugin_block.contains(r##"pipe_workspace_format "#[fg=#00ff88,bold]{output}""##));
        assert!(!plugin_block.contains("command_workspace_command"));
        assert!(plugin_block.contains("/runtime/libexec/yazelix_zellij_bar_widget cpu"));
    }
}
