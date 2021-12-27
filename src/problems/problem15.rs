use std::borrow::Borrow;
use std::cell::RefCell;
use std::rc::Rc;
use std::cmp::Ordering;
use super::common::read_file;
use std::collections::{BinaryHeap, HashMap, HashSet};

pub fn part1() -> f32 {
    let path = "test_files/15.txt".to_string();
    let lines = read_file(path);
    let (graph, max_x, max_y) = create_graph(&lines);

    a_star_search(&graph, (0, 0), (max_x, max_y))
}

pub fn part2() -> usize {
    let path = "test_files/15.txt".to_string();
    let lines = read_file(path);
    0
}

fn a_star_search(graph: &HashMap<(isize, isize), Rc<RefCell<Node>>>, start: (isize, isize), end: (isize, isize)) -> f32 {

    let base_neighbors = [(0,1), (1,0), (0,-1), (-1,0)];

    let mut open =  BinaryHeap::new();
    let mut closed = HashSet::new();
    let mut already_opened = HashSet::new();

    let start = Rc::clone(graph.get(&start).unwrap());

    let goal =  Rc::clone(graph.get(&end).unwrap());

    start.borrow_mut().acc_cost = 0f32;

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
            //.map(|pos| graph.get_mut(&pos).unwrap())
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
                node_value: item.to_digit(10).unwrap() as f32,
                acc_cost: f32::MAX,
                // Using Euclidean distance as heuristic
                heuristic: ((i as f32 - y_len as f32).powf(2f32 ) + (j as f32- x_len  as f32).powf(2f32)).sqrt(),
                best_predecessor: None,
                pos: (j as isize, i as isize),
            };
            graph.insert((j as isize, i as isize),Rc::new(RefCell::new(node)));
        }

    }
    (graph, x_len, y_len)
}

struct Node {
    node_value: f32,
    acc_cost: f32,
    heuristic: f32,
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
        if self.acc_cost < other.acc_cost { Ordering::Greater }
        else if self.acc_cost > other.acc_cost { Ordering::Less }
        else { Ordering::Equal }
    }

}