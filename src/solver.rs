use crate::board::Board;
use rand_chacha::rand_core::RngCore;
use rand_pcg::Pcg64;
use rand_seeder::Seeder;

pub fn solve(board: &mut Board) {
    let mut empty_positions: Vec<(usize, usize)> = vec![];
    let mut rng: Pcg64 = Seeder::from(1).make_rng();

    for i in 0..9 {
        for j in 0..9 {
            if board.get_tile(i, j) == None {
                empty_positions.push((i, j));
            }
        }
    }

    let mut pointer = 0;

    let mut backtrack_blocklist = [[[false; 9]; 9]; 9];

    while pointer < empty_positions.len() {
        let cur = empty_positions.get(pointer).unwrap();
        let candidates: Vec<u8> = board.get_candidates(cur.0, cur.1)
            .iter()
            .filter(|v| !backtrack_blocklist[cur.0][cur.1][(*v - 1) as usize])
            .map(|v| *v)
            .collect();

        if candidates.len() > 0 {
            let index = (rng.next_u32() % candidates.len() as u32) as usize;
            let winner = candidates.get(index).unwrap();
            board.set_tile(cur.0, cur.1, *winner);
            backtrack_blocklist[cur.0][cur.1][(*winner - 1) as usize] = true;
            pointer += 1;
        } else {
            for i in 0..9 {
                backtrack_blocklist[cur.0][cur.1][i] = false;
            }

            // walk back 1, clear that tiles current value and blacklist that value
            pointer -= 1;
            let cur = empty_positions.get(pointer).unwrap();
            board.clear_tile(cur.0, cur.1);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn generates_a_valid_board() {
        let mut board = Board::new();
        solve(&mut board);
        assert_eq!(board.is_complete(), true);
    }

    #[test]
    fn solve_partial() {
        let i = [
            "5????8??3",
            "83?946??7",
            "?7213?9??",
            "1?769??32",
            "?9?812???",
            "?8?35??96",
            "4152?9?78",
            "?????1?29",
            "?2?47?5??",
        ];

        let mut board = Board::new();

        for (i, &s) in i.iter().enumerate() {
            for (j, v) in s.split("").enumerate() {
                if v != "?" && v.len() > 0 {
                    board.set_tile(i, j - 1, u8::from_str_radix(v, 10).unwrap())
                }
            }
        }

        solve(&mut board);

        assert_eq!(board.is_complete(), true);
    }
}
