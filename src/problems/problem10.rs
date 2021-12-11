use super::common::read_file;
use std::fmt::{Display, Formatter};

pub fn part1() -> u32 {
    let path = "test_files/10.txt".to_string();
    let lines = read_file(path);
    let mut result = 0;
    for line in lines {
        let tree = create_tree(&line);
        result += get_invalid_value(&tree);
    }
    result
}

pub fn part2() -> u64 {
    let path = "test_files/10.txt".to_string();
    let lines = read_file(path);
    let mut values = Vec::new();
    for line in lines {
        let tree = create_tree(&line);
        if get_invalid_value(&tree) > 0 {
            continue;
        }

        let completion_cost = get_completion_cost(&tree);
        values.push(completion_cost);
    }
    values.sort();
    values[values.len() / 2]
}

fn create_tree(line: &str) -> Node {
    let chars = line.chars().collect::<Vec<char>>();
    let mut children = Vec::new();
    let mut consumed = 0;

    loop {
        let (child, _, children_consumed) = create_node(&chars[consumed..]);
        consumed += children_consumed;
        match child {
            NodeOrChar::Node(n) => children.push(n),
            NodeOrChar::Char(c) => panic!("Huh? {}", c),
        }
        if consumed >= chars.len() {
            break;
        }
    }

    let parent = Node {
        children,
        value_a: ' ',
        value_b: ' ',
        valid: true,
    };
    parent
}

fn create_node(rest: &[char]) -> (NodeOrChar, char, usize) {
    if rest.len() < 1 {
        return (NodeOrChar::Char(' '), ' ', 0);
    }

    let current = rest[0];
    return match current {
        '[' | '{' | '(' | '<' => {
            let mut consumed = 1;
            let mut new_children = Vec::new();
            let mut end = '0';
            loop {
                let (child, next, consumed_in_child) = create_node(&rest[consumed..]);
                consumed += consumed_in_child;

                match child {
                    NodeOrChar::Node(n) => new_children.push(n),
                    NodeOrChar::Char(c) => {
                        end = c;
                        if next == '{' || next == '[' || next == '(' || next == '<' || next == ' ' {
                            break;
                        }
                    }
                }
                if end == '}' || end == ']' || end == ')' || end == '>' || end == ' ' {
                    break;
                }
            }

            let valid = match (current, end) {
                ('{', '}') | ('[', ']') | ('(', ')') | ('<', '>') => true,
                _ => false,
            };
            let node = Node {
                children: new_children,
                value_a: current,
                value_b: end,
                valid,
            };
            let next = if rest.len() > consumed {
                rest[consumed]
            } else {
                ' '
            };
            (NodeOrChar::Node(node), next, consumed)
        }
        _ => {
            let next = if rest.len() > 1 { rest[1] } else { ' ' };
            (NodeOrChar::Char(rest[0]), next, 1)
        }
    };
}

fn get_invalid_value(n: &Node) -> u32 {
    if n.valid && n.children.len() == 0 {
        0
    } else {
        return match n.value_b {
            ')' if !n.valid => 3,
            ']' if !n.valid => 57,
            '}' if !n.valid => 1197,
            '>' if !n.valid => 25137,
            _ => {
                let mut res = 0;
                for child in &n.children {
                    let a = get_invalid_value(child);
                    if a != 0 {
                        res = a;
                        break;
                    }
                }
                res
            }
        };
    }
}

fn get_completion_cost(n: &Node) -> u64 {
    let mut result = 0;
    for child in &n.children {
        result += get_completion_cost(child);
        if n.value_a != ' ' {
            result *= 5;
        }
    }
    if !n.valid && n.value_b == ' ' {
        let sum = match n.value_a {
            '(' => 1,
            '[' => 2,
            '{' => 3,
            '<' => 4,
            _ => panic!("Wtf"),
        };
        result += sum;
    }

    result
}

enum NodeOrChar {
    Node(Node),
    Char(char),
}

struct Node {
    children: Vec<Node>,
    value_a: char,
    value_b: char,
    valid: bool,
}

impl Display for Node {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}{} - {}", self.value_a, self.value_b, self.valid)
    }
}
