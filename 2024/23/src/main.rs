use std::collections::HashMap;
use std::env;
use std::fs;

use petgraph::graph;
use petgraph::visit::EdgeRef;
use petgraph::visit::IntoNodeReferences;

fn parse_input(contents: &String) -> graph::UnGraph<&str, ()> {
    let mut g = graph::UnGraph::new_undirected();
    let mut node_names = HashMap::new();

    for line in contents.split('\n') {
        if line.is_empty() {
            continue;
        }

        let name_a = &line[..2];
        let name_b = &line[3..];

        let node_a = *node_names
            .entry(name_a)
            .or_insert_with(|| g.add_node(name_a));
        let node_b = *node_names
            .entry(name_b)
            .or_insert_with(|| g.add_node(name_b));

        g.add_edge(node_a, node_b, ());
    }

    return g;
}

fn main() {
    let args: Vec<_> = env::args().collect();
    let contents = fs::read_to_string(&args[1]).unwrap();
    let g = parse_input(&contents);

    let mut r = 0;
    for e in g.edge_references() {
        for (n, n_name) in g.node_references() {
            let s = e.source();
            let t = e.target();
            if n == s || n == t {
                continue;
            }

            if !g.contains_edge(n, s) || !g.contains_edge(n, t) {
                continue;
            }

            let s_name = g.node_weight(s).unwrap();
            let t_name = g.node_weight(t).unwrap();

            let n_1 = n_name.chars().next().unwrap();
            let s_1 = s_name.chars().next().unwrap();
            let t_1 = t_name.chars().next().unwrap();

            if n_1 != 't' && s_1 != 't' && t_1 != 't' {
                continue;
            }

            r += 1;
        }
    }

    assert_eq!(r % 3, 0);
    r /= 3;

    println!("{r}");
}
