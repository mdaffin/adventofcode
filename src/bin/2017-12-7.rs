extern crate adventofcode;

use adventofcode::input_reader;
use std::env;
use std::io::BufRead;
use std::collections::HashMap;

#[derive(Debug)]
struct Node {
    name: String,
    weight: u32,
    children: Vec<Node>,
}

impl Node {
    pub fn new(name: &str, nodes: &HashMap<String, (u32, Vec<String>)>) -> Node {
        let &(weight, ref children) = nodes.get(name).expect("missing node");
        let children = children.iter().map(|name| Node::new(name, nodes)).collect();
        Node {
            name: name.to_string(),
            weight,
            children,
        }
    }

    // The weight of this node + all of its children.
    pub fn weight(&self) -> (u32, u32) {
        let mut child_weights = self.children.iter().map(|c| c.weight()).fold(
            HashMap::new(),
            |mut buckets,
             (node, child)| {
                buckets.entry(node + child).or_insert(Vec::new()).push(node);
                buckets
            },
        );
        let child_weight = child_weights
            .iter()
            .map(|(weight, nodes)| weight * nodes.len() as u32)
            .sum::<u32>();

        if child_weights.len() > 1 {
            let (key, _, value) = child_weights
                .iter()
                .map(|(&key, w)| (key, w.len(), w[0]))
                .find(|&(_, c, _)| c == 1)
                .expect("could not find single value");

            child_weights.remove(&key);
            let (other_key, _) = child_weights.iter().next().expect(
                "there should be more than one element",
            );
            println!("Part 2: {:?}", value - (key - other_key));
        }
        (self.weight, child_weight)
    }
}

fn main() {
    let input_file = env::args().nth(1).unwrap_or("input/2017/day-7.txt".into());

    let nodes = input_reader(input_file)
        .expect("failed to open file")
        .lines()
        .map(|line| line.expect("could not read line"))
        .map(|line| {
            let mut parts = line.splitn(2, " ");
            let node = parts.next().expect("missing node name");
            let mut parts = parts.next().expect("missing node weight").splitn(2, " -> ");
            let weight = parts
                .next()
                .expect("missing node weight")
                .trim_matches(|c| c == '(' || c == ')')
                .parse()
                .expect("weight is not a number");
            let children = match parts.next() {
                None => Vec::new(),
                Some(s) => s.split(", ").map(|e| e.into()).collect(),
            };
            (node.into(), (weight, children))
        })
        .collect::<HashMap<String, (u32, Vec<String>)>>();

    let mut root_map = HashMap::new();

    for (node, &(_, ref children)) in nodes.iter() {
        root_map.entry(node.to_string()).or_insert(false);
        for child in children {
            root_map.insert(child.to_string(), true);
        }
    }

    let (parent, _) = root_map.iter().find(|&(_, &is_child)| !is_child).expect(
        "no parent found",
    );
    println!("Part 1: {}", parent);

    let root = Node::new(parent, &nodes);

    root.weight();
}
