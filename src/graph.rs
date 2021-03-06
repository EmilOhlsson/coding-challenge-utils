use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap, HashSet, VecDeque};
use std::fmt::Debug;
use std::hash::Hash;
use std::rc::Rc;
use std::usize;

pub trait Vertex {
    fn neighbors(&self) -> Vec<Rc<Self>>;
    fn distance(&self, other: &Self) -> usize;
}

struct ScoredVertex<T>
where
    T: Vertex + Hash + Eq,
{
    score: usize,
    vertex: Rc<T>,
}

impl<T> Ord for ScoredVertex<T>
where
    T: Vertex + Hash + Eq,
{
    fn cmp(&self, other: &Self) -> Ordering {
        other.score.cmp(&self.score)
    }
}

impl<T> PartialOrd for ScoredVertex<T>
where
    T: Vertex + Hash + Eq,
{
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<T> PartialEq for ScoredVertex<T>
where
    T: Vertex + Hash + Eq,
{
    fn eq(&self, other: &Self) -> bool {
        self.score == other.score
    }
}

impl<T> Eq for ScoredVertex<T> where T: Vertex + Hash + Eq {}

impl<T> ScoredVertex<T>
where
    T: Vertex + Hash + Eq,
{
    fn new(vertex: Rc<T>, score: usize) -> ScoredVertex<T> {
        ScoredVertex {
            vertex: vertex,
            score: score,
        }
    }
}

fn reconstruct_path<T>(mut goal: Rc<T>, came_from: &HashMap<Rc<T>, Rc<T>>) -> Vec<Rc<T>>
where
    T: Vertex + Hash + Eq + Debug,
{
    let mut path = vec![goal.clone()];
    while let Some(prev) = came_from.get(goal.as_ref()) {
        path.push(prev.clone());
        goal = prev.clone();
    }
    return path;
}

/// Search for paths from `start` to `goal` using BFS traversing.
/// Return list of all found paths
pub fn bfs_search_all<T>(start: Rc<T>, goal: Rc<T>) -> Vec<Vec<Rc<T>>>
where
    T: Vertex + Hash + Eq + Debug,
{
    let mut queue: VecDeque<Rc<T>> = VecDeque::new();
    let mut prev: HashMap<Rc<T>, Rc<T>> = HashMap::new();
    let mut result = Vec::new();

    queue.push_back(start.clone());
    while queue.len() > 0 {
        let current = queue.pop_front().unwrap();
        if current.distance(&goal) == 0 {
            /* Found path */
            result.push(reconstruct_path(current.clone(), &prev));
        } else {
            current.neighbors().iter().for_each(|n| {
                prev.insert(n.clone(), current.clone());
                queue.push_back(n.clone());
            });
        }
    }

    result
}

/// Search for the shortest path between two Vertices.
/// Uses `Rc`, as it is otherwise hard to know size of
/// objects at compile time.
pub fn astar_search<T>(start: Rc<T>, goal: Rc<T>) -> Option<Vec<Rc<T>>>
where
    T: Vertex + Hash + Eq + Debug,
{
    let mut open = BinaryHeap::<ScoredVertex<T>>::new();
    let mut closed = HashSet::<Rc<T>>::new();
    let mut came_from = HashMap::<Rc<T>, Rc<T>>::new();

    /* g_score, cost of getting from start to that node */
    let mut g_score = HashMap::<Rc<T>, usize>::new();

    /* f_score, cost of gett from start to finish by passing that node */
    let mut f_score = HashMap::<Rc<T>, usize>::new();

    open.push(ScoredVertex::new(start.clone(), usize::MAX));
    g_score.entry(start.clone()).or_insert(0);
    f_score.entry(start.clone()).or_insert(usize::MAX);

    while !open.is_empty() {
        let current = open.pop().unwrap();
        if current.vertex.distance(&*goal) == 0 {
            // Path found, reconstruct path
            return Some(reconstruct_path(current.vertex, &came_from));
        }

        closed.insert(current.vertex.clone());
        for neighbor in current
            .vertex
            .neighbors()
            .iter()
            .filter(|&n| !closed.contains(n))
        {
            let tentative_gscore = g_score[&current.vertex] + 1;
            let tentative_fscore = tentative_gscore + neighbor.distance(goal.as_ref());

            open.push(ScoredVertex::new(neighbor.clone(), tentative_fscore));
            if tentative_gscore < *g_score.entry(neighbor.clone()).or_insert(usize::MAX) {
                g_score.insert(neighbor.clone(), tentative_gscore);
                f_score.insert(neighbor.clone(), tentative_fscore);
                came_from.insert(neighbor.clone(), current.vertex.clone());
            }
        }
    }
    return None;
}

// TODO Describe purpose of this function
pub fn count_paths<T>(node: Rc<T>) -> usize
where
    T: Vertex + Hash + Eq + Debug,
{
    count_paths_internal(&mut HashMap::new(), node)
}

fn count_paths_internal<T>(nodes: &mut HashMap<Rc<T>, usize>, node: Rc<T>) -> usize
where
    T: Vertex + Hash + Eq + Debug,
{
    if !nodes.contains_key(&node) {
        let neighbors = node.neighbors();
        let mut paths = 0;
        if neighbors.len() > 0 {
            for n in neighbors {
                paths += count_paths_internal(nodes, n.clone());
            }
        } else {
            paths = 1;
        }
        nodes.insert(node.clone(), paths);
        paths
    } else {
        *nodes.get(&node).unwrap()
    }
}
