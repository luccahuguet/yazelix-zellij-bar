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
        [command] if command == "cpu" => Ok(yazelix_bar::current_cpu_usage_widget_text()),
        [command] if command == "ram" => Ok(yazelix_bar::current_ram_usage_widget_text()),
        [command] => Err(format!("unknown widget command: {command}")),
        [] => Err("expected widget command: cpu or ram".to_string()),
        _ => Err("expected exactly one widget command".to_string()),
    }
}
