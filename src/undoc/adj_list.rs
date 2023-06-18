#![allow(dead_code)]

use std::slice::Iter;

pub trait Graph<'a, T>
where T: 'a + Copy {
    fn v(&self) -> usize;
    fn e(&self) -> usize;
    fn neighbours(&'a self, node: T) -> Iter<'a, T>;
    fn nodes(&self) -> Vec<T>;
    fn edges(&'a self) -> Vec<(T, T)> {
        let mut edges = Vec::<(T, T)>::new();

        for node in self.nodes() {
            for neigh in self.neighbours(node) {
                edges.push((node, *neigh));
            }
        }

        edges
    }
}

pub struct AdjListGraph {
    pub v: usize,
    pub adj: Vec<Vec<usize>>,
}

impl<'a> Graph<'a, usize> for AdjListGraph {
    fn v(&self) -> usize { self.adj.len() }
    fn e(&self) -> usize { self.adj.iter().fold(0, |ac, e| { ac + e.len() } ) }
    
    fn nodes(&self) -> Vec<usize> {
        (0..self.v()).collect()
    }

    fn neighbours(&'a self, node: usize) -> Iter<'a, usize> {
        self.adj[node].iter()
    }
}

impl AdjListGraph {
    pub fn empty(v: usize) -> AdjListGraph {
        AdjListGraph {
            v,
            adj: vec![vec![]; v],
        }
    } 
    
    pub fn from_edges(v: usize, edges: &[(usize, usize)]) -> AdjListGraph {
        let mut g = AdjListGraph::empty(v);

        for edge in edges {
            g.push_edge(edge.0, edge.1);
        }

        g
    } 

    pub fn from_undirected_edges(v: usize, edges: &[(usize, usize)]) -> AdjListGraph {
        let mut g = AdjListGraph::empty(v);

        for edge in edges {
            g.push_undirected_edge(edge.0, edge.1);
        }

        g
    } 

    pub fn push_edge(&mut self, a: usize, b: usize) {
        self.adj[a].push(b);
    }

    pub fn push_undirected_edge(&mut self, a: usize, b: usize) {
        self.push_edge(a, b);
        self.push_edge(b, a);
    }

    pub fn is_undirected(&self) -> bool {
        use std::collections::HashMap;

        let mut hashmap = HashMap::<(usize, usize), usize>::new();
        
        for edge in self.edges() {
            let entry = *hashmap.entry(edge)
                .or_insert(0);
            hashmap.insert(edge, entry + 1);
        }

        for edge in self.edges() {
            if hashmap.get(&edge) != hashmap.get(&(edge.1, edge.0)) {
                return false;
            }
        }

        return true;
    }
}

