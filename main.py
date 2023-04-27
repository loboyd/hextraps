#!/usr/bin/env python3

class Triangle:
    def __init__(self, index):
        self.index = index
        self.neighbors = set()

    def __repr__(self):
        return f'{self.index} -> [{", ".join([str(n.index) for n in self.neighbors])}]'

class Hexagon:
    def __init__(self):

        # create triangles

        self.triangles = [Triangle(i) for i in range(54)]

        # define neighbors
        self.add_neighbors_by_index(0, (0, 3)) # edge
        self.add_neighbors_by_index(1, (0, 4)) # edge
        self.add_neighbors_by_index(2, (3, 7)) # edge
        self.add_neighbors_by_index(3, (0, 2, 8))
        self.add_neighbors_by_index(4, (1, 5, 9))
        self.add_neighbors_by_index(5, (4, 10)) # edge
        self.add_neighbors_by_index(6, (7, 12)) # edge
        self.add_neighbors_by_index(7, (2, 6, 13))
        self.add_neighbors_by_index(8, (3, 9, 14))
        self.add_neighbors_by_index(9, (4, 8, 15))

        self.add_neighbors_by_index(10, (5, 11, 16))
        self.add_neighbors_by_index(11, (10, 17)) # edge
        self.add_neighbors_by_index(12, (6, 18)) # edge
        self.add_neighbors_by_index(13, (7, 14, 19))
        self.add_neighbors_by_index(14, (8, 13, 20))
        self.add_neighbors_by_index(15, (9, 16, 21))
        self.add_neighbors_by_index(16, (10, 15, 22))
        self.add_neighbors_by_index(17, (11, 23)) # edge
        self.add_neighbors_by_index(18, (12, 19, 24))
        self.add_neighbors_by_index(19, (13, 18, 25))

        self.add_neighbors_by_index(20, (14, 21, 26))
        self.add_neighbors_by_index(21, (15, 20, 27))
        self.add_neighbors_by_index(22, (16, 23, 28))
        self.add_neighbors_by_index(23, (17, 22, 29))
        self.add_neighbors_by_index(24, (18, 30)) # edge
        self.add_neighbors_by_index(25, (19, 26, 31))
        self.add_neighbors_by_index(26, (20, 25, 32))
        self.add_neighbors_by_index(27, (21, 28, 33))
        self.add_neighbors_by_index(28, (22, 27, 34))
        self.add_neighbors_by_index(29, (23, 35)) # edge

        self.add_neighbors_by_index(30, (24, 31, 36))
        self.add_neighbors_by_index(31, (25, 30, 37))
        self.add_neighbors_by_index(32, (26, 33, 38))
        self.add_neighbors_by_index(33, (27, 32, 39))
        self.add_neighbors_by_index(34, (28, 35, 40))
        self.add_neighbors_by_index(35, (29, 34, 41))
        self.add_neighbors_by_index(36, (30, 42)) # edge
        self.add_neighbors_by_index(37, (31, 38, 43))
        self.add_neighbors_by_index(38, (32, 37, 44))
        self.add_neighbors_by_index(39, (33, 40, 45))

        self.add_neighbors_by_index(40, (34, 39, 46))
        self.add_neighbors_by_index(41, (35, 47)) # edge
        self.add_neighbors_by_index(42, (36, 43)) # edge
        self.add_neighbors_by_index(43, (37, 42, 48))
        self.add_neighbors_by_index(44, (38, 45, 49))
        self.add_neighbors_by_index(45, (39, 44, 50))
        self.add_neighbors_by_index(46, (40, 47, 51))
        self.add_neighbors_by_index(47, (41, 46)) # edge
        self.add_neighbors_by_index(48, (43, 49)) # edge
        self.add_neighbors_by_index(49, (44, 48, 52))

        self.add_neighbors_by_index(50, (45, 51, 53))
        self.add_neighbors_by_index(51, (46, 50)) # edge
        self.add_neighbors_by_index(52, (49, 53)) # edge
        self.add_neighbors_by_index(53, (50, 52)) # edge

    def add_neighbors_by_index(self, triangle_ind, neighbor_inds):
        for neighbor_ind in neighbor_inds:
            self.triangles[triangle_ind].neighbors.add(self.triangles[neighbor_ind])

    def __repr__(self):
        return '\n'.join([t.__repr__() for t in self.triangles])
            

h = Hexagon()
print(h)


