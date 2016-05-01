use std::collections::{HashMap, HashSet};
use std::collections::hash_map::Entry::{Vacant, Occupied};
use std::collections::binary_heap::BinaryHeap;
use std::hash::Hash;
use std::cmp::{Eq, Ord, Ordering};
use std::clone::Clone;

pub trait Graph {
    type Node: Clone + Hash + Eq;
    fn neighbors(&self, n: &Self::Node) -> Vec<Self::Node>;
    fn distance(&self, n1: &Self::Node, n2: &Self::Node) -> usize;
}

#[derive(Clone)]
/// Wrapper for holding objects in a priority queue, ordered by S
struct QueueEntry<S: PartialOrd, T>(S, T);

impl<S: PartialOrd, T> Eq for QueueEntry<S, T> {}
impl<S: PartialOrd, T> PartialEq for QueueEntry<S, T> {
    fn eq(&self, other: &QueueEntry<S, T>) -> bool {
        self.0.eq(&other.0)
    }
}
impl<S: PartialOrd, T> PartialOrd for QueueEntry<S, T> {
    fn partial_cmp(&self, other: &QueueEntry<S, T>) -> Option<Ordering> {
        other.0.partial_cmp(&self.0)
    }
}
impl<S: PartialOrd, T> Ord for QueueEntry<S, T> {
    fn cmp(&self, other: &QueueEntry<S, T>) -> Ordering {
        match other.0.partial_cmp(&self.0) {
            Some(r) => r,
            _ => panic!(),
        }
    }
}

pub fn dfs_search<G: Graph>(graph: &G, start: G::Node, goal: G::Node) -> Option<Vec<G::Node>> {
    let mut visited = HashSet::new();
    fn dfs<G: Graph>(visited: &mut HashSet<G::Node>,
                     graph: &G,
                     current: G::Node,
                     goal: &G::Node)
                     -> Option<Vec<G::Node>> {
        if current == *goal {
            return Some(vec![current]);
        }
        visited.insert(current.clone());
        for neighbor in graph.neighbors(&current) {
            if !visited.contains(&neighbor) {
                if let Some(mut path) = dfs(visited, graph, neighbor, goal) {
                    path.push(current);
                    return Some(path);
                }
            }
        }
        visited.remove(&current);
        None
    };
    if let Some(mut path) = dfs(&mut visited, graph, start, &goal) {
        path.reverse();
        Some(path)
    } else {
        None
    }
}

pub fn a_star_search<G: Graph>(graph: &G, start: G::Node, goal: G::Node) -> Option<Vec<G::Node>> {
    struct State<Node> {
        visited: bool,
        prior: Option<Node>,
        prior_cost: usize,
        cost_guess: usize,
    };
    let mut table = HashMap::new();
    let start_cost_guess = graph.distance(&start, &goal);
    table.insert(start.clone(),
                 State::<G::Node> {
                     visited: false,
                     prior: None,
                     prior_cost: 0,
                     cost_guess: start_cost_guess,
                 });
    let mut frontier = BinaryHeap::new();
    frontier.push(QueueEntry(start_cost_guess, start));
    while let Some(QueueEntry(_, ref current)) = frontier.pop() {
        if current == &goal {
            let mut path = vec![goal];
            let mut node = current;
            while let &Some(ref prev) = &table.get(node).unwrap().prior {
                node = prev;
                path.push(node.clone());
            }
            return Some(path);
        }
        let prior_cost = {
            let entry = table.get_mut(current).unwrap();
            if entry.visited {
                continue;
            }
            entry.visited = true;
            entry.prior_cost
        };
        for neighbor in graph.neighbors(current) {
            let new_prior_cost = prior_cost + graph.distance(current, &neighbor);
            let cost_guess = new_prior_cost + graph.distance(&neighbor, &goal);
            let candidate_entry = State::<G::Node> {
                visited: false,
                prior: Some(current.clone()),
                prior_cost: new_prior_cost,
                cost_guess: cost_guess,
            };
            // if unseen or cost_guess is better, update/insert and requeue
            let should_enqueue = match table.entry(neighbor.clone()) {
                Occupied(occ) => {
                    let v = occ.into_mut();
                    if v.cost_guess > cost_guess {
                        *v = candidate_entry;
                        true
                    } else {
                        false
                    }
                }
                Vacant(vac) => {
                    vac.insert(candidate_entry);
                    true
                }
            };
            if should_enqueue {
                frontier.push(QueueEntry(cost_guess, neighbor));
            }
        }
    }
    None
}
