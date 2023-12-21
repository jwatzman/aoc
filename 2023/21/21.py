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

def scan(plot, sz, p):
	nxt = mk_next(sz, p)
	return filter(lambda pp: get_plot(plot, sz, pp) != "#", nxt)

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

steps = int(sys.argv[2])
work = set()
work.add(start)
while steps > 0:
	if (steps % 100 == 0):
		print(steps, len(work))
	new_work = set()
	for p in work:
		new_work.update(scan(plot, sz, p))
	work = new_work
	steps -= 1

print(len(work))
