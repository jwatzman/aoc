#!/usr/bin/env python3

import sys

def mk_next(sz, p):
	candidates = [
		(p[0]-1, p[1]),
		(p[0]+1, p[1]),
		(p[0],   p[1]-1),
		(p[0],   p[1]+1),
	]
	#return map(lambda pp: ((pp[0] + sz[0]) % sz[0], (pp[1] + sz[1]) % sz[1]))
	#return map(lambda pp: (pp[0] % sz[0], pp[1] % sz[1]), candidates)
	return candidates

def get_plot(plot, sz, p):
	return plot[p[0] % sz[0]][p[1] % sz[1]]

def scan(plot, sz, visited, reachable, steps, p):
	if (p, steps) in visited:
		return
	visited.add((p, steps))
	nxt = mk_next(sz, p)
	if steps == 0:
		reachable.add(p)
	else:
		for nxt_p in filter(lambda pp: get_plot(plot, sz, pp) != "#", nxt):
			scan(plot, sz, visited, reachable, steps - 1, nxt_p)

f = open(sys.argv[1], "r")
plot = list(map(lambda l: l.strip(), f.readlines()))
sz = (len(plot), len(plot[0]))

start = None
for row in range(len(plot)):
	if "S" in plot[row]:
		start = (row, plot[row].index("S"))
		break
assert start != None
assert get_plot(plot, sz, start) == "S"

visited = set()
reachable = set()
scan(plot, sz, visited, reachable, int(sys.argv[2]), start)
print(len(reachable))
