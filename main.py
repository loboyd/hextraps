#!/usr/bin/env python3

import copy

from itertools import combinations

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
        self.deleted = set()

    def __getitem__(self, key):
        if key in self.deleted:
            raise KeyError(key)
        return {neighbor for neighbor in super().__getitem__(key) if neighbor not in self.deleted}

    def keys(self):
        return [key for key in super().keys() if key not in self.deleted]

    def items(self):
        return [(key, self[key]) for key in self.keys()]

    def __contains__(self, key):
        return key not in self.deleted and super().__contains__(key)

    def __len__(self):
        return len(self.keys())

    def hide(self, index):
        """"Hide" a node in the graph. This makes the node and all "references" to it appear to not
            be in the graph despite still existing in it's underlying data structure."""
        self.deleted.add(index)

        def reveal(index):
            self.deleted.remove(index)

        action = lambda i=index: reveal(i)

        def undo():
            action()

        return undo

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

    def placement_possible(self):
        """Determine whether there is any valid placement."""
        for key, value in self.items():
            if 2 <= len(value):
                return True
        return False

    def place(self, placement):
        """Remove all nodes of a given placement."""
        restoration_actions = []
        for node in placement:
            restoration = self.hide(node)
            restoration_actions.append(restoration)
        def undo():
            for f in reversed(restoration_actions):
                f()
        return undo

    def __repr__(self):
        return '\n'.join([f"{k} -> {v}" for k, v in self.items()])
            
    def trim_to_radius_1(self):
        """Remove all nodes that aren't in the radius-1 Hexagon."""
        self.hide(2)
        self.hide(5)
        self.hide(6)
        self.hide(7)
        self.hide(10)
        self.hide(11)
        self.hide(12)
        self.hide(13)
        self.hide(14)
        self.hide(15)
        self.hide(16)
        self.hide(17)
        self.hide(18)
        self.hide(19)
        self.hide(20)
        self.hide(21)
        self.hide(22)
        self.hide(23)
        self.hide(24)
        self.hide(25)
        self.hide(26)
        self.hide(27)
        self.hide(28)
        self.hide(29)
        self.hide(30)
        self.hide(31)
        self.hide(32)
        self.hide(33)
        self.hide(34)
        self.hide(35)
        self.hide(36)
        self.hide(37)
        self.hide(38)
        self.hide(39)
        self.hide(40)
        self.hide(41)
        self.hide(42)
        self.hide(43)
        self.hide(44)
        self.hide(45)
        self.hide(46)
        self.hide(47)
        self.hide(48)
        self.hide(49)
        self.hide(50)
        self.hide(51)
        self.hide(52)
        self.hide(53)

        return self

    def find_inclusion_forcing_tiles(self, pick):
        """Return all tile placements which force `pick` to be included in the tiling"""
        neighborhood = self[pick]
        pairs_of_neighbors = combinations(neighborhood, 2)
        return [(u, v, pick) for u, v in pairs_of_neighbors]

    def find_exclusion_forcing_tiles(self, pick):
        """Return all tile placements which force `pick` to be excluded from the tiling"""
        neighborhood = self[pick]
        placements = []
        for neighbor in neighborhood: # neighbor will be the center of a placement
            for second_order_neighbor in self[neighbor]:
                if second_order_neighbor != pick:
                    placements.append((neighbor, second_order_neighbor, pick))
        return placements

    def count_tilings(self):
        """Count valid tilings (not accounting for symmetric tilings)"""
        if not self.placement_possible():
            return len(self) == 0

        pick, neighborhood = next(iter(self.items()))
        ct = 0
        for p in self.find_inclusion_forcing_tiles(pick) + self.find_exclusion_forcing_tiles(pick):
            unplace = self.place(p)
            ct += self.count_tilings()
            unplace()

        return ct


h = Hexagon()#.trim_to_radius_1()

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
#
#h.remove_node(50)
#h.remove_node(51)
#h.remove_node(52)
#h.remove_node(53)

print(h.count_tilings())

