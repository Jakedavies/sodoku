
pub struct Board {
    state: [[u8; 9]; 9],
}

impl Board {
    pub fn new() -> Board {
        Self {
            state: [[0; 9]; 9]
        }
    }
    pub fn set_tile(&mut self, x: usize, y: usize, value: u8) {
        println!("Setting tile at ({}, {}) to {}", x, y, value);
        self.state[x][y] = value;
    }

    pub fn validate(&self) -> bool {
        // check sodoku board is valid
        // check rows
        for i in 0..9 {
            let mut sum_1 = 0;
            let mut sum_2 = 0;
            for j in 0..9 {
                sum_1 += self.state[i][j];
                sum_2 += self.state[j][i];
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
                        sum += self.state[k][l];
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


