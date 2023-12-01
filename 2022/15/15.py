#!/usr/bin/env python3

from collections import defaultdict
import re
import sys

r = re.compile("Sensor at x=(.+), y=(.+): closest beacon is at x=(.+), y=(.+)")

allhitranges = defaultdict(list)

class Rng:
	def __init__(self, l, r):
		self.l = l
		self.r = r

	def __str__(self):
		return "range(" + str(self.l) + "," + str(self.r) + ")"

def mergeinto(rng, l, r):
	if l >= rng.l and r <= rng.r:
		return True
	if l < rng.l and r <= rng.r and r >= rng.l:
		rng.l = l
		return True
	if l > rng.l and r >= rng.r and l <= rng.r:
		rng.r = r
		return True
	if l < rng.l and r > rng.r:
		rng.l = l
		rng.r = r
		return True
	return False

def addrange(hitranges, l, r):
	for rng in hitranges:
		if mergeinto(rng, l, r):
			return
	hitranges.append(Rng(l, r))

def flattenranges(hitranges):
	for rng1 in hitranges:
		for rng2 in hitranges:
			if rng1 == rng2:
				continue
			if mergeinto(rng1, rng2.l, rng2.r):
				rng2.l = 0
				rng2.r = 0

f = open(sys.argv[1], "r")
for line in f.readlines():
	print(line.rstrip())
	m = r.match(line)
	sx = int(m.group(1))
	sy = int(m.group(2))
	bx = int(m.group(3))
	by = int(m.group(4))

	steps = abs(sx-bx) + abs(sy-by)
	for stepy in range(steps+1):
		stepx = steps - stepy
		addrange(allhitranges[sy+stepy], sx-stepx, sx+stepx)
		addrange(allhitranges[sy-stepy], sx-stepx, sx+stepx)

print("Done processing.")

end = int(sys.argv[2])
for i in range(end):
	if i % 10000 == 0:
		print("Processing y=", i)
	hitranges = allhitranges[i]
	flattenranges(hitranges)
	for hitrange in hitranges:
		if hitrange.l < 0 and hitrange.r < end:
			print("GAP FOUND AT y=", i, hitrange)
			for hitrange2 in hitranges:
				print(hitrange2)
			print((hitrange.r+1)*4000000+i)
			sys.exit(0)
