//! # Douglas-Rachford Splitting Algorithms
//!
//! This crate provides basic abstractions for using Douglas-Rachford Splitting, DRS, algorithms
//! for constrained optimization problems. These algorithms are general-purpose and can be
//! applicable to a wide variety of problems. For more details on DRS algorithsm, consult the
//! following resources:
//!    * [*Difference-map algorithm*](https://en.wikipedia.org/wiki/Difference-map_algorithm)
//!    * [*Remembering the Douglas-Rachford iteration*](https://regularize.wordpress.com/2014/07/09/remembering-the-douglas-rachford-iteration/)
//!    * [*Augmented Lagrangian method*](https://en.wikipedia.org/wiki/Augmented_Lagrangian_method)
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
//!      clauses
//!
//! The examples' `main.rs` files contain more-detailed descriptions.
//!
//! A completely minimal (and not useful at all) example. The state is represented by a simple
//! tuple of floating-point numbers, and the projections are the simple identity functions.
//!
//! ```rust
//! use drs::prelude::*;
//!
//! #[derive(Debug, Clone)]
//! struct MyState(f32, f32, f32);
//!
//! impl Add for MyState {
//!     type Output = Self;
//!     fn add(self, rhs: Self) -> Self {
//!         Self(self.0 + rhs.0, self.1 + rhs.1, self.2 + rhs.2)
//!     }
//! }
//!
//! impl Mul<f32> for MyState {
//!     type Output = Self;
//!     fn mul(self, rhs: f32) -> Self {
//!         Self(self.0 * rhs, self.1 * rhs, self.2 * rhs)
//!     }
//! }
//!
//! impl State for MyState {}
//!
//! fn proj(state: MyState) -> DrsResult<MyState> {
//!     Ok(state)
//! }
//!
//! fn norm(current: &MyState, previous: &MyState) -> f32 {
//!     0f32
//! }
//!
//! let state = MyState(1f32, 2f32, 3f32);
//! let solver = DivideAndConcurSolver::new(proj, proj, norm, 1.0, 0.1, 100);
//! match solver.run(state) {
//!     Ok((solution, steps, delta)) => println!("solution={solution:?}, steps={steps},
//!     delta={delta}"),
//!     Err(err) => eprintln!("{err}"),
//! }
//! ```

pub mod errors;
pub mod prelude;
pub mod solvers;

use std::ops::{Add, Mul};

/// A specialized [`Result`] type for DRS operations.
///
/// This is used to create a unified interface for the user-provided functions. The
/// [`DrsError::Projection`] error can wrap any error from the user projection functions.
///
/// [`Result`]: crate::Result
/// [`DrsError::Projection`]: crate::errors::DrsError
pub type Result<T> = std::result::Result<T, crate::errors::DrsError>;

/// Represents the output of a successful run of the solver.
///
/// The elements are:
///    1. Solution
///    2. Steps required for the success
///    3. Difference between the current and previous state
pub type SolverSolution<T> = (T, usize, f32);

/// Basic operations required by the [`Solver`]
///
/// [`Solver`]: crate::Solver
pub trait State: Clone + std::fmt::Debug + Add<Output = Self> + Mul<f32, Output = Self> {}

/// Defines required behaviors to use a [`Solver`]
///
/// The names for the trait bounds are inspired by the first algorithm that was implemented,
/// *Divide and Concur*. The four required bounds are:
///    * **D**: Projector mapping the state from Euclidean space to solution space
///    * **C**: Projector mapping the state from solution space to Euclidean space
///    * **N**: Computes the difference between the current state and the previous state
///    * **S**: This is only used to define the **D**, **C**, and **N** bounds
///
/// [`Solver`]: crate::Solver
pub trait Solver<S, D, C, N>
where
    S: State,
    D: Fn(S) -> Result<S>,
    C: Fn(S) -> Result<S>,
    N: Fn(&S, &S) -> f32,
{
    /// Runs the solver on the initial state
    fn run(&self, initial_state: S) -> Result<SolverSolution<S>>;
}
