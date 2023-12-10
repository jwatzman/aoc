#!/usr/bin/env python3

from math import ceil
import sys

def find_start(pipes):
	for i in range(len(pipes)):
		for j in range(len(pipes[i])):
			if pipes[i][j] == "S":
				return (i,j)
	assert False

def run_step(pipes, pos, direc):
	pos = (pos[0] + direc[0], pos[1] + direc[1])
	pipe = pipes[pos[0]][pos[1]]
	assert pipe != "."
	if pipe == "J" or pipe == "F":
		direc = (-direc[1], -direc[0])
	elif pipe == "7" or pipe == "L":
		direc = (direc[1], direc[0])
	return (pos, direc)

f = open(sys.argv[1], "r")
pipes = list(map(lambda l: list(l.strip()), f.readlines()))

start = find_start(pipes)

if sys.argv[1] == "example.txt":
	direc = (0, 1)
	pipes[start[0]][start[1]] = "F"
elif sys.argv[1] == "input.txt":
	direc = (1, 0)
	pipes[start[0]][start[1]] = "|"
else:
	assert False

walls = dict()
pos = start
while True:
	walls[pos] = pipes[pos[0]][pos[1]]
	(pos, direc) = run_step(pipes, pos, direc)
	if pos == start:
		break

inside = set()
outside = set()
for i in range(len(pipes)):
	is_inside = False
	prev = None
	for j in range(len(pipes[i])):
		pos = (i, j)
		if pos in walls:
			pipe = walls[pos]
			if pipe == "-":
				assert prev != None
			elif pipe == "|":
				assert prev == None
				is_inside = not is_inside
			elif prev == None:
				prev = pipe
			elif ((prev, pipe) == ("7", "L")) or ((prev, pipe) == ("L", "7")) or ((prev, pipe) == ("F", "J")) or ((prev, pipe) == ("J", "F")):
				prev = None
				is_inside = not is_inside
			else:
				prev = None
		elif is_inside:
			inside.add(pos)
			assert prev == None
		else:
			outside.add(pos)
			assert prev == None
	assert prev == None
	assert not is_inside

print(len(inside))
