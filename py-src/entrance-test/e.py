def solution(l, target, el):
    ltr = [-1] * l
    i = 0
    # TC: O(N)
    while i <= l - 1:
        if el[i] == 1:
            ltr[i] = i
        elif i > 0:
            ltr[i] = ltr[i - 1]
        i += 1

    rtl = [-1] * l
    i = l - 1
    # TC: O(N)
    while i >= 0:
        if el[i] == 1:
            rtl[i] = i
        elif i < l - 1:
            rtl[i] = rtl[i + 1]
        i -= 1

    ops = 0
    l_ind, r_ind = 0, l - 1
    # Optimize by substract the indexed value from total sum while iterating
    if sum(el[l_ind: r_ind+1]) < target:
        return -1
    # TC: O(N * OPERATIONS)
    while sum(el[l_ind: r_ind+1]) > target:
        if rtl[l_ind] < l - 1 - ltr[r_ind]:
            l_ind += 1
        else:
            right_index -= 1
        ops += 1
    return ops


t = int(input())
while t > 0:
    first = [int(el) for el in input().split()]
    second = [int(el) for el in input().split()]
    print(solution(first[0], first[1], second))
    t -= 1
