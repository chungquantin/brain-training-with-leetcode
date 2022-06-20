use std::collections::HashMap;

pub struct Solution;

impl Solution {
    fn add_adjacency(node: i32, child: i32, graph: &mut HashMap<i32, Vec<i32>>) {
        if graph.get(&node).is_none() {
            graph.insert(node, vec![child]);
        } else {
            let mut temp = graph.get(&node).unwrap().to_vec();
            temp.push(child);
            graph.insert(node, temp.to_vec());
        }
    }
    fn build_graph(connections: Vec<Vec<i32>>) -> HashMap<i32, Vec<i32>> {
        let graph = &mut HashMap::<i32, Vec<i32>>::new();
        for conn in connections.to_vec() {
            Solution::add_adjacency(conn[0], conn[1], graph);
            Solution::add_adjacency(conn[1], conn[0], graph);
        }
        graph.clone()
    }
    pub fn critical_connections(n: i32, connections: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let graph = Solution::build_graph(connections);
        let critical_edges: &mut Vec<Vec<i32>> = &mut vec![];
        let ids = &mut vec![0; n as usize];
        let low_links = &mut vec![0; n as usize];
        let id = &mut 0;

        let no_parent = -999;
        Solution::tarjan_dfs(
            0,
            no_parent,
            id,
            ids,
            low_links,
            graph.clone(),
            critical_edges,
        );

        critical_edges.clone()
    }

    pub fn tarjan_dfs(
        node: usize,
        parent: i32,
        id: &mut i32,
        ids: &mut Vec<i32>,
        low_links: &mut Vec<i32>,
        graph: HashMap<i32, Vec<i32>>,
        critical_edges: &mut Vec<Vec<i32>>,
    ) {
        *id += 1;
        ids[node] = *id;
        low_links[node] = *id;
        let get_children = graph.get(&(node as i32));
        if !get_children.is_none() {
            for child in get_children.unwrap() {
                let usize_child = *child as usize;
                if child == &parent {
                    continue;
                }
                if ids[usize_child] == 0 {
                    Solution::tarjan_dfs(
                        usize_child,
                        node as i32,
                        id,
                        ids,
                        low_links,
                        graph.clone(),
                        critical_edges,
                    );
                }
                low_links[node] = std::cmp::min(low_links[node], low_links[usize_child]);
                if low_links[usize_child] > ids[node] {
                    critical_edges.push(vec![node as i32, *child]);
                }
            }
        }
    }
}
