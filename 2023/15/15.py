#!/usr/bin/env python3

import re
import sys

def hashalgo(s):
	cv = 0
	for c in s:
		cv += ord(c)
		cv *= 17
		cv %= 256
	return cv

f = open(sys.argv[1], "r")
line = f.readline().strip()

boxes = []
for i in range(256):
	boxes.append([])

r = re.compile("(.+)(-|=)(.*)")
def run_step(step):
	m = r.match(step)
	label = m.group(1)
	box = boxes[hashalgo(label)]
	command = m.group(2)
	if command == "-":
		for i in range(len(box)):
			if box[i][0] == label:
				box.pop(i)
				return
	elif command == "=":
		focallength = int(m.group(3))
		for i in range(len(box)):
			if box[i][0] == label:
				box[i] = (label, focallength)
				return
		box.append((label, focallength))

for step in line.split(","):
	run_step(step)

tot = 0
for i in range(len(boxes)):
	box = boxes[i]
	for j in range(len(box)):
		tot += (i + 1) * (j + 1) * box[j][1]
print(tot)
