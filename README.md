# Yazelix Zellij Bar

`yazelix-zellij-bar` is a standalone Zellij bar plugin package for users who want the Yazelix top-bar style without adopting the full Yazelix workspace

## Install Shape

The flake package is:

```bash
nix build github:luccahuguet/yazelix-zellij-bar#yazelix_zellij_bar
nix profile install github:luccahuguet/yazelix-zellij-bar#yazelix_zellij_bar
```

The package installs:

- `bin/yazelix_zellij_bar_generate`
- `bin/yazelix_zellij_bar_widget`
- `share/yazelix_zellij_bar/zjstatus.wasm`
- `share/yazelix_zellij_bar/yazelix_zellij_bar.kdl`
- `share/yazelix_zellij_bar/yazelix_zellij_bar.template.kdl`
- `share/yazelix_zellij_bar/generated/yazelix_zellij_bar.kdl`
- `share/yazelix_zellij_bar/examples/custom_command_widgets.kdl`
- `share/yazelix_zellij_bar/examples/standalone_zellij_layout.kdl`
- `share/yazelix_zellij_bar/examples/yazelix_runtime_widgets.kdl`
- `share/doc/yazelix_zellij_bar/README.md`

Use `yazelix_zellij_bar.kdl` as a Zellij layout plugin block. The template keeps `__YAZELIX_ZELLIJ_BAR_ZJSTATUS_WASM__` for users who want to substitute a different pinned `zjstatus.wasm`. The generated preset is emitted by `yazelix_zellij_bar_generate` with package-local paths. The example snippets are small blocks to copy into the plugin body rather than alternate full presets

## Minimal Zellij Layout Snippet

```kdl
layout {
    pane size=1 borderless=true {
        // Paste the contents of share/yazelix_zellij_bar/yazelix_zellij_bar.kdl here
    }
    pane
}
```

The packaged `yazelix_zellij_bar.kdl` already points at the package's `zjstatus.wasm` with a `file:` URL, and that wasm is installed from the repo's pinned `zjstatus` flake input

## Generic Boundary

The standalone default is intentionally generic:

- mode
- tabs
- session
- datetime
- Yazelix-branded colors and tab overflow behavior

It does not require:

- `yzx`
- `yzx_control`
- Yazelix runtime paths
- the Yazelix pane orchestrator
- Nushell
- full Yazelix installation

The optional provider widgets only need their own upstream facts:

- `codex_usage` and `claude_usage` use `tokenusage` when it is available on `PATH`
- `opencode_go_usage` reads OpenCode Go SQLite databases from default locations or explicit `--db` paths
- `cursor` reads `YAZELIX_CURSOR_*` environment facts or a small `--facts` file
- `cpu` and `ram` read Linux `/proc` directly

None of those widget commands require Yazelix runtime paths, `yzx_control`, or a Yazelix session cache

## Optional Command Widgets

Standalone users can add zjstatus command widgets directly in their own copied preset. Command stdout should be short plain text because the KDL format owns the style

The main customization knobs are:

- `format_left`, `format_center`, and `format_right` for widget order
- inline `#[fg=...]` and `#[bg=...]` style tags for color
- mode and tab format keys for labels and tab display
- `command_*_command`, `command_*_format`, and `command_*_interval` for custom command widgets

Generic zjstatus placeholders such as `{mode}`, `{tabs}`, `{session}`, and `{datetime}` work without Yazelix. To add a host/status command widget, start from:

```kdl
format_right "#[fg=#ff0088,bold]{session} {datetime} {command_host} #[fg=#00ccff,bold]YAZELIX BAR "

command_host_command "hostname -s"
command_host_format " #[fg=#00ff88,bold][{stdout}]"
command_host_interval "30"
```

The packaged `share/yazelix_zellij_bar/examples/custom_command_widgets.kdl` contains a slightly larger version of this pattern

## Complete Standalone Widget Layout

`share/yazelix_zellij_bar/examples/standalone_zellij_layout.kdl` is a complete plain Zellij layout that uses every bar-owned non-workspace widget:

```kdl
layout {
    pane size=1 borderless=true {
        plugin location="file:/absolute/path/to/share/yazelix_zellij_bar/zjstatus.wasm" {
            format_left "{mode} {tabs}"
            format_center ""
            format_right "#[fg=#ff0088,bold]{session}{command_cursor}{command_claude_usage}{command_codex_usage}{command_opencode_go_usage}{command_cpu}{command_ram} #[fg=#00ccff,bold]YAZELIX BAR "

            command_cursor_command "yazelix_zellij_bar_widget cursor"
            command_cursor_format "{stdout}"
            command_cursor_rendermode "dynamic"
            command_cursor_interval "10"

            command_claude_usage_command "yazelix_zellij_bar_widget claude"
            command_claude_usage_format "#[fg=#bb88ff,bold]{stdout}"
            command_claude_usage_interval "10"

            command_codex_usage_command "yazelix_zellij_bar_widget codex"
            command_codex_usage_format "#[fg=#bb88ff,bold]{stdout}"
            command_codex_usage_interval "10"

            command_opencode_go_usage_command "yazelix_zellij_bar_widget opencode_go"
            command_opencode_go_usage_format "#[fg=#bb88ff,bold]{stdout}"
            command_opencode_go_usage_interval "10"

            command_cpu_command "yazelix_zellij_bar_widget cpu"
            command_cpu_format " #[fg=#ff6600][cpu {stdout}]"
            command_cpu_interval "5"

            command_ram_command "yazelix_zellij_bar_widget ram"
            command_ram_format " #[fg=#ff6600][ram {stdout}]"
            command_ram_interval "5"
        }
    }
    pane
}
```

Replace the `zjstatus.wasm` path with the installed package path. If you install through Nix profiles, `nix profile list` shows the profile entry, and the installed files live under that package output

Minimal cursor fact file:

```bash
mkdir -p "$HOME/.config/yazelix_zellij_bar"
cat > "$HOME/.config/yazelix_zellij_bar/cursor.env" <<'EOF'
YAZELIX_CURSOR_NAME=reef
YAZELIX_CURSOR_COLOR=#14d9a0
YAZELIX_CURSOR_FAMILY=mono
EOF
```

Provider widgets maintain their own cache, lock, freshness, and error-backoff files under `$XDG_CACHE_HOME/yazelix_zellij_bar` or `$HOME/.cache/yazelix_zellij_bar`. Use `--cache` only when overriding the default. Yazelix may omit it because the full runtime exports `YAZELIX_STATUS_BAR_CACHE_PATH`

## Preset Generator

Use `yazelix_zellij_bar_generate` when brand text, colors, widget order, or generic command widgets should come from structured options instead of manual KDL edits

```bash
yazelix_zellij_bar_generate \
  --wasm-url "file:/path/to/zjstatus.wasm" \
  --brand-label "DEV BAR" \
  --right "session,datetime,command:host,brand" \
  --command "host=hostname -s"
```

The `--left`, `--center`, and `--right` flags accept comma-separated tokens:

- `mode`
- `tabs`
- `session`
- `datetime`
- `brand`
- `command:name`

Command widgets use `--command name=command`, with optional `--command-format name=format` and `--command-interval name=seconds`. Use this for custom commands that are not built into `yazelix_zellij_bar_widget`

## Standalone Fact Renderers

The Rust crate also exposes renderer helpers for embedders that want Yazelix-style widget text without a Yazelix runtime:

- `render_zjstatus_bar_segments` for editor, shell, terminal, custom text, and widget-tray placeholders
- `render_zjstatus_tab_label_formats` for full and compact tab labels
- `render_cursor_status_widget` for `yazelix-cursors`-compatible cursor facts
- `render_codex_usage_status_widget` for cached Codex usage facts
- `render_windowed_agent_usage_status_widget` for Claude, OpenCode Go, or another cached windowed provider

Minimal cursor fact example:

```rust
use yazelix_zellij_bar::{CursorWidgetFacts, render_cursor_status_widget};

let text = render_cursor_status_widget(&CursorWidgetFacts {
    name: "reef".into(),
    color: Some("#14d9a0".into()),
    family: Some("mono".into()),
    ..CursorWidgetFacts::default()
});
```

Minimal cached provider example:

```rust
use yazelix_zellij_bar::{
    AgentUsageDisplay, AgentUsagePeriod, WindowedAgentUsageFacts,
    render_windowed_agent_usage_status_widget,
};

let text = render_windowed_agent_usage_status_widget(
    "claude",
    &WindowedAgentUsageFacts {
        five_hour_tokens: Some(15_456_373),
        weekly_tokens: Some(66_610_005),
        five_hour_remaining_percent: Some(49),
        weekly_remaining_percent: Some(80),
        ..WindowedAgentUsageFacts::default()
    },
    &[AgentUsagePeriod::FiveHour, AgentUsagePeriod::Weekly],
    AgentUsageDisplay::Both,
);
```

Those helpers are facts-in, styled-text-out. They do not read `~/.config/yazelix`, `~/.local/share/yazelix`, `yzx_control`, Yazelix session cache files, tokenusage, OpenCode databases, or pane-orchestrator state

## Yazelix-Specific Widgets

Workspace remains Yazelix-only because it is derived from Yazelix session facts. Yazelix version display is also Yazelix-only because it reads Yazelix runtime constants

CPU, RAM, cursor, Codex, Claude, and OpenCode Go widgets are bar-owned standalone commands. Yazelix-only integration for those widgets is limited to generated layout wiring, environment hydration, and default cache-path derivation from the full runtime

The full Yazelix runtime consumes this child repo for widget tray rendering, tab label formatting, and the integrated standalone package. The child repo packages `zjstatus.wasm` from its pinned `zjstatus` flake input so the package does not require manual artifact copying

The Rust API also renders Yazelix runtime command-widget definitions from resolved helper paths. Yazelix core still owns workspace facts, session config, and runtime path resolution; this repo owns non-workspace widget behavior and the generic zjstatus command-definition KDL shape

When the main Yazelix repo forwards `#yazelix_zellij_bar`, it may make this repo's `zjstatus` input follow Yazelix's own `zjstatus` pin. Standalone users get the pin recorded in this repo's `flake.lock`

Use `share/yazelix_zellij_bar/examples/yazelix_runtime_widgets.kdl` only inside a full Yazelix runtime or after replacing the Yazelix-only workspace/version helpers. Use `share/yazelix_zellij_bar/examples/standalone_zellij_layout.kdl` for plain Zellij

## Current Limit

Zellij/zjstatus presets do not currently have a native include or variables layer. Use the generator for structured brand, color, order, and command-widget changes; copy `yazelix_zellij_bar.template.kdl` only when editing lower-level zjstatus keys that the generator does not expose

## Release Process

Maintainers update the pinned zjstatus input deliberately, then validate:

```bash
nix flake lock --update-input zjstatus
nix build .#yazelix_zellij_bar
cargo test
```

If the standalone preset grows beyond zjstatus configuration, the next step is a real plugin decision rather than forking zjstatus by default
