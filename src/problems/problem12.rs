use super::common::read_file;
use std::collections::{BTreeMap, BTreeSet};

pub fn part1() -> usize {
    let path = "test_files/12.txt".to_string();
    let lines = read_file(path);
    let graph = create_graph(&lines);
    count_viable_paths(graph)
}

pub fn part2() -> usize {
    let path = "test_files/12.txt".to_string();
    let lines = read_file(path);
    let graph = create_graph(&lines);
    count_viable_paths_allow_extra(graph)
}

fn create_graph(lines: &[String]) -> BTreeMap<&str, Node> {
    let mut graph = BTreeMap::new();
    for line in lines {
        let parts = line.split("-").collect::<Vec<_>>();
        let init = parts[0];
        let end = parts[1];
        let entry = graph.entry(init).or_insert(Node {
            is_big: init.chars().next().unwrap().is_uppercase(),
            connections: BTreeSet::new(),
        });
        (*entry).connections.insert(end);

        let entry = graph.entry(end).or_insert(Node {
            is_big: end.chars().next().unwrap().is_uppercase(),
            connections: BTreeSet::new(),
        });
        (*entry).connections.insert(init);
    }
    graph
}

fn count_viable_paths(graph: BTreeMap<&str, Node>) -> usize {
    let start = graph.get("start").unwrap();
    let mut complete_paths = Vec::new();

    let mut paths_to_explore = Vec::new();
    for &connection in start.connections.iter() {
        paths_to_explore.push(vec!["start", connection]);
    }

    while !paths_to_explore.is_empty() {
        let current_path = paths_to_explore.pop().unwrap();
        let current_node = graph.get(current_path.last().unwrap()).unwrap();
        for &conn in current_node.connections.iter() {
            let conn_node = graph.get(conn).unwrap();

            if current_path.contains(&conn) && !conn_node.is_big {
                continue;
            }

            let mut new_list = current_path.clone();
            new_list.push(conn);
            if conn == "end" {
                complete_paths.push(new_list);
            } else {
                paths_to_explore.push(new_list);
            }
        }
    }

    complete_paths.len()
}

fn count_viable_paths_allow_extra(graph: BTreeMap<&str, Node>) -> usize {
    let start = graph.get("start").unwrap();
    let mut complete_paths = Vec::new();

    let mut paths_to_explore = Vec::new();
    for &connection in start.connections.iter() {
        paths_to_explore.push(vec![("start", 0), (connection, 1)]);
        paths_to_explore.push(vec![("start", 0), (connection, 0)]);
    }

    while !paths_to_explore.is_empty() {
        let current_path = paths_to_explore.pop().unwrap();
        let current_node = graph.get(current_path.last().unwrap().0).unwrap();
        for &conn in current_node.connections.iter() {
            let conn_node = graph.get(conn).unwrap();

            if current_path.contains(&(conn, 0)) && !conn_node.is_big {
                continue;
            }

            if !conn_node.is_big && !current_path.iter().any(|(_, num)| *num > 0) && conn != "end" {
                let mut new_list = current_path.clone();
                new_list.push((conn, 1));
                paths_to_explore.push(new_list);
            }

            let mut new_list = current_path.clone();
            new_list.push((conn, 0));
            if conn == "end" {
                complete_paths.push(new_list);
            } else {
                paths_to_explore.push(new_list);
            }
        }
    }

    let mut complete_paths = complete_paths
        .iter()
        .map(|x| x.into_iter().map(|(c, _)| *c).collect::<Vec<_>>())
        .collect::<Vec<_>>();
    complete_paths.sort();
    complete_paths.dedup();
    complete_paths.len()
}

struct Node<'a> {
    is_big: bool,
    connections: BTreeSet<&'a str>,
}
