#!/usr/bin/env python3

import sys

sys.setrecursionlimit(10000)

f = open(sys.argv[1], "r")
maze = list(map(lambda l: l.strip(), f.readlines()))

START = (0, 1)
DEST = (len(maze) - 1, len(maze[0]) - 2)

def get_maze(p):
	return maze[p[0]][p[1]]

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

def findpath(p, prev, path):
	if p == DEST:
		return (0, path)
	new_path = path.copy()
	new_path.append(p)
	best = (-100, None)
	succs = succ(p, prev)
	if len(succs) > 1:
		print(p)
	for pp in succs:
		if pp not in path:
			maybe = findpath(pp, p, new_path)
			if maybe[0] + 1 > best[0]:
				best = (maybe[0] + 1, maybe[1])
	return best

dist, path = findpath(START, None, [])
print("***")

print(dist)
pb = set(path)
for row in range(len(maze)):
	for col in range(len(maze[row])):
		if (row, col) in pb:
			print("O", end="")
		else:
			print(maze[row][col], end="")
	print()
