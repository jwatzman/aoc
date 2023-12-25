#!/usr/bin/env python3

from collections import defaultdict
import sys

def read_adj():
	adj = defaultdict(set)
	f = open(sys.argv[1], "r")
	for line in f.readlines():
		node, adj_nodes = line.strip().split(":")
		for adj_node in adj_nodes.strip().split(" "):
			adj[node].add(adj_node)
			adj[adj_node].add(node)
	return adj

def remove_edges(adj):
	banned = [("rxt", "bqq"), ("qxr", "btp"), ("vfx", "bgl")]
	for (n1, n2) in banned:
		adj[n1].remove(n2)
		adj[n2].remove(n1)

def do_dfs(node, visited, adj):
	if node in visited:
		return
	visited.add(node)
	for adj_node in adj[node]:
		do_dfs(adj_node, visited, adj)

adj = read_adj()
remove_edges(adj)

v1 = set()
do_dfs("rxt", v1, adj)

v2 = set()
do_dfs("bqq", v2, adj)

print(len(v1))
print(len(v2))
print(len(v1) * len(v2))
