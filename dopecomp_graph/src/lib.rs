//! Graph utilities.

/// Trait for implementing an edge.
pub trait Edge {
    /// Returns the node that this edge points to.
    ///
    /// `from` is required for bidirectional edges, otherwise the user wouldn't know in which
    /// direction to walk the edge.
    fn to(&self, from: usize) -> usize;
}

/// Trait for implementing bidirectional edges.
pub trait BidirectionalEdge: Edge {
    /// Returns the pair of nodes that it connects.
    fn as_pair(&self) -> (usize, usize);
}

/// Data structure to represent graphs.
#[derive(Debug)]
pub struct Graph<V, E>
where V: Default,
      E: Edge {
    /// Data to be stored inside every node. Usually it's going to be [()]
    pub nodes: Vec<V>,

    /// Every edge of the graph.
    pub edges: Vec<E>,

    /// Adjacency list. Every entry in this vector refers to the index in the [edges] vector.
    pub adj_list: Vec<Vec<usize>>,

    /// `true` if the graph is undirected, `false` otherwise.
    pub undirected: bool,
}

impl<V, E> Graph<V, E> 
where V: Default,
      E: Edge {
    
    /// Create a graph from the edge vector.
    ///
    /// `v` is the number of nodes in the graph.
    ///
    /// `edges` is the vector of edges to create the graph from. This should be a
    /// [BidirectionalEdge] so that the function know to whom to add the edge.
    ///
    /// `transf` is a closure that transforms each bidirectional edge to the current type of edge.
    /// This is used mostly to reduce the memory used. For instance, instead of storing a pair for
    /// each of the edges, you can store just the destination of the edge, because the other one is
    /// implicit.
    ///
    /// `undirected` should be `true` if the graph is undirected.
    pub fn from_edges<T, F>(
        v: usize, 
        edges: Vec<T>, 
        transf: F,
        undirected: bool
    ) -> Graph<V, E>
    where T: BidirectionalEdge,
          F: Fn(T) -> E {
        let mut nodes = Vec::with_capacity(v);
        for _ in 0..v {
            nodes.push(V::default());
        }

        let mut capacity = vec![0; v];
        for e in &edges {
            let (a, b) = e.as_pair();
            capacity[a] += 1;
            if undirected {
                capacity[b] += 1;
            }
        }

        let mut adj_list: Vec<Vec<usize>> = Vec::with_capacity(v);
        for i in 0..v {
            adj_list.push(Vec::with_capacity(capacity[i]));
        }

        for id in 0..edges.len() {
            let edge = &edges[id];
            let (a, b) = edge.as_pair();
            adj_list[a].push(id);

            if undirected {
                adj_list[b].push(id);
            }
        }
        
        let edges: Vec<E> = edges.into_iter().map(transf).collect();

        Graph::<V, E> {
            nodes,
            edges,
            adj_list,
            undirected
        }
    }

    /// Create an empty graph with capacities for the nodes and edges.
    pub fn with_capacity(v: usize, e: usize, undirected: bool) -> Graph<V, E> {
        Graph::<V, E> {
            nodes: (0..v).map(|_| V::default()).collect(),
            edges: Vec::with_capacity(e),
            adj_list: vec![Vec::new(); v],
            undirected
        }
    }

    /// Push a directed edge in the graph.
    pub fn push_directed_edge(&mut self, from: usize, edge: E) {
        let id = self.edges.len();
        self.edges.push(edge);
        self.adj_list[from].push(id);
    }

    /// Push an undirected edge in the graph.
    pub fn push_undirected_edge(&mut self, edge: E)
    where E: BidirectionalEdge {
        let id = self.edges.len();
        let (a, b) = edge.as_pair();

        self.edges.push(edge);
        self.adj_list[a].push(id);
        self.adj_list[b].push(id);
    }

    /// Return the number of nodes.
    pub fn v(&self) -> usize { self.nodes.len() }
    
    /// Return the number of edges.
    pub fn e(&self) -> usize { self.edges.len() }
}

impl Edge for usize {
    fn to(&self, _: usize) -> usize { *self }
}

impl Edge for (usize, usize) {
    fn to(&self, from: usize) -> usize {
        return self.0 ^ self.1 ^ from;
    }
}

impl BidirectionalEdge for (usize, usize) {
    fn as_pair(&self) -> (usize, usize) {
        return *self;
    }
}

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

