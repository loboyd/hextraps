use std::fmt;

const N_NODES: usize = 54;

type Neighborhood = [Option<usize>; 3];

struct Board {
    neighborhoods: [Neighborhood; N_NODES],
    deleted: [bool; N_NODES],
}

impl Board {
    fn new() -> Self {
        Self {
            neighborhoods: [
                [Some(1), Some(3), None],       // 0
                [Some(0), Some(4), None],       // 1
                [Some(3), Some(7), None],       // 2
                [Some(0), Some(2), Some(8)],    // 3
                [Some(1), Some(5), Some(9)],    // 4
                [Some(4), Some(10), None],      // 5
                [Some(7), Some(12), None],      // 6
                [Some(2), Some(6), Some(13)],   // 7
                [Some(3), Some(9), Some(14)],   // 8
                [Some(4), Some(8), Some(15)],   // 9
                [Some(5), Some(11), Some(16)],  // 10
                [Some(10), Some(17), None],     // 11
                [Some(6), Some(18), None],      // 12
                [Some(7), Some(14), Some(19)],  // 13
                [Some(8), Some(13), Some(20)],  // 14
                [Some(9), Some(16), Some(21)],  // 15
                [Some(10), Some(15), Some(22)], // 16
                [Some(11), Some(23), None],     // 17
                [Some(12), Some(19), Some(24)], // 18
                [Some(13), Some(18), Some(25)], // 19
                [Some(14), Some(21), Some(26)], // 20
                [Some(15), Some(20), Some(27)], // 21
                [Some(16), Some(23), Some(28)], // 22
                [Some(17), Some(22), Some(29)], // 23
                [Some(18), Some(30), None],     // 24
                [Some(19), Some(26), Some(31)], // 25
                [Some(20), Some(25), Some(32)], // 26
                [Some(21), Some(28), Some(33)], // 27
                [Some(22), Some(27), Some(34)], // 28
                [Some(23), Some(35), None],     // 29
                [Some(24), Some(31), Some(36)], // 30
                [Some(25), Some(30), Some(37)], // 31
                [Some(26), Some(33), Some(38)], // 32
                [Some(27), Some(32), Some(39)], // 33
                [Some(28), Some(35), Some(40)], // 34
                [Some(29), Some(34), Some(41)], // 35
                [Some(30), Some(42), None],     // 36
                [Some(31), Some(38), Some(43)], // 37
                [Some(32), Some(37), Some(44)], // 38
                [Some(33), Some(40), Some(45)], // 39
                [Some(34), Some(39), Some(46)], // 40
                [Some(35), Some(47), None],     // 41
                [Some(36), Some(43), None],     // 42
                [Some(37), Some(42), Some(48)], // 43
                [Some(38), Some(45), Some(49)], // 44
                [Some(39), Some(44), Some(50)], // 45
                [Some(40), Some(47), Some(51)], // 46
                [Some(41), Some(46), None],     // 47
                [Some(43), Some(49), None],     // 48
                [Some(44), Some(48), Some(52)], // 49
                [Some(45), Some(51), Some(53)], // 50
                [Some(46), Some(50), None],     // 51
                [Some(49), Some(53), None],     // 52
                [Some(50), Some(52), None],     // 53
            ],

            deleted: [false; N_NODES],
        }
    }

    fn _small() -> Self {
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
        if self.deleted[index] {
            return [None; 3];
        }

        let neighborhood = self.neighborhoods[index];
        let mut masked_neighborhood = neighborhood.clone();
        for (idx, maybe_neighbor) in neighborhood.iter().enumerate() {
            if let Some(neighbor) = maybe_neighbor {
                if self.deleted[*neighbor] {
                    masked_neighborhood[idx] = None;
                }
            }
        }
        masked_neighborhood
    }

    fn placement_possible(&self) -> bool {
        for i in 0..N_NODES {
            let neighborhood = self.get(i);
            if neighborhood.iter().filter(|n| n.is_some()).count() >= 2 {
                return true;
            }
        }
        return false;
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

        for &maybe_neighbor in self.get(pick).iter() {
            if let Some(neighbor) = maybe_neighbor {
                // inclusion forcing tiles
                for &maybe_other_neighbor in self.get(pick).iter() {
                    if let Some(other_neighbor) = maybe_other_neighbor {
                        if neighbor < other_neighbor {
                            output[n_tiles] = Some((neighbor, pick, other_neighbor));
                            n_tiles += 1;
                        }
                    }
                }

                // exclusion forcing tiles
                let neighborhood_neighborhood = self.get(neighbor);
                for &maybe_second_order_neighbor in neighborhood_neighborhood.iter() {
                    if let Some(second_order_neighbor) = maybe_second_order_neighbor {
                        if second_order_neighbor != pick {
                            output[n_tiles] = Some((pick, neighbor, second_order_neighbor));
                            n_tiles += 1;
                        }
                    }
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
                write!(f, "{} -> ", i,)?;
                for maybe_neighbor in self.get(i) {
                    if let Some(neighbor) = maybe_neighbor {
                        write!(f, "{} ", neighbor)?;
                    }
                }
                writeln!(f)?;
            }
        }

        Ok(())
    }
}

fn main() {
    let start = std::time::Instant::now();
    let mut board = Board::new();
    let num_tilings = board.count_tilings();
    let elapsed = start.elapsed();
    println!("Found {} tilings in {:?}", num_tilings, elapsed);
}
