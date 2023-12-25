use std::cmp::Ordering;
use std::{collections::HashMap, fs};

use petgraph::algo::dijkstra;
use petgraph::graph::NodeIndex;
use petgraph::{Graph, Undirected};

use petgraph::algo::astar;

fn load_graph(file: &str) -> (Graph<String, i32, Undirected>, HashMap<String, NodeIndex>) {
    let f = fs::read_to_string(file).expect("");

    let mut deps = Graph::<String, i32, Undirected>::new_undirected();
    let mut nodes = HashMap::new();
    for l in f.replace('\r', "").split('\n').collect::<Vec<_>>() {
        let line = l.to_string().clone();
        if line.contains(':') {
            let line = line.trim().split(':').collect::<Vec<_>>();
            let node_name = line[0].to_string();

            let node: NodeIndex;
            if nodes.contains_key(&node_name) {
                node = *nodes.get(&node_name).unwrap();
            } else {
                node = deps.add_node(node_name.clone());
                nodes.insert(node_name.clone(), node);
            }

            for n in line[1].trim().split_whitespace() {
                let neighbour: NodeIndex;
                if nodes.contains_key(&n.to_string()) {
                    neighbour = *nodes.get(&n.to_string()).unwrap();
                } else {
                    neighbour = deps.add_node(n.to_string().clone());
                    nodes.insert(n.to_string(), neighbour);
                }
                deps.add_edge(node, neighbour, 1);
            }
        }
    }
    (deps, nodes)
}

fn find_name(idx: NodeIndex, nodes: &HashMap<String, NodeIndex>) -> String {
    nodes
        .iter()
        .find(|n| n.1.eq(&idx))
        .map(|n| n.0.clone())
        .unwrap()
}

fn p1(file: &str) -> u64 {
    let (mut g, n) = load_graph(file);

        let mut paths = Vec::new();
        let mut i = 0;
        for p in n.iter() {
            //calculate traversal statistics for the first 50 nodes
            //should be enough for finding the top 3 most commonly traversed edges
            if i> 50 {
                break;
            }
            i += 1;
            let first = p;

            let result = dijkstra(&g, *first.1, None, |_e| 1);
    
            assert!(!result.is_empty());
            let max = result.iter().map(|e| *e.1).max().unwrap();
            let avg: u64 = result.iter().map(|e| *e.1 as u64).sum::<u64>() / result.len() as u64;
            println!("node {} : avg max {:?} {}", p.0, avg, max);
    
    
            for (_, idx_other) in n.iter() {
                let length_to_other = *result.get(idx_other).unwrap();
                //optimised to just use max path for each node
                //if it does not work on your input, use length_to_other > avg
                if length_to_other == max {
                    //println!("{} {}", node_other, length_to_other);
                    let result = astar(&g, *first.1, |finish| finish == *idx_other, |_e| 1, |_| 0);
    
                    let path_result = result.unwrap();
                    let node_path = path_result.1;
                    for i in 1 .. node_path.len() {
                        let start = node_path[i-1];
                        let end = node_path[i];
                        paths.push((start,end));
                    }
                }
                
            }
        }
        
        let mut group: HashMap<(String, String), u64> = HashMap::new();
        for p in paths {
            let src_name = find_name(p.0, &n);
            let end_name = find_name(p.1, &n);
            let key = 
            match src_name.cmp(&end_name) {
                Ordering::Less => {(src_name, end_name)},
                _ =>  {(end_name,src_name )}
            };

            if !group.contains_key(&key) {
                group.insert(key.clone(), 0);
            }

            *group.get_mut(&key).unwrap() += 1;
        }
        //println!("{:?}",group );

        let mut v = group.iter().collect::<Vec<_>>();

        v.sort_by(|a, b| b.1.cmp(&a.1));
        println!("Most common edges: {:?}", &v[0 .. 30]);

        
        let top_3_edge = &v[0 .. 3];
        for e in top_3_edge.iter() {
            let n1 = n.get(&e.0.0).unwrap();
            let n2 = n.get(&e.0.1).unwrap();

            //println!("removing {} <-> {} : {:?} <-> {:?}", e.0.0, e.0.1, n1, n2);
            let edge = g.find_edge_undirected(*n1, *n2).unwrap();
            g.remove_edge(edge.0);
        }

        let first = n.iter().next().unwrap();
        
        let result = dijkstra(&g, *first.1, None, |_e| 1);
        //some should be empty
        //println!("{:?}", result);

        let group_1_len = result.len() as u64;
        let group_2_len = n.len() as u64 - group_1_len;
        group_1_len * group_2_len
}

fn main() {
    println!("p1: {}", p1("data/day25.txt"));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dijkstra_all() {
        let file = "data/day25_ex.txt";
        assert_eq!(p1(file), 54);
    }

    #[test]
    fn test_load() {
        let file = "data/day25_ex.txt";

        let (g, n) = load_graph(file);

        assert_eq!(g.node_count(), n.len());
        assert_eq!(g.node_count(), 15);

        let result = g
            .neighbors(*n.get(&String::from("qnr")).unwrap())
            .collect::<Vec<_>>();
        assert_eq!(result.is_empty(), false);
        assert_eq!(result.len(), 4);

        let result = g
            .neighbors(*n.get(&String::from("hfx")).unwrap())
            .collect::<Vec<_>>();
        assert_eq!(result.is_empty(), false);
        assert_eq!(result.len(), 5);
    }
}
