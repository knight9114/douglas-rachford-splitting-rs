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

            //info!(target: "drs_solver_step", delta = delta, step = t; "divide_and_concur_step");
            //trace!(target: "drs_solver_step", state:? = state, update:? = state; "divide_and_concur_states");

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
    let gamma_a = -1f32 / beta;
    let gamma_b = 1f32 / beta;
    //trace!(target: "drs_solver_step", gamma_a = gamma_a; "divide_and_concur_step: gamma_a");
    //trace!(target: "drs_solver_step", gamma_b = gamma_b; "divide_and_concur_step: gamma_b");

    let fa = concur(state.clone())? * (1.0 + gamma_a) + state.clone() * -gamma_a;
    let fb = divide(state.clone())? * (1.0 + gamma_b) + state.clone() * -gamma_b;
    //trace!(target: "drs_solver_step", fa:? = fa; "divide_and_concur_step: fa");
    //trace!(target: "drs_solver_step", fb:? = fb; "divide_and_concur_step: fb");

    let pafb = concur(fb)?;
    let pbfa = divide(fa)?;
    //trace!(target: "drs_solver_step", pafb:? = pafb; "divide_and_concur_step: pafb");
    //trace!(target: "drs_solver_step", pbfa:? = pbfa; "divide_and_concur_step: pbfa");

    let inner = pafb + pbfa * -1f32;
    //trace!(target: "drs_solver_step", inner:? = inner; "divide_and_concur_step: inner");

    let result = state + inner * beta;
    //trace!(target: "drs_solver_step", result:? = result; "divide_and_concur_step: result");

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
