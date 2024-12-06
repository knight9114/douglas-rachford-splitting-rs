//! # Douglas-Rachford Splitting Algorithms
//!
//! This crate provides basic abstractions for using Douglas-Rachford Splitting, DRS, algorithms
//! for constrained optimization problems. These algorithms are general-purpose and can be
//! applicable to a wide variety of problems.
//!
//!
//! ## Introduction
//!
//!
//! ## Requirements
//!
//! Due to the varied applications of DRS algorithms, this crate makes heavy use of generics. For
//! most variants of the algorithms, there are two core components - problem state, projectors.
//!
//!
//! ### Problem State
//!
//! Problem states represent the current position of the optimization problem. For example, when
//! representing bits in a 3-SAT problem, the state will have a floating-point number for each bit,
//! representing how close that value is to `true` or `false`.
//!
//! The state needs to be meaningful to the problem as well as being embeddable in some other
//! mathematical space. The examples provided in this repository uses Euclidean space as the
//! mathematical state.
//!
//!
//! ### Projectors
//!
//! Projectors are functions that map inputs from one space into the other space. Most DRS
//! algorithms require two projectors - one that maps Euclidean states to solution states, and one
//! that maps solution states to Euclidean states. The first projector finds a solution that
//! satisfies each individual constraint without consideration for the others. The second takes
//! putative solutions for each constraint and merges them into a single estimated state.
//!
//! There is one major restriction on projectors, each projector must be *idempotent*. Simply put,
//! if you run the projector on the same input twice, the result should be the same as running on
//! the input once. Mathematically: $f(X) = f(f(X))$
//!
//!
//! ## Examples
//!
//! Included in the repository are examples of applying DRS to different problems. Currently
//! implemented:
//!    * `sudoku`: Solves one of the New York Times hard sudokus
//!    * `boolean-satisfiability`: Solves a simple 3-SAT instance with two variables and three
//!    clauses
//!
//! The examples' `main.rs` files contain more-detailed descriptions.

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
