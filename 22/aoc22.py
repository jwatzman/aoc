import sys

def move(grid, pos, dpos):
	(row, col) = pos
	(drow, dcol) = dpos
	assert abs(drow) + abs(dcol) == 1
	newrow = row
	newcol = col
	lengrid = len(grid)
	c = " "
	while c == " ":
		newrow += drow
		newcol += dcol

		if newrow < 0:
			newrow += lengrid
		elif newrow >= lengrid:
			newrow -= lengrid
		assert newrow >= 0 and newrow < lengrid

		lenrow = len(grid[newrow])
		if newcol < 0:
			newcol += lenrow
		elif newcol >= lenrow:
			if dcol == 0:
				c = " "
				continue
			newcol -= lenrow
		assert newcol >= 0 and newcol < lenrow

		try:
			c = grid[newrow][newcol]
		except:
			c = " "
	if c == "#":
		return pos
	else:
		assert c == "."
		return (newrow, newcol)

def split_instrs(s):
	res = []
	accum = 0
	for c in s:
		if c == "L" or c == "R":
			assert accum > 0
			res.append(accum)
			res.append(c)
			accum = 0
		else:
			accum *= 10
			accum += int(c)
	assert accum > 0
	res.append(accum)
	return res

f = open(sys.argv[1], "r")
grid = list(map(lambda l: l.rstrip(), f.readlines()))
instrs = split_instrs(grid.pop())
grid.pop()

pos = (0,0)
drow = 0
dcol = 1
move(grid, pos, (drow, dcol))

for instr in instrs:
	if instr == "R":
		(drow, dcol) = (dcol, -drow)
	elif instr == "L":
		(drow, dcol) = (-dcol, drow)
	else:
		for _ in range(instr):
			pos = move(grid, pos, (drow, dcol))

print(pos)
print(drow, dcol)

password = 1000 * (pos[0] + 1) + 4 * (pos[1] + 1)
if dcol == 1: # right
	pass
elif dcol == -1: # left
	password += 2
elif drow == 1: # down
	password += 1
elif drow == -1: # up
	password += 3
else:
	assert False

print(password)
