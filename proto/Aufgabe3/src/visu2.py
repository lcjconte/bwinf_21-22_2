import matplotlib.pyplot as plt
import matplotlib.patches as patches

from time import sleep
import json

fig, ax = plt.subplots()

lines = [
    [1.5, 6.5, "r"],
    [0.75, 4.5, "u"],
    [3.75, 4.5, "u"],
    [1.5, 3.75, "r"],
    [0.75, 2, "u"],
    [3.75, 2, "u"],
    [1.5, 1, "r"]
]
ad = {
    "r": [2, 0.5],
    "u": [0.5, 1.5],
}

class Digit:
    def __init__(self, pos, sc) -> None:
        self.pos = pos
        self.sc = sc
        self.pat = []
    def draw(self, mask):
        for i in range(7):
            li = lines[i].copy()
            li[0] *= self.sc
            li[1] *= self.sc
            li[0] += self.pos[0]
            li[1] += self.pos[1]
            a = ad[li[2]].copy()
            a[0] *= self.sc
            a[1] *= self.sc
            if (mask & (1 << i)):
                npat = patches.Rectangle(li[:2], a[0], a[1], linewidth=1, edgecolor='r', facecolor='black')
                self.pat.append(npat)
                ax.add_patch(npat)
    def clear(self):
        for i in self.pat:
            i.remove()
        self.pat = []
import glob
from PIL import Image

# filepaths
fp_in = "proto/Aufgabe3/frames/a*.png"
fp_out = "proto/Aufgabe3/res.gif"


chars = json.load(open("proto/Aufgabe3/chars.json"))

with open("proto/Aufgabe3/ausgaben/ausgabe1.txt", "r") as fin:
    lin = fin.readlines()
w = len(lin[0].strip())
digits = [Digit((0.75*i, 0), 0.1) for i in range(w)]
plt.axis([0, 0.75*w, 0, 0.75*w])
for t, li in enumerate(lin[2:]):
    items = li.split(" ")[4:]
    for i, el in enumerate(items):
        digits[i].draw(int(el))
    plt.savefig(f"proto/Aufgabe3/frames/a{t}.png")
    for i in digits:
        i.clear()
imgs = (Image.open(f) for f in sorted(glob.glob(fp_in)))
img = next(imgs)  # extract first image from iterator
img.save(fp=fp_out, format='GIF', append_images=imgs,
         save_all=True, duration=1000, loop=0)

