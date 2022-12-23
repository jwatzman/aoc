import sys

def propose(elves, stepnum, elf):
	(srcrow, srccol) = elf

	checks_n = [(srcrow-1, srccol), (srcrow-1, srccol-1), (srcrow-1, srccol+1)]
	checks_s = [(srcrow+1, srccol), (srcrow+1, srccol-1), (srcrow+1, srccol+1)]
	checks_w = [(srcrow, srccol-1), (srcrow-1, srccol-1), (srcrow+1, srccol-1)]
	checks_e = [(srcrow, srccol+1), (srcrow-1, srccol+1), (srcrow+1, srccol+1)]

	all_checks = set()
	all_checks.update(checks_n)
	all_checks.update(checks_s)
	all_checks.update(checks_w)
	all_checks.update(checks_e)
	all_results = map(lambda loc: loc in elves, all_checks)
	if not any(all_results):
		return elf

	for direc in range(4):
		direc = (direc + stepnum) % 4
		if direc == 0:
			checks = checks_n
		elif direc == 1:
			checks = checks_s
		elif direc == 2:
			checks = checks_w
		elif direc == 3:
			checks = checks_e

		results = map(lambda loc: loc in elves, checks)
		if not any(results):
			return checks[0]
		#else:
			#print(f"elf {elf} failed {results}")
	return elf


def step(elves, stepnum):
	proposals = dict()

	for elf in elves:
		proposal = propose(elves, stepnum, elf)
		#print(f"propose {elf} to {proposal}")
		if proposal in proposals:
			assert proposal != elf
			# Only two elves can ever collide
			elf_collision = proposals[proposal]
			del proposals[proposal]
			proposals[elf] = elf
			proposals[elf_collision] = elf_collision
		else:
			proposals[proposal] = elf

	assert len(proposals) == len(elves)
	return proposals.keys()

elves = set()
f = open(sys.argv[1], "r")
lines = f.readlines()

for row in range(len(lines)):
	line = lines[row].rstrip()
	for col in range(len(line)):
		c = line[col]
		if c == "#":
			elves.add((row, col))
		else:
			assert c == "."

for stepnum in range(10):
	elves = step(elves, stepnum)
#print(elves)

rows = set()
cols = set()
for (r, c) in elves:
	rows.add(r)
	cols.add(c)

min_row = min(rows)
max_row = max(rows)
min_col = min(cols)
max_col = max(cols)
#print(min_row, max_row, min_col, max_col)

bounding_rect = (max_row - min_row + 1) * (max_col - min_col + 1)
print(bounding_rect - len(elves))
