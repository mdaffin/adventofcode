extern crate adventofcode;

use adventofcode::input_reader;
use std::env;
use std::io::BufRead;
use std::collections::HashMap;
use std::rc::Rc;

#[derive(Debug)]
struct Node {
    name: String,
    weight: u32,
    children: Vec<Rc<Node>>,
}

impl Node {
    pub fn new(name: String, weight: u32) -> Node {
        Node {
            name,
            weight,
            children: vec![],
        }
    }
}

fn main() {
    let input_file = env::args().nth(1).unwrap_or("input/2017/day-7.txt".into());

    let mut nodes = input_reader(input_file)
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
            (node.into(), (
                Rc::new(Node::new(node.into(), weight)),
                children,
            ))
        })
        .collect::<HashMap<String, (Rc<Node>, Vec<String>)>>();

    let keys = nodes.keys().map(|e| e.to_string()).collect::<Vec<_>>();

    for key in keys {
        match nodes.get_mut(&key) {
            Some(ref mut w) => {
                let mut node = &mut w.0;
                let children = &w.1;
                println!("{:?} {:?}", node, children);
                for child in children {
                    node.children.push(
                        nodes
                            .get(child)
                            .expect("missing child")
                            .0
                            .clone(),
                    );
                }

            }
            None => continue,
        };

        //nodes.remove(&key);
    }
    println!("{:?}", nodes);
}
