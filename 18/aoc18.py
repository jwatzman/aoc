#!/usr/bin/env python3

import sys

droplets = set()
minx = None
maxx = None
miny = None
maxy = None
minz = None
maxz = None

def raycast(x, y, z, dx, dy, dz):
	while x >= minx and x <= maxx and y >= miny and y <= maxy and z >= minz and z <= maxz:
		x += dx
		y += dy
		z += dz
		if (x,y,z) in droplets:
			return 1
	return 0

f = open(sys.argv[1], "r")
for line in f.readlines():
	coords = line.rstrip().split(",")
	coords = list(map(int, coords))
	(x,y,z) = coords
	droplets.add((x,y,z))
	if minx == None or x < minx:
		minx = x
	if miny == None or y < miny:
		miny = y
	if minz == None or z < minz:
		minz = z

	if maxx == None or x > maxx:
		maxx = x
	if maxy == None or y > maxy:
		maxy = y
	if maxz == None or z > maxz:
		maxz = z

adj = 0
for (x,y,z) in droplets:
	adj += raycast(x,y,z, 1,0,0)
	adj += raycast(x,y,z, -1,0,0)
	adj += raycast(x,y,z, 0,1,0)
	adj += raycast(x,y,z, 0,-1,0)
	adj += raycast(x,y,z, 0,0,1)
	adj += raycast(x,y,z, 0,0,-1)

print(minx, maxx)
print(miny, maxy)
print(minz, maxz)
print(adj)
print(6*len(droplets) - adj)
