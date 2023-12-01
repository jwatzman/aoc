#!/usr/bin/env python3

import numpy as np
import open3d as o3d
import sys

pts = []
f = open(sys.argv[1], "r")
for line in f.readlines():
	coords = line.rstrip().split(",")
	pts.append(tuple(map(int, coords)))

npts = np.vstack(pts)
pcd = o3d.geometry.PointCloud()
pcd.points = o3d.utility.Vector3dVector(npts)
o3d.visualization.draw_geometries([pcd])
