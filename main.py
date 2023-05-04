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
        """Remove node corresponding to given index and all connection to and from it. Returns a
           lambda function which when called will restore the state of the graph as this method is
           called on it"""

        def add(index, neighbor_index):
            self[index].add(neighbor_index)

        def update(index, neighborhood):
            self.update({index: neighborhood})

        restoration_actions = [] # list of lambda to restore the node being removed
        for neighbor_index in self[index]:
            # remove self from neighbor's neighborhood
            self[neighbor_index].remove(index)

            # add self back to neighbor's neighborhood
            restoration_actions.append(lambda n=neighbor_index, i=index: add(n, i))

        # remove self from graph, capture neighborhood value for restoration
        neighborhood = self.pop(index)

        # add index -> neighborhood back to self
        restoration_actions.append(lambda i=index, h=neighborhood: update(i, h))

        def undo():
            for f in reversed(restoration_actions):
                f()
        return undo

    def enumerate_possible_tile_placements(self):
        """Find all valid tile placements. Represented by a list of index triples. To "place" a
            tile, one must call `self.remove_node(<index>) for each index in the triple."""
        placements = []
        for (key, value) in self.items():
            if len(value) == 2:
                t = tuple(sorted(list({v for v in value} | {key})))
                placements.append(t)
            if len(value) == 3:
                for neighbor in value:
                    t = tuple(sorted(list({v for v in value if v is not neighbor} | {key})))
                    placements.append(t)
        return sorted(placements)

    def place(self, placement):
        """Remove all nodes of a given placement."""
        restoration_actions = []
        for node in placement:
            restoration = self.remove_node(node)
            restoration_actions.append(restoration)
        #return lambda: [f() for f in reversed(restoration_actions)]
        def undo():
            for f in reversed(restoration_actions):
                f()
        return undo

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

    # since there is no most-recent tile placement, provide a dummy value known to be smaller
    # than all actual tile placements
    def count_valid_tilings(self, initial=(0, 0, 0)):
        """Count valid ordered tilings. `initial` is the most-recently-placed tile. If in any tile
        placement is less than the initial tile, the tiling is non-ordered and thus would be a
        duplicate of the properly ordered version, so it is not considered."""
        candidate_placements = self.enumerate_possible_tile_placements()
        if len(candidate_placements) == 0:
            # if no nodes remain, we have a valid tiling, otherwise, invalid
            return int(len(self) == 0)
        placements = self.enumerate_possible_tile_placements()
        ct = 0
        for p in placements:
            if p < initial:
                return 0

            unplace = self.place(p)
            ct += h.count_valid_tilings(initial=p)
            unplace()

        return ct

h = Hexagon().trim_to_radius_1()

#h.remove_node(0)
#h.remove_node(1)
#h.remove_node(2)
#h.remove_node(3)
#h.remove_node(4)
#h.remove_node(5)
#h.remove_node(6)
#h.remove_node(7)
#h.remove_node(8)
#h.remove_node(9)

#h.remove_node(10)
#h.remove_node(11)
#h.remove_node(12)
#h.remove_node(13)
#h.remove_node(14)
#h.remove_node(15)
#h.remove_node(16)
#h.remove_node(17)
#h.remove_node(18)
#h.remove_node(19)

#h.remove_node(20)
#h.remove_node(21)
#h.remove_node(22)
#h.remove_node(23)
#h.remove_node(24)
#h.remove_node(25)
#h.remove_node(26)
#h.remove_node(27)
#h.remove_node(28)
#h.remove_node(29)

#h.remove_node(30)
#h.remove_node(31)
#h.remove_node(32)
#h.remove_node(33)
#h.remove_node(34)
#h.remove_node(35)
#h.remove_node(36)
#h.remove_node(37)
#h.remove_node(38)
#h.remove_node(39)

#h.remove_node(40)
#h.remove_node(41)
#h.remove_node(42)
#h.remove_node(43)
#h.remove_node(44)
#h.remove_node(45)
#h.remove_node(46)
#h.remove_node(47)
#h.remove_node(48)
#h.remove_node(49)

#h.remove_node(50)
#h.remove_node(51)
#h.remove_node(52)
#h.remove_node(53)

print(h.count_valid_tilings())

