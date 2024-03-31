use std::{collections::HashMap, fmt, thread, time};

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

impl Board {
    fn solve(&self) -> Option<Board> {
        let mut soulution = self.nums.clone();
        let map = self.gen_map();
        if self.recursive_solve(&mut soulution, &map, 0) {
            Some(Board {
                height : self.height,
                width : self.width,
                nums : soulution,
            })
        } else {
            None
        }
    }


    fn recursive_solve(&self, soulution : &mut Vec<u8>, map : &Vec<Vec<usize>>, _last_index : usize) -> bool {
        // {
        //     // clear();
        //     thread::sleep(time::Duration::from_millis(100));
        //     let b = Board {
        //         height : self.height,
        //         width : self.width,
        //         nums : soulution.to_owned(),
        //     };
        //     println!("{b}");
        // }
        // find first zero-index
        let index = soulution.iter().position(|num| *num == 0);
        if let Some(i) = index {
            // println!("{i}");
            // try all locally valid guesses on index
            let highest_num = (self.width * self.height) as u8;
            for num in 1..(highest_num+1) {
                if map[i].iter().any(|i| soulution[*i] == num) {continue;}
                soulution[i] = num;
                // solve recursivly
                if self.recursive_solve(soulution, map, i) {
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

    fn gen_map(&self) -> Vec<Vec<usize>> {
        let mut map : Vec<Vec<usize>> = vec![];
        let size = self.width * self.height;
        for index in 0..(size*size) {
            let mut index_set = vec![];
            { // row
                let start = index - index % size;
                let end = start + size;
                for i in start..end {
                    index_set.push(i);
                }
            }
            { // col
                for row in 0..size {
                    index_set.push(row * size + index % size);
                }
            }
            { // box
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

    fn valid_nums(&self, map : &Vec<Vec<&u8>>, index : usize) -> Vec<u8> {
        let size = self.height * self.width;
        let mut nums : Vec<usize> = (1..(size+1)).collect();
        map[index]
            .iter()
            .filter(|num| ***num != 0)
            .for_each(|num| nums[(**num as usize)-1] = 0);
        nums
            .iter()
            .filter(|num| **num != 0)
            .map(|num| *num as u8)
            .collect()
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
            0,0,0,0,0,0,0,0,0,
            0,0,0,0,0,0,0,0,0,
            0,0,0,0,0,0,0,0,0,
            0,0,0,0,0,0,0,0,0,
            0,0,0,0,0,2,0,0,0],
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
    let start_time = time::Instant::now();
    let solved = _b2.solve();
    let end_time = start_time.elapsed();
    println!("finished in {:?}",end_time);
    match solved {
        Some(solved) => println!("{solved}"),
        None => println!("Unable to solve"),
    }
}
