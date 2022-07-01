pub struct Board {
    state: [[Option<u8>; 9]; 9],
}

impl Board {
    pub fn new() -> Board {
        Self {
            state: [[None; 9]; 9]
        }
    }
    pub fn set_tile(&mut self, x: usize, y: usize, value: u8) {
        println!("Setting tile at ({}, {}) to {}", x, y, value);
        self.state[x][y] = Some(value);
    }

    pub fn clear_tile(&mut self, x: usize, y: usize) {
        self.state[x][y] = None;
    }

    pub fn get_tile(&self, x: usize, y: usize) -> Option<u8> {
        return self.state[x][y];
    }

    pub fn print(&self) {
        for i in self.state {
           println!("");
           print!(" ");
           for j in i {
                if let Some(n) = j {
                print!("{} ", n);
               } else {
                print!( "? ");
               }
           } 
        }
    }

    pub fn validate(&self) -> bool {
        for i in 0..9 {
            for j in 0..9 {
                if let None = self.state[i][j] {
                    return false;
                } 
            }
        }
        // check sodoku board is valid
        // check rows
        for i in 0..9 {
            let mut sum_1 = 0;
            let mut sum_2 = 0;
            for j in 0..9 {
                sum_1 += self.state[i][j].unwrap();
                sum_2 += self.state[j][i].unwrap();
            }
            if sum_1 != 45 || sum_2 != 45 {
                return false;
            }
        }
        // check 3x3 squares
        for i in 0..3 {
            for j in 0..3 {
                let mut sum = 0;
                for k in i*3..(i+1) * 3 {
                    for l in j*3..(j+1) * 3 {
                        sum += self.state[k][l].unwrap();
                    }
                }
                if sum != 45 {
                    return false;
                }
            }
        }
        true
    }
}


