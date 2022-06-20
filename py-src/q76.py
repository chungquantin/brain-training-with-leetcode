from collections import Counter


class Solution:
    def minWindow(self, s: str, t: str) -> str:
        ans = s + "a"
        start, end = 0, 0
        counter = len(t)
        freq = dict(Counter([c for c in t]))

        while end < len(s):
            e_char = s[end]
            if e_char in freq.keys() and freq[e_char] > 0:
                counter -= 1
            if e_char in freq.keys():
                freq[e_char] -= 1
            else:
                freq[e_char] = -1
            while counter == 0:
                sl = s[start : end+1]
                if len(sl) < len(ans):
                    ans = sl
                s_char = s[start]
                if freq[s_char] == 0:
                    counter += 1
                freq[s_char] += 1
                start += 1
            end += 1
        if len(ans) == len(s) + 1:
            return ""
        return ans
