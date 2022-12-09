#!/usr/bin/env python3

import sys

hx = 0
hy = 0
tx = 0
ty = 0
all_pos = []

def printp(x,y):
	print("(" + str(x) + "," + str(y) + ")")

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
		hx += dx
		hy += dy

		if abs(hx-tx) > 1 or abs(hy-ty) > 1:
			tx += sign(hx-tx)
			ty += sign(hy-ty)
		all_pos.append(str(tx) + "," + str(ty))

#printp(hx,hy)
#printp(tx,ty)
print(len(set(all_pos)))
