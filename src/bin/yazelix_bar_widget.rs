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
    match args.as_slice() {
        [command] if command == "cursor" => Ok(yazelix_bar::cursor_widget_text_from_env()),
        [command, flag, path] if command == "cursor" && flag == "--facts" => {
            yazelix_bar::cursor_widget_text_from_fact_file(path)
                .map_err(|error| format!("failed to read cursor fact file {path}: {error}"))
        }
        [command] if command == "cpu" => Ok(yazelix_bar::current_cpu_usage_widget_text()),
        [command] if command == "ram" => Ok(yazelix_bar::current_ram_usage_widget_text()),
        [command] => Err(format!("unknown widget command: {command}")),
        [] => Err("expected widget command: cursor, cpu, or ram".to_string()),
        _ => Err("expected exactly one widget command".to_string()),
    }
}
