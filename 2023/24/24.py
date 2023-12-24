#!/usr/bin/env python3

import re
from statistics import mean
import sys

EPS = 0.00001

point_re = re.compile("(.+), +(.+), +(.+) +@ +(.+), +(.+), +(.+)")
def parse_point(line):
	m = point_re.match(line)
	g = list(map(int, m.groups()))
	return ((g[0], g[1], g[2]), (g[3], g[4], g[5]))

def read_points():
	f = open(sys.argv[1], "r")
	r = []
	for line in f.readlines():
		r.append(parse_point(line))
	return r

points = read_points()

print("clear")
print("function a = a (p)")
for i in range(len(points)):
	p = points[i]
	ii = i + 1
	nn = (i * 3) + 1
	print("  a(%d) = p(1) + p(4)*p(%d) - (%d + %d*p(%d));" % (nn,     ii + 6, p[0][0], p[1][0], ii + 6))
	print("  a(%d) = p(2) + p(5)*p(%d) - (%d + %d*p(%d));" % (nn + 1, ii + 6, p[0][1], p[1][1], ii + 6))
	print("  a(%d) = p(3) + p(6)*p(%d) - (%d + %d*p(%d));" % (nn + 2, ii + 6, p[0][2], p[1][2], ii + 6))
print("endfunction")

#print("options = optimset(\"MaxIter\", 100000000, \"MaxFunEvals\", 100000000, \"AutoScaling\", 1);")
print("options = optimset(\"TolX\", 0.0000000000000001);")

avgx = mean(map(lambda p: p[0][0], points))
avgy = mean(map(lambda p: p[0][1], points))
avgz = mean(map(lambda p: p[0][2], points))

guesses = [avgx, avgy, avgz]
guesses.extend([1]*3)
guesses.extend([1]*len(points))
print("[aa, fvec, info] = fsolve(\"a\", [%s], options);" % (";".join(map(str, guesses))))
print("format long")
print("disp(info)")
print("disp(aa(1:3))")
