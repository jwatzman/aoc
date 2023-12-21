#!/usr/bin/env python3

import sys

def mk_next(p):
	return [
		(p[0]-1, p[1]),
		(p[0]+1, p[1]),
		(p[0],   p[1]-1),
		(p[0],   p[1]+1),
	]

def get_plot(plot, p):
	return plot[p[0]][p[1]]

def scan(plot, visited, reachable, steps, p):
	if (p, steps) in visited:
		return
	visited.add((p, steps))
	nxt = mk_next(p)
	if steps == 0:
		reachable.add(p)
	else:
		for nxt_p in filter(lambda pp: get_plot(plot, pp) != "#", nxt):
			scan(plot, visited, reachable, steps - 1, nxt_p)

f = open(sys.argv[1], "r")
plot = list(map(lambda l: l.strip(), f.readlines()))

start = None
for row in range(len(plot)):
	if "S" in plot[row]:
		start = (row, plot[row].index("S"))
		break
assert start != None
assert get_plot(plot, start) == "S"

visited = set()
reachable = set()
scan(plot, visited, reachable, int(sys.argv[2]), start)
print(len(reachable))
