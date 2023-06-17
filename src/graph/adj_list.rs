#![allow(dead_code)]

use crate::graph::Graph;
use std::slice::Iter;

/// Generic implementation for graphs.
///
/// By default, this is a directed graph. To make it an undirected graph,
/// consider the graph as a directed one.
///
/// Any function that uses this graph as an undirected graph will panic if this
/// is not actually an undirected graph.
///
/// The nodes are numbered from 0 to V - 1.
struct AdjListGraph {
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
    /// Construct a graph with `v` nodes and no edges.
    fn empty(v: usize) -> AdjListGraph {
        AdjListGraph {
            v,
            adj: vec![vec![]; v],
        }
    } 

    /// Construct a graph with `v` nodes and the given edges.
    fn from_edges(v: usize, edges: &[(usize, usize)]) -> AdjListGraph {
        let mut g = AdjListGraph::empty(v);

        for edge in edges {
            g.push_edge(edge.0, edge.1);
        }

        g
    } 

    /// Construct a graph with `v` nodes and the given undirected edges.
    fn from_undirected_edges(v: usize, edges: &[(usize, usize)]) -> AdjListGraph {
        let mut g = AdjListGraph::empty(v);

        for edge in edges {
            g.push_undirected_edge(edge.0, edge.1);
        }

        g
    } 

    /// Push a directed edge in the graph.
    fn push_edge(&mut self, a: usize, b: usize) {
        self.adj[a].push(b);
    }

    /// Push an undirected edge in the graph.
    fn push_undirected_edge(&mut self, a: usize, b: usize) {
        self.push_edge(a, b);
        self.push_edge(b, a);
    }

    /// Check if the graph is undirected.
    ///
    /// If you need to assert that a given graph is undirected, don't use
    /// [assert], use [debug_assert] instead.
    fn is_undirected(&self) -> bool {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn graph_v_empty() {
        let g = AdjListGraph::empty(10);

        assert_eq!(g.v(), 10);
    }

    #[test]
    fn graph_e_empty() {
        let g = AdjListGraph::empty(10);

        assert_eq!(g.e(), 0);
    }

    #[test]
    fn directed_graph_count() {
        let edges = vec![(0, 1), (1, 2)];
        let g = AdjListGraph::from_edges(3, &edges);

        assert_eq!(g.e(), edges.len());
    }

    #[test]
    fn undirected_edge_count() {
        let edges = vec![(0, 1), (1, 2)];
        let g = AdjListGraph::from_undirected_edges(3, &edges);

        assert_eq!(g.e(), edges.len() * 2);
    }

    #[test]
    fn test_undirected_adjacency() {
        let edges = vec![(0, 1), (2, 0)];
        let g = AdjListGraph::from_undirected_edges(3, &edges);

        assert_eq!(g.neighbours(0).map(|e| *e).collect::<Vec<usize>>(), vec![1, 2]);
        assert_eq!(g.neighbours(1).map(|e| *e).collect::<Vec<usize>>(), vec![0]);
        assert_eq!(g.neighbours(2).map(|e| *e).collect::<Vec<usize>>(), vec![0]);
    }

    #[test]
    fn test_directed_adjacency() {
        let edges = vec![(0, 1), (2, 0)];
        let g = AdjListGraph::from_edges(3, &edges);

        assert_eq!(g.neighbours(0).map(|e| *e).collect::<Vec<usize>>(), vec![1]);
        assert_eq!(g.neighbours(1).map(|e| *e).collect::<Vec<usize>>(), vec![]);
        assert_eq!(g.neighbours(2).map(|e| *e).collect::<Vec<usize>>(), vec![0]);
    }

    #[test]
    fn test_nodes() {
        let g = AdjListGraph::empty(10);

        assert_eq!(g.nodes(), vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9]);
    }
}

