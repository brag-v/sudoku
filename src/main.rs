use std::{fmt, thread, time};
use rand::thread_rng;
use rand::seq::SliceRandom;


fn clear() {
    print!("{}[2J", 27 as char);
}


struct Board {
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
            result.push_str(&"—".repeat(((((self.width + 1) * self.height) as u8) * (numwidth + 1) - 1).into()));
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
        result.push_str(&"—".repeat(((((self.width + 1) * self.height) as u8) * (numwidth + 1) - 1).into()));
        write!(f, "{}", result)
    }
}

fn gen_board(width : usize, height : usize, redundent_nums : u8) -> Board {
    let bigsize = width*width*height*height;
    // generate random board filled board
    let mut board = Board {
        width,
        height,
        nums : vec![0; bigsize],
    };
    let mut soulution = board.nums.clone();
    let map = board.gen_map();
    let guess_priority = |highest_num| {
        let mut v : Vec<u8> = (1..highest_num+1).collect();
        v.shuffle(&mut thread_rng());
        v.into_iter()
    };
    board.recursive_solve(&mut soulution, &map, guess_priority);
    board.nums = soulution;

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

impl Board {
    pub fn solve(&self) -> Option<Board> {
        let mut soulution = self.nums.clone();
        let map = self.gen_map();
        let guess_priority = |highest_num| (1..highest_num+1);
        if self.recursive_solve(&mut soulution, &map, guess_priority) {
            Some(Board {
                height : self.height,
                width : self.width,
                nums : soulution,
            })
        } else {
            None
        }
    }


    fn one_soulution(&self) -> bool {
        let mut soulution1 = self.nums.clone();
        let mut soulution2 = self.nums.clone();
        let low_num_priority = |highest_num| (1..highest_num+1);
        let high_num_priority = |highest_num| (1..highest_num+1).rev();
        let map = self.gen_map();
        if self.recursive_solve(&mut soulution1, &map, low_num_priority) {
            self.recursive_solve(&mut soulution2, &map, high_num_priority);
            soulution1 == soulution2
        } else {
            false
        }
    }


    fn recursive_solve<T: Iterator<Item = u8>>(&self, soulution : &mut Vec<u8>, map : &Vec<Vec<usize>>, guess_priority: fn(u8) -> T) -> bool {
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
                if map[index].iter().any(|i| soulution[*i] == num) { continue }
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

    fn gen_map(&self) -> Vec<Vec<usize>> {
        let size = self.width * self.height;
        let mut map : Vec<Vec<usize>> = Vec::with_capacity(size*size);
        for index in 0..(size*size) {
            let mut index_set = Vec::with_capacity(size*3);
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
            map.push(
                index_set
                    .into_iter()
                    .filter(|i| *i as usize != index)
                    .collect()
            );
        }
        map
    }
}

fn main() {
    let _b1 = Board {
        height : 2,
        width : 2,
        nums : vec![0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,4],
    };
    let _b2 = Board {
        height : 3,
        width : 3,
        nums : vec![
            1,0,0,0,3,0,5,9,0,
            0,6,0,4,8,0,3,0,0,
            0,0,0,0,0,2,0,0,0,
            0,3,1,0,0,0,6,0,7,
            2,0,0,0,0,0,0,0,1,
            5,0,6,0,0,0,8,2,0,
            0,0,0,2,0,0,0,0,0,
            0,0,4,0,7,8,0,3,0,
            0,7,8,0,9,0,0,0,5,
        ],
    };
    let _b3 = Board {
        height : 2,
        width : 3,
        nums : vec![
            6,0,0,0,0,2,
            0,0,0,0,0,0,
            0,0,0,0,0,0,
            0,1,0,0,0,0,
            0,0,0,3,0,1,
            0,0,0,0,0,0,
        ],
    };
    let _b4 = Board {
        height : 5,
        width : 4,
        nums : vec![0; 4*4*5*5],
    };
    // println!("{:?}",_b2.one_soulution());
    // let start_time = time::Instant::now();
    // let solved = _b4.solve();
    // let end_time = start_time.elapsed();
    // println!("finished in {:?}",end_time);
    // match solved {
    //     Some(solved) => {
    //         println!("{solved}")
    //     }
    //     None => println!("Unable to solve"),
    // }
    println!("{}", gen_board(4,3,0));
}
