#!/usr/bin/env python3

import re
import sys

EPS = 0.00001

point_re = re.compile("(.+), +(.+), +(.+) +@ +(.+), +(.+), +(.+)")
def parse_point(line):
	m = point_re.match(line)
	g = list(map(float, m.groups()))
	return ((g[0], g[1], g[2]), (g[3], g[4], g[5]))

def read_points():
	f = open(sys.argv[1], "r")
	r = []
	for line in f.readlines():
		r.append(parse_point(line))
	return r

def find_intersection(p1, p2):
	((x1, y1, z1), (vx1, vy1, vz1)) = p1
	((x2, y2, z2), (vx2, vy2, vz2)) = p2
	m1 = vy1/vx1
	m2 = vy2/vx2
	if abs(m1-m2) < EPS:
		return None
	b1 = -m1*x1+y1
	b2 = -m2*x2+y2
	xx = (b2-b1)/(m1-m2)
	yy1 = m1*xx+b1
	yy2 = m2*xx+b2
	#assert abs(yy1-yy2) < EPS
	#if abs(yy1-yy2) > EPS:
		#print(abs(yy1-yy2))
	return (xx, yy1)

def is_in_future(p, i):
	a = (i[0] - p[0][0]) > 0
	b = p[1][0] > 0
	return a == b

points = read_points()
BOUND_MIN = float(sys.argv[2])
BOUND_MAX = float(sys.argv[3])

tot = 0
for i in range(len(points)):
	for j in range(i + 1, len(points)):
		p1 = points[i]
		p2 = points[j]
		inter = find_intersection(p1, p2)
		if inter == None:
			continue
		if inter[0] < BOUND_MIN or inter[0] > BOUND_MAX:
			continue
		if inter[1] < BOUND_MIN or inter[1] > BOUND_MAX:
			continue
		if not is_in_future(p1, inter):
			continue
		if not is_in_future(p2, inter):
			continue
		tot += 1

print(tot)
