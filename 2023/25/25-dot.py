#!/usr/bin/env python3

import sys

f = open(sys.argv[1], "r")

print("strict graph {")
for line in f.readlines():
	node, adjs = line.strip().split(":")
	for adj in adjs.strip().split(" "):
		print("  %s -- %s" % (node, adj))
print("}")
