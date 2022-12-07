#!/usr/bin/env python3

import sys

class DNode:
	def __init__(self):
		self.children = {}
		self.files = {}
		self.size = None
		self.parent = None

def printdir(n, indent):
	for fname, fsize in n.files.items():
		print(indent + fname + ":" + str(fsize))
	for cname, child in n.children.items():
		print(indent + cname + ":")
		printdir(child, indent + "  ")
	print(indent + str(n.size))

root = DNode()
cur = root
allnodes = []
allnodes.append(root)

f = open(sys.argv[1], "r")
lines = f.readlines()
lines.pop(0)

for line in lines:
	split = line.rstrip().split(" ")
	if split[0] == "$":
		if split[1] == "ls":
			pass
		elif split[1] == "cd":
			if split[2] == "..":
				cur = cur.parent
			else:
				new = DNode()
				allnodes.append(new)
				new.parent = cur
				cur.children[split[2]] = new
				cur = new
		else:
			print("unknown command")
			sys.exit(1)
	else:
		if split[0] == "dir":
			pass
		else:
			cur.files[split[1]] = int(split[0])

def calcsize(n):
	tot = 0
	for _, child in n.children.items():
		calcsize(child)
		tot += child.size
	for _, fsize in n.files.items():
		tot += fsize
	n.size = tot

calcsize(root)
#printdir(root, "")

tot = 0
MAXSZ = 100000
for node in allnodes:
	if node.size < MAXSZ:
		tot += node.size

print(tot)
