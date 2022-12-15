#!/usr/bin/env python3

import re
import sys

r = re.compile("Sensor at x=(.+), y=(.+): closest beacon is at x=(.+), y=(.+)")
targety = int(sys.argv[2])
bhitx = set()

hitranges = []

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

def addrange(l, r):
	for rng in hitranges:
		if mergeinto(rng, l, r):
			return
	hitranges.append(Rng(l, r))

def flattenranges(l):
	for rng1 in l:
		for rng2 in l:
			if rng1 == rng2:
				continue
			if mergeinto(rng1, rng2.l, rng2.r):
				rng2.l = 0
				rng2.r = 0

f = open(sys.argv[1], "r")
for line in f.readlines():
	m = r.match(line)
	sx = int(m.group(1))
	sy = int(m.group(2))
	bx = int(m.group(3))
	by = int(m.group(4))

	if by == targety:
		bhitx.add(bx)
	
	steps = abs(sx-bx) + abs(sy-by)
	if abs(sy-targety) <= steps:
		steps -= abs(sy-targety)
		addrange(sx-steps-1, sx+steps+1)

tot = 0
flattenranges(hitranges)
for rng in hitranges:
	tot += rng.r - rng.l
tot -= len(bhitx)

print(tot - 1)
