use rand::seq::SliceRandom;
use rand::thread_rng;
use std::fmt;

#[cfg(test)]
mod test;

#[allow(dead_code)]
fn clear() {
    print!("{}[2J", 27 as char);
}

pub struct Board {
    height: usize,
    width: usize,
    nums: Vec<u8>,
}

impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut result = String::new();
        let numwidth = 1 + (self.width * self.height).ilog10() as u8;
        for ybox in 0..self.width {
            result.push(' ');
            result.push_str(
                &"—".repeat(((((self.width + 1) * self.height) as u8) * (numwidth + 1) - 1).into()),
            );
            result.push('\n');
            for y in 0..self.height {
                for xbox in 0..self.height {
                    result.push_str("| ");
                    result.push_str(&" ".repeat((numwidth - 1).into()));
                    for x in 0..self.width {
                        let num = &self.nums[(x
                            + xbox * self.width
                            + (y + ybox * self.height) * self.width * self.height)
                            as usize]
                            .to_string();
                        result.push_str(if num == "0" { "-" } else { num });
                        result.push_str(&" ".repeat((numwidth + 1) as usize - num.len()));
                    }
                }
                result.push_str("|\n");
            }
        }
        result.push(' ');
        result.push_str(
            &"—".repeat(((((self.width + 1) * self.height) as u8) * (numwidth + 1) - 1).into()),
        );
        write!(f, "{}", result)
    }
}

impl Default for Board {
    fn default() -> Board {
        Board {
            width: 3,
            height: 3,
            nums: vec![0; 3*3*3*3],
        }
    }
}



impl Board {

    /// Returns a copy of the board in a solved state, 
    /// or None if board is unsolvable
    pub fn solve(&self) -> Option<Board> {
        if !self.no_invalid() {
            println!("swag");
            return None;
        }
        let mut soulution = self.nums.clone();
        let map = self.gen_map();
        let guess_priority = |highest_num| (1..highest_num + 1);
        if self.recursive_solve(&mut soulution, &map, guess_priority) {
            Some(Board {
                height: self.height,
                width: self.width,
                nums: soulution,
                ..Default::default()
            })
        } else {
            None
        }
    }

    /// The board is filled and contains only valid, non-conflicting numbers
    pub fn is_solved(&self) -> bool {
        self.no_invalid() && self.is_filled()
    }

    /// All numbers are in the valid range (e.g. 1 to 9 for a 9x9 board)
    /// and is not in direct conflict with another number
    fn no_invalid(&self) -> bool {
        let map = self.gen_map();
        self.nums
            .iter()
            .filter(|num| **num != 0)
            .enumerate()
            .all(|(i, num)| {
                *num <= (self.width * self.height) as u8
                    && !map[i]
                        .iter()
                        .any(|map_num| self.nums[*map_num] == *num)
            })
    }

    /// No numbers are zero
    fn is_filled(&self) -> bool {
        !self.nums.contains(&0)
    }

    /// The board has a single unique valid soulution
    fn one_soulution(&self) -> bool {
        // tries to solve the board using two methods,
        // one priorities low guesses, and one priorities high guesses.
        // Since the recursive algorithm is depth first,
        // and the two methods search the DFS-tree in opposite order
        // only one soulution will exist if the two soultions are equal.
        let mut soulution_1 = self.nums.clone();
        let mut soulution_2 = self.nums.clone();
        let low_num_priority = |highest_num| (1..highest_num + 1);
        let high_num_priority = |highest_num| (1..highest_num + 1).rev();
        let map = self.gen_map();
        if self.recursive_solve(&mut soulution_1, &map, low_num_priority) {
            self.recursive_solve(&mut soulution_2, &map, high_num_priority);
            soulution_1 == soulution_2
        } else {
            false
        }
    }

    fn recursive_solve<T: Iterator<Item = u8>>(
        &self,
        soulution: &mut Vec<u8>,
        map: &Vec<Vec<usize>>,
        guess_priority: fn(u8) -> T,
    ) -> bool {
        // {
        //     clear();
        //     thread::sleep(time::Duration::from_millis(200));
        //     let b = Board {
        //         height : self.height,
        //         width : self.width,
        //         nums : soulution.to_owned(),
        //     };
        //     println!("{b}");
        // }

        // find index of first zero
        let index = soulution.iter().position(|num| *num == 0);
        if let Some(index) = index {
            // try all locally valid guesses on index
            let highest_num = (self.height * self.width) as u8;
            for num in guess_priority(highest_num) {
                if map[index].iter().any(|i| soulution[*i] == num) {
                    continue;
                }
                soulution[index] = num;
                // solve recursivly
                if self.recursive_solve(soulution, map, guess_priority) {
                    return true;
                }
            }
            // board is in ivalid position, undo guess
            soulution[index] = 0;
            return false;
        }
        // board is filled
        return true;
    }

    /// Returns a map from indecies to all "conflicting" indecies.
    /// Index i of the map is a vector of all other indecies in the same
    /// row, column or box as i.
    fn gen_map(&self) -> Vec<Vec<usize>> {
        let size = self.width * self.height;
        let mut map: Vec<Vec<usize>> = Vec::with_capacity(size * size);
        for index in 0..(size * size) {
            let mut index_set = Vec::with_capacity(size * 3);
            // row
            let start = index - index % size;
            let end = start + size;
            for i in start..end {
                index_set.push(i);
            }
            // col
            for row in 0..size {
                index_set.push(row * size + index % size);
            }
            // box
            let big_size = size * self.height;
            let start_row = index / big_size * self.height;
            let end_row = start_row + self.height;
            let start_col = index % size - index % self.width;
            let end_col = start_col + self.width;
            for row in start_row..end_row {
                for col in start_col..end_col {
                    index_set.push(row * size + col);
                }
            }
            index_set.sort();
            index_set.dedup();
            map.push(index_set.into_iter().filter(|i| *i != index).collect());
        }
        map
    }
}

// Generates an unsolved sudoku board
fn gen_board(width: usize, height: usize) -> Board {
    let bigsize = width * width * height * height;
    // generate random solved board
    let mut board = Board {
        width,
        height,
        nums: vec![0; bigsize],
    };
    let mut soulution = board.nums.clone();
    let map = board.gen_map();
    let guess_priority = |highest_num| {
        let mut v: Vec<u8> = (1..highest_num + 1).collect();
        v.shuffle(&mut thread_rng());
        v.into_iter()
    };
    board.recursive_solve(&mut soulution, &map, guess_priority);
    board.nums = soulution;
    println!("{board}");

    // remove numbers from board in random order
    // if multiple solution exist after removing a number, add back the number
    let mut removing_order = (0..bigsize).collect::<Vec<usize>>();
    removing_order.shuffle(&mut thread_rng());
    let mut removed = vec![];

    for num in removing_order {
        removed.push(board.nums[num]);
        board.nums[num] = 0;
        if !board.one_soulution() {
            board.nums[num] = removed.pop().unwrap();
        }
    }

    board
}

fn main() {
    let generated_board = gen_board(2, 2);
    println!("{}", generated_board);
    if let Some(solved) = generated_board.solve() {
        println!("{}", solved);
    }
}
