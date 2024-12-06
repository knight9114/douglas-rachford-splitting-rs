mod projectors;
mod states;

use crate::{
    projectors::{concur_projector, divide_projector, norm},
    states::SudokuState,
};
use drs::prelude::{DivideAndConcurSolver, Result, Solver};

// This puzzle is taken from the New York Times Hard Sudoku from 5 Dec 2024
#[rustfmt::skip]
const PUZZLE: [usize; 81] = [
    // 7 _ _ | _ _ _ | _ 5 9
    // 2 _ 6 | _ _ _ | _ _ _
    // 4 _ _ | 8 _ _ | _ _ 1
    // ------+-------+------
    // _ 3 _ | _ _ _ | 9 6 _
    // _ _ _ | _ 4 _ | _ 3 _
    // _ _ _ | _ _ 5 | 7 _ _
    // ------+-------+------
    // _ _ _ | _ 2 _ | 8 _ _
    // _ 8 _ | 1 _ 6 | _ _ _
    // _ _ 5 | _ _ 3 | _ 2 _

    7,0,0, 0,0,0, 0,5,9,
    2,0,6, 0,0,0, 0,0,0,
    4,0,0, 8,0,0, 0,0,1,

    0,3,0, 0,0,0, 9,6,0,
    0,0,0, 0,4,0, 0,3,0,
    0,0,0, 0,0,5, 7,0,0,

    0,0,0, 0,2,0, 8,0,0,
    0,8,0, 1,0,6, 0,0,0,
    0,0,5, 0,0,3, 0,2,0,
];

fn main() -> Result<()> {
    let states = SudokuState::from(PUZZLE);
    let solver =
        DivideAndConcurSolver::new(divide_projector, concur_projector, norm, 0.9, 1.0, 100000);
    let (states, steps, delta) = solver.run(states)?;
    let solutions = states.solution();

    println!("Solved in {steps} steps, with delta={delta}");
    for (r, row) in solutions.iter().enumerate() {
        for (c, val) in row.iter().enumerate() {
            print!("{val} ");
            if c % 3 == 2 && c != 8 {
                print!("| ");
            }
        }
        println!();
        if r % 3 == 2  && r != 8 {
            println!("------+-------+------");
        }
    }
    Ok(())
}
