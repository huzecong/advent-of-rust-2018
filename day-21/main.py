h = set()
history = []

r3 = 65536
r4 = 16098955
while True:
    r4 = (r4 + (r3 & 255)) & 16777215
    r4 = (r4 * 65899) & 16777215
    if r3 < 256:
        if len(h) == 0:
            print(r4)
        if r4 in h:
            print(history[-1])
            break
        h.add(r4)
        history.append(r4)
        r3 = r4 | 65536
        r4 = 16098955
    else:
        # find least multiple of 256 > r3
        r3 = r3 // 256
