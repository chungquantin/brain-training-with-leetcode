# Definition for singly-linked list.
from typing import List, Optional


class ListNode:
    def __init__(self, val=0, next=None):
        self.val = val
        self.next = next


class Solution:
    def mergeKLists(self, lists: List[Optional[ListNode]]) -> Optional[ListNode]:
        if len(lists) == 0:
            return None

        while True:
            if len(lists) == 1:
                return lists[0]
            first, second = lists.pop(), lists.pop()
            merged = self.mergeTwoSortedLists(first, second)
            lists.append(merged)

    def mergeTwoSortedLists(self, list1: Optional[ListNode], list2: Optional[ListNode]):
        if not list1:
            return list2

        if not list2:
            return list1

        dummy = ListNode(val=-1, next=None)
        p = dummy

        while list1 != None and list2 != None:
            val1, val2 = list1.val, list2.val
            if val1 > val2:
                p.next = list2
                list2 = list2.next
            else:
                p.next = list1
                list1 = list1.next
            p = p.next
        p.next = list1 if list1 else list2
        return dummy.next
