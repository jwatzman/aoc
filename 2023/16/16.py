#!/usr/bin/env python3

import sys

f = open(sys.argv[1], "r")
grid = list(map(lambda l: l.strip(), f.readlines()))

NROWS = len(grid)
NCOLS = len(grid[0])

def inbounds(beam):
	loc = beam[0]
	row = loc[0]
	col = loc[1]
	return row >= 0 and row < NROWS and col >= 0 and col < NCOLS

def add_beam(loc, direc):
	return (loc[0] + direc[0], loc[1] + direc[1])

beams = set()
beams.add(((0, 0), (0, 1)))
loop_detect = set()
energized = set()

while len(beams) > 0:
	new_beams = set()

	for beam in beams:
		loop_detect.add(beam)
		energized.add(beam[0])
		loc = beam[0]
		direc = beam[1]
		c = grid[loc[0]][loc[1]]
		if c == "." or (c == "|" and direc[1] == 0) or (c == "-" and direc[0] == 0):
			new_beams.add((add_beam(loc, direc), direc))
		elif c == "\\":
			new_direc = (direc[1], direc[0])
			new_beams.add((add_beam(loc, new_direc), new_direc))
		elif c == "/":
			new_direc = (-direc[1], -direc[0])
			new_beams.add((add_beam(loc, new_direc), new_direc))
		elif c == "|":
			new_beams.add(((loc[0] - 1, loc[1]), (-1, 0)))
			new_beams.add(((loc[0] + 1, loc[1]), (1, 0)))
		elif c == "-":
			new_beams.add(((loc[0], loc[1] - 1), (0, -1)))
			new_beams.add(((loc[0], loc[1] + 1), (0, 1)))
		else:
			assert False

	beams = set(filter(lambda beam: (beam not in loop_detect) and inbounds(beam), new_beams))

print(len(energized))
