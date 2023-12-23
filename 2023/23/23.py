#!/usr/bin/env python3

import sys

sys.setrecursionlimit(10000)

f = open(sys.argv[1], "r")
maze = list(map(lambda l: l.strip(), f.readlines()))
prevs = dict()

START = (0, 1)
DEST = (len(maze) - 1, len(maze[0]) - 2)

def get_maze(p):
	return maze[p[0]][p[1]]

def pathlen(p):
	cur = p
	l = 0
	while cur != START:
		#assert l < 100
		cur = prevs[cur]
		l += 1
	return l

def succ(p, prev):
	if p == DEST:
		return []
	tile = get_maze(p)
	row, col = p
	if tile == ">":
		return [(row, col + 1)]
	elif tile == "<":
		return [(row, col - 1)]
	elif tile == "v":
		return [(row + 1, col)]
	elif tile == "^":
		return [(row - 1, col)]
	else:
		assert tile == "."
		candidates = [
			((row, col + 1), ">"),
			((row, col - 1), "<"),
			((row + 1, col), "v"),
			((row - 1, col), "^"),
		]
		res = []
		for pp, allowed_tile in candidates:
			if pp == prev:
				continue
			tt = get_maze(pp)
			if tt == "." or tt == allowed_tile:
				res.append(pp)
		return res

def explore(p, prev):
	if p in prevs:
		best = pathlen(p)
		mine = pathlen(prev) + 1
		if mine > best:
			prevs[p] = prev
	else:
		prevs[p] = prev
		for pp in succ(p, prev):
			explore(pp, p)

explore(START, None)
print(pathlen(DEST))
