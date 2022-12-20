import sys

class Node:
	def __init__(self, val):
		self.val = val
		self.orig_prev = None
		self.orig_next = None
		self.adj_prev = None
		self.adj_next = None

def advance_adj(cur, amt):
	assert amt > 0
	while amt > 0:
		cur = cur.adj_next
		amt -= 1
	return cur

def print_adj(start):
	cur = start
	while True:
		print(cur.val)
		next = cur.adj_next
		assert next.adj_prev == cur
		if next == start:
			return
		cur = next

def mix(cur):
	global tot
	while cur != None:
		amt = cur.val % (tot - 1)
		#if cur.val < 0:
			#amt -= 1
		assert amt >= 0 and amt < tot
		if amt > 0:
			#print("moving", cur)
			target = advance_adj(cur, amt)
			assert target != cur

			cur.adj_prev.adj_next = cur.adj_next
			cur.adj_next.adj_prev = cur.adj_prev

			cur.adj_prev = target
			cur.adj_next = target.adj_next
			target.adj_next = cur
			cur.adj_next.adj_prev = cur
		cur = cur.orig_next

head = None
cur = None
znode = None
tot = 0

f = open(sys.argv[1], "r")
for line in f.readlines():
	n = Node(int(line.rstrip()) * 811589153)
	if head == None:
		head = n
	else:
		assert cur != None
		cur.orig_next = n
		cur.adj_next = n
		n.orig_prev = cur
		n.adj_prev = cur
	cur = n
	tot += 1
	if cur.val == 0:
		znode = cur

cur.adj_next = head
head.adj_prev = cur

assert znode != None

for _ in range(10):
	mix(head)

#print_adj(znode)

answer = 0
final_move = 1000 % tot
cur = advance_adj(znode, final_move)
answer += cur.val
cur = advance_adj(cur, final_move)
answer += cur.val
cur = advance_adj(cur, final_move)
answer += cur.val
print(answer)
