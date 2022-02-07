import json

class Chars:
    def __init__(self) -> None:
        self.positions = 0
        self.chars = []
        self.char_rep = {}
    @classmethod
    def read_from(cls, file_name):
        with open(file_name) as fIn:
            data = json.load(fIn)
        nObj = cls()
        nObj.positions = int(data["positions"])
        for char in data["combinations"]:
            nObj.chars.append(char)
            nObj.char_rep[char] = data["combinations"][char] #TODO: Check valid
        return nObj
    def conv_effect(self, a, b):
        effect = [0, 0]
        r1, r2 = self.char_rep[a], self.char_rep[b]
        for i in range(len(r1)):
            if (r1[i], r2[i]) == ("0", "1"):
                effect[1] += 1
            elif (r1[i], r2[i]) == ("1", "0"):
                effect[0] += 1
        return effect
    def cost(self, s1, s2):
        s1 = s1.lower()
        s2 = s2.lower()
        seffect = [0, 0]
        assert len(s1)==len(s2)
        for i in range(len(s1)):
            effect = self.conv_effect(s1[i], s2[i])
            seffect[0] += effect[0]
            seffect[1] += effect[1]
        assert seffect[0]==seffect[1]
        return seffect[0]

class TInputv1:
    def __init__(self) -> None:
        self.s = ""
    @classmethod
    def read_from(cls, file_name):
        nObj = TInputv1()
        with open(file_name) as fIn:
            nObj.s = fIn.readline().strip()
        return nObj

if __name__ == "__main__":
    chars = Chars.read_from("proto/Aufgabe3/chars.json")