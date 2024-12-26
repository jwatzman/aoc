mod maximal_cliques;

use std::collections::HashMap;
use std::env;
use std::fs;

use petgraph::graph;

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

    let mut cliques = maximal_cliques::maximal_cliques(&g);
    let mut best = cliques.pop().unwrap();
    for clique in cliques.into_iter() {
        if clique.len() > best.len() {
            best = clique;
        }
    }

    let mut names: Vec<_> = best
        .into_iter()
        .map(|n| *g.node_weight(n).unwrap())
        .collect();
    names.sort();
    println!("{}", names.join(","));
}
