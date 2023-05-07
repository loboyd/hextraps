use std::fmt;

const N_NODES: usize = 54;

type Neighborhood = [Option<usize>; 3];

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
        if self.deleted(index) {
            return [None; 3];
        }

        let neighborhood = self.neighborhoods[index];
        let mut masked_neighborhood = neighborhood.clone();
        for (idx, maybe_neighbor) in neighborhood.iter().enumerate() {
            if let Some(neighbor) = maybe_neighbor {
                if self.deleted(*neighbor) {
                    masked_neighborhood[idx] = None;
                }
            }
        }
        masked_neighborhood
    }

    fn placement_possible(&self) -> bool {
        let mut i = N_NODES - 1;
        while i < N_NODES { // `usize` is non-negative and will wrap around to `usize::MAX`
            let neighborhood = self.get(i);
            if neighborhood.iter().filter(|n| n.is_some()).count() >= 2 {
                return true;
            }
            i -= 1;
        }
        return false;
    }

    fn count_tilings_with_stack(&mut self) -> u32 {
        let mut ct = 0;
        let mut stack = Stack::new();
        let mut start = true;
        loop {
            if start {
                let (tiles, n_tiles) = self.distinct_tiles(0);
                for t in 0..n_tiles {
                    let tile = tiles[t].unwrap();
                    stack.push(Frame {
                        action: Action::Remove,
                        tile,
                    });
                }

                start = false;
            } else {
                let Frame { tile, action } = stack.pop().unwrap();
                match action {
                    Action::Remove => {
                        // remove the tile
                        self.delete(tile.0);
                        self.delete(tile.1);
                        self.delete(tile.2);

                        // push the restore action to the stack
                        stack.push(Frame {
                            action: Action::Restore,
                            tile,
                        });

                        // see if more placements are possible
                        if !self.placement_possible() {
                            // 1 if board is filled, 0 otherwise (invalid tiling)
                            ct += (self.deleted & Board::NODE_MASK == Board::NODE_MASK) as u32;
                            continue;
                        }

                        // if so, select a pick
                        let pick = (!self.deleted).trailing_zeros() as usize;

                        // get distinct tiles and push them to the stack
                        let (tiles, n_tiles) = self.distinct_tiles(pick);
                        for t in 0..n_tiles {
                            let tile = tiles[t].unwrap();
                            stack.push(Frame {
                                action: Action::Remove,
                                tile,
                            });
                        }

                    },
                    Action::Restore => {
                        // restore the tile
                        self.undelete(tile.0);
                        self.undelete(tile.1);
                        self.undelete(tile.2);
                    },
                }
            }

            if stack.is_empty() { break; }
        }

        ct
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
            if !self.deleted(i) {
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

#[derive(Copy, Clone)]
enum Action {
    Remove,
    Restore,
}

#[derive(Copy, Clone)]
struct Frame {
    action: Action,
    tile: (usize, usize, usize),
}

const N_FRAMES: usize = 128;

struct Stack {
    data: [Option<Frame>; N_FRAMES],
    length: usize,
}

impl Stack {
    fn new() -> Self {
        Self {
            data: [None; N_FRAMES],
            length: 0,
        }
    }

    fn pop(&mut self) -> Option<Frame> {
        if 0 <= self.length {
            self.length -= 1;
            return self.data[self.length + 1];
        }

        None
    }

    fn push(&mut self, frame: Frame) {
        self.length += 1;
        if N_FRAMES <= self.length { panic!(); }

        self.data[self.length] = Some(frame);
    }

    fn is_empty(&self) -> bool {
        self.length <= 0
    }
}

fn main() {
    let start = std::time::Instant::now();
    let mut board = Board::new();
    let num_tilings = board.count_tilings_with_stack();
    let elapsed = start.elapsed();
    println!("Found {} tilings in {:?}", num_tilings, elapsed);
}
