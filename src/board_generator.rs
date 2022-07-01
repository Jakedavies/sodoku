#![feature(array_map)]
use super::board::Board;
use rand::prelude::*;
use rand_chacha::rand_core::RngCore;
use rand_seeder::Seeder;
use rand_pcg::Pcg64;

pub enum Difficulty {
    Hard,
    Medium,
    Easy,
    Solved
}

pub struct GenerationParams {
    seed: i32,
    difficulty: Difficulty,
}

impl GenerationParams {
    pub fn new(seed: i32, difficulty: Difficulty) -> GenerationParams {
        GenerationParams {
            seed,
            difficulty
        }
    }
}

impl Default for GenerationParams {
    fn default() -> GenerationParams {
        GenerationParams::new(0, Difficulty::Medium)
    }
}



#[derive(Debug, Clone)]
struct TileGenerationState {
    candidates: Vec<u8>,
    side_effects: Vec<usize>,
}


impl TileGenerationState {
    pub fn new() -> TileGenerationState {
        TileGenerationState {
            candidates: vec![1,2,3,4,5,6,7,8,9],
            side_effects: vec![],
        }
    }

    pub fn get_candidates(&self) -> &Vec<u8> {
        &self.candidates
    }

    pub fn remove_candidate_value(&mut self, value: u8) -> bool {
        let index = self.candidates.iter().position(|f| *f == value);
        if let Some(index) = index {
            self.candidates.swap_remove(index);
            return true
        }
        false
    }

    pub fn add_side_effect(&mut self, p: usize) {
       self.side_effects.push(p);
    }
}

// if we have available candidates, pick one
// if we do not, go back to previous node and remove the previously chosen candidate from the available pool. 
// for every number at every position, we represent its "constraint" space using a 64 bit number. 9
// bits for 
//

fn pointer_to_x_y(p: usize) -> (usize, usize) {
    return (p / 9, p % 9)
}
 

pub fn generate_board(params: GenerationParams) -> Board {
    let mut board = Board::new();
    let mut rng: Pcg64 = Seeder::from(params.seed).make_rng();
    
    let mut candidate_board: [TileGenerationState; 81] = [(); 81].map(|_| TileGenerationState::new());

    let mut pointer = 0;
    let mut iterations = 0;
    while pointer < candidate_board.len() && iterations < 500 {
        let tile_generation_state = &mut candidate_board[pointer];
        println!("Filling tile {}", pointer);
        let (x, y) = pointer_to_x_y(pointer);
        if tile_generation_state.get_candidates().len() == 0 {
            candidate_board[pointer] = TileGenerationState::new();
            pointer -= 1;
            let (x, y) = pointer_to_x_y(pointer);
            let current = board.get_tile(x, y).unwrap();
            candidate_board[pointer].remove_candidate_value(current);
            // undo the side effects of this assignment
            for side_effect in &candidate_board[pointer].side_effects {
                candidate_board[*side_effect].candidates.push(current);
            }
            candidate_board[pointer].side_effects.clear();
            // set current to none
            board.clear_tile(x, y);
        } else {
            let candidates = tile_generation_state.get_candidates();
            let index = (rng.next_u32() % candidates.len() as u32) as usize;
            let value = *candidates.get(index).unwrap();
            board.set_tile(x, y, value);

            // remove the selected value from the current row and column
            for k in 0..9 {
                if k != y {
                    let removed = candidate_board[x * 9 + k].remove_candidate_value(value);
                    if removed {
                        candidate_board[pointer].add_side_effect(x * 9 + k)
                    }
                }
            }
            for k in 0..9 {
                if k != x {
                    let removed = candidate_board[k * 9  + y].remove_candidate_value(value);
                    if removed {
                        candidate_board[pointer].add_side_effect(k * 9 + y)
                    }
                }
            }

            // remove the value from this 9x9 segment
            let segment_row_start = (x / 3) * 3;
            let segment_column_start = (y / 3) * 3;

            for i in segment_row_start..segment_row_start + 3 {
                for j in segment_column_start..segment_column_start + 3 {
                    if i != x && j != y {
                        println!("Removing {} from {} {}", value, i, j);
                        let removed = candidate_board[i * 9 + j].remove_candidate_value(value);
                        if removed {
                            candidate_board[pointer].add_side_effect(i * 9 + j)
                        }
                    }
                }
            }
            pointer += 1
        }
        iterations += 1;
    }
    println!("{:?}", candidate_board);
    board
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn board_generates() {
        let board = generate_board(GenerationParams::default());
        board.print();
        assert_eq!(board.validate(), true);
    }
}
