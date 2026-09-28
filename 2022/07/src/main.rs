use regex::Regex;
use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader, Lines};
use std::rc::Rc;

const COMMAND_REGEX: &str = r"\$ (dir|ls|cd) ?([\.\w\d]+)?";
const DIR_REGEX: &str = r"(dir|\d+) (.*)";

enum NodeType {
  Dir,
  File,
}

struct Node {
  node_type: NodeType,
  name: String,
  size: u32,
  children: Option<Vec<Rc<Node>>>,
}

fn main() {
  let _data = load_file(env::args().nth(1));
}

fn load_file(file_name: Option<String>) -> Rc<Node> {
  match file_name {
    None => panic!("File name expected"),
    Some(file) => process_file(file),
  }
}

fn process_file(file_name: String) -> Rc<Node> {
  let cmd_rx = Regex::new(COMMAND_REGEX).unwrap();
  let dir_rx = Regex::new(DIR_REGEX).unwrap();
  let file = File::open(file_name).unwrap();
  let mut lines = BufReader::new(file).lines();
  let root = Rc::new(Node {
    node_type: NodeType::Dir,
    name: "/".to_string(),
    size: 0,
    children: Some(Vec::new()),
  });
  lines.next();
  match process_line(&mut lines, &cmd_rx, &dir_rx, Some(root)) {
    None => panic!("Error no data"),
    Some(_) => {
      return root;
    }
  }
}

fn process_line(
  lines: &mut Lines<BufReader<File>>,
  cmd_rx: &regex::Regex,
  dir_rx: &regex::Regex,
  node: Option<Rc<Node>>,
) -> Option<Rc<Node>> {
  match lines.next() {
    None => return None,
    Some(line) => {
      let the_line = line.unwrap();
      match cmd_rx.captures(&the_line) {
        None => match dir_rx.captures(&the_line) {
          None => panic!("Error unexpectd line"),
          Some(dir_caps) => {
            let new_node = process_dir_caps(dir_caps);
            node.unwrap().children.unwrap().push(new_node);
            process_line(lines, cmd_rx, dir_rx, node);
          }
        },
        Some(_cmd_caps) => {}
      }
    }
  };
  return node;
}

fn process_dir_caps(caps: regex::Captures) -> Rc<Node> {
  match caps.get(1).unwrap().as_str().parse::<u32>() {
    Ok(file_size) => {
      return Rc::new(Node {
        node_type: NodeType::File,
        name: caps.get(2).unwrap().as_str().to_string(),
        size: file_size,
        children: None,
      })
    }
    _ => {
      return Rc::new(Node {
        node_type: NodeType::Dir,
        name: caps.get(2).unwrap().as_str().to_string(),
        size: 0,
        children: Some(Vec::new()),
      })
    }
  };
}
