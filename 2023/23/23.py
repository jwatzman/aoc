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

def pathback(p):
	cur = p
	path = set()
	while cur != START:
		cur = prevs[cur]
		assert (not (cur in path))
		path.add(cur)
	return path

def succ(p, prev):
	if p == DEST:
		return []
	tile = get_maze(p)
	row, col = p
	candidates = [
		(row, col + 1),
		(row, col - 1),
		(row + 1, col),
		(row - 1, col),
	]
	return list(filter(lambda pp: pp != prev and get_maze(pp) != "#", candidates))

"""
def explore(p, prev):
	if p in prevs:
		best = pathback(p)
		mine = pathback(prev)
		#if prev not in best:
		#if p not in best:
		if p not in mine:
			if len(mine) + 1 > len(best):
				print("UPDATING", p, prevs[p], prev)
				prevs[p] = prev
			else:
				print("not long enough", len(mine), len(best), p)
		else:
			print("loop avoided", p)
		#pathback(p)
	else:
		print("SETTING ", p, prev)
		prevs[p] = prev
		#pathback(p)
		for pp in succ(p, prev):
			explore(pp, p)
"""

"""
def explore():
	todo = [(START, None)]
	while len(todo) > 0:
		new_todo = []
		for p, prev in todo:
			if p in prevs:
				best = pathback(p)
				mine = pathback(prev)
				if p not in mine:
					assert len(mine) + 1 >= len(best)
					prevs[p] = prev
			else:
				prevs[p] = prev
				for pp in succ(p, prev):
					new_todo.append((pp, p))
		todo = new_todo
"""

explore()
print("***")

pb = pathback(DEST)
print(len(pb))
for row in range(len(maze)):
	for col in range(len(maze[row])):
		if (row, col) in pb:
			print("O", end="")
		else:
			print(maze[row][col], end="")
	print()
