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

