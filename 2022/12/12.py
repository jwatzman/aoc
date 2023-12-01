#!/usr/bin/env python3

import sys

starts = []
end = None
heights = []

f = open(sys.argv[1], "r")
lines = f.readlines()
for i in range(len(lines)):
	line = lines[i].rstrip()
	lineheights = []
	for j in range(len(line)):
		c = line[j]
		if c == "S":
			c = 'a'
		elif c == "E":
			end = (i, j)
			c = 'z'
		
		if c == 'a':
			starts.append((i,j))

		lineheights.append(ord(c) - ord("a"))
	heights.append(lineheights)

MAX_I = len(heights)
MAX_J = len(heights[0])

def movedir(i, j, di, dj):
	i2 = i + di
	j2 = j + dj
	if i2 < 0 or i2 >= MAX_I or j2 < 0 or j2 >= MAX_J:
		return None
	h = heights[i][j]
	h2 = heights[i2][j2]
	if h2 > h + 1:
		return None
	return (i2, j2)

def move(i, j):
	moves = [movedir(i,j,1,0), movedir(i,j,-1,0), movedir(i,j,0,1), movedir(i,j,0,-1)]
	return filter(lambda x: x != None, moves)

mindist = 9999
for start in starts:
	dist = [[None for _ in range(MAX_J)] for _ in range(MAX_I)]
	cur = [start]
	nxt = []
	curdist = 0
	while len(cur) > 0:
		while len(cur) > 0:
			(i,j) = cur.pop()
			if dist[i][j] != None:
				continue
			dist[i][j] = curdist
			nxt.extend(move(i,j))
		curdist += 1
		cur = nxt
		nxt =  []
	startdist = dist[end[0]][end[1]]
	if startdist != None and startdist < mindist:
		mindist = startdist

print(mindist)
