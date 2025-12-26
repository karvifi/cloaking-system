//! Multi-path routing algorithms

use crate::mixnet::NodeInfo;
use petgraph::graph::{DiGraph, NodeIndex};
use std::collections::{HashMap, HashSet};

/// Multi-path router for finding disjoint paths
pub struct MultipathRouter {
    /// Network graph
    graph: DiGraph<NodeInfo, EdgeInfo>,
    
    /// Node index mapping
    node_map: HashMap<[u8; 32], NodeIndex>,
}

#[derive(Clone, Debug)]
struct EdgeInfo {
    latency_ms: u64,
    bandwidth: u64,
}

impl MultipathRouter {
    pub fn new() -> Self {
        Self {
            graph: DiGraph::new(),
            node_map: HashMap::new(),
        }
    }
    
    /// Add a node to the network graph
    pub fn add_node(&mut self, node_info: NodeInfo) -> NodeIndex {
        let idx = self.graph.add_node(node_info.clone());
        self.node_map.insert(node_info.id, idx);
        idx
    }
    
    /// Add an edge between two nodes
    pub fn add_edge(&mut self, from_id: [u8; 32], to_id: [u8; 32], latency_ms: u64, bandwidth: u64) {
        if let (Some(&from_idx), Some(&to_idx)) = (self.node_map.get(&from_id), self.node_map.get(&to_id)) {
            self.graph.add_edge(from_idx, to_idx, EdgeInfo { latency_ms, bandwidth });
        }
    }
    
    /// Find k disjoint paths between source and destination
    pub fn find_disjoint_paths(
        &self,
        source_id: [u8; 32],
        dest_id: [u8; 32],
        k: usize,
    ) -> Vec<Vec<NodeIndex>> {
        let source_idx = match self.node_map.get(&source_id) {
            Some(&idx) => idx,
            None => return Vec::new(),
        };
        
        let dest_idx = match self.node_map.get(&dest_id) {
            Some(&idx) => idx,
            None => return Vec::new(),
        };
        
        let mut paths = Vec::new();
        let mut used_nodes = HashSet::new();
        let mut used_edges = HashSet::new();
        
        // Preserve source and destination
        used_nodes.insert(source_idx);
        used_nodes.insert(dest_idx);
        
        for _ in 0..k {
            if let Some(path) = self.find_path_excluding(
                source_idx,
                dest_idx,
                &used_nodes,
                &used_edges,
            ) {
                // Mark intermediate nodes and edges as used
                for window in path.windows(2) {
                    let from = window[0];
                    let to = window[1];
                    
                    // Don't mark source/dest as used for future paths
                    if from != source_idx && from != dest_idx {
                        used_nodes.insert(from);
                    }
                    if to != source_idx && to != dest_idx {
                        used_nodes.insert(to);
                    }
                    
                    used_edges.insert((from, to));
                }
                
                paths.push(path);
            } else {
                break;
            }
        }
        
        paths
    }
    
    /// Find a single path excluding certain nodes and edges
    fn find_path_excluding(
        &self,
        source: NodeIndex,
        dest: NodeIndex,
        excluded_nodes: &HashSet<NodeIndex>,
        excluded_edges: &HashSet<(NodeIndex, NodeIndex)>,
    ) -> Option<Vec<NodeIndex>> {
        // Simplified pathfinding that avoids excluded nodes and edges
        // Use BFS with manual filtering instead of astar's edge callback
        
        use petgraph::visit::EdgeRef;
        use std::collections::VecDeque;
        
        let mut queue = VecDeque::new();
        let mut visited = HashSet::new();
        let mut parent: HashMap<NodeIndex, (NodeIndex, u64)> = HashMap::new();
        
        queue.push_back((source, 0u64));
        visited.insert(source);
        
        while let Some((current, cost)) = queue.pop_front() {
            if current == dest {
                // Reconstruct path
                let mut path = vec![dest];
                let mut node = dest;
                while node != source {
                    if let Some(&(pred, _)) = parent.get(&node) {
                        path.push(pred);
                        node = pred;
                    } else {
                        break;
                    }
                }
                path.reverse();
                return Some(path);
            }
            
            // Explore neighbors
            for edge in self.graph.edges(current) {
                let next = edge.target();
                
                // Skip excluded edges
                if excluded_edges.contains(&(current, next)) {
                    continue;
                }
                
                // Skip excluded nodes (except dest)
                if next != dest && excluded_nodes.contains(&next) {
                    continue;
                }
                
                if !visited.contains(&next) {
                    visited.insert(next);
                    let edge_cost = edge.weight().latency_ms;
                    queue.push_back((next, cost + edge_cost));
                    parent.insert(next, (current, cost + edge_cost));
                }
            }
        }
        
        None
    }
    
    /// Get node information
    pub fn get_node(&self, node_id: &[u8; 32]) -> Option<&NodeInfo> {
        self.node_map.get(node_id).map(|&idx| &self.graph[idx])
    }
}

/// Find k disjoint paths (convenience function)
pub fn find_disjoint_paths(
    router: &MultipathRouter,
    source: [u8; 32],
    dest: [u8; 32],
    k: usize,
) -> Vec<Vec<NodeIndex>> {
    router.find_disjoint_paths(source, dest, k)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mixnet::NodeRole;
    
    #[test]
    fn test_multipath_routing() {
        let mut router = MultipathRouter::new();
        
        // Create a simple network
        let nodes: Vec<NodeInfo> = (0..6).map(|i| {
            NodeInfo {
                id: {
                    let mut id = [0u8; 32];
                    id[0] = i;
                    id
                },
                layer: (i % 5) + 1,
                role: NodeRole::MixNode,
                reputation: 0.9,
                stake: 1000,
                address: format!("node-{}", i),
                public_key_bytes: vec![i; 1568],
            }
        }).collect();
        
        // Add nodes
        for node in &nodes {
            router.add_node(node.clone());
        }
        
        // Add edges to create paths
        router.add_edge(nodes[0].id, nodes[1].id, 10, 1000);
        router.add_edge(nodes[0].id, nodes[2].id, 15, 1000);
        router.add_edge(nodes[1].id, nodes[3].id, 10, 1000);
        router.add_edge(nodes[2].id, nodes[4].id, 10, 1000);
        router.add_edge(nodes[3].id, nodes[5].id, 10, 1000);
        router.add_edge(nodes[4].id, nodes[5].id, 10, 1000);
        
        // Find disjoint paths
        let paths = router.find_disjoint_paths(nodes[0].id, nodes[5].id, 2);
        assert!(paths.len() >= 1);
    }
}
