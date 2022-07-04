#[derive(Clone, Copy, Debug)]
pub struct Tile {
    value: Option<u8>,
    blockers: [u8; 9],
}

impl Tile {
    pub fn new() -> Tile {
        Self {
            value:  None,
            blockers: [0; 9],
        }
    }
}

pub struct Board {
    state: [[Tile; 9]; 9],
}

impl Board {
    pub fn new() -> Board {
        Self {
            state: [[Tile::new(); 9]; 9],
        }
    }

    pub fn set_tile(&mut self, x: usize, y: usize, value: u8) {
        self.state[x][y].value = Some(value);
        // update_candidates
        for i in 0..9 {
            if y != i {
                self.state[x][i].blockers[(value - 1) as usize] += 1;
            };
            if x != i  {
                self.state[i][y].blockers[(value - 1) as usize] += 1;
            };
        }

        let x_start = (x / 3) * 3;
        let y_start = (y / 3) * 3;
        for i in x_start..x_start + 3 {
            for j in y_start..y_start + 3 {
                self.state[i][j].blockers[(value - 1) as usize] += 1;
            }
        }
    }

    pub fn clear_tile(&mut self, x: usize, y: usize) {
        let v = self.state[x][y].value;
        if let Some(set_val) = v {
            // update_candidates
            for i in 0..9 {
                if y != i {
                    self.state[x][i].blockers[(set_val - 1) as usize] -= 1;
                };
                if x != i  {
                    self.state[i][y].blockers[(set_val - 1) as usize] -= 1;
                };
            }

            let x_start = (x / 3) * 3;
            let y_start = (y / 3) * 3;
            for i in x_start..x_start + 3 {
                for j in y_start..y_start + 3 {
                    self.state[i][j].blockers[(set_val - 1) as usize] -= 1;
                }
            }

            self.state[x][y].value = None;
        } else {
            panic!("Cannot clear unset value")
        }
    }

    pub fn get_candidates(&self, x: usize, y: usize) -> Vec<u8> {
        let mut vec: Vec<usize> = Vec::with_capacity(9);
        let iter = self.state[x][y].blockers.iter().enumerate()
            .filter(|(_, &v)| v == 0)
            .map(|(i, _)| (i + 1));
        vec.extend(iter);
        return vec.iter().map(|v| *v as u8).collect()
    }

    pub fn get_tile(&self, x: usize, y: usize) -> Option<u8> {
        self.state[x][y].value
    }

    pub fn print(&self) {
        for i in self.state {
            print!(" ");
            for j in i {
                if let Some(n) = j.value {
                    print!("{} ", n);
                } else {
                    print!("? ");
                }
            }
            println!("");
        }
        println!("------------------");
    }

    pub fn print_candidate_count(&self) {
        for i in 0..9{
            print!(" ");
            for j in 0..9 {
                let c = self.get_candidates(i, j).len();
                print!("{} ", c);
            }
            println!("");
        }
        println!("------------------");
    }



    pub fn is_complete(&self) -> bool {
        for i in 0..9 {
            for j in 0..9 {
                if let None = self.state[i][j].value {
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
                sum_1 += self.state[i][j].value.unwrap();
                sum_2 += self.state[j][i].value.unwrap();
            }
            if sum_1 != 45 || sum_2 != 45 {
                return false;
            }
        }
        // check 3x3 squares
        for i in 0..3 {
            for j in 0..3 {
                let mut sum = 0;
                for k in i * 3..(i + 1) * 3 {
                    for l in j * 3..(j + 1) * 3 {
                        sum += self.state[k][l].value.unwrap();
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
