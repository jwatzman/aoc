#!/usr/bin/env python3

import sys

droplets = set()

def checkadj(x, y, z, dx, dy, dz):
	if ((x+dx), (y+dy), (z+dz)) in droplets:
		return 1
	else:
		return 0

f = open(sys.argv[1], "r")
for line in f.readlines():
	coords = line.rstrip().split(",")
	coords = list(map(int, coords))
	droplets.add((coords[0], coords[1], coords[2]))

adj = 0
for (x,y,z) in droplets:
	adj += checkadj(x,y,z, 1,0,0)
	adj += checkadj(x,y,z, -1,0,0)
	adj += checkadj(x,y,z, 0,1,0)
	adj += checkadj(x,y,z, 0,-1,0)
	adj += checkadj(x,y,z, 0,0,1)
	adj += checkadj(x,y,z, 0,0,-1)

print(6*len(droplets) - adj)
