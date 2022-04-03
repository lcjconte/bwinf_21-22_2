from turtle import *
from time import sleep
import json

trt = Turtle()
trt.resizemode("user")
trt.pensize(5)
trt.ht()
trt.speed(0)
trt.pu()

lines = [
    (0.5, 2.33, "r"),
    (0.5, 1.33, "u"),
    (1.5, 1.33, "u"),
    (0.5, 1.33, "r"),
    (0.5, 0.33, "u"),
    (1.5, 0.33, "u"),
    (0.5, 0.33, "r")
]

class Digit:
    def __init__(self, pos, sc) -> None:
        self.pos = pos
        self.sc = sc
    def draw(self, mask):
        for i in range(7):
            li = lines[i]
            if (mask & (1 << i)):
                trt.goto(self.pos[0]+self.sc*(li[0]+(0.25 if li[2] == "r" else 0)), self.pos[1]+self.sc*(li[1]+(0.33 if li[2] == "u" else 0)))
                if li[2] == "r":
                    trt.setheading(0)
                else:
                    trt.setheading(90)
                trt.pd()
                trt.forward(self.sc*(0.5 if li[2] == "r" else 0.33))
                trt.pu()
chars = json.load(open("proto/Aufgabe3/chars.json"))
d = Digit((0, 0), 70)
for c in chars["combinations"].keys():
    d.draw(int(chars["combinations"][c][::-1], base=2))
    sleep(2)
    trt.clear()
input()