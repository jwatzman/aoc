#!/usr/bin/env python3

import sys

f = open(sys.argv[1], "r")
lines = f.readlines()
linesep = 0
while lines[linesep] != "\n":
	linesep += 1

stacks = []
numstacks = len(lines[linesep - 2].split(" "))
i = 0
while i < numstacks:
	stacks.append([])
	i += 1

lineno = linesep - 2
while lineno >= 0:
	line = lines[lineno]
	cno = 1
	stackno = 0
	while cno < len(line):
		c = line[cno]
		if c != " ":
			stacks[stackno].append(c)
		cno += 4
		stackno += 1
	lineno -= 1

lineno = linesep + 1
while lineno < len(lines):
	instr = lines[lineno].split(" ")
	howmany = int(instr[1])
	src = int(instr[3]) - 1
	dest = int(instr[5]) - 1
	while howmany > 0:
		stacks[dest].append(stacks[src].pop())
		howmany -= 1
	lineno += 1

output = ""
i = 0
while i < numstacks:
	output += stacks[i].pop()
	i += 1
print(output)
