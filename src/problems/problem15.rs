use std::borrow::Borrow;
use std::cell::RefCell;
use std::rc::Rc;
use std::cmp::Ordering;
use super::common::read_file;
use std::collections::{BinaryHeap, HashMap, HashSet};

pub fn part1() -> u32 {
    let path = "test_files/15.txt".to_string();
    let lines = read_file(path);
    let (graph, max_x, max_y) = create_graph(&lines);

    a_star_search(&graph, (0, 0), (max_x, max_y))
}

pub fn part2() -> u32 {
    let path = "test_files/15.txt".to_string();
    let lines = read_file(path);
    let (mut graph, mut max_x, mut max_y) = create_graph(&lines);
    //println!("{}", graph.len());
    expand_graph(&mut graph, &mut max_x, &mut max_y);
    //println!("{}", graph.len());
    a_star_search(&graph, (0, 0), (max_x, max_y))
}

fn a_star_search(graph: &HashMap<(isize, isize), Rc<RefCell<Node>>>, start: (isize, isize), end: (isize, isize)) -> u32 {

    let base_neighbors = [(0,1), (1,0), (0,-1), (-1,0)];

    let mut open =  BinaryHeap::new();
    let mut closed = HashSet::new();
    let mut already_opened = HashSet::new();

    let start = Rc::clone(graph.get(&start).unwrap());

    let goal =  Rc::clone(graph.get(&end).unwrap());

    start.borrow_mut().acc_cost = 0;

    open.push(start);

    while !open.is_empty() {
        let current = open.pop().unwrap();
        if Rc::ptr_eq(&current, &goal) {
            break;
        }
        let current: &RefCell<Node> = current.borrow();
        closed.insert(current.borrow().pos);

        let neighbors = base_neighbors
            .iter()
            .map(|(x,y)| (current.borrow().pos.0 + x, current.borrow().pos.1 + y))
            .filter(|pos| graph.contains_key(pos) && !closed.contains(pos))
            .collect::<Vec<_>>();

        for pos in neighbors {
            let neighbor = Rc::clone(graph.get(&pos).unwrap());
            let neighbor_un: &RefCell<Node> = neighbor.borrow();
            let tent = current.borrow().acc_cost + neighbor_un.borrow().node_value;
            if tent < neighbor_un.borrow().acc_cost {
                neighbor.borrow_mut().best_predecessor = Option::from(current.borrow().pos);
                neighbor.borrow_mut().acc_cost = tent;
            }
            if !already_opened.contains(&neighbor_un.borrow().pos){
                already_opened.insert(neighbor_un.borrow().pos);
                open.push(neighbor);

            }

        }

    }
    let borrowed: &RefCell<Node> =goal.borrow();
    return borrowed.borrow().acc_cost;
}

fn create_graph(lines:&[String]) ->(HashMap<(isize, isize), Rc<RefCell<Node>> >, isize, isize) {
    let mut graph = HashMap::new();

    let x_len = (lines.iter().next().unwrap().chars().count() - 1) as isize;
    let y_len = (lines.iter().len() - 1) as isize;

    for (i, line) in lines.into_iter().enumerate() {
        let items = line.chars().collect::<Vec<_>>();
        for (j, item) in items.into_iter().enumerate() {
            let node = Node{
                node_value: item.to_digit(10).unwrap() as u32,
                acc_cost: u32::MAX,
                // Using Mannhatan distance as heuristic
                heuristic: ((y_len - i as isize)+ (x_len - j as isize)) as u32,
                best_predecessor: None,
                pos: (j as isize, i as isize),
            };
            graph.insert((j as isize, i as isize),Rc::new(RefCell::new(node)));
        }

    }
    (graph, x_len, y_len)
}

fn expand_graph(graph: &mut HashMap<(isize, isize), Rc<RefCell<Node>>>, x_size: &mut isize, y_size: &mut isize) {
    let mut new_hashmap = HashMap::new();
    let max_x = *x_size;
    let max_y = *y_size;
    *x_size = max_x + (max_x + 1) * 4;
    *y_size = max_y + (max_y + 1) * 4;
    for &key in graph.keys() {
        let base= (graph.get(&key).unwrap().borrow() as &RefCell<Node>).borrow().node_value;
        for j in 0..5 {
            for i in 0..5 {
                if i == 0 && j == 0 {
                    continue
                }
                let (x, y) = key;
                let new_key =  (x + (max_x + 1) * i, y + (max_y + 1) * j);
                let node_value = base + i as u32 + j as u32;
                let node_value = if node_value > 9 {
                    (node_value + 1) % 10
                } else {
                    node_value
                };
                let value = Node{
                    node_value,
                    acc_cost: u32::MAX,
                    heuristic: ((*y_size - new_key.1 as isize)+ (*x_size - new_key.0 as isize)) as u32,
                    best_predecessor: None,
                    pos: new_key,
                };
                new_hashmap.insert(new_key, Rc::new(RefCell::new(value)));
            }
        }
    }
    graph.extend(new_hashmap.into_iter());
}

#[derive(Debug)]
struct Node {
    node_value: u32,
    acc_cost: u32,
    heuristic: u32,
    best_predecessor: Option<(isize, isize)>,
    pos: (isize, isize),
}

impl Eq for Node {}

impl PartialEq<Self> for Node {
    fn eq(&self, other: &Self) -> bool {
        self.acc_cost == other.acc_cost
    }
}

impl PartialOrd<Self> for Node {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.acc_cost + self.heuristic < other.acc_cost + other.heuristic { Ordering::Greater }
        else if self.acc_cost + self.heuristic> other.acc_cost + other.heuristic { Ordering::Less }
        else { Ordering::Equal }
    }

}