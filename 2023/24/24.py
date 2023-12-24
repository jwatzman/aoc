#!/usr/bin/env python3

import re
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

eqns = []
vars = ["x", "y", "z", "vx", "vy", "vz"]
for i in range(len(points)):
	p = points[i]
	ii = i + 1
	print("x%d = %d" % (ii, p[0][0]))
	print("y%d = %d" % (ii, p[0][1]))
	print("z%d = %d" % (ii, p[0][2]))
	print("vx%d = %d" % (ii, p[1][0]))
	print("vy%d = %d" % (ii, p[1][1]))
	print("vz%d = %d" % (ii, p[1][2]))
	vars.append("t%d" % (ii))
	eqns.append("x + vx*t%d == x%d + vx%d*t%d" % (ii, ii, ii, ii))
	eqns.append("y + vy*t%d == y%d + vy%d*t%d" % (ii, ii, ii, ii))
	eqns.append("z + vz*t%d == z%d + vz%d*t%d" % (ii, ii, ii, ii))

print("Solve[{%s}, {%s}]" % (", ".join(eqns), ", ".join(vars)))
