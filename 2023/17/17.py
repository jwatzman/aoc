#!/usr/bin/env python3

from dataclasses import dataclass
from heapq import heappush, heappop
import itertools
import sys

# https://docs.python.org/3/library/heapq.html#priority-queue-implementation-notes
class PriQ:
	def __init__(self):
		self.pq = []
		self.entry_finder = {}
		self.REMOVED = '<removed-task>'
		self.counter = itertools.count()

	def add_task(self, task, priority):
		if task in self.entry_finder:
			if self.entry_finder[task][0] <= priority:
				return
			self.remove_task(task)
		count = next(self.counter)
		entry = [priority, count, task]
		self.entry_finder[task] = entry
		heappush(self.pq, entry)

	def remove_task(self, task):
		entry = self.entry_finder.pop(task)
		entry[-1] = self.REMOVED

	def pop_task(self):
		while self.pq:
			priority, count, task = heappop(self.pq)
			if task is not self.REMOVED:
				del self.entry_finder[task]
				return (priority, task)
		raise KeyError('pop from an empty priority queue')

@dataclass(frozen=True)
class State:
	pos: (int, int)
	direction: (int, int)
	consecutive: int

def add_pos(p1, p2):
	return (p1[0] + p2[0], p1[1] + p2[1])

def succ(s):
	res = []
	p0 = s.pos
	d0 = s.direction
	d1 = (-d0[1], d0[0])
	d2 = (d0[1], -d0[0])
	res.append(State(add_pos(p0, d1), d1, 1))
	res.append(State(add_pos(p0, d2), d2, 1))
	if (s.consecutive < 3):
		res.append(State(add_pos(p0, d0), d0, s.consecutive + 1))
	return res

f = open(sys.argv[1], "r")
heatmap = list(map(lambda l: list(map(int, l.strip())), f.readlines()))
NROWS = len(heatmap)
NCOLS = len(heatmap[0])

visited = set()
q = PriQ()
q.add_task(State((0,0), (0,1), 0), 0)
q.add_task(State((0,0), (1,0), 0), 0)
while True:
	(cur_heat, cur_state) = q.pop_task()
	if cur_state.pos[0] == NROWS - 1 and cur_state.pos[1] == NCOLS - 1:
		print(cur_heat)
		sys.exit(0)
	visited.add(cur_state)
	maybe_next = succ(cur_state)
	for s in maybe_next:
		if s.pos[0] >= 0 and s.pos[0] < NROWS and s.pos[1] >=0 and s.pos[1] < NCOLS:
			if s not in visited:
				q.add_task(s, cur_heat + heatmap[s.pos[0]][s.pos[1]])
