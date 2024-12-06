use crate::states::{Clause, SatState};
use drs::{errors::Error, Result};
use pathfinding::num_traits::Float;

pub fn divide_projector(state: SatState) -> Result<SatState> {
    Ok(
        SatState {
            clauses: state.clauses
                .into_iter()
                .map(|c| c.solve())
                .collect(),
            nvars: state.nvars,
        }
    )
}

pub fn concur_projector(state: SatState) -> Result<SatState> {
    let mut variables = vec![0f32; state.nvars];
    for clause in &state.clauses {
        for (i, &val) in clause.values.iter().enumerate() {
            variables[clause.indices[i]] += val;
        }
    }

    let update = state.clauses
        .into_iter()
        .map(|c| {
            Clause {
                values: c.indices
                    .iter()
                    .map(|&i| variables[i])
                    .collect(),
                indices: c.indices,
                negating: c.negating,
            }
        })
        .collect();

    Ok(
        SatState {
            clauses: update,
            nvars: state.nvars,
        }
    )
}

pub fn norm(current: &SatState, previous: &SatState) -> f32 {
    let d = current.clauses.len() as f32;
    let mut delta = 0f32;

    for (curr, prev) in current.clauses.iter().zip(previous.clauses.iter()) {
        let mut diff = 0f32;
        for (c, p) in curr.values.iter().zip(prev.values.iter()) {
            diff += (c - p).powi(2);
        }
        delta += diff.sqrt() / d;
    }

    delta
}

#[cfg(test)]
mod tests {
    use super::*;
}
