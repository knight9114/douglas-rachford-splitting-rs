mod projectors;
mod states;

use crate::projectors::{concur_projector, divide_projector, norm};
use crate::states::SatState;
use drs::prelude::{DivideAndConcurSolver, Result, Solver};
use rand::prelude::*;
use tracing_subscriber::{fmt, prelude::*, EnvFilter};

const NVARS: usize = 2;
const INDICES: [[usize; 3]; 3] = [[0, 0, 1], [0, 1, 1], [0, 1, 1]];
const NEGATINGS: [[bool; 3]; 3] = [
    [false, false, false],
    [true, true, true],
    [true, false, false],
];

fn main() -> Result<()> {
    tracing_subscriber::registry()
        .with(fmt::layer())
        .with(EnvFilter::from_default_env())
        .init();

    let states = create_sat_instance();
    let solver =
        DivideAndConcurSolver::new(divide_projector, concur_projector, norm, 1.0, 0.4, 1000);
    let (states, steps, delta) = solver.run(states)?;

    println!("Solved in {steps} steps, with delta={delta}");
    let solutions = states.solution();
    for (i, x) in solutions.into_iter().enumerate() {
        println!("var #{i} = {x}");
    }

    Ok(())
}

fn create_sat_instance() -> SatState {
    let mut rng = thread_rng();
    let vars: [f32; NVARS] = rng.gen();
    let indices: Vec<Vec<usize>> = INDICES.iter().map(Vec::from).collect();
    let negations: Vec<Vec<bool>> = NEGATINGS.iter().map(Vec::from).collect();

    SatState::new(Vec::from(vars), indices, negations)
}
