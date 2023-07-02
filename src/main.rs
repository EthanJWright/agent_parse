use colored::*;
use std::env;
extern crate shellexpand;

mod log_parser;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 4 {
        eprintln!(
            "Usage: cargo run -- --input=<input_file> --include_flags=<flags> [--require_output]"
        );
        return;
    }

    let mut input_file = "";
    let mut include_flags = Vec::new();
    let mut require_output = false;

    for arg in args.iter().skip(1) {
        if arg.starts_with("--input=") {
            input_file = &arg[8..];
        } else if arg.starts_with("--include_flags=") {
            include_flags = arg[16..].split(',').map(String::from).collect();
        } else if arg == "--require_output" {
            require_output = true;
        }
    }

    if input_file.is_empty() {
        eprintln!("Missing input file path. Usage: cargo run -- --input=<input_file> --include_flags=<flags> [--require_output]");
        return;
    }

    if let Ok(nodes) = log_parser::parse_log_file(&shellexpand::tilde(input_file)) {
        let mut filters: Vec<Box<dyn Fn(&log_parser::Node) -> bool>> = Vec::new();
        filters.push(Box::new(move |node| {
            log_parser::has_matching_flags(node, &include_flags)
        }));

        if require_output {
            filters.push(Box::new(log_parser::has_output));
        }

        for node in log_parser::filter_nodes(&nodes, &filters) {
            println!("Task: {}", node.task.yellow());
            println!("Output:\n{}", node.output.green());
            println!();
        }
    } else {
        eprintln!("Error parsing log file");
    }
}
