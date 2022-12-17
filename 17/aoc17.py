#!/usr/bin/env python3

from collections import defaultdict
import sys

WIDTH = 7

def emptyrow():
	return [False] * 7

class Chamber:
	def __init__(self):
		self.rows = defaultdict(emptyrow)

	def occupied(self, x, y):
		if x < 0 or x >= WIDTH or y < 0:
			return True
		return self.rows[y][x]

	def occupy(self, x, y):
		#assert self.occupied(x, y) == False
		self.rows[y][x] = True
		yy = y - 1000
		if yy in self.rows:
			del self.rows[yy]

	def height(self):
		l = max(self.rows.keys(), default=0)
		for y in reversed(range(l + 1)):
			if any(self.rows[y]):
				return y + 1
		return 0

	def print(self):
		for y in reversed(range(self.height())):
			print("|", end="")
			for x in range(WIDTH):
				if self.occupied(x, y):
					print("#", end="")
				else:
					print(".", end="")
			print("|")
		print("+-------+")

class Rock:
	def __init__(self, chamber, coords):
		self.chamber = chamber
		self.coords = coords

	def maybemove(self, dx, dy):
		newcoords = []
		for (x, y) in self.coords:
			x += dx
			y += dy
			if self.chamber.occupied(x, y):
				return False
			newcoords.append((x, y))
		self.coords = newcoords
		return True

	def descent(self, gJets):
		while True:
			self.maybemove(next(gJets), 0)
			if not self.maybemove(0, -1):
				self.fix()
				return

	def fix(self):
		for (x, y) in self.coords:
			self.chamber.occupy(x, y)
		self.coords = None

def genRockCoords():
	i = 0
	while True:
		match i:
			case 0:
				yield [(0,0), (1,0), (2,0), (3,0)]
			case 1:
				yield [(1,0), (0,1), (1,1), (2,1), (1,2)]
			case 2:
				yield [(0,0), (1,0), (2,0), (2,1), (2,2)]
			case 3:
				yield [(0,0), (0,1), (0,2), (0,3)]
			case 4:
				yield [(0,0), (1,0), (0,1), (1,1)]
			case _:
				assert False
		i += 1
		i %= 5

def genJets(line):
	i = 0
	l = len(line)
	while True:
		c = line[i]
		if c == "<":
			yield -1
		elif c == ">":
			yield 1
		else:
			assert False
		i += 1
		i %= l

f = open(sys.argv[1], "r")
gJets = genJets(f.read().rstrip())
gRockCoords = genRockCoords()

c = Chamber()
ITERS = int(sys.argv[2])
for i in range(ITERS):
	if i % 100000 == 0:
		print(i)
	h = c.height()
	r = Rock(c, next(gRockCoords))
	assert r.maybemove(2, h + 3) == True
	r.descent(gJets)
	#c.print()
print(c.height())
