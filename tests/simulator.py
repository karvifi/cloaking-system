#!/usr/bin/env python3
"""
Network simulator for testing Aether anonymity properties
"""

import networkx as nx
import numpy as np
from dataclasses import dataclass
from typing import List, Tuple
import matplotlib.pyplot as plt

@dataclass
class SimulationConfig:
    num_nodes: int = 100
    num_layers: int = 5
    num_packets: int = 1000
    adversary_coverage: float = 0.2  # 20% of network monitored
    simulation_duration: int = 3600  # seconds

class AetherSimulator:
    def __init__(self, config: SimulationConfig):
        self.config = config
        self.network = self.create_network()
        self.metrics = {
            'anonymity_set': [],
            'entropy': [],
            'correlation': [],
            'latency': [],
        }
        
    def create_network(self) -> nx.DiGraph:
        """Create a stratified network graph"""
        G = nx.DiGraph()
        
        nodes_per_layer = self.config.num_nodes // self.config.num_layers
        
        # Create nodes in layers
        for layer in range(self.config.num_layers):
            for i in range(nodes_per_layer):
                node_id = layer * nodes_per_layer + i
                G.add_node(node_id, layer=layer, reputation=np.random.uniform(0.7, 1.0))
        
        # Create edges between adjacent layers
        for layer in range(self.config.num_layers - 1):
            current_layer = range(layer * nodes_per_layer, (layer + 1) * nodes_per_layer)
            next_layer = range((layer + 1) * nodes_per_layer, (layer + 2) * nodes_per_layer)
            
            for node in current_layer:
                # Connect to 3-5 random nodes in next layer
                targets = np.random.choice(list(next_layer), size=np.random.randint(3, 6), replace=False)
                for target in targets:
                    latency = np.random.exponential(50)  # Exponential delay
                    G.add_edge(node, target, latency=latency)
        
        return G
    
    def simulate_packet_flow(self, source: int, destination: int) -> Tuple[List[int], float]:
        """Simulate a packet flowing through the network"""
        try:
            path = nx.shortest_path(self.network, source, destination)
            total_latency = sum(
                self.network[path[i]][path[i+1]]['latency']
                for i in range(len(path) - 1)
            )
            return path, total_latency
        except nx.NetworkXNoPath:
            return [], 0.0
    
    def calculate_anonymity_set(self, observed_flows: List[Tuple[int, int]]) -> int:
        """Calculate size of anonymity set"""
        # Anonymity set = number of potential senders for any received message
        unique_senders = set(flow[0] for flow in observed_flows)
        return len(unique_senders)
    
    def calculate_entropy(self, traffic_distribution: np.ndarray) -> float:
        """Calculate Shannon entropy of traffic distribution"""
        # Normalize to probability distribution
        probs = traffic_distribution / np.sum(traffic_distribution)
        # Filter out zeros
        probs = probs[probs > 0]
        # Calculate entropy
        entropy = -np.sum(probs * np.log2(probs))
        return entropy
    
    def simulate_adversary_correlation(self, num_monitored: int) -> float:
        """Simulate correlation attack success rate"""
        monitored_nodes = np.random.choice(
            list(self.network.nodes()),
            size=num_monitored,
            replace=False
        )
        
        successful_correlations = 0
        total_attempts = 100
        
        for _ in range(total_attempts):
            # Random source and destination
            source = np.random.choice(list(self.network.nodes()))
            dest = np.random.choice(list(self.network.nodes()))
            
            path, _ = self.simulate_packet_flow(source, dest)
            
            if not path:
                continue
            
            # Check if adversary can correlate
            monitored_in_path = [node for node in path if node in monitored_nodes]
            
            # If >50% of path is monitored, correlation succeeds
            if len(monitored_in_path) / len(path) > 0.5:
                successful_correlations += 1
        
        return successful_correlations / total_attempts
    
    def run_simulation(self):
        """Run full simulation"""
        print(f"Starting Aether Network Simulation...")
        print(f"Nodes: {self.config.num_nodes}, Layers: {self.config.num_layers}")
        print(f"Adversary coverage: {self.config.adversary_coverage * 100}%")
        
        # Simulate packet flows
        flows = []
        latencies = []
        
        for _ in range(self.config.num_packets):
            source = np.random.choice(list(self.network.nodes()))
            dest = np.random.choice(list(self.network.nodes()))
            
            if source != dest:
                path, latency = self.simulate_packet_flow(source, dest)
                if path:
                    flows.append((source, dest))
                    latencies.append(latency)
        
        # Calculate metrics
        anonymity_set = self.calculate_anonymity_set(flows)
        
        # Traffic distribution entropy
        traffic_dist = np.zeros(self.config.num_nodes)
        for source, _ in flows:
            traffic_dist[source] += 1
        entropy = self.calculate_entropy(traffic_dist)
        
        # Adversary correlation
        num_monitored = int(self.config.num_nodes * self.config.adversary_coverage)
        correlation_rate = self.simulate_adversary_correlation(num_monitored)
        
        avg_latency = np.mean(latencies) if latencies else 0
        
        # Results
        print("\n=== SIMULATION RESULTS ===")
        print(f"Anonymity Set Size: {anonymity_set}")
        print(f"Traffic Entropy: {entropy:.2f} bits")
        print(f"Correlation Success Rate: {correlation_rate * 100:.1f}%")
        print(f"Average Latency: {avg_latency:.2f} ms")
        print(f"Anonymity Level: {'HIGH' if correlation_rate < 0.1 else 'MEDIUM' if correlation_rate < 0.3 else 'LOW'}")
        
        return {
            'anonymity_set': anonymity_set,
            'entropy': entropy,
            'correlation_rate': correlation_rate,
            'avg_latency': avg_latency,
        }
    
    def plot_network(self):
        """Visualize the network topology"""
        pos = {}
        nodes_per_layer = self.config.num_nodes // self.config.num_layers
        
        for layer in range(self.config.num_layers):
            for i in range(nodes_per_layer):
                node_id = layer * nodes_per_layer + i
                x = layer
                y = i - nodes_per_layer / 2
                pos[node_id] = (x, y)
        
        plt.figure(figsize=(12, 8))
        nx.draw(
            self.network,
            pos,
            node_size=50,
            node_color='lightblue',
            with_labels=False,
            arrows=True,
            edge_color='gray',
            alpha=0.6,
        )
        plt.title("Aether Network Topology (Stratified)")
        plt.savefig("network_topology.png", dpi=300, bbox_inches='tight')
        print("\nNetwork topology saved to network_topology.png")

if __name__ == "__main__":
    config = SimulationConfig(
        num_nodes=100,
        num_layers=5,
        num_packets=1000,
        adversary_coverage=0.2,
    )
    
    sim = AetherSimulator(config)
    results = sim.run_simulation()
    sim.plot_network()
    
    print("\n=== SECURITY ANALYSIS ===")
    print(f"Against 20% passive adversary:")
    print(f"- Unlinkability: {'STRONG' if results['correlation_rate'] < 0.15 else 'MODERATE'}")
    print(f"- Unobservability: {'STRONG' if results['entropy'] > 5 else 'MODERATE'}")
    print(f"- Resistance: {(1 - results['correlation_rate']) * 100:.1f}%")
