use std::fmt;

const N_NODES: usize = 54;

struct Neighborhood(u64);

impl Neighborhood {
    fn into_iter(&self) -> impl Iterator<Item=usize> {
        let mut v = Vec::with_capacity(3);
        for n in 0..N_NODES {
            if (self.0 & (1 << n)) != 0 {
                v.push(n);
            }
        }
        v.into_iter()
    }
}

struct Board {
    neighborhoods: [Neighborhood; N_NODES],

    /// bitmask for tracking the `deleted` status of nodes in the board
    deleted: u64,
}

impl Board {
    /// all the bits of `self.deleted` that we actually care about
    const NODE_MASK: u64 = (1 << 54) - 1;

    fn new() -> Self {
        Self {
            neighborhoods: [
                Neighborhood((1 <<  1) | (1 <<  3)),             // 0
                Neighborhood((1 <<  0) | (1 <<  4)),             // 1
                Neighborhood((1 <<  3) | (1 <<  7)),             // 2
                Neighborhood((1 <<  0) | (1 <<  2) | (1 <<  8)), // 3
                Neighborhood((1 <<  1) | (1 <<  5) | (1 <<  9)), // 4
                Neighborhood((1 <<  4) | (1 << 10)),             // 5
                Neighborhood((1 <<  7) | (1 << 12)),             // 6
                Neighborhood((1 <<  2) | (1 <<  6) | (1 << 13)), // 7
                Neighborhood((1 <<  3) | (1 <<  9) | (1 << 14)), // 8
                Neighborhood((1 <<  4) | (1 <<  8) | (1 << 15)), // 9
                Neighborhood((1 <<  5) | (1 << 11) | (1 << 16)), // 10
                Neighborhood((1 << 10) | (1 << 17)),             // 11
                Neighborhood((1 <<  6) | (1 << 18)),             // 12
                Neighborhood((1 <<  7) | (1 << 14) | (1 << 19)), // 13
                Neighborhood((1 <<  8) | (1 << 13) | (1 << 20)), // 14
                Neighborhood((1 <<  9) | (1 << 16) | (1 << 21)), // 15
                Neighborhood((1 << 10) | (1 << 15) | (1 << 22)), // 16
                Neighborhood((1 << 11) | (1 << 23)),             // 17
                Neighborhood((1 << 12) | (1 << 19) | (1 << 24)), // 18
                Neighborhood((1 << 13) | (1 << 18) | (1 << 25)), // 19
                Neighborhood((1 << 14) | (1 << 21) | (1 << 26)), // 20
                Neighborhood((1 << 15) | (1 << 20) | (1 << 27)), // 21
                Neighborhood((1 << 16) | (1 << 23) | (1 << 28)), // 22
                Neighborhood((1 << 17) | (1 << 22) | (1 << 29)), // 23
                Neighborhood((1 << 18) | (1 << 30)),             // 24
                Neighborhood((1 << 19) | (1 << 26) | (1 << 31)), // 25
                Neighborhood((1 << 20) | (1 << 25) | (1 << 32)), // 26
                Neighborhood((1 << 21) | (1 << 28) | (1 << 33)), // 27
                Neighborhood((1 << 22) | (1 << 27) | (1 << 34)), // 28
                Neighborhood((1 << 23) | (1 << 35)),             // 29
                Neighborhood((1 << 24) | (1 << 31) | (1 << 36)), // 30
                Neighborhood((1 << 25) | (1 << 30) | (1 << 37)), // 31
                Neighborhood((1 << 26) | (1 << 33) | (1 << 38)), // 32
                Neighborhood((1 << 27) | (1 << 32) | (1 << 39)), // 33
                Neighborhood((1 << 28) | (1 << 35) | (1 << 40)), // 34
                Neighborhood((1 << 29) | (1 << 34) | (1 << 41)), // 35
                Neighborhood((1 << 30) | (1 << 42)),             // 36
                Neighborhood((1 << 31) | (1 << 38) | (1 << 43)), // 37
                Neighborhood((1 << 32) | (1 << 37) | (1 << 44)), // 38
                Neighborhood((1 << 33) | (1 << 40) | (1 << 45)), // 39
                Neighborhood((1 << 34) | (1 << 39) | (1 << 46)), // 40
                Neighborhood((1 << 35) | (1 << 47)),             // 41
                Neighborhood((1 << 36) | (1 << 43)),             // 42
                Neighborhood((1 << 37) | (1 << 42) | (1 << 48)), // 43
                Neighborhood((1 << 38) | (1 << 45) | (1 << 49)), // 44
                Neighborhood((1 << 39) | (1 << 44) | (1 << 50)), // 45
                Neighborhood((1 << 40) | (1 << 47) | (1 << 51)), // 46
                Neighborhood((1 << 41) | (1 << 46)),             // 47
                Neighborhood((1 << 43) | (1 << 49)),             // 48
                Neighborhood((1 << 44) | (1 << 48) | (1 << 52)), // 49
                Neighborhood((1 << 45) | (1 << 51) | (1 << 53)), // 50
                Neighborhood((1 << 46) | (1 << 50)),             // 51
                Neighborhood((1 << 49) | (1 << 53)),             // 52
                Neighborhood((1 << 50) | (1 << 52)),             // 53
            ],

            deleted: 0b0,
        }
    }

    fn _small() -> Self {
        let mut board = Self::new();
        board.deleted = !0b0;
        board.undelete(0);
        board.undelete(1);
        board.undelete(3);
        board.undelete(4);
        board.undelete(8);
        board.undelete(9);
        board
    }

    fn deleted(&self, index: usize) -> bool {
        self.deleted & (1 << index) != 0
    }

    fn delete(&mut self, index: usize) {
        self.deleted |= 1 << index;
    }

    fn undelete(&mut self, index: usize) {
        self.deleted &= !(1 << index);
    }

    // TODO check that this access pattern isn't slow af
    fn get(&self, index: usize) -> Neighborhood {
        Neighborhood(self.neighborhoods[index].0 & !self.deleted)
    }

    fn placement_possible(&self) -> bool {
        let mut i = N_NODES - 1;
        while i < N_NODES { // `usize` is non-negative and will wrap around to `usize::MAX`
            let neighborhood = self.get(i);
            if neighborhood.0.count_ones() >= 2 {
                return true;
            }
            i -= 1;
        }
        return false;
    }

    fn count_tilings(&mut self) -> u32 {
        if !self.placement_possible() {
            // 1 if board is filled, 0 otherwise (invalid tiling)
            return (self.deleted & Board::NODE_MASK == Board::NODE_MASK) as u32;
        }

        let pick = (!self.deleted).trailing_zeros() as usize;

        let mut ct = 0;
        let (tiles, n_tiles) = self.distinct_tiles(pick);
        for t in 0..n_tiles {
            let tile = tiles[t].unwrap();
            self.delete(tile.0);
            self.delete(tile.1);
            self.delete(tile.2);

            ct += self.count_tilings();

            self.undelete(tile.0);
            self.undelete(tile.1);
            self.undelete(tile.2);
        }

        ct
    }

    // this returns an array with possibly empty values, and the actual len
    fn distinct_tiles(&self, pick: usize) -> ([Option<(usize, usize, usize)>; 9], usize) {
        let mut output = [None; 9];
        let mut n_tiles = 0;

        for neighbor in self.get(pick).into_iter() {
            // inclusion forcing tiles
            for other_neighbor in self.get(pick).into_iter() {
                if neighbor < other_neighbor {
                    output[n_tiles] = Some((neighbor, pick, other_neighbor));
                    n_tiles += 1;
                }
            }

            // exclusion forcing tiles
            let neighborhood_neighborhood = self.get(neighbor);
            for second_order_neighbor in neighborhood_neighborhood.into_iter() {
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
            if !self.deleted(i) {
                write!(f, "{} -> ", i,)?;
                for neighbor in self.get(i).into_iter() {
                    write!(f, "{} ", neighbor)?;
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
