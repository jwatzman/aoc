#!/usr/bin/env python3

from collections import defaultdict
from itertools import pairwise, zip_longest, repeat
import sys

class SandMap:
	def __init__(self):
		self.maxy = None
		self.dict = defaultdict(set)

	def block(self, x, y):
		if self.maxy is None or y > self.maxy:
			self.maxy = y
		self.dict[x].add(y)

	def contains(self, x, y):
		return y in self.dict[x]

	def drop(self, x, y):
		assert self.maxy != None
		assert not self.contains(x, y)
		if y == self.maxy + 1:
			self.dict[x].add(y)
		elif not self.contains(x, y+1):
			self.drop(x, y+1)
		elif not self.contains(x-1, y+1):
			self.drop(x-1, y+1)
		elif not self.contains(x+1, y+1):
			self.drop(x+1, y+1)
		else:
			self.dict[x].add(y)

def all_points(x1, y1, x2, y2):
	r = []
	if x1 == x2:
		for y in range(min(y1,y2), max(y1,y2)):
			r.append((x1, y))
	elif y1 == y2:
		for x in range(min(x1,x2), max(x1,x2)):
			r.append((x, y1))
	else:
		print("Diagonal line")
		sys.exit(1)
	return r

def block_all(m, ps):
	for ((x1, y1), (x2, y2)) in pairwise(ps):
		m.block(x1, y1)
		m.block(x2, y2)
		for (x,y) in all_points(x1,y1,x2,y2):
			m.block(x,y)

def str_to_point(s):
	l = s.split(",")
	return (int(l[0]), int(l[1]))

m = SandMap()
f = open(sys.argv[1], "r")
for line in f.readlines():
	spoints = line.split(" -> ")
	points = map(str_to_point, spoints)
	block_all(m, points)

dropped = 0
while not m.contains(500,0):
	m.drop(500,0)
	dropped += 1
print(dropped)
