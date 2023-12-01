#!/usr/bin/env python3

from dataclasses import dataclass
from itertools import chain, combinations
import re
import sys

FIRST_VALVE_NAME = 'AA'

@dataclass
class Valve:
	name: str
	rate: int
	tunnels: list

@dataclass
class Graph:
	valves: dict

def dists_from(g, valve):
	seen = set()
	seen.add(valve.name)
	dists = dict()
	dist = 1
	todo = valve.tunnels.copy()
	todo_next = []
	while len(todo) > 0:
		cur = g.valves[todo.pop()]
		#print("iterating", dist, cur, seen)
		if cur.name not in seen:
			seen.add(cur.name)
			if cur.rate > 0:
				dists[cur.name] = dist
			todo_next.extend(cur.tunnels)
		if len(todo) == 0:
			dist += 1
			todo = todo_next
			todo_next = []
	return dists

@dataclass
class AccumRef:
	best: int

def all_dists(g):
	all = dict()
	for valve in g.valves.values():
		if valve.rate > 0 or valve.name == FIRST_VALVE_NAME:
			all[valve.name] = dists_from(g, valve)
	return all

def perms(accum, g, all_dists, cur, opened, score, steps_remaining):
	assert steps_remaining > 0
	dists = all_dists[cur.name]
	for next_name in dists:
		if next_name in opened:
			continue
		next = g.valves[next_name]
		steps_continued = steps_remaining - dists[next_name] - 1
		if steps_continued > 0:
			opened.add(next_name)
			new_score = score + (next.rate * steps_continued)
			if new_score > accum.best:
				accum.best = new_score
			perms(accum, g, all_dists, next, opened, new_score, steps_continued)
			opened.remove(next_name)

def best(g, all_dists, skipme):
	accum = AccumRef(-1)
	perms(accum, g, all_dists, g.valves[FIRST_VALVE_NAME], skipme, 0, 26)
	return accum.best

def powerset(iterable):
	s = list(iterable)
	return chain.from_iterable(combinations(s, r) for r in range(len(s)+1))

def main():
	r = re.compile("Valve (..) has flow rate=(.+); tunnels? leads? to valves? (.+)")
	g = Graph(dict())
	f = open(sys.argv[1], "r")
	for line in f.readlines():
		m = r.match(line)
		v = Valve(m.group(1), int(m.group(2)), m.group(3).split(", "))
		g.valves[v.name] = v

	"""
	d = all_dists(g)
	b = best(g, d, set())
	print(b)
	"""

	d = all_dists(g)
	splitme = set(d.keys())
	bb = -1
	ps = list(powerset(d.keys()))
	i = 0
	print(len(ps))
	for subs in ps:
		i += 1
		if i % 1000 == 0:
			print(i)
		human = set(subs)
		elephant = splitme.difference(human)
		b1 = best(g, d, human)
		b2 = best(g, d, elephant)
		if b1 + b2 > bb:
			bb = b1 + b2
	print(bb)

if __name__ == "__main__":
	main()
