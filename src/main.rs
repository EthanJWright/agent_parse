use std::fs::File;
use std::io::{self, BufRead, BufReader};
use regex::Regex;

#[derive(Debug, Clone)]
struct Node {
    task: String,
    output: String,
    parent: Option<usize>,
    children: Vec<usize>,
    flags: Vec<String>, // New field to store flags
}

fn parse_log_file(file_path: &str) -> io::Result<Node> {
    let file = File::open(file_path)?;
    let reader = BufReader::new(file);

    let mut nodes: Vec<Node> = Vec::new();
    let mut root_index: Option<usize> = None;

    let task_added_regex = Regex::new(r"Task Added:(.*)").unwrap();
    let starting_task_regex = Regex::new(r"✨ Starting task: (.*)").unwrap();
    let finished_task_regex = Regex::new(r"Finished:(.*)").unwrap();
    let executing_task_regex = Regex::new(r"Executing: (.*)").unwrap();

    let mut current_node_index: Option<usize> = None;

    for line in reader.lines() {
        let line = line?;

        if let Some(captures) = task_added_regex.captures(&line) {
            let task = captures.get(1).unwrap().as_str().to_string();
            let new_node = Node {
                task: task.clone(),
                output: "".to_string(),
                parent: current_node_index,
                children: Vec::new(),
                flags: vec!["added".to_string()], // Set "added" flag
            };
            let new_node_index = nodes.len();

            if let Some(parent_index) = current_node_index {
                nodes[parent_index].children.push(new_node_index);
            } else {
                root_index = Some(new_node_index); // Assign root_index here
            }

            current_node_index = Some(new_node_index);
            nodes.push(new_node);
        } else if let Some(captures) = starting_task_regex.captures(&line) {
            let task = captures.get(1).unwrap().as_str().to_string();
            let new_node = Node {
                task: task.clone(),
                output: "".to_string(),
                parent: current_node_index,
                children: Vec::new(),
                flags: vec!["starting".to_string()], // Set "starting" flag
            };
            let new_node_index = nodes.len();

            if let Some(parent_index) = current_node_index {
                nodes[parent_index].children.push(new_node_index);
            } else {
                root_index = Some(new_node_index); // Assign root_index here
            }

            current_node_index = Some(new_node_index);
            nodes.push(new_node);
        } else if finished_task_regex.is_match(&line) {
            if let Some(current_index) = current_node_index {
                if let Some(parent_index) = nodes[current_index].parent {
                    current_node_index = Some(parent_index);
                } else {
                    current_node_index = None;
                }
            }
        } else if let Some(captures) = executing_task_regex.captures(&line) {
            if let Some(current_index) = current_node_index {
                nodes[current_index].output = line.clone();
            }

            let task = captures.get(1).unwrap().as_str().to_string();
            let new_node = Node {
                task: task.clone(),
                output: "".to_string(),
                parent: current_node_index,
                children: Vec::new(),
                flags: vec!["executing".to_string()], // Set "executing" flag
            };
            let new_node_index = nodes.len();

            if let Some(parent_index) = current_node_index {
                nodes[parent_index].children.push(new_node_index);
            } else {
                root_index = Some(new_node_index); // Assign root_index here
            }

            current_node_index = Some(new_node_index);
            nodes.push(new_node);
        } else if let Some(current_index) = current_node_index {
            nodes[current_index].output.push_str(&line);
            nodes[current_index].output.push('\n');
        }
    }

    println!("Nodes: {:#?}", nodes);
    if let Some(root_index) = root_index {
        Ok(nodes[root_index].clone())
    } else {
        Err(io::Error::new(io::ErrorKind::Other, "No root node found"))
    }
}

fn main() {
    if let Ok(root) = parse_log_file("input/haiku.txt") {
        println!("{:#?}", root);
    } else {
        eprintln!("Error parsing log file");
    }
}

