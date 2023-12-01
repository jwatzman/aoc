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

scenic_score = [[None for j in range(MAX_J)] for i in range(MAX_I)]

def scan(i, j, deltai, deltaj):
	tot = 0
	height = heights[i][j]
	i += deltai
	j += deltaj
	while i >= 0 and i < MAX_I and j >= 0 and j < MAX_J:
		tot += 1
		if heights[i][j] >= height:
			break
		i += deltai
		j += deltaj
	return tot

for i in range(MAX_I):
	for j in range(MAX_J):
		a = scan(i, j, 1, 0)
		b = scan(i, j, -1, 0)
		c = scan(i, j, 0, 1)
		d = scan(i, j, 0, -1)
		scenic_score[i][j] = a*b*c*d

print(max(map(max, scenic_score)))
