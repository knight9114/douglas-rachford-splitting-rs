use drs::State;
use rand::prelude::*;
use std::ops::{Add, Mul};

#[derive(Debug, Clone)]
pub struct Clause {
    pub values: Vec<f32>,
    pub indices: Vec<usize>,
    pub negating: Vec<f32>,
}

impl Clause {
    fn extract_variables(&self) -> Vec<f32> {
        self.indices.iter()
            .zip(&self.negating)
            .map(|(&i, &n)| self.values[i] * n)
            .collect()
    }

    pub fn solve(self) -> Self {
        let extracted = self.extract_variables();
        let solved = solve_clause(extracted)
            .into_iter()
            .zip(&self.negating)
            .map(|(v, &n)| v * n)
            .collect();

        Self {
            values: solved,
            indices: self.indices,
            negating: self.negating,
        }
    }
}

#[derive(Debug, Clone)]
pub struct SatState {
    pub clauses: Vec<Clause>,
    pub nvars: usize,
}

impl Add for SatState {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        Self {
            clauses: self.clauses.into_iter()
                .zip(rhs.clauses.into_iter())
                .map(|(left, right)| {
                    Clause {
                        values: left.values.into_iter()
                            .zip(right.values.into_iter())
                            .map(|(l, r)| l + r)
                            .collect(),
                        indices: left.indices,
                        negating: left.negating,
                    }
                })
                .collect(),
            nvars: self.nvars,
        }
    }
}

impl Mul<f32> for SatState {
    type Output = Self;

    fn mul(self, rhs: f32) -> Self {
        Self {
            clauses: self.clauses.into_iter()
                .map(|left| {
                    Clause {
                        values: left.values.into_iter()
                            .map(|l| l * rhs)
                            .collect(),
                        indices: left.indices,
                        negating: left.negating,
                    }
                })
                .collect(),
            nvars: self.nvars,
        }
    }
}

impl State for SatState {}

fn solve_clause(clause: Vec<f32>) -> Vec<f32> {
    let mut putative: Vec<f32> = clause.iter()
        .map(|&v| v.signum())
        .collect();

    if putative.iter().all(|&f| f.is_sign_negative()) {
        let idx = argmax(&clause[..]);
        putative[idx] = 1f32;
    }

    putative
}

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

    #[test]
    fn test_solve_clause() {
        let truth = vec![1f32, -1f32, -1f32];
        let solved = solve_clause(vec![1f32, -1f32, -1f32]);
        assert_eq!(truth, solved);

        let truth = vec![-1f32, 1f32, -1f32];
        let solved = solve_clause(vec![-0.9, -0.2, -0.8]);
        assert_eq!(truth, solved);
    }
}
