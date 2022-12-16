#!/usr/bin/env python3

from dataclasses import dataclass
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

def all_dists(g):
	all = dict()
	for valve in g.valves.values():
		if valve.rate > 0 or valve.name == FIRST_VALVE_NAME:
			all[valve.name] = dists_from(g, valve)
	return all

"""
def perms(accum, g, all_dists, cur, path, opened, score, steps_remaining):
	assert steps_remaining > 0
	if not cur.name in opened:
		steps_continued = steps_remaining - 1
		if steps_continued > 0:
			path.append(cur.name)
			opened.add(cur.name)
			new_score = score + (cur.rate * steps_continued)
			accum[",".join(path)] = new_score
			perms(accum, g, all_dists, cur, path, opened, new_score, steps_continued)
			opened.remove(cur.name)
			path.pop()
	dists = all_dists[cur.name]
	for next_name in dists:
		next = g.valves[next_name]
		steps_continued = steps_remaining - dists[next_name]
		if steps_continued > 1:
			path.append(next_name)
			perms(accum, g, all_dists, next, path, opened, score, steps_continued)
			path.pop()
"""

def perms(accum, g, all_dists, cur, path, opened, score, steps_remaining):
	assert steps_remaining > 0
	dists = all_dists[cur.name]
	for next_name in dists:
		if next_name in opened:
			continue
		next = g.valves[next_name]
		steps_continued = steps_remaining - dists[next_name] - 1
		if steps_continued > 0:
			path.append(next_name)
			opened.add(next_name)
			new_score = score + (next.rate * steps_continued)
			accum[",".join(path)] = new_score
			perms(accum, g, all_dists, next, path, opened, new_score, steps_continued)
			opened.remove(next_name)
			path.pop()

def main():
	r = re.compile("Valve (..) has flow rate=(.+); tunnels? leads? to valves? (.+)")
	g = Graph(dict())
	f = open(sys.argv[1], "r")
	for line in f.readlines():
		m = r.match(line)
		v = Valve(m.group(1), int(m.group(2)), m.group(3).split(", "))
		g.valves[v.name] = v

	#print(g)
	#print(dists_from(g, g.valves["BB"]))
	#print(all_dists(g))

	accum = dict()
	perms(accum, g, all_dists(g), g.valves[FIRST_VALVE_NAME], [FIRST_VALVE_NAME], set(), 0, 30)
	keys = accum.keys()
	sorted(keys)
	best = -1
	for key in keys:
		val = accum[key]
		if val > best:
			best = val
		#print(f'{key} = {val}')
	print(best)

if __name__ == "__main__":
	main()
