from math import ceil, floor, comb
def o(n, k):
  if k == 0:
    return 1
  if n == k:
    return 1
  if n < k:
    return 0
  res = 0
  for i in range(k+1):
    res += comb(ceil(n/4), i)*o(floor(3*n/4), k-i)
  return res
print(o(8, 3))