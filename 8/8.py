#!/usr/bin/env python3

import sys

def parse_line(l):
	l = l.rstrip()
	l = list(l)
	l = map(int, l)
	return list(l)

f = open(sys.argv[1], "r")
heights = list(map(parse_line, f.readlines()))

MAX_I = len(heights)
MAX_J = len(heights[0])

can_see = [[False for j in range(MAX_J)] for i in range(MAX_I)]

def check(i, j, maxheight):
	height = heights[i][j]
	if height > maxheight:
		can_see[i][j] = True
		return height
	return maxheight


for i in range(MAX_I):
	maxheight = -1
	for j in range(MAX_J):
		maxheight = check(i, j, maxheight)
	maxheight = -1
	for j in reversed(range(MAX_J)):
		maxheight = check(i, j, maxheight)

for j in range(MAX_J):
	maxheight = -1
	for i in range(MAX_I):
		maxheight = check(i, j, maxheight)
	maxheight = -1
	for i in reversed(range(MAX_I)):
		maxheight = check(i, j, maxheight)

#print(can_see)

tot = 0
for i in range(MAX_I):
	for j in range(MAX_J):
		if can_see[i][j]:
			tot += 1
print(tot)
