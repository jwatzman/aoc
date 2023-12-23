#!/usr/bin/env python3

import random
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
	path.add(p)
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
	random.shuffle(candidates)
	return list(filter(lambda pp: pp != prev and get_maze(pp) != "#", candidates))

def explore(p, prev):
	if p in prevs:
		best = pathback(p)
		mine = pathback(prev)
		if p not in mine:
			if len(mine) + 1 > len(best):
				#print("UPDATING", p, prevs[p], prev)
				prevs[p] = prev
			#else:
				#print("not long enough", len(mine), len(best), p)
		#else:
			#print("loop avoided", p)
	else:
		prevs[p] = prev
		for pp in succ(p, prev):
			explore(pp, p)

def improve(pb):
	global prevs
	print("Found a path with length", len(pb))
	cleared_prevs = dict()
	for p in pb:
		cleared_prevs[p] = prevs[p]
	for p in pb:
		prevs = cleared_prevs.copy()
		prev = prevs[p]
		if len(succ(p, prev)) == 1:
			continue
		del prevs[p]
		explore(p, prev)
		newpb = pathback(DEST)
		assert len(newpb) >= len(pb)
		if len(newpb) > len(pb):
			return improve(newpb)
	return pb

pb = set()
for i in range(100):
	prevs = dict()
	explore(START, None)
	newpb = improve(pathback(DEST))
	if len(newpb) > len(pb):
		pb = newpb

print("The very best is", len(pb) - 1)

"""
for row in range(len(maze)):
	for col in range(len(maze[row])):
		if (row, col) in pb:
			print("O", end="")
		else:
			print(maze[row][col], end="")
	print()
"""
