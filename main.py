#!/usr/bin/env python3

class Hexagon(dict):
    def __init__(self):
        self[0] = (0, 3) # edge
        self[1] = (0, 4) # edge
        self[2] = (3, 7) # edge
        self[3] = (0, 2, 8)
        self[4] = (1, 5, 9)
        self[5] = (4, 10) # edge
        self[6] = (7, 12) # edge
        self[7] = (2, 6, 13)
        self[8] = (3, 9, 14)
        self[9] = (4, 8, 15)

        self[10] = (5, 11, 16)
        self[11] = (10, 17) # edge
        self[12] = (6, 18) # edge
        self[13] = (7, 14, 19)
        self[14] = (8, 13, 20)
        self[15] = (9, 16, 21)
        self[16] = (10, 15, 22)
        self[17] = (11, 23) # edge
        self[18] = (12, 19, 24)
        self[19] = (13, 18, 25)

        self[20] = (14, 21, 26)
        self[21] = (15, 20, 27)
        self[22] = (16, 23, 28)
        self[23] = (17, 22, 29)
        self[24] = (18, 30) # edge
        self[25] = (19, 26, 31)
        self[26] = (20, 25, 32)
        self[27] = (21, 28, 33)
        self[28] = (22, 27, 34)
        self[29] = (23, 35) # edge

        self[30] = (24, 31, 36)
        self[31] = (25, 30, 37)
        self[32] = (26, 33, 38)
        self[33] = (27, 32, 39)
        self[34] = (28, 35, 40)
        self[35] = (29, 34, 41)
        self[36] = (30, 42) # edge
        self[37] = (31, 38, 43)
        self[38] = (32, 37, 44)
        self[39] = (33, 40, 45)

        self[40] = (34, 39, 46)
        self[41] = (35, 47) # edge
        self[42] = (36, 43) # edge
        self[43] = (37, 42, 48)
        self[44] = (38, 45, 49)
        self[45] = (39, 44, 50)
        self[46] = (40, 47, 51)
        self[47] = (41, 46) # edge
        self[48] = (43, 49) # edge
        self[49] = (44, 48, 52)

        self[50] = (45, 51, 53)
        self[51] = (46, 50) # edge
        self[52] = (49, 53) # edge
        self[53] = (50, 52) # edge

    def __repr__(self):
        return '\n'.join([f"{k} -> {v}" for k, v in self.items()])
            

h = Hexagon()
print(h)

