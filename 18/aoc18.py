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
		if (x,y,z) in droplets:
			return 1
		x += dx
		y += dy
		z += dz
	return 0

def inclrange(l, r):
	return range(l, r+1)

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

hits = 0
for x in inclrange(minx, maxx):
	for y in inclrange(miny, maxy):
		hits += raycast(x,y,minz, 0,0,1)
		hits += raycast(x,y,maxz, 0,0,-1)

	for z in inclrange(minz, maxz):
		hits += raycast(x,miny,z, 0,1,0)
		hits += raycast(x,maxy,z, 0,-1,0)

for y in inclrange(miny, maxy):
	for z in inclrange(minz, maxz):
		hits += raycast(minx,y,z, 1,0,0)
		hits += raycast(maxx,y,z, -1,0,0)

print(hits)
