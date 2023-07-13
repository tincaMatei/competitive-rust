//! Euler path and cycles finder.

extern crate dopecomp_graph;
use dopecomp_graph::{Graph, Edge};

struct EulerianCycleSolver<'a, V, E>
where V: Default,
      E: Edge {
    
    graph: &'a Graph<V, E>,
    used_edge: Vec<bool>,
    last_edge: Vec<usize>,
    result: Vec<usize>,
}

impl<'a, V, E> EulerianCycleSolver<'a, V, E>
where V: Default,
      E: Edge {
    
    fn eulerian_dfs(&mut self, node: usize) {
        while self.last_edge[node] < self.graph.adj_list[node].len() {
            let cnt_edge = self.last_edge[node];
            let id = self.graph.adj_list[node][cnt_edge];
            
            if !self.used_edge[id] {
                let other = self.graph.edges[id].to(node);
                self.used_edge[id] = true;

                self.eulerian_dfs(other);
            }

            self.last_edge[node] += 1; 
        }

        self.result.push(node);
    }
}

/// Find an eulerian path in the given graph. `cycle` should be true if you want to find a cycle,
/// otherwise it will find just a path. Returns [None] if there is no eulerian cycle/path. You can
/// also override whether the graph is directed or not. Use this function if you want to override
/// the `undirected` field.
pub fn solve_eulerian<V, E>(
    graph: &Graph<V, E>,
    undirected: bool,
    cycle: bool
) -> Option<Vec<usize>>
where V: Default,
      E: Edge{
    
    let mut degree = vec![0; graph.v()];

    for node in 0..graph.v() {
        for it in &graph.adj_list[node] {
            let edge = &graph.edges[*it];
            degree[edge.to(node)] += 1;
        }
    }

    let mut start_node = 0;
    let mut cnt_even = 0;
    let mut cnt_pos = 0;
    let mut cnt_neg = 0;

    for node in 0..graph.v() {
        if undirected && degree[node] % 2 == 1 {
            start_node = node;
            cnt_even += 1;
        } else if !undirected && degree[node] < graph.adj_list[node].len() {
            start_node = node;
            cnt_pos += 1;
        } else if !undirected && degree[node] > graph.adj_list[node].len() {
            cnt_neg += 1;
        }
    }

    if undirected {
        if cnt_even > 2 {
            return None;
        }
        if cycle && cnt_even > 0 {
            return None;
        }
    } else {
        if cnt_pos > 1 || cnt_neg > 1 {
            return None;
        }
        if cycle && (cnt_pos > 0 || cnt_neg > 0) {
            return None;
        }
    }
    
    let mut solver = EulerianCycleSolver {
        graph,
        used_edge: vec![false; graph.e()],
        last_edge: vec![0; graph.v()],
        result: Vec::with_capacity(graph.e() + 1),
    };

    solver.eulerian_dfs(start_node);

    if solver.result.len() != graph.e() + 1 {
        None
    } else {
        if !undirected {
            solver.result.reverse();
        }
        Some(solver.result)
    }
}

/// Find an eulerian cycle in the graph.
pub fn find_eulerian_cycle<V, E>(graph: &Graph<V, E>) -> Option<Vec<usize>>
where V: Default,
      E: Edge {
    solve_eulerian(graph, graph.undirected, true)
}

/// Find an eulerian path in the graph.
pub fn find_eulerian_path<V, E>(graph: &Graph<V, E>) -> Option<Vec<usize>>
where V: Default,
      E: Edge {
    solve_eulerian(graph, graph.undirected, false)
}


#[cfg(test)]
mod tests {
    use super::*;

    fn unordered_edge(a: usize, b: usize) -> (usize, usize) {
        if a < b {
            (a, b)
        } else {
            (b, a)
        }
    }

    #[test]
    fn test_eulerian_undirected_cycle() {
        let mut edges: Vec<(usize, usize)> = vec![
            (0, 1),
            (0, 2),
            (1, 1),
            (1, 2),
            (2, 3),
            (2, 3)
        ];

        let graph = Graph::<(), (usize, usize)>::from_edges(
            4,
            edges.clone(),
            |x| { x },
            true
        );

        let cycle = find_eulerian_cycle(&graph).unwrap();

        let mut cycle_edges = Vec::new();
        for i in 0..cycle.len() - 1 {
            cycle_edges.push(unordered_edge(cycle[i], cycle[i + 1]));
        }

        cycle_edges.sort();
        edges.sort();
    
        assert_eq!(edges, cycle_edges);
    }
    
    #[test]
    fn test_eulerian_undirected_path() {
        let mut edges: Vec<(usize, usize)> = vec![
            (0, 2),
            (1, 1),
            (1, 2),
            (2, 3),
            (2, 3),
        ];

        let graph = Graph::<(), (usize, usize)>::from_edges(
            4,
            edges.clone(),
            |x| { x },
            true
        );

        let cycle = find_eulerian_path(&graph).unwrap();

        let mut cycle_edges = Vec::new();
        for i in 0..cycle.len() - 1 {
            cycle_edges.push(unordered_edge(cycle[i], cycle[i + 1]));
        }

        cycle_edges.sort();
        edges.sort();
    
        assert_eq!(edges, cycle_edges);
    }
    
    #[test]
    fn test_eulerian_directed_cycle() {
        let mut edges: Vec<(usize, usize)> = vec![
            (0, 1),
            (1, 2),
            (2, 0),
            (0, 3),
            (3, 4),
            (4, 0),
            (0, 5),
            (5, 6),
            (6, 0)
        ];

        let graph = Graph::<(), usize>::from_edges(
            7,
            edges.clone(),
            |x| { x.1 },
            false
        );

        let cycle = find_eulerian_cycle(&graph).unwrap();

        let mut cycle_edges = Vec::new();
        for i in 0..cycle.len() - 1 {
            cycle_edges.push((cycle[i], cycle[i + 1]));
        }

        cycle_edges.sort();
        edges.sort();

        assert_eq!(edges, cycle_edges);
    }
    
    #[test]
    fn test_eulerian_directed_path() {
        let mut edges: Vec<(usize, usize)> = vec![
            (0, 1),
            (1, 2),
            (2, 3),
            (3, 2),
            (2, 4),
            (4, 3),
            (3, 2),
            (2, 4)
        ];

        let graph = Graph::<(), usize>::from_edges(
            5,
            edges.clone(),
            |x| { x.1 },
            false
        );

        let cycle = find_eulerian_path(&graph).unwrap();

        let mut cycle_edges = Vec::new();
        for i in 0..cycle.len() - 1 {
            cycle_edges.push((cycle[i], cycle[i + 1]));
        }

        cycle_edges.sort();
        edges.sort();

        assert_eq!(edges, cycle_edges);
    }
}


