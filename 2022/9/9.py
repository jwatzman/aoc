#!/usr/bin/env python3

import sys

class Point:
	def __init__(self, ix, iy):
		self.x = ix
		self.y = iy
	
	def __str__(self):
		return "(" + str(self.x) + "," + str(self.y) + ")"

NTAILS = 9
LENROPE = NTAILS + 1
rope = []
for _ in range(LENROPE):
	rope.append(Point(0,0))

all_pos = [str(rope[NTAILS])]

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
		rope[0].x += dx
		rope[0].y += dy

		for i in range(1, LENROPE):
			head = rope[i-1]
			tail = rope[i]
			if abs(head.x-tail.x) > 1 or abs(head.y-tail.y) > 1:
				tail.x += sign(head.x-tail.x)
				tail.y += sign(head.y-tail.y)
		all_pos.append(str(rope[NTAILS]))

print(len(set(all_pos)))
