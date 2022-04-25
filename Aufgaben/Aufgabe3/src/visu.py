import sys
from math import log10
import os, shutil

import matplotlib.patches as patches
import matplotlib.pyplot as plt

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

if os.path.isdir("visu_frames"):
    sys.exit(1)
os.mkdir("visu_frames")
# filepaths
fp_in = "visu_frames/a*.png"
fp_out = "res.gif"

input_path = sys.argv[1]
with open(input_path, "r") as fin:
    lin = fin.readlines()
w = len(lin[0].strip())
digits = [Digit((0.75*i, 0), 0.1) for i in range(w)]
plt.axis([0, 0.75*w, 0, 0.75*w])
max_idx = len(lin)-2-1
for t, li in enumerate(lin[2:]):
    items = li.split(" ")[4:]
    for i, el in enumerate(items):
        digits[i].draw(int(el))
    plt.savefig(f"visu_frames/a{str(t).zfill(int(log10(max_idx)+1))}.png")
    for i in digits:
        i.clear()
imgs = (Image.open(f) for f in sorted(glob.glob(fp_in)))
img = next(imgs)  # extract first image from iterator
img.save(fp=fp_out, format='GIF', append_images=imgs,
         save_all=True, duration=1000, loop=0)
shutil.rmtree("visu_frames")
