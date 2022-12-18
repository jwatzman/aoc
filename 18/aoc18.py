#!/usr/bin/env python3

from dataclasses import dataclass
import sys

xs = []
ys = []
zs = []
rocks = set()
f = open(sys.argv[1], "r")
for line in f.readlines():
	coords = line.rstrip().split(",")
	(x,y,z) = map(int, coords)
	xs.append(x)
	ys.append(y)
	zs.append(z)
	rocks.add((x,y,z))

minx = min(xs) - 1
maxx = max(xs) + 1
miny = min(ys) - 1
maxy = max(ys) + 1
minz = min(zs) - 1
maxz = max(zs) + 1

@dataclass
class Void:
	x: int
	y: int
	z: int
	comp: int = None

def inclrange(l,r):
	return range(l, r+1)

voids = dict()
for x in inclrange(minx, maxx):
	for y in inclrange(miny, maxy):
		for z in inclrange(minz, maxz):
			if not (x,y,z) in rocks:
				voids[(x,y,z)] = Void(x,y,z)

bordercomp = None

def succs(x,y,z):
	return [(x+1,y,z), (x-1,y,z), (x,y+1,z), (x,y-1,z), (x,y,z+1), (x,y,z-1)]

def flood(void, ncomp):
	global voids
	global bordercomp
	if void.comp != None:
		return
	void.comp = ncomp
	for (sx,sy,sz) in succs(void.x, void.y, void.z):
		if sx < minx or sx > maxx or sy < miny or sy > maxy or sz < minz or sz > maxz:
			assert bordercomp == None or bordercomp == ncomp
			bordercomp = ncomp
		elif (sx,sy,sz) in voids:
			flood(voids[(sx,sy,sz)], ncomp)

ncomp = 1
sys.setrecursionlimit(len(voids) + 50)
for loc in voids:
	void = voids[loc]
	flood(void, ncomp)
	ncomp += 1

for loc in voids:
	void = voids[loc]
	assert void.comp != None
	if void.comp != bordercomp:
		rocks.add(loc)

adj = 0
for (x,y,z) in rocks:
	for succ in succs(x,y,z):
		if succ in rocks:
			adj += 1

print(6*len(rocks) - adj)
