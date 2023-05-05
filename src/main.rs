use std::fmt;

const N_NODES: usize = 54;

#[derive(Clone)]
enum Neighborhood {
    Three(usize, usize, usize),
    Two(usize, usize),
    One(usize),
    None,
}

impl Neighborhood {
    fn iter(&self) -> impl Iterator<Item=&usize> {
        match self {
            Neighborhood::Three(x, y, z) => vec![x, y, z].into_iter(),
            Neighborhood::Two(x, y) => vec![x, y].into_iter(),
            Neighborhood::One(x) => vec![x].into_iter(),
            Neighborhood::None => vec![].into_iter(),
        }
    }
}

impl fmt::Display for Neighborhood {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Neighborhood::Three(x, y, z) => write!(f, "{{{}, {}, {}}}", x, y, z),
            Neighborhood::Two(x, y) => write!(f, "{{{}, {}}}", x, y),
            Neighborhood::One(x) => write!(f, "{{{}}}", x),
            Neighborhood::None => write!(f, "{{}}"),
        }
    }
}

struct Board {
    neighborhoods: [Neighborhood; N_NODES],
    deleted: [bool; N_NODES],
}

impl Board {
    fn new() -> Self {
        Self {
            neighborhoods: [
                Neighborhood::Two(1, 3),         // 0
                Neighborhood::Two(0, 4),         // 1
                Neighborhood::Two(3, 7),         // 2
                Neighborhood::Three(0, 2, 8),    // 3
                Neighborhood::Three(1, 5, 9),    // 4
                Neighborhood::Two(4, 10),        // 5
                Neighborhood::Two(7, 12),        // 6
                Neighborhood::Three(2, 6, 13),   // 7
                Neighborhood::Three(3, 9, 14),   // 8
                Neighborhood::Three(4, 8, 15),   // 9

                Neighborhood::Three(5, 11, 16),  // 10
                Neighborhood::Two(10, 17),       // 11
                Neighborhood::Two(6, 18),        // 12
                Neighborhood::Three(7, 14, 19),  // 13
                Neighborhood::Three(8, 13, 20),  // 14
                Neighborhood::Three(9, 16, 21),  // 15
                Neighborhood::Three(10, 15, 22), // 16
                Neighborhood::Two(11, 23),       // 17
                Neighborhood::Three(12, 19, 24), // 18
                Neighborhood::Three(13, 18, 25), // 19

                Neighborhood::Three(14, 21, 26), // 20
                Neighborhood::Three(15, 20, 27), // 21
                Neighborhood::Three(16, 23, 28), // 22
                Neighborhood::Three(17, 22, 29), // 23
                Neighborhood::Two(18, 30),       // 24
                Neighborhood::Three(19, 26, 31), // 25
                Neighborhood::Three(20, 25, 32), // 26
                Neighborhood::Three(21, 28, 33), // 27
                Neighborhood::Three(22, 27, 34), // 28
                Neighborhood::Two(23, 35),       // 29

                Neighborhood::Three(24, 31, 36), // 30
                Neighborhood::Three(25, 30, 37), // 31
                Neighborhood::Three(26, 33, 38), // 32
                Neighborhood::Three(27, 32, 39), // 33
                Neighborhood::Three(28, 35, 40), // 34
                Neighborhood::Three(29, 34, 41), // 35
                Neighborhood::Two(30, 42),       // 36
                Neighborhood::Three(31, 38, 43), // 37
                Neighborhood::Three(32, 37, 44), // 38
                Neighborhood::Three(33, 40, 45), // 39

                Neighborhood::Three(34, 39, 46), // 40
                Neighborhood::Two(35, 47),       // 41
                Neighborhood::Two(36, 43),       // 42
                Neighborhood::Three(37, 42, 48), // 43
                Neighborhood::Three(38, 45, 49), // 44
                Neighborhood::Three(39, 44, 50), // 45
                Neighborhood::Three(40, 47, 51), // 46
                Neighborhood::Two(41, 46),       // 47
                Neighborhood::Two(43, 49),       // 48
                Neighborhood::Three(44, 48, 52), // 49

                Neighborhood::Three(45, 51, 53), // 50
                Neighborhood::Two(46, 50),       // 51
                Neighborhood::Two(49, 53),       // 52
                Neighborhood::Two(50, 52)        // 53

            ],

            deleted: [ false; N_NODES ],
        }
    }

    fn small() -> Self {
        let mut board = Self::new();
        board.deleted = [true; 54];
        board.deleted[0] = false;
        board.deleted[1] = false;
        board.deleted[3] = false;
        board.deleted[4] = false;
        board.deleted[8] = false;
        board.deleted[9] = false;
        board
    }

    // TODO check that this access pattern isn't slow af
    fn get(&self, index: usize) -> Neighborhood {
        if self.deleted[index] { return Neighborhood::None }

        let neighborhood = &self.neighborhoods[index];
        match neighborhood {
            Neighborhood::Three(x, y, z) => {
                if self.deleted[*x] && self.deleted[*y] && self.deleted[*z] {
                    return Neighborhood::None;
                } else if self.deleted[*x] && self.deleted[*y] {
                    return Neighborhood::One(*z);
                } else if self.deleted[*x] && self.deleted[*z] {
                    return Neighborhood::One(*y);
                } else if self.deleted[*y] && self.deleted[*z] {
                    return Neighborhood::One(*x);
                } else if self.deleted[*x] {
                    return Neighborhood::Two(*y, *z);
                } else if self.deleted[*y] {
                    return Neighborhood::Two(*x, *z);
                } else if self.deleted[*z] {
                    return Neighborhood::Two(*x, *y);
                } else {
                    return Neighborhood::Three(*x, *y, *z);
                }
            },
            Neighborhood::Two(x, y) => {
                if self.deleted[*x] && self.deleted[*y] {
                    return Neighborhood::None;
                } else if self.deleted[*x] {
                    return Neighborhood::One(*y);
                } else if self.deleted[*y] {
                    return Neighborhood::One(*x);
                } else {
                    return Neighborhood::Two(*x, *y);
                }
            },
            Neighborhood::One(x) => {
                if self.deleted[*x] {
                    return Neighborhood::None;
                } else {
                    return Neighborhood::One(*x);
                }
            }
            x => { return x.clone() },
        }
    }

    fn placement_possible(&self) -> bool {
        for i in 0..N_NODES {
            if let Neighborhood::Two(_, _) | Neighborhood::Three(_, _, _) = self.get(i) {
                return true;
            }
        }
        return false
    }

    fn count_tilings(&mut self) -> u32 {
        if !self.placement_possible() {
            // 1 if board is filled, 0 otherwise (invalid tiling)
            return (self.deleted == [true; N_NODES]) as u32;
        }

        // find the first non-deleted node
        let mut pick = 0;
        while self.deleted[pick] {
            pick += 1;
        }

        let mut ct = 0;
        let (tiles, n_tiles) = self.distinct_tiles(pick);
        for t in 0..n_tiles {
            let tile = tiles[t].unwrap();
            self.deleted[tile.0] = true;
            self.deleted[tile.1] = true;
            self.deleted[tile.2] = true;

            ct += self.count_tilings();

            self.deleted[tile.0] = false;
            self.deleted[tile.1] = false;
            self.deleted[tile.2] = false;
        }

        ct
    }

    // this returns an array with possibly empty values, and the actual len
    fn distinct_tiles(&self, pick: usize) -> ([Option<(usize, usize, usize)>; 9], usize) {
        let mut output = [None; 9];
        let mut n_tiles = 0;

        for &neighbor in self.get(pick).iter() {
            // inclusion forcing tiles
            for &other_neighbor in self.get(pick).iter() {
                if neighbor < other_neighbor {
                    output[n_tiles] = Some((neighbor, pick, other_neighbor));
                    n_tiles += 1;
                }
            }

            // exclusion forcing tiles
            let neighborhood_neighborhood = self.get(neighbor);
            for &second_order_neighbor in neighborhood_neighborhood.iter() {
                if second_order_neighbor != pick {
                    output[n_tiles] = Some((pick, neighbor, second_order_neighbor));
                    n_tiles += 1;
                }
            }
        }

        (output, n_tiles)
    }
}

impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for i in 0..N_NODES {
            if !self.deleted[i] {
                writeln!(f, "{} -> {}", i, self.get(i))?;
            }
        }

        Ok(())
    }
}

fn main() {
    let mut board = Board::new();

    println!("no of tilings: {}", board.count_tilings());
}
