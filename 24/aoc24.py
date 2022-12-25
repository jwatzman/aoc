from dataclasses import dataclass, field
import sys
from typing import List

@dataclass
class Blizzard:
	row: int
	col: int
	drow: int
	dcol: int

	def advance(self, numrows, numcols):
		self.row = (self.row + self.drow) % numrows
		self.col = (self.col + self.dcol) % numcols
		return (self.row, self.col)

@dataclass
class Grid:
	numrows: int
	numcols: int
	blizzards: List[Blizzard] = field(default_factory=lambda: [])

	def advance(self):
		s = set()
		for blizzard in self.blizzards:
			s.add(blizzard.advance(self.numrows, self.numcols))
		return s
	
	def print(self):
		occ = set()
		for blizzard in self.blizzards:
			occ.add((blizzard.row, blizzard.col))
		for row in range(self.numrows):
			for col in range(self.numcols):
				if (row, col) in occ:
					print("X", end="")
				else:
					print(".", end="")
			print()

f = open(sys.argv[1], "r")
lines = f.readlines()
grid = Grid(len(lines) - 2, len(lines[0].rstrip()) - 2)

for row in range(0, grid.numrows):
	for col in range(0, grid.numcols):
		c = lines[row + 1][col + 1] # skip borders
		if c == ".":
			pass
		elif c == "^":
			grid.blizzards.append(Blizzard(row, col, -1, 0))
		elif c == "v":
			grid.blizzards.append(Blizzard(row, col, 1, 0))
		elif c == "<":
			grid.blizzards.append(Blizzard(row, col, 0, -1))
		elif c == ">":
			grid.blizzards.append(Blizzard(row, col, 0, 1))
		else:
			assert False

def wander(grid):
	playerlocs = set()
	dest = (grid.numrows - 1, grid.numcols - 1)
	time = 0
	while True:
		#print(f"Computing {time}...")
		newplayerlocs = set()
		invalid = grid.advance()
		if (0,0) not in invalid:
			playerlocs.add((0,0)) # (implicitly) waited until now to step out into the fray
		for loc in playerlocs:
			if loc == dest:
				return time + 1 # last step is always out of the main grid
			(row, col) = loc
			for drow in [-1, 0, 1]:
				for dcol in [-1, 0, 1]:
					if abs(drow) + abs(dcol) > 1:
						continue
					mayberow = row + drow
					maybecol = col + dcol
					if mayberow < 0 or mayberow >= grid.numrows or maybecol < 0 or maybecol >= grid.numcols:
						continue
					if (mayberow, maybecol) not in invalid:
						newplayerlocs.add((mayberow, maybecol))
		playerlocs = newplayerlocs
		time += 1

print(wander(grid))
