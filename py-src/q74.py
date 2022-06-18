from typing import List


class Solution:
    def searchMatrix(self, matrix: List[List[int]], target: int) -> bool:
        s_row = 0 
        e_row = len(matrix)
        f_row = []
        while s_row < e_row:
          m_row = int((s_row + e_row) / 2)
          first = matrix[m_row][0]
          last = matrix[m_row][len(matrix[m_row]) - 1]
          if first <= target and last >= target:
            f_row = matrix[m_row]
          if first < target:
            s_row = m_row + 1
          else:
            e_row = m_row
        
        l, r = 0, len(f_row) - 1
        while l < r:
            m = (l + r) // 2
            if f_row[m] < target:
                l = m + 1
            else:
                r = m
        return len(f_row) > 0 and f_row[l] == target

print(Solution().searchMatrix([[1]], 0))