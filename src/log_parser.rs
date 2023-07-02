use regex::Regex;
use std::fs::File;
use std::io::{self, BufRead, BufReader};

#[derive(Debug, Clone)]
pub struct Node {
    pub task: String,
    pub output: String,
    pub parent: Option<usize>,
    pub children: Vec<usize>,
    pub flags: Vec<String>,
}

pub fn parse_log_file(file_path: &str) -> io::Result<Vec<Node>> {
    let file = File::open(file_path)?;
    let reader = BufReader::new(file);

    let mut nodes: Vec<Node> = Vec::new();

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
                flags: vec!["added".to_string()],
            };
            let new_node_index = nodes.len();

            if let Some(parent_index) = current_node_index {
                nodes[parent_index].children.push(new_node_index);
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
                flags: vec!["starting".to_string()],
            };
            let new_node_index = nodes.len();

            if let Some(parent_index) = current_node_index {
                nodes[parent_index].children.push(new_node_index);
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
                flags: vec!["executing".to_string()],
            };
            let new_node_index = nodes.len();

            if let Some(parent_index) = current_node_index {
                nodes[parent_index].children.push(new_node_index);
            }

            current_node_index = Some(new_node_index);
            nodes.push(new_node);
        } else if let Some(current_index) = current_node_index {
            nodes[current_index].output.push_str(&line);
            nodes[current_index].output.push('\n');
        }
    }

    Ok(nodes.clone())
}

pub fn filter_nodes<F>(nodes: &[Node], filters: &[F]) -> Vec<Node>
where
    F: Fn(&Node) -> bool,
{
    nodes
        .iter()
        .filter(|node| filters.iter().all(|filter| filter(node)))
        .cloned()
        .collect()
}

pub fn has_matching_flags(node: &Node, flags: &[String]) -> bool {
    flags.iter().any(|flag| node.flags.contains(flag))
}

pub fn has_output(node: &Node) -> bool {
    !node.output.is_empty()
}
