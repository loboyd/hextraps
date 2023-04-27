#!/usr/bin/env python3

import copy

class Hexagon(dict):
    """
    Represents a Hexagon-shaped triangularly-tiled region of sidelength 3 as a graph in which each
    node is a triangle and each connection indicates adjacency. In particular, this is implemented
    as a dictionary where the keys are simple integer-valued indices and the the values are sets of
    indices--the "neighborhood" for the triangle of the given index.

    """
    def __init__(self, *args, **kwargs):
        if args or kwargs:
            super().__init__(*args, **kwargs)
        super().__init__({
            0: {1, 3},
            1: {0, 4},
            2: {3, 7},
            3: {0, 2, 8},
            4: {1, 5, 9},
            5: {4, 10},
            6: {7, 12},
            7: {2, 6, 13},
            8: {3, 9, 14},
            9: {4, 8, 15},

            10: {5, 11, 16},
            11: {10, 17},
            12: {6, 18},
            13: {7, 14, 19},
            14: {8, 13, 20},
            15: {9, 16, 21},
            16: {10, 15, 22},
            17: {11, 23},
            18: {12, 19, 24},
            19: {13, 18, 25},

            20: {14, 21, 26},
            21: {15, 20, 27},
            22: {16, 23, 28},
            23: {17, 22, 29},
            24: {18, 30},
            25: {19, 26, 31},
            26: {20, 25, 32},
            27: {21, 28, 33},
            28: {22, 27, 34},
            29: {23, 35},

            30: {24, 31, 36},
            31: {25, 30, 37},
            32: {26, 33, 38},
            33: {27, 32, 39},
            34: {28, 35, 40},
            35: {29, 34, 41},
            36: {30, 42},
            37: {31, 38, 43},
            38: {32, 37, 44},
            39: {33, 40, 45},

            40: {34, 39, 46},
            41: {35, 47},
            42: {36, 43},
            43: {37, 42, 48},
            44: {38, 45, 49},
            45: {39, 44, 50},
            46: {40, 47, 51},
            47: {41, 46},
            48: {43, 49},
            49: {44, 48, 52},

            50: {45, 51, 53},
            51: {46, 50},
            52: {49, 53},
            53: {50, 52}
        })

    def remove_node(self, index):
        """Remove node corresponding to given index and all connection to and from it."""
        while 0 < len(self[index]):
            neighbor_index = self[index].pop() # remove neighbor from own neighborhood
            self[neighbor_index].remove(index) # remove self from neighbor's neighborhood
        self.pop(index) # remove self from graph

    def enumerate_possible_tile_placements(self):
        """Find all valid tile placements. Represented by a list of index triples. To "place" a
            tile, one must call `self.remove_node(<index>) for each index in the triple."""
        placements = []
        for (key, value) in self.items():
            if len(value) == 2:
                t = {v for v in value} | {key}
                placements.append(t)
            if len(value) == 3:
                for neighbor in value:
                    t = {v for v in value if v is not neighbor} | {key}
                    placements.append(t)
        return placements

    def place(self, placement):
        """Remove all nodes of a given placement."""
        for node in placement:
            self.remove_node(node)

    def placed(self, placement):
        """Returns a new graph which is equivalent to self less the given placement."""
        t = self.deepcopy()
        t.place(placement)
        return t

    def deepcopy(self):
        """Creates a deepcopy of `self`."""
        t = self.__class__()
        t.clear()
        t.update(copy.deepcopy(self))
        return t

    def __repr__(self):
        return '\n'.join([f"{k} -> {v}" for k, v in self.items()])
            
    def trim_to_radius_1(self):
        """Remove all nodes that aren't in the radius-1 Hexagon."""
        self.remove_node(2)
        self.remove_node(5)
        self.remove_node(6)
        self.remove_node(7)
        self.remove_node(10)
        self.remove_node(11)
        self.remove_node(12)
        self.remove_node(13)
        self.remove_node(14)
        self.remove_node(15)
        self.remove_node(16)
        self.remove_node(17)
        self.remove_node(18)
        self.remove_node(19)
        self.remove_node(20)
        self.remove_node(21)
        self.remove_node(22)
        self.remove_node(23)
        self.remove_node(24)
        self.remove_node(25)
        self.remove_node(26)
        self.remove_node(27)
        self.remove_node(28)
        self.remove_node(29)
        self.remove_node(30)
        self.remove_node(31)
        self.remove_node(32)
        self.remove_node(33)
        self.remove_node(34)
        self.remove_node(35)
        self.remove_node(36)
        self.remove_node(37)
        self.remove_node(38)
        self.remove_node(39)
        self.remove_node(40)
        self.remove_node(41)
        self.remove_node(42)
        self.remove_node(43)
        self.remove_node(44)
        self.remove_node(45)
        self.remove_node(46)
        self.remove_node(47)
        self.remove_node(48)
        self.remove_node(49)
        self.remove_node(50)
        self.remove_node(51)
        self.remove_node(52)
        self.remove_node(53)

        return self

h = Hexagon().trim_to_radius_1()

print(h.enumerate_possible_tile_placements())

