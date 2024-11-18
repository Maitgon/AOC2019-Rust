use crate::{Solution, SolutionPair};
use std::fs::read_to_string;
use std::collections::HashMap;
use std::collections::HashSet;

///////////////////////////////////////////////////////////////////////////////

pub fn solve() -> SolutionPair {
    let input_string = read_to_string("inputs/day06.txt").unwrap().trim().to_string();

    let input: Vec<(&str, &str)> = input_string
        .split('\n')
        .map(|n| {
            n.split_once(')').unwrap()
        })
        .collect();

    // Your solution here...
    let sol1: u64 = part1(&input);
    let sol2: u64 = part2(&input);

    (Solution::from(sol1), Solution::from(sol2))
}

// Part 1

fn part1(input: &Vec<(&str, &str)>) -> u64 {
    let tree = create_tree(input);
    let root = get_root(&tree);

    count_orbits(root, &tree).0
}

fn create_tree<'a>(input: &Vec<(&'a str, &'a str)>) -> HashMap<&'a str, Vec<&'a str>> {
    let mut tree = HashMap::new();

    for (parent, child) in input {
        tree.entry(*parent).or_insert(Vec::new()).push(*child);
    }

    tree
}

fn count_orbits(planet: &str, tree: &HashMap<&str, Vec<&str>>) -> (u64, u64) {
    let direct = tree.get(planet).map_or(0, |children| {children.len() as u64});
    let mut count = 0;
    let mut indirect = 0;

    if let Some(children) = tree.get(planet) {
        for child in children {
            let aux = count_orbits(child, tree);
            count += aux.0;
            indirect += aux.1;
        }
    }

    (count + direct + indirect, direct + indirect)
}

fn get_root<'a>(tree: &HashMap<&'a str, Vec<&str>>) -> &'a str {
    let all_plantes: Vec<&str> = tree.keys().copied().collect();
    let all_orbits: Vec<&str> = tree.values().flatten().copied().collect();

    all_plantes.iter().find(|&planet| !all_orbits.contains(planet)).unwrap()
}

// part 2

fn part2(input: &Vec<(&str, &str)>) -> u64 {
    let graph = create_graph(input);
    let start = "YOU";
    let end = "SAN";

    dfs(&graph, start, end).unwrap() - 2
}

struct Graph<'a> {
    vertex: HashSet<&'a str>,
    adyacency: HashMap<&'a str, Vec<&'a str>>
}

fn create_graph<'a>(input: &Vec<(&'a str, &'a str)>) -> Graph<'a> {
    let mut graph = Graph {
        vertex: HashSet::new(), 
        adyacency: HashMap::new(),
    };

    for (parent, child) in input {
        graph.vertex.insert(*parent);
        graph.vertex.insert(*child);

        graph.adyacency.entry(*parent).or_default().push(*child);
        graph.adyacency.entry(*child).or_default().push(*parent);
    }

    graph
}

fn dfs<'a>(graph: &Graph<'a>, start: &'a str, end: &'a str) -> Option<u64> {
    let mut visited = HashMap::new();
    let mut stack = vec![(start, 0)];

    while let Some((node, dist)) = stack.pop() {
        if node == end {
            return Some(dist);
        }

        if visited.contains_key(node) {
            continue;
        }

        visited.insert(node, ());
        for neighbor in graph.adyacency.get(node).unwrap() {
            stack.push((neighbor, dist + 1));
        }
    }

    None
}
