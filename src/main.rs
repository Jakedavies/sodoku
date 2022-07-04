mod board;
mod solver;

fn main() {
    let mut boards: Vec<board::Board>= Vec::with_capacity(10000);

    for _ in 0..boards.len() {
        let mut b = board::Board::new();
        solver::solve(&mut b);
        boards.push(b);
    }

}
