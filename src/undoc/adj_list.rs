#![allow(dead_code)]

pub trait Edge {
    fn to(&self, from: usize) -> usize;
}

pub trait BidirectionalEdge: Edge {
    fn as_pair(&self) -> (usize, usize);
}

#[derive(Debug)]
pub struct Graph<V, E>
where V: Default,
      E: Edge {
    pub nodes: Vec<V>,
    pub edges: Vec<E>,
    pub adj_list: Vec<Vec<usize>>,
    pub undirected: bool,
}

impl<V, E> Graph<V, E> 
where V: Default,
      E: Edge {
    
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

    pub fn with_capacity(v: usize, e: usize, undirected: bool) -> Graph<V, E> {
        Graph::<V, E> {
            nodes: (0..v).map(|_| V::default()).collect(),
            edges: Vec::with_capacity(e),
            adj_list: vec![Vec::new(); v],
            undirected
        }
    }

    pub fn push_directed_edge(&mut self, from: usize, edge: E) {
        let id = self.edges.len();
        self.edges.push(edge);
        self.adj_list[from].push(id);
    }

    pub fn push_undirected_edge(&mut self, edge: E)
    where E: BidirectionalEdge {
        let id = self.edges.len();
        let (a, b) = edge.as_pair();

        self.edges.push(edge);
        self.adj_list[a].push(id);
        self.adj_list[b].push(id);
    }

    pub fn v(&self) -> usize { self.nodes.len() }
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

