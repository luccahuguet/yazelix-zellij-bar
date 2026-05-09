use std::env;
use std::process;

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
            "expected widget command: claude_usage, codex_usage, cursor, cpu, opencode_go_usage, or ram".to_string(),
        );
    };
    match command.as_str() {
        "cursor" => run_cursor(rest),
        "claude_usage" => run_claude_usage(rest),
        "codex_usage" => run_codex_usage(rest),
        "opencode_go_usage" => run_opencode_go_usage(rest),
        "cpu" if rest.is_empty() => Ok(yazelix_bar::current_cpu_usage_widget_text()),
        "ram" if rest.is_empty() => Ok(yazelix_bar::current_ram_usage_widget_text()),
        "cpu" | "ram" => Err(format!("{command} accepts no arguments")),
        _ => Err(format!("unknown widget command: {command}")),
    }
}

fn run_cursor(args: &[String]) -> Result<String, String> {
    match args {
        [] => Ok(yazelix_bar::cursor_widget_text_from_env()),
        [flag, path] if flag == "--facts" => yazelix_bar::cursor_widget_text_from_fact_file(path)
            .map_err(|error| format!("failed to read cursor fact file {path}: {error}")),
        _ => Err("cursor usage: yazelix_bar_widget cursor [--facts <path>]".to_string()),
    }
}

fn run_claude_usage(args: &[String]) -> Result<String, String> {
    let mut cache_path = None;
    let mut display = yazelix_bar::AgentUsageDisplay::Both;
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
    let cache_path = cache_path.ok_or_else(|| {
        "claude_usage usage: yazelix_bar_widget claude_usage --cache <path> [--display quota|token|both]".to_string()
    })?;
    yazelix_bar::claude_usage_widget_text(yazelix_bar::ClaudeUsageWidgetOptions {
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
    let mut display = yazelix_bar::AgentUsageDisplay::Quota;
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
    let cache_path = cache_path.ok_or_else(|| {
        "codex_usage usage: yazelix_bar_widget codex_usage --cache <path> [--display quota|token|both]".to_string()
    })?;
    yazelix_bar::codex_usage_widget_text(yazelix_bar::CodexUsageWidgetOptions {
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
    let mut display = yazelix_bar::AgentUsageDisplay::Both;
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
    let cache_path = cache_path.ok_or_else(|| {
        "opencode_go_usage usage: yazelix_bar_widget opencode_go_usage --cache <path> [--db <path>] [--display quota|token|both]".to_string()
    })?;
    if db_paths.is_empty() {
        db_paths = yazelix_bar::opencode_db_candidates_from_env();
    }
    yazelix_bar::opencode_go_usage_widget_text(yazelix_bar::OpenCodeGoUsageWidgetOptions {
        cache_path: &cache_path,
        db_paths: &db_paths,
        now_unix_seconds: unix_time_seconds(),
        max_age_seconds,
        error_backoff_seconds,
        display,
    })
}

fn parse_agent_usage_display(raw: &str) -> Result<yazelix_bar::AgentUsageDisplay, String> {
    match raw {
        "both" => Ok(yazelix_bar::AgentUsageDisplay::Both),
        "token" | "tokens" => Ok(yazelix_bar::AgentUsageDisplay::Token),
        "quota" => Ok(yazelix_bar::AgentUsageDisplay::Quota),
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
