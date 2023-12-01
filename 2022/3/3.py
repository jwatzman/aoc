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
lines = f.readlines()
sc = 0
i = 0
while i < len(lines):
	l1 = lines[i].rstrip()
	l2 = lines[i+1].rstrip()
	l3 = lines[i+2].rstrip()
	s = set(l1).intersection(set(l2)).intersection(set(l3))
	if (len(s) != 1):
		raise Exception("too many matches")
	for x in s:
		sc += score(x)
	i += 3
print(sc)
