def solve():
    while (str1 := input()):
        str2 = input()
        count1 = [0] * 26
        count2 = [0] * 26

        # calculate frequency of characters
        for i in range(len(str1)):
            count1[ord(str1[i]) - ord('a')] += 1
        for i in range(len(str2)):
            count2[ord(str2[i]) - ord('a')] += 1

        # Now traverse hash array
        result = ""
        for i in range(26):

            # append character ('a'+i) in
            # resultant string 'result' by
            # min(count1[i],count2i]) times
            for j in range(1, min(count1[i],
                                  count2[i]) + 1):
                result = result + chr(ord('a') + i)

        for res in result:
            print(res, end="")


solve()
