pub mod errors;
pub mod prelude;
pub mod solvers;

use std::ops::{Add, Mul};

pub type Result<T> = std::result::Result<T, crate::errors::Error>;
pub type SolverSolution<T> = (T, usize, f32);

pub trait State: Clone + std::fmt::Debug + Add<Output = Self> + Mul<f32, Output = Self> {}

pub trait Solver<S, D, C, N>
where
    S: State,
    D: Fn(S) -> Result<S>,
    C: Fn(S) -> Result<S>,
    N: Fn(&S, &S) -> f32,
{
    fn run(&self, initial_state: S) -> Result<SolverSolution<S>>;
}
