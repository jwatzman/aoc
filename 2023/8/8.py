#!/usr/bin/env python3

from itertools import cycle
import re
import sys

def read_nodes(lines):
	r = re.compile("(.*) = \((.*), (.*)\)")
	res = dict()
	while len(lines) > 0:
		line = lines.pop(0).strip()
		m = r.match(line)
		res[m.group(1)] = (m.group(2), m.group(3))
	return res

def is_end(positions):
	for position in positions:
		if position[2] != 'Z':
			return False
	return True

f = open(sys.argv[1], "r")
lines = f.readlines()

path = list(lines.pop(0).strip())
lines.pop(0)

nodes = read_nodes(lines)

positions = []
for node in nodes.keys():
	if node[2] == 'A':
		positions.append(node)

print(positions)
steps = 0
for step in cycle(path):
	new_positions = []
	for position in positions:
		node = nodes[position]
		if step == "L":
			new_positions.append(node[0])
		else:
			new_positions.append(node[1])
	positions = new_positions
	steps += 1
	if is_end(positions):
		break
print(steps)
