use crate::{errors::Error, Result, Solver, SolverSolution, State};
use tracing::{event, span, Level};

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
            let span = span!(tracing::Level::DEBUG, "divide_and_concur_outer_step");
            let _guard = span.enter();

            let update = step(state.clone(), &self.divide, &self.concur, self.beta)?;
            delta = (self.norm)(&update, &state);

            event!(Level::INFO, delta, step = t);
            event!(Level::DEBUG, ?state, ?update);

            if delta < self.epsilon {
                state = solution(state, &self.divide, &self.concur, self.beta)?;
                return Ok((state, t, delta));
            }

            state = update;
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
    let span = span!(tracing::Level::DEBUG, "divide_and_concur_inner_step");
    let _guard = span.enter();

    let gamma_a = -1f32 / beta;
    let gamma_b = 1f32 / beta;
    event!(Level::DEBUG, gamma_a);
    event!(Level::DEBUG, gamma_b);

    let fa = concur(state.clone())? * (1.0 + gamma_a) + state.clone() * -gamma_a;
    let fb = divide(state.clone())? * (1.0 + gamma_b) + state.clone() * -gamma_b;
    event!(Level::DEBUG, ?fa);
    event!(Level::DEBUG, ?fb);

    let pafb = concur(fb)?;
    let pbfa = divide(fa)?;
    event!(Level::DEBUG, ?pafb);
    event!(Level::DEBUG, ?pbfa);

    let inner = pafb + pbfa * -1f32;
    event!(Level::DEBUG, ?inner);

    let result = state + inner * beta;
    event!(Level::DEBUG, ?result);

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
