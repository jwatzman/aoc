#!/usr/bin/env python3

import sys

def read_walls():
	walls = set()
	p = complex(0,0)

	f = open(sys.argv[1], "r")
	for line in f.readlines():
		spl = line.split(" ")
		amt = int(spl[2][2:7], 16)
		direc_letter = spl[2][7]
		if direc_letter == "3":
			direc = complex(-1,0)
		elif direc_letter == "1":
			direc = complex(1,0)
		elif direc_letter == "2":
			direc = complex(0,-1)
		elif direc_letter == "0":
			direc = complex(0,1)
		else:
			assert False
		for i in range(amt):
			p += direc
			walls.add(p)

	assert p.real == 0
	assert p.imag == 0
	print("Walls computed")
	return walls

def scan_all(walls):
	rows = list(map(lambda c: int(c.real), walls))
	cols = list(map(lambda c: int(c.imag), walls))
	minrow = min(rows)
	maxrow = max(rows)
	mincol = min(cols)
	maxcol = max(cols)
	print(minrow, maxrow)

	tot = 0
	for row in range(minrow, maxrow+1):
		if row % 10000 == 0:
			print(row)
		wall_from_above = None
		wall_thickness = 0
		in_trench = False
		for col in range(mincol, maxcol+1):
			p = complex(row, col)
			if p in walls:
				tot += 1
				wall_thickness += 1
				if wall_from_above == None:
					wall_from_above = (complex(row-1, col) in walls)
			else:
				if wall_thickness == 1:
					in_trench = not in_trench
				elif wall_from_above != None:
					wall_ends_above = (complex(row-1,col-1) in walls)
					if wall_from_above != wall_ends_above:
						in_trench = not in_trench
				wall_thickness = 0
				wall_from_above = None
				if in_trench:
					tot += 1

	return tot

print(scan_all(read_walls()))
