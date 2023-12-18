#!/usr/bin/env python3

from collections import defaultdict
import sys

def read_walls():
	walls = defaultdict(set)
	p = complex(0,0)
	minrow = 0
	maxrow = 0
	mincol = 0
	maxcol = 0

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
			walls[int(p.real)].add(int(p.imag))
		if p.real > maxrow:
			maxrow = int(p.real)
		if p.real < minrow:
			minrow = int(p.real)
		if p.imag > maxcol:
			maxcol = int(p.imag)
		if p.imag < mincol:
			mincol = int(p.imag)

	assert p.real == 0
	assert p.imag == 0
	print("Walls computed")
	return (walls, minrow, maxrow, mincol, maxcol)

def scan_all(tpl):
	(walls, minrow, maxrow, mincol, maxcol) = tpl
	print(minrow, maxrow)

	tot = 0
	for row in range(minrow, maxrow+1):
		if row % 100 == 0:
			print(row / maxrow * 100)
		wall_from_above = None
		wall_thickness = 0
		in_trench = False
		cur_wall = walls[row]
		prev_wall = walls[row-1]
		for col in range(mincol, maxcol+1):
			if col in cur_wall:
				tot += 1
				wall_thickness += 1
				if wall_from_above == None:
					wall_from_above = (col in prev_wall)
			else:
				if wall_thickness == 1:
					in_trench = not in_trench
				elif wall_from_above != None:
					wall_ends_above = ((col-1) in prev_wall)
					if wall_from_above != wall_ends_above:
						in_trench = not in_trench
				wall_thickness = 0
				wall_from_above = None
				if in_trench:
					tot += 1

	return tot

print(scan_all(read_walls()))
