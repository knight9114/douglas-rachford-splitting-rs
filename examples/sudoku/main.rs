#![allow(unused_imports, unused_variables, dead_code, unused_mut)]
mod projectors;
mod states;

use crate::{
    projectors::{concur_projector, divide_projector, iroot, norm},
    states::SudokuState,
};
use drs::prelude::{divide_and_concur_solution, divide_and_concur_step, Result, Solver, State};

const EASY: [usize; 81] = [
    // 5 6 ? | 9 ? 2 | 1 ? ?
    // 8 ? ? | 5 3 ? | ? ? 4
    // 9 7 ? | ? ? 1 | 2 5 6
    // ------+-------+------
    // 6 1 ? | ? 8 ? | 9 3 ?
    // ? ? 8 | ? 9 5 | ? ? ?
    // ? ? ? | ? ? 3 | ? 7 2
    // ------+-------+------
    // ? ? ? | ? ? 9 | 4 2 7
    // 3 9 2 | 4 1 ? | ? ? ?
    // ? ? 5 | 6 ? ? | ? ? 9
    5, 6, 0, 9, 0, 2, 1, 0, 0, 8, 0, 0, 5, 3, 0, 0, 0, 4, 9, 7, 0, 0, 0, 1, 2, 5, 6, 6, 1, 0, 0, 8,
    0, 9, 3, 0, 0, 0, 8, 0, 9, 5, 0, 0, 0, 0, 0, 0, 0, 0, 3, 0, 7, 2, 0, 0, 0, 0, 0, 9, 4, 2, 7, 3,
    9, 2, 4, 1, 0, 0, 0, 0, 0, 0, 5, 6, 0, 0, 0, 0, 9,
];

fn main() -> Result<()> {
    let mut states = SudokuState::from(EASY);

    Ok(())
}
