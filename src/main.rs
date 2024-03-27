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


fn main() {
    let b1 = Board {
        height : 2,
        width : 2,
        nums : vec![1,2,3,4,1,2,3,4,1,2,3,4,1,2,3,4],
    };
    let b2 = Board {
        height : 3,
        width : 4,
        nums : vec![
            1,2,3,4,5,6,7,8,9,10,11,12,
            0,0,0,0,0,0,0,0,0,0,0,0,
            0,0,0,0,0,0,0,0,0,0,0,0,
            0,0,0,0,0,0,0,0,0,0,0,0,
            0,0,0,0,0,0,0,0,0,0,0,0,
            0,0,0,0,0,0,0,0,0,0,0,0,
            0,0,0,0,0,0,0,0,0,0,0,0,
            0,0,0,0,0,0,0,0,0,0,0,0,
            0,0,0,0,0,0,0,0,0,0,0,0,
            0,0,0,0,0,0,0,0,0,0,0,0,
            0,0,0,0,0,0,0,0,0,0,0,0,
            0,0,0,0,11,0,0,0,0,0,0,0],
    };
    let b3 = Board {
        height : 2,
        width : 3,
        nums : vec![1,2,3,4,5,6,
            1,2,3,4,5,6,
            1,2,3,4,5,6,
            1,2,3,4,5,6,
            1,2,3,4,5,6,
            1,2,3,4,5,6],
    };
    println!("{b1}");
    println!("{b2}");
    println!("{b3}");
}
