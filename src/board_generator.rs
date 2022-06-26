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

#[derive(Debug, Copy, Clone)]
struct TileGenerationState {
    candidate_count: u8,
    candidates: [u8; 9],
}

impl TileGenerationState {
    pub fn new() -> TileGenerationState {
        TileGenerationState {
            candidates: [1,2,3,4,5,6,7,8,9],
            candidate_count: 9,
        }
    }

    pub fn get_candidates(&mut self, index: usize) -> &[u8] {
        &self.candidates[0..self.candidate_count as usize]
    }

    pub fn remove_candidate_value(&mut self, value: u8) {
        for i in 0..self.candidate_count as usize {
            if self.candidates[i] == value {
                self.candidates[i] = self.candidates[self.candidate_count as usize - 1];
                self.candidate_count -= 1;
                break;
            }
        }
    }

    pub fn elect_candidate_at_index(&mut self, index: usize) -> u8{
        self.candidates[0] = self.candidates[index as usize];
        self.candidate_count = 1;
        self.candidates[0]
    }
}


pub fn generate_board(params: GenerationParams) -> Board {
    let mut board = Board::new();
    let mut rng: Pcg64 = Seeder::from(params.seed).make_rng();
    
    let mut candidate_board: [[TileGenerationState; 9]; 9] = [[TileGenerationState::new(); 9]; 9];

    for i in 0..9 {
        for j in 0..9 {
            let tile_generation_state = &mut candidate_board[i][j];
            println!("Tile state for {} {} {:?}", i, j, tile_generation_state);
            let index = (rng.next_u32() % tile_generation_state.candidate_count as u32) as usize;
            let value = tile_generation_state.elect_candidate_at_index(index);
            board.set_tile(i, j, value);

            // remove the selected value from the current row and column
            for k in 0..9 {
                if k != i {
                    candidate_board[k][j].remove_candidate_value(value);
                }
                if k != j {
                    candidate_board[i][k].remove_candidate_value(value);
                }
            }

            // remove the value from this 9x9 segment
            let segment_row_start = (i / 3) * 3;
            let segment_column_start = (j / 3) * 3;

            for k in segment_row_start..segment_row_start + 3 {
                for l in segment_column_start..segment_column_start + 3 {
                    if k != i && l != j {
                        println!("Removing {} from {} {}", value, k, l);
                        candidate_board[k][l].remove_candidate_value(value);
                    }
                }
            }
        }
    }
       
    board
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn board_generates() {
        let board = generate_board(GenerationParams::default());
        assert_eq!(board.validate(), true);
    }
}
