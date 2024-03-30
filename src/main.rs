use std::fmt;

struct Board {
    height: u8,
    width: u8,
    nums: Vec<u8>,
}

impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut result = String::new();
        let numwidth = 1 + (self.width * self.height).ilog10() as u8;
        for ybox in 0..self.width {
            result.push(' ');
            result.push_str(&"—".repeat(((self.width + 1) * self.height * (numwidth + 1) - 1).into()));
            result.push('\n');
            for y in 0..self.height {
                for xbox in 0..self.height {
                    result.push_str("| ");
                    result.push_str(&" ".repeat((numwidth - 1).into()));
                    for x in 0..self.width {
                        let num = &self.nums[(x + xbox * self.width + (y + ybox * self.height) * self.width * self.height) as usize].to_string();
                        result.push_str(if num == "0" {"-"} else {num});
                        result.push_str(&" ".repeat((numwidth + 1) as usize - num.len()));
                    }
                }
                result.push_str("|\n");
            }
        }
        result.push(' ');
        result.push_str(&"—".repeat(((self.width + 1) * self.height * (numwidth + 1) - 1).into()));
        write!(f, "{}", result)
    }
}

impl Board {
    fn solve(&self) -> Option<Board> {
        let mut soulution = self.nums.clone();
        if self.recursive_solve(&mut soulution) {
            Some(Board {
                height : self.height,
                width : self.width,
                nums : soulution,
            })
        } else {
            None
        }
    }

    fn recursive_solve(&self, soulution : &mut Vec<u8>) -> bool {
        // find first zero-index
        let index = soulution.iter().position(|num| *num == 0);
        if let Some(i) = index {
            // try all locally valid guesses on index
            for num in self.valid_nums(&soulution, i) {
                soulution[i] = num;
                // solve recursivly
                if self.recursive_solve(soulution) {
                    return true;
                }
            }
            // board is in ivalid position, undo guess
            soulution[i] = 0;
            return false;
        } else {
            // board is filled
            return true;
        }
    }

    fn valid_nums(&self, current_board : &Vec<u8>, index : usize) -> Vec<u8> {
        let mut nums: Vec<u8> = (0..(self.width * self.height + 1)).collect();
        let mut must_check = self.get_col(&current_board, index);
        // println!("{must_check:?}");
        must_check.append(&mut self.get_row(&current_board,index));
        // println!("{must_check:?}");
        must_check.append(&mut self.get_box(&current_board,index));
        // println!("{must_check:?}");
        must_check.iter()
            .for_each(|num| nums[*num as usize] = 0);
        return nums.iter().filter(|num| **num != 0)
            .copied()
            .collect();
    }

    fn get_box(&self, current_board : &Vec<u8>, index : usize) -> Vec<u8> {
        let size = (self.width * self.height) as usize;
        let big_size = size * self.height as usize;
        let start_row_index = index / big_size * self.height as usize;
        let end_row_index = start_row_index + self.height as usize;
        let start_col_index = index % size - index % self.width as usize;
        let end_col_index = start_col_index + self.width as usize;
        // println!("{},{},{},{}",start_row_index,end_row_index,start_col_index,end_col_index);
        return (start_row_index..end_row_index).into_iter()
            .map(|row_index| row_index * size)
            .map(|index| current_board[(index + start_col_index)..(index + end_col_index)].to_vec())
            .collect::<Vec<Vec<u8>>>()
            .iter()
            .flatten()
            .filter(|num| **num != 0)
            .copied()
            .collect();
    }

    fn get_row(&self, current_board : &Vec<u8>, index : usize) -> Vec<u8> {
        let size = (self.width * self.height) as usize;
        let start = index - index % size;
        let end = start + size;
        current_board[start..end]
            .iter()
            .filter(|num| **num != 0)
            .copied()
            .collect()
    }

    fn get_col(&self, current_board : &Vec<u8>, index : usize) -> Vec<u8> {
        let size = (self.width * self.height) as usize;
        return current_board[(index % size)..(size * size)]
            .iter()
            .step_by(size)
            .filter(|num| **num != 0)
            .copied()
            .collect();
    }
}

fn main() {
    let _b1 = Board {
        height : 2,
        width : 2,
        // nums : vec![1,2,3,4,3,4,1,0,2,3,4,1,4,1,2,3],
        nums : vec![0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,4],
    };
    let _b2 = Board {
        height : 3,
        width : 3,
        nums : vec![
            0,0,0,0,0,0,0,0,0,
            0,0,0,0,0,0,0,0,0,
            0,0,0,0,0,0,0,0,0,
            0,0,0,0,0,0,0,0,0,
            0,0,0,1,0,0,0,0,0,
            0,0,0,0,0,0,0,0,0,
            0,0,0,0,0,0,0,0,0,
            0,0,0,0,0,0,0,0,0,
            1,0,0,0,0,0,0,0,0],
    };
    let _b3 = Board {
        height : 2,
        width : 3,
        nums : vec![1,2,3,4,5,6,
            1,2,3,4,5,6,
            1,2,3,4,0,6,
            1,2,3,4,5,6,
            1,2,3,4,5,6,
            1,2,3,4,5,6],
    };
    // println!("{b1}");
    println!("{_b2}");
    match _b2.solve() {
        Some(solved) => println!("{solved}"),
        None => println!("Unable to solve"),
    }
    // println!("{b3}");
}
