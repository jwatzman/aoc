#!/usr/bin/env python3

import sys

class Point:
	def __init__(self, ix, iy):
		self.x = ix
		self.y = iy
	
	def __str__(self):
		return "(" + str(self.x) + "," + str(self.y) + ")"

head = Point(0,0)
tail = Point(0,0)
all_pos = []

def sign(x):
	if x > 0:
		return 1
	elif x < 0:
		return -1
	else:
		return 0

f = open(sys.argv[1], "r")
for line in f.readlines():
	dx = 0
	dy = 0
	spl = line.rstrip().split(" ")
	udlr = spl[0]
	if udlr == "U":
		dy = 1
	elif udlr == "D":
		dy = -1
	elif udlr == "L":
		dx = -1
	elif udlr == "R":
		dx = 1
	else:
		print("unknown direction " + udlr)
		sys.exit(1)

	for _ in range(int(spl[1])):
		head.x += dx
		head.y += dy

		if abs(head.x-tail.x) > 1 or abs(head.y-tail.y) > 1:
			tail.x += sign(head.x-tail.x)
			tail.y += sign(head.y-tail.y)
		all_pos.append(str(tail))

print(len(set(all_pos)))
