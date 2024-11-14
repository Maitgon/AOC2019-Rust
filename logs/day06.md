# Day 6: Universal Orbit Map

* Difficulty: ⭐⭐⭐

## Description

In this problem, we need to calculate the total number of direct and indirect orbits of some planets. The goal is to calculate the total number of orbits (direct and indirect) of all planets. In part 2, we need to calculate the minumum number of orbital transfers required to move from the object YOU are orbiting to the object SAN is orbiting.

## Problem solution

For part 1, I decided to create a tree with the orbits:

```rust
fn create_tree<'a>(input: &Vec<(&'a str, &'a str)>) -> HashMap<&'a str, Vec<&'a str>> {
    let mut tree = HashMap::new();

    for (parent, child) in input {
        tree.entry(*parent).or_insert(Vec::new()).push(*child);
    }

    tree
}
```

The tree is a HashMap where the key is the parent planet and the value is a vector of all the planets orbiting it.

To calculate the number of orbits, first we need to find the root of the tree, which is the planet that is not orbiting any other planet.

```rust
fn get_root<'a>(tree: &HashMap<&'a str, Vec<&str>>) -> &'a str {
    let all_plantes: Vec<&str> = tree.keys().copied().collect();
    let all_orbits: Vec<&str> = tree.values().flatten().copied().collect();

    all_plantes.iter().find(|&planet| !all_orbits.contains(planet)).unwrap()
}
```

Then, to calculate the number of orbits, we can use a recursive function that calculates the number of orbits of a planet and all its children. The idea of this function is to find 2 values, the number of direct and indirect orbits that a planet has, and the amount of orbits that the planet and the planets orbiting it have. That way, we can calculate the total number of orbits faster without calculating the same orbit multiple times.

```rust
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
```

Finally, we combine these functions together to get the result:

```rust
fn part1(input: &Vec<(&str, &str)>) -> u64 {
    let tree = create_tree(input);
    let root = get_root(&tree);

    count_orbits(root, &tree).0
}
```

For part 2, I used a dfs, firts, I calculate the grpah corresponding to the orbits. For the graph representtion, I decided to use a Vertex and adyacency list representation:

```rust
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
```

Then I implemented the dfs algorithm to find the shortest path between the two nodes:

```rust
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

fn part2(input: &Vec<(&str, &str)>) -> u64 {
    let graph = create_graph(input);
    let start = "YOU";
    let end = "SAN";

    dfs(&graph, start, end).unwrap() - 2
}
```

## Opinion

This problem was the hardest yet, and I it wasn't as straightforward to find a solution. I think my solution ended up beeing quite efficent, running in about 1-3ms, so I'm happy with the result.
