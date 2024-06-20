"""
# Definition for a Node.
class Node:
    def __init__(self, val: int = 0, left: 'Node' = None, right: 'Node' = None, next: 'Node' = None):
        self.val = val
        self.left = left
        self.right = right
        self.next = next
"""


from collections import deque 

class Solution:
    def connect(self, root: 'Optional[Node]') -> 'Optional[Node]':
        if not root:
            return root
        queue = deque([root])
        while True:
            for idx in range(len(queue)):
                if idx != len(queue) -1 and queue[idx]:
                    queue[idx].next = queue[idx+1]
                elif idx==len(queue)-1 and queue[idx]:
                    queue[idx].next = None
            have_next = False
            for _ in range(len(queue)):
                node = queue.popleft()
                if node:
                    have_next = True
                    queue.append(node.left)
                    queue.append(node.right)
                else:
                    queue.append(None)
                    queue.append(None)
            if not have_next:
                break
        return root