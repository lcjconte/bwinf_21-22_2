from proto.Aufgabe3.common import *
from functools import lru_cache

MAX = int(1e9)

def process():
    @lru_cache(maxsize=None)
    def explore(k, bal) -> int:
        if bal < 0:
            bal = [-bal, 0]
        else:
            bal = [0, bal]
        cmin = MAX
        for c in chars.chars:
            e = chars.conv_effect(s[i], c)
            nBal = [bal[i]+e[i] for i in range(2)]
            overlap = min(bal[0], bal[1])
            acost = overlap
            nBal = [nBal[i]-overlap for i in range(2)]
            if k == n-1:
                if sum(nBal) == 0:
                    cmin = min(cmin, acost)
            else:
                acost += explore(k+1, -nBal[0]+nBal[1])
                cmin = min(cmin, acost)
        return cmin

    s = input("Input hex number: ").lower()
    mmax = int(input("Move limit: "))
    n = len(s)
    chars = Chars.read_from("proto/Aufgabe3/chars.json")
    cbal = [0, 0]
    cost = 0
    nString = []
    for i in range(n):
        for c in reversed(chars.chars):
            e = chars.conv_effect(s[i], c)
            nBal = [cbal[i]+e[i] for i in range(2)]
            overlap = min(nBal[0], nBal[1])
            acost = overlap
            nBal = [nBal[i]-overlap for i in range(2)]
            if i == n-1:
                if cost+acost <= mmax and sum(nBal) == 0:
                    nString.append(c)
                    cost = cost + acost
                    cbal = nBal
                    break
            else:
                acost += explore(i+1, nBal[1]-nBal[0])
                if cost+acost <= mmax:
                    nString.append(c)
                    cost = cost + acost
                    cbal = nBal
                    break
    print("Res: ", "".join(nString))

process()