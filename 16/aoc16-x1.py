#!/usr/bin/env python3

from dataclasses import dataclass
import copy
from itertools import chain
import re
import sys

r = re.compile("Valve (..) has flow rate=(.+); tunnels? leads? to valves? (.+)")

class Valve:
	def __init__(self, name, rate, tunnels):
		self.name = name
		self.rate = rate
		self.tunnels = tunnels

	def __repr__(self):
		return f'Valve(name={self.name} rate={self.rate} tunnels={",".join(self.tunnels)})'

class Graph:
	def __init__(self):
		self.valves = dict()
	
	def add(self, valve):
		self.valves[valve.name] = valve

	def get(self, name):
		return self.valves[name]

	def __repr__(self):
		return f'Graph(valves={self.valves})'

class Path:
	def __init__(self):
		self.steps = ['AA']
		self.opened = set()
		self.press = 0

	def loc(self):
		return self.steps[-1]

	def ptot(self):
		return self.ptot

	def succ(self, g, rem_steps):
		loc = self.loc()
		valve = g.get(loc)
		assert loc == valve.name
		result = []
		for tunnel in valve.tunnels:
			next = copy.deepcopy(self)
			next.steps.append(tunnel)
			result.append(next)
		if valve.rate > 0 and loc not in self.opened:
			next = copy.deepcopy(self)
			next.steps.append(loc)
			next.opened.add(loc)
			next.press += rem_steps * valve.rate
			result.append(next)
		return result

	def __repr__(self):
		return f'Path(steps={self.steps} opened={self.opened} press={self.press})'

def collapse(all_succs):
	"""
	best_ptot = dict()
	best_pps = dict()
	for succs in all_succs:
		for succ in succs:
			loc = succ.loc()
			if loc not in best_ptot or best_ptot[loc].ptot < succ.ptot:
				best_ptot[loc] = succ
			if loc not in best_pps or best_pps[loc].pps < succ.pps:
				best_pps[loc] = succ
	return chain(best_ptot.values(), best_pps.values())
	"""
	best = dict()
	for succs in all_succs:
		for succ in succs:
			loc = succ.loc()
			if loc not in best or best[loc].press < succ.press:
				best[loc] = succ
	return best.values()

if __name__ == "__main__":
	g = Graph()
	f = open(sys.argv[1], "r")
	for line in f.readlines():
		m = r.match(line)
		v = Valve(m.group(1), int(m.group(2)), m.group(3).split(", "))
		g.add(v)

	MAX_STEPS = 30
	paths = [Path()]
	for step in range(MAX_STEPS):
		all_succs = map(lambda p: p.succ(g, MAX_STEPS - step - 1), paths)
		paths = collapse(all_succs)

	paths = list(paths)
	best_path = paths[0]
	for path in paths:
		if path.press > best_path.press:
			best_path = path
	print(best_path)
	print(best_path.press)
