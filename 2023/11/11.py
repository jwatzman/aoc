#!/usr/bin/env python3

import sys

def minmax(a, b):
	if a > b:
		return (b, a)
	else:
		return (a, b)

f = open(sys.argv[1], "r")
lines = f.readlines()

galaxies = []
empty_rows = set(range(len(lines)))
empty_cols = set(range(len(lines[0].strip())))
for i in range(len(lines)):
	line = lines[i]
	for j in range(len(line.strip())):
		if line[j] == "#":
			galaxies.append((i, j))
			if i in empty_rows:
				empty_rows.remove(i)
			if j in empty_cols:
				empty_cols.remove(j)

tot = 0
for i in range(len(galaxies)):
	for j in range(i+1, len(galaxies)):
		g1 = galaxies[i]
		g2 = galaxies[j]
		(rmin, rmax) = minmax(g1[0], g2[0])
		(cmin, cmax) = minmax(g1[1], g2[1])
		dist = 0
		for r in range(rmin, rmax):
			dist += 1
			if r in empty_rows:
				dist += 1
		for c in range(cmin, cmax):
			dist += 1
			if c in empty_cols:
				dist += 1
		tot += dist

print(tot)
