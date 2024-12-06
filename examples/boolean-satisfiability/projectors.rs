use crate::states::{Clause, SatState};
use drs::{errors::Error, Result};
use pathfinding::num_traits::{float::FloatCore, Float};

pub fn divide_projector(state: SatState) -> Result<SatState> {
    let solutions = state.clauses.into_iter()
        .map(Clause::solve)
        .collect();

    Ok(SatState {
        clauses: solutions,
        nvars: state.nvars,
    })
}

pub fn concur_projector(state: SatState) -> Result<SatState> {
    let mut values = vec![vec![f32::NAN; state.nvars]; state.clauses.len()];
    for (i, clause) in state.clauses.iter().enumerate() {
        for (&j, &x) in clause.indices.iter().zip(clause.values.iter()) {
            if values[i][j].is_nan() {
                values[i][j] = x;
            }
        }
    }

    let mut mean = vec![0f32; state.nvars];
    for v in values.into_iter() {
        for (i, x) in v.into_iter().enumerate() {
            mean[i] += x / state.nvars as f32;
        }
    }

    Ok(SatState {
        clauses: state.clauses
            .into_iter()
            .map(|c| Clause::new(&mean[..], c.indices, c.negating))
            .collect(),
        nvars: state.nvars,
    })
}

pub fn norm(current: &SatState, previous: &SatState) -> f32 {
    let mut delta = 0f32;
    for (curr, prev) in current.clauses.iter().zip(previous.clauses.iter()) {
        for (&c, &p) in curr.values.iter().zip(prev.values.iter()) {
            if c.signum() != p.signum() {
                delta += 1.0;
            }
        }
    }
    delta / current.clauses.len() as f32
}

#[cfg(test)]
mod tests {
    use super::*;

    const VARS: [f32; 2] = [-0.2, 0.7];
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
    fn test_norm() {
        let vars = Vec::from(VARS);
        let indices: Vec<Vec<usize>> = INDICES.iter().map(Vec::from).collect();
        let negations: Vec<Vec<bool>> = NEGATINGS.iter().map(Vec::from).collect();
        let state = SatState::new(vars, indices, negations);
        assert_eq!(norm(&state, &state), 0f32);
    }

    #[test]
    fn test_divide_projector() {
        let vars = Vec::from(VARS);
        let indices: Vec<Vec<usize>> = INDICES.iter().map(Vec::from).collect();
        let negations: Vec<Vec<bool>> = NEGATINGS.iter().map(Vec::from).collect();
        let state = SatState::new(vars, indices, negations);
        let update = divide_projector(state).unwrap();
        assert_eq!(update.clauses[0].values, vec![-1.0, -1.0, 1.0]);
        assert_eq!(update.clauses[1].values, vec![-1.0, 1.0, 1.0]);
        assert_eq!(update.clauses[2].values, vec![-1.0, 1.0, 1.0]);

        let check = divide_projector(update).unwrap();
        assert_eq!(check.clauses[0].values, vec![-1.0, -1.0, 1.0]);
        assert_eq!(check.clauses[1].values, vec![-1.0, 1.0, 1.0]);
        assert_eq!(check.clauses[2].values, vec![-1.0, 1.0, 1.0]);
    }

    #[test]
    fn test_concur_projector() {
        let vars = Vec::from(VARS);
        let indices: Vec<Vec<usize>> = INDICES.iter().map(Vec::from).collect();
        let negations: Vec<Vec<bool>> = NEGATINGS.iter().map(Vec::from).collect();
        let state = SatState::new(vars, indices, negations);
        let state = divide_projector(state).unwrap();
        let update = concur_projector(state).unwrap();
        assert_eq!(update.clauses[0].values, vec![-1.5, -1.5, 1.5]);
        assert_eq!(update.clauses[1].values, vec![-1.5, 1.5, 1.5]);
        assert_eq!(update.clauses[2].values, vec![-1.5, 1.5, 1.5]);

        let check = divide_projector(update).unwrap();
        assert_eq!(check.clauses[0].values, vec![-1.0, -1.0, 1.0]);
        assert_eq!(check.clauses[1].values, vec![-1.0, 1.0, 1.0]);
        assert_eq!(check.clauses[2].values, vec![-1.0, 1.0, 1.0]);
    }
}
