use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashMap, HashSet},
};

type Node = usize;
type Cost = usize;

struct Graph {
    edges: HashMap<Node, Vec<(Node, Cost)>>,
    nodes: HashSet<Node>,
}

impl Graph {
    fn from_edge_list(edge_list: &Vec<(Node, Node, Cost)>) -> Self {
        let mut adjacency_list: HashMap<Node, Vec<(Node, Cost)>> = HashMap::new();
        let mut nodes = HashSet::new();

        for &(source, destination, cost) in edge_list.iter() {
            let destinations = adjacency_list.entry(source).or_insert_with(|| Vec::new());

            destinations.push((destination, cost));

            nodes.insert(source);
            nodes.insert(destination);
        }

        Graph {
            edges: adjacency_list,
            nodes,
        }
    }
}

fn shortest_path(g: &Graph, start: Node, goal: Node) -> Option<(Vec<Node>, Cost)> {
    // Create few structures used to keep track of various elements:
    // visited for the nodes that we have already visited
    let mut visited: HashSet<Node> = HashSet::new();
    // to_visit is a priority queue which is used to store the nodes that have already
    // been visited and are sorted based on the cost (weight/distance) from the previous node
    // Rust BinaryHeap sorts them with the greatest first, so it needs to be used with Reverse
    // to get the smallest item.
    let mut to_visit = BinaryHeap::new();
    let mut distances: HashMap<Node, Cost> = g.nodes.iter().map(|x| (*x, usize::MAX)).collect();
    let mut previous: HashMap<Node, Option<Node>> = g.nodes.iter().map(|x| (*x, None)).collect();

    distances.insert(start, 0);

    // Add the start node to the binary heap
    to_visit.push((Reverse(0), start));

    // Go through the BinaryHeap as long as there's an entry
    while let Some((Reverse(current_distance), current_node)) = to_visit.pop() {
        // If the node is already visited or the distance is greater of some 
        // other already found paths, continue with the next
        if !visited.insert(current_node) {
            continue;
        }
        if current_distance > distances[&current_node] {
            continue;
        }

        // Is this the Goal? If yes break out
        if current_node == goal {
            break;
        }

        // Get all the adjacent nodes
        let Some(edges) = g.edges.get(&current_node) else {
            continue;
        };

        // Go through the adjacent nodes
        for (neigh, neigh_cost) in edges.iter() {
            let calculated_distance = current_distance + neigh_cost;
            // If the cost is less than cost already found in the
            // distance table, update it.
            // Add the found node and cost to the BinaryHeap
            // and store also the neighbour node and the current node mapping
            if calculated_distance < distances[neigh] {
                previous.insert(*neigh, Some(current_node));
                distances.insert(*neigh, calculated_distance);
                to_visit.push((Reverse(calculated_distance), *neigh));
            }
        }
    }

    let mut path = vec![];
    let mut current_node = Some(goal);

    // Backtrack from the target node using predecessors
    while let Some(node) = current_node {
        path.push(node);
        current_node = previous[&node];
    }

    // Reverse the path and return it
    path.reverse();

    Some((path, distances[&goal]))
}

fn main() {
    let edge_list = include!("large_graph.in");
    let g = Graph::from_edge_list(&edge_list);

    if let Some((path, cost)) = shortest_path(&g, 1000, 9000) {
        println!("1000->9000, {:?} {}", path, cost);
    };
}

#[test]
fn large_graph() {
    let edge_list = include!("large_graph.in");
    let g = Graph::from_edge_list(&edge_list);

    let path = shortest_path(&g, 1000, 9000);
    assert!(path.is_some());
    assert_eq!(path.unwrap().1, 24);
}
