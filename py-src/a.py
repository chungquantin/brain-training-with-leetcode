def gcd(a, b):
    if a == 0:
        return b
    return gcd(b % a, a)


def solution(a: int, b: int):
    d = gcd(a, b)
    i = 1
    x, y = 1, 1
    while True:
        for j in [-i, i]:
            if (a + b * j) != 0:
                x = int(d / (a + b * j))
                y = int(j * x)
                if x != 0 and y != 0:
                    print(x, y, d)
                    return
                i += 1
            else:
                i += 1
                break


a, b = map(int, input().split())
solution(a, b)
