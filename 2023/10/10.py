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
pipes = list(map(lambda l: l.strip(), f.readlines()))

if sys.argv[1] == "example.txt":
	direc = (0, 1)
elif sys.argv[1] == "input.txt":
	direc = (1, 0)
else:
	assert False

start = find_start(pipes)
pos = start
steps = 0
while True:
	#print(pos, direc, pipes[pos[0]][pos[1]])
	(pos, direc) = run_step(pipes, pos, direc)
	steps += 1
	if pos == start:
		break

print(steps)
print(ceil(steps/2))
