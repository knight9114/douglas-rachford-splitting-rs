use crate::{errors::Error, Result, Solver, SolverSolution, State};

pub struct DivideAndConcurSolver<S, D, C, N>
where
    S: State,
    D: Fn(S) -> Result<S>,
    C: Fn(S) -> Result<S>,
    N: Fn(&S, &S) -> f32,
{
    divide: D,
    concur: C,
    norm: N,
    beta: f32,
    epsilon: f32,
    n_steps: usize,
    _marker: std::marker::PhantomData<S>,
}

impl<S, D, N, C> DivideAndConcurSolver<S, D, C, N>
where
    S: State,
    D: Fn(S) -> Result<S>,
    C: Fn(S) -> Result<S>,
    N: Fn(&S, &S) -> f32,
{
    pub fn new(divide: D, concur: C, norm: N, beta: f32, epsilon: f32, n_steps: usize) -> Self {
        Self {
            divide,
            concur,
            norm,
            beta,
            epsilon,
            n_steps,
            _marker: std::marker::PhantomData,
        }
    }
}

impl<S, D, N, C> Solver<S, D, C, N> for DivideAndConcurSolver<S, D, C, N>
where
    S: State,
    D: Fn(S) -> Result<S>,
    C: Fn(S) -> Result<S>,
    N: Fn(&S, &S) -> f32,
{
    fn run(&self, initial_state: S) -> Result<SolverSolution<S>> {
        let mut state = initial_state;
        let mut delta = f32::NAN;

        for t in 0..self.n_steps {
            let update = step(state.clone(), &self.divide, &self.concur, self.beta)?;
            delta = (self.norm)(&update, &state);

            if delta < self.epsilon {
                state = solution(state, &self.divide, &self.concur, self.beta)?;
                return Ok((state, t, delta));
            }
        }

        Err(Error::Convergence(self.n_steps, delta))
    }
}

pub fn step<S, D, C>(state: S, divide: D, concur: C, beta: f32) -> Result<S>
where
    S: State,
    D: Fn(S) -> Result<S>,
    C: Fn(S) -> Result<S>,
{
    let gamma_a = -1f32 / beta;
    let gamma_b = 1f32 / beta;

    let fa = concur(state.clone())? * (1.0 + gamma_a) + state.clone() * -gamma_a;
    let fb = divide(state.clone())? * (1.0 + gamma_b) + state.clone() * -gamma_b;

    let inner = concur(fb)? + divide(fa)? * -1f32;
    let result = state + inner * beta;

    Ok(result)
}

pub fn solution<S, D, C>(state: S, divide: D, concur: C, beta: f32) -> Result<S>
where
    S: State,
    D: Fn(S) -> Result<S>,
    C: Fn(S) -> Result<S>,
{
    let gamma_a = -1f32 / beta;
    let fa = concur(state.clone())? * (1.0 + gamma_a) + state.clone() * -gamma_a;
    divide(fa)
}
