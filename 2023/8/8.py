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

f = open(sys.argv[1], "r")
lines = f.readlines()

path = list(lines.pop(0).strip())
lines.pop(0)

nodes = read_nodes(lines)

position = 'AAA'
steps = 0
for step in cycle(path):
	node = nodes[position]
	if step == "L":
		position = node[0]
	else:
		position = node[1]
	steps += 1
	if position == 'ZZZ':
		break
print(steps)
