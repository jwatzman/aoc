#!/usr/bin/env python3

import sys

def score(c):
	o = ord(c)
	if (o >= 65 and o <= 90):
		return o - 65 + 27
	elif (o >= 97 and o <= 122):
		return o - 97 + 1
	else:
		raise Exception("invalid char")

f = open(sys.argv[1], "r")
sc = 0
for line in f.readlines():
	line = line.rstrip()
	l = len(line) // 2
	h1 = line[l:]
	h2 = line[:l]
	s = set(h1)
	for c in list(h2):
		if c in s:
			sc += score(c)
			break
print(sc)
