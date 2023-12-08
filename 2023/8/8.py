#!/usr/bin/env python3

from functools import reduce
from itertools import cycle
from math import lcm
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
cycle_steps = []
for position in positions:
	cur = position
	steps = 0
	for step in cycle(path):
		node = nodes[cur]
		if step == "L":
			cur = node[0]
		else:
			cur = node[1]
		steps += 1
		if cur[2] == 'Z':
			break
	cycle_steps.append(steps)
print(cycle_steps)
print(reduce(lambda x,y: lcm(x,y), cycle_steps, 1))
