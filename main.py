#!/usr/bin/env python3

class Hexagon(dict):
    """
    Represents a Hexagon-shaped triangularly-tiled region of sidelength 3 as a graph in which each
    node is a triangle and each connection indicates adjacency. In particular, this is implemented
    as a dictionary where the keys are simple integer-valued indices and the the values are sets of
    indices--the "neighborhood" for the triangle of the given index.

    """
    def __init__(self):
        super().__init__({
            0: (0, 3),
            1: (0, 4),
            2: (3, 7),
            3: (0, 2, 8),
            4: (1, 5, 9),
            5: (4, 10),
            6: (7, 12),
            7: (2, 6, 13),
            8: (3, 9, 14),
            9: (4, 8, 15),

            10: (5, 11, 16),
            11: (10, 17),
            12: (6, 18),
            13: (7, 14, 19),
            14: (8, 13, 20),
            15: (9, 16, 21),
            16: (10, 15, 22),
            17: (11, 23),
            18: (12, 19, 24),
            19: (13, 18, 25),

            20: (14, 21, 26),
            21: (15, 20, 27),
            22: (16, 23, 28),
            23: (17, 22, 29),
            24: (18, 30),
            25: (19, 26, 31),
            26: (20, 25, 32),
            27: (21, 28, 33),
            28: (22, 27, 34),
            29: (23, 35),

            30: (24, 31, 36),
            31: (25, 30, 37),
            32: (26, 33, 38),
            33: (27, 32, 39),
            34: (28, 35, 40),
            35: (29, 34, 41),
            36: (30, 42),
            37: (31, 38, 43),
            38: (32, 37, 44),
            39: (33, 40, 45),

            40: (34, 39, 46),
            41: (35, 47),
            42: (36, 43),
            43: (37, 42, 48),
            44: (38, 45, 49),
            45: (39, 44, 50),
            46: (40, 47, 51),
            47: (41, 46),
            48: (43, 49),
            49: (44, 48, 52),

            50: (45, 51, 53),
            51: (46, 50),
            52: (49, 53),
            53: (50, 52)
        })

    def __repr__(self):
        return '\n'.join([f"{k} -> {v}" for k, v in self.items()])
            

h = Hexagon()
print(h)

