use drs::State;
use pathfinding::num_traits::Float;
use rand::prelude::*;
use std::ops::{Add, Mul};

#[derive(Debug, Clone)]
pub struct Clause {
    pub values: Vec<f32>,
    pub indices: Vec<usize>,
    pub negating: Vec<bool>,
    n: usize,
}

impl Clause {
    pub fn new(variables: &[f32], indices: Vec<usize>, negating: Vec<bool>) -> Self {
        let values = indices.iter().map(|&i| variables[i]).collect();
        Self {
            values,
            indices,
            negating,
            n: variables.len(),
        }
    }

    pub fn solve(self) -> Self {
        let values: Vec<f32> = self
            .values
            .iter()
            .zip(&self.negating)
            .map(|(&val, &neg)| if neg { -1.0 * val} else { val })
            .collect();

        let mut putative: Vec<f32> = values
            .iter()
            .map(|&val| if val < 0.0 { -1.0 } else { 1.0 })
            .collect();

        if putative.iter().all(|&v| v < 0.0) {
            let mut costs = vec![0f32; self.n];
            for (&i, &v) in self.indices.iter().zip(values.iter()) {
                costs[i] += v;
            }

            let idx = argmax(&costs[..]);
            for (i, &j) in self.indices.iter().enumerate() {
                if j == idx {
                    putative[i] = 1f32;
                }
            }
        }

        let solution = putative
            .into_iter()
            .zip(&self.negating)
            .map(|(val, &neg)| if neg { -1.0 * val} else { val })
            .collect();

        Self {
            values: solution,
            indices: self.indices,
            negating: self.negating,
            n: self.n,
        }
    }
}

impl Add for Clause {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        Self {
            values: self
                .values
                .into_iter()
                .zip(rhs.values)
                .map(|(l, r)| l + r)
                .collect(),
            indices: self.indices,
            negating: self.negating,
            n: self.n,
        }
    }
}

impl Mul<f32> for Clause {
    type Output = Self;

    fn mul(self, rhs: f32) -> Self {
        Self {
            values: self.values.into_iter().map(|l| l * rhs).collect(),
            indices: self.indices,
            negating: self.negating,
            n: self.n,
        }
    }
}

#[derive(Debug, Clone)]
pub struct SatState {
    pub clauses: Vec<Clause>,
    pub nvars: usize,
}

impl SatState {
    pub fn new(variables: Vec<f32>, indices: Vec<Vec<usize>>, negating: Vec<Vec<bool>>) -> Self {
        let nvars = variables.len();
        let clauses = indices
            .into_iter()
            .zip(negating)
            .map(|(i, n)| Clause::new(&variables[..], i, n))
            .collect();

        Self { clauses, nvars }
    }

    pub fn solution(&self) -> Vec<bool> {
        let mut output = vec![f32::NAN; self.nvars];
        for clause in &self.clauses {
            for (&i, &x) in clause.indices.iter().zip(clause.values.iter()) {
                if !output[i].is_nan() && output[i] != x {
                    println!("{:#?}", self);
                    panic!("inconsistent results");
                }
                output[i] = x;
            }
        }

        if output.iter().any(|&v| v.is_nan()) {
            panic!("failed to set all variables");
        }

        output.into_iter()
            .map(|v| v == 1.0)
            .collect()
    }
}

impl Add for SatState {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        Self {
            clauses: self
                .clauses
                .into_iter()
                .zip(rhs.clauses)
                .map(|(l, r)| l + r)
                .collect(),
            nvars: self.nvars,
        }
    }
}

impl Mul<f32> for SatState {
    type Output = Self;

    fn mul(self, rhs: f32) -> Self {
        Self {
            clauses: self.clauses.into_iter().map(|l| l * rhs).collect(),
            nvars: self.nvars,
        }
    }
}

impl State for SatState {}

fn argmax(vars: &[f32]) -> usize {
    let (idx, _) = vars.iter()
        .enumerate()
        .fold((0, vars[0]), |(imax, vmax), (i, &v)| {
            if vmax > v {
                (imax, vmax)
            } else {
                (i, v)
            }
        });
    idx
}

#[cfg(test)]
mod tests {
    use super::*;

    const VARS_1: [f32; 2] = [-0.2, 0.7];
    const VARS_2: [f32; 2] = [-0.2, -0.7];
    const VARS_3: [f32; 2] = [0.2, 0.7];
    const VARS_4: [f32; 2] = [0.2, -0.7];
    const INDICES: [[usize; 3]; 3] = [
        [0, 0, 1],
        [0, 1, 1],
        [0, 1, 1],
    ];
    const NEGATINGS: [[bool; 3]; 3] = [
        [false, false, false],
        [true, true, true],
        [true, false, false],
    ];

    #[test]
    fn test_clause_solve() {
        let vars = Vec::from(VARS_1);
        let indices: Vec<Vec<usize>> = INDICES.iter().map(Vec::from).collect();
        let negations: Vec<Vec<bool>> = NEGATINGS.iter().map(Vec::from).collect();
        let state = SatState::new(vars, indices, negations);
        let solutions: Vec<Clause> = state.clauses
            .into_iter()
            .map(Clause::solve)
            .collect();
        assert_eq!(solutions[0].values, vec![-1.0, -1.0, 1.0]);
        assert_eq!(solutions[1].values, vec![-1.0, 1.0, 1.0]);
        assert_eq!(solutions[2].values, vec![-1.0, 1.0, 1.0]);

        let vars = Vec::from(VARS_2);
        let indices: Vec<Vec<usize>> = INDICES.iter().map(Vec::from).collect();
        let negations: Vec<Vec<bool>> = NEGATINGS.iter().map(Vec::from).collect();
        let state = SatState::new(vars, indices, negations);
        let solutions: Vec<Clause> = state.clauses
            .into_iter()
            .map(Clause::solve)
            .collect();
        assert_eq!(solutions[0].values, vec![1.0, 1.0, -1.0]);
        assert_eq!(solutions[1].values, vec![-1.0, -1.0, -1.0]);
        assert_eq!(solutions[2].values, vec![-1.0, -1.0, -1.0]);

        let vars = Vec::from(VARS_3);
        let indices: Vec<Vec<usize>> = INDICES.iter().map(Vec::from).collect();
        let negations: Vec<Vec<bool>> = NEGATINGS.iter().map(Vec::from).collect();
        let state = SatState::new(vars, indices, negations);
        let solutions: Vec<Clause> = state.clauses
            .into_iter()
            .map(Clause::solve)
            .collect();
        assert_eq!(solutions[0].values, vec![1.0, 1.0, 1.0]);
        assert_eq!(solutions[1].values, vec![-1.0, 1.0, 1.0]);
        assert_eq!(solutions[2].values, vec![1.0, 1.0, 1.0]);

        let vars = Vec::from(VARS_4);
        let indices: Vec<Vec<usize>> = INDICES.iter().map(Vec::from).collect();
        let negations: Vec<Vec<bool>> = NEGATINGS.iter().map(Vec::from).collect();
        let state = SatState::new(vars, indices, negations);
        let solutions: Vec<Clause> = state.clauses
            .into_iter()
            .map(Clause::solve)
            .collect();
        assert_eq!(solutions[0].values, vec![1.0, 1.0, -1.0]);
        assert_eq!(solutions[1].values, vec![1.0, -1.0, -1.0]);
        assert_eq!(solutions[2].values, vec![-1.0, -1.0, -1.0]);
    }
}
