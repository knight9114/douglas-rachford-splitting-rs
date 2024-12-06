use crate::states::{ConstraintState, SudokuState};
use drs::{errors::Error, Result};
use pathfinding::prelude::{kuhn_munkres, Matrix};

pub fn divide_projector(state: SudokuState) -> Result<SudokuState> {
    let n = iroot(state.given.0.len(), 3);
    let mut output = Vec::with_capacity(3);

    for (i, s) in state.states.into_iter().enumerate() {
        let indices = match i {
            0 => get_row_indices(n),
            1 => get_column_indices(n),
            2 => get_block_indices(n),
            _ => panic!("invalid constraint: expected [0, 2], got {i}"),
        };

        let mut update = vec![0f32; n.pow(3)];
        let s = s + state.given.clone();
        for inds in indices.iter().take(n) {
            let extracted = extract_and_round_values(&s.0, inds);
            let weights = Matrix::square_from_vec(extracted)
                .map_err(|err| Error::Projection(Box::new(err)))?;
            let (_, assignments) = kuhn_munkres(&weights);

            for (r, c) in assignments.into_iter().enumerate() {
                let idx = inds[r * n + c];
                update[idx] = 1f32;
            }
        }

        output.push(ConstraintState(update));
    }

    Ok(SudokuState {
        given: state.given,
        states: output,
    })
}

pub fn concur_projector(state: SudokuState) -> Result<SudokuState> {
    let c = state.states.len();
    let n = state.given.0.len();
    let d = c as f32;

    let mut mean = ConstraintState(vec![0f32; n]);
    for constraint in state.states.into_iter() {
        for (i, val) in constraint.0.into_iter().enumerate() {
            mean.0[i] += val / d;
        }
    }

    Ok(SudokuState {
        given: state.given,
        states: vec![mean; 3],
    })
}

pub fn norm(current: &SudokuState, previous: &SudokuState) -> f32 {
    let d = current.states.len() as f32;
    let mut delta = 0f32;

    for (curr, prev) in current.states.iter().zip(previous.states.iter()) {
        let mut diff = 0f32;
        for (c, p) in curr.0.iter().zip(prev.0.iter()) {
            diff += (c - p).powi(2);
        }
        delta += diff.sqrt() / d;
    }

    delta
}

pub fn iroot(n: usize, p: usize) -> usize {
    let x = n as f32;
    let root = x.powf(1f32 / p as f32).round() as usize;

    if root.pow(p as u32) != n {
        panic!("invalid puzzle size: expected perfect power of {p}, got {n}")
    }

    root
}

fn get_row_indices(n: usize) -> Vec<Vec<usize>> {
    (0..n)
        .map(|r| {
            let start = r * n.pow(2);
            let end = start + n.pow(2);
            (start..end).collect()
        })
        .collect()
}

fn get_column_indices(n: usize) -> Vec<Vec<usize>> {
    let mut constraints = Vec::with_capacity(n);

    for col in 0..n {
        let mut constraint = Vec::with_capacity(n.pow(2));
        for row in 0..n {
            let start = row * n.pow(2) + col * n;
            let end = start + n;
            constraint.extend(start..end);
        }
        constraints.push(constraint);
    }

    constraints
}

fn get_block_indices(n: usize) -> Vec<Vec<usize>> {
    let mut constraints = Vec::with_capacity(n);
    let nsqrt = iroot(n, 2);

    for block in 0..n {
        let (row, col) = (block / nsqrt, block % nsqrt);
        let (row, col) = (row * nsqrt, col * nsqrt);
        let mut constraint = Vec::with_capacity(n.pow(2));

        for r in row..row + nsqrt {
            for c in col..col + nsqrt {
                let start = r * n.pow(2) + c * n;
                let end = start + n;
                constraint.extend(start..end);
            }
        }

        constraints.push(constraint);
    }

    constraints
}

fn extract_and_round_values(vector: &[f32], indices: &[usize]) -> Vec<isize> {
    indices
        .iter()
        .map(|&i| (vector[i] * 1000f32).round() as isize)
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::states::ConstraintState;

    #[test]
    fn test_isort_successful() {
        for truth in 2usize..16 {
            assert_eq!(truth, iroot(truth.pow(2), 2));
            assert_eq!(truth, iroot(truth.pow(3), 3));
        }
    }

    #[test]
    #[should_panic(expected = "invalid puzzle size: expected perfect power of 2, got 10")]
    fn test_isort_failure() {
        iroot(10, 2);
    }

    #[test]
    fn test_get_row_indices() {
        let n = 4;
        let indices = get_row_indices(n);
        let truth: Vec<Vec<usize>> = vec![
            (0 * n.pow(2)..1 * n.pow(2)).collect(),
            (1 * n.pow(2)..2 * n.pow(2)).collect(),
            (2 * n.pow(2)..3 * n.pow(2)).collect(),
            (3 * n.pow(2)..4 * n.pow(2)).collect(),
        ];
        assert_eq!(indices, truth);

        let n = 9;
        let indices = get_row_indices(n);
        let truth: Vec<Vec<usize>> = vec![
            (0 * n.pow(2)..1 * n.pow(2)).collect(),
            (1 * n.pow(2)..2 * n.pow(2)).collect(),
            (2 * n.pow(2)..3 * n.pow(2)).collect(),
            (3 * n.pow(2)..4 * n.pow(2)).collect(),
            (4 * n.pow(2)..5 * n.pow(2)).collect(),
            (5 * n.pow(2)..6 * n.pow(2)).collect(),
            (6 * n.pow(2)..7 * n.pow(2)).collect(),
            (7 * n.pow(2)..8 * n.pow(2)).collect(),
            (8 * n.pow(2)..9 * n.pow(2)).collect(),
        ];
        assert_eq!(indices, truth);
    }

    #[test]
    #[rustfmt::skip]
    fn test_get_column_indices() {
        let n = 4;
        let indices = get_column_indices(n);
        let truth = vec![
            vec![ 0,  1,  2,  3, 16, 17, 18, 19, 32, 33, 34, 35, 48, 49, 50, 51],
            vec![ 4,  5,  6,  7, 20, 21, 22, 23, 36, 37, 38, 39, 52, 53, 54, 55],
            vec![ 8,  9, 10, 11, 24, 25, 26, 27, 40, 41, 42, 43, 56, 57, 58, 59],
            vec![12, 13, 14, 15, 28, 29, 30, 31, 44, 45, 46, 47, 60, 61, 62, 63],
        ];
        assert_eq!(indices, truth);
    }

    #[test]
    #[rustfmt::skip]
    fn test_get_block_indices() {
        let n = 4;
        let indices = get_block_indices(n);
        let truth = vec![
            vec![ 0,  1,  2,  3,  4,  5,  6,  7, 16, 17, 18, 19, 20, 21, 22, 23],
            vec![ 8,  9, 10, 11, 12, 13, 14, 15, 24, 25, 26, 27, 28, 29, 30, 31],
            vec![32, 33, 34, 35, 36, 37, 38, 39, 48, 49, 50, 51, 52, 53, 54, 55],
            vec![40, 41, 42, 43, 44, 45, 46, 47, 56, 57, 58, 59, 60, 61, 62, 63],
        ];
        assert_eq!(indices, truth);
    }

    #[test]
    #[rustfmt::skip]
    fn test_divide_projector() {
        // 1 2 | 3 4
        // 3 4 | 1 2
        // ----+----
        // 2 3 | 4 1
        // 4 1 | 2 3
        let given = ConstraintState(vec![
            1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0,
            0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0,
            0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 1.0, 0.0, 0.0, 0.0,
            0.0, 0.0, 0.0, 1.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0,
        ]);
        let solved = SudokuState{
            given: given.clone(),
            states: vec![given.clone(); 3],
        };
        let output = divide_projector(solved.clone()).unwrap();
        assert_eq!(output.states[0].0, solved.states[0].0);
        assert_eq!(output.states[1].0, solved.states[1].0);
        assert_eq!(output.states[2].0, solved.states[2].0);

        // ? ? | 3 4
        // ? ? | 1 2
        // ----+----
        // 2 3 | ? ?
        // 4 1 | ? ?
        let unsolved = SudokuState{
            given: ConstraintState(
                vec![
                    0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0,
                    0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0,
                    0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
                    0.0, 0.0, 0.0, 1.0, 1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
                ]
            ),
            states: vec![
                ConstraintState(
                    vec![
                        0.5, 0.1, 0.1, 0.1, 0.1, 0.5, 0.1, 0.1, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0,
                        0.1, 0.1, 0.5, 0.1, 0.1, 0.1, 0.1, 0.5, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0,
                        0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.1, 0.1, 0.1, 0.5, 0.5, 0.1, 0.1, 0.1,
                        0.0, 0.0, 0.0, 1.0, 1.0, 0.0, 0.0, 0.0, 0.1, 0.5, 0.1, 0.1, 0.1, 0.1, 0.5, 0.1,
                    ]
                );
            3
            ],
        };
        let output = divide_projector(unsolved.clone()).unwrap();
        assert_eq!(output.states[0].0, solved.states[0].0);
        assert_eq!(output.states[1].0, solved.states[1].0);
        assert_eq!(output.states[2].0, solved.states[2].0);
    }

    #[test]
    #[rustfmt::skip]
    fn test_concur_projector() {
        let input = SudokuState{
            given: ConstraintState(vec![1f32; 16 * 4]),
            states: vec![ConstraintState(vec![1f32; 16 * 4]); 3],
        };
        let output = concur_projector(input.clone()).unwrap();
        assert_eq!(output.states[0].0, input.states[0].0);
        assert_eq!(output.states[1].0, input.states[1].0);
        assert_eq!(output.states[2].0, input.states[2].0);
    }

    #[test]
    #[rustfmt::skip]
    fn test_norm() {
        let ones = SudokuState{
            given: ConstraintState(vec![1f32; 16 * 4]),
            states: vec![ConstraintState(vec![1f32; 16 * 4]); 3],
        };
        let delta = norm(&ones, &ones);
        assert_eq!(delta, 0f32);

        let zeros = SudokuState{
            given: ConstraintState(vec![0f32; 16 * 4]),
            states: vec![ConstraintState(vec![0f32; 16 * 4]); 3],
        };
        let delta = norm(&ones, &zeros);
        assert_eq!(delta, 8f32);
    }

    #[test]
    #[rustfmt::skip]
    fn test_extract_and_round_values() {
        let rows = get_row_indices(4);
        let cols = get_column_indices(4);
        let blks = get_block_indices(4);
        let input = vec![
            0.5, 0.1, 0.1, 0.1, 0.1, 0.5, 0.1, 0.1, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0,
            0.1, 0.1, 0.5, 0.1, 0.1, 0.1, 0.1, 0.5, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0,
            0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.1, 0.1, 0.1, 0.5, 0.5, 0.1, 0.1, 0.1,
            0.0, 0.0, 0.0, 1.0, 1.0, 0.0, 0.0, 0.0, 0.1, 0.5, 0.1, 0.1, 0.1, 0.1, 0.5, 0.1,
        ];

        let values: Vec<Vec<isize>> = (0..4)
            .map(|i| extract_and_round_values(&input[..], &rows[i]))
            .collect();
        let truth = vec![
            vec![ 500, 100, 100, 100, 100, 500, 100, 100, 0, 0, 1000, 0, 0, 0, 0, 1000 ],
            vec![ 100, 100, 500, 100, 100, 100, 100, 500, 1000, 0, 0, 0, 0, 1000, 0, 0 ],
            vec![ 0, 1000, 0, 0, 0, 0, 1000, 0, 100, 100, 100, 500, 500, 100, 100, 100 ],
            vec![ 0, 0, 0, 1000, 1000, 0, 0, 0, 100, 500, 100, 100, 100, 100, 500, 100 ],
        ];
        assert_eq!(values, truth);

        let values: Vec<Vec<isize>> = (0..4)
            .map(|i| extract_and_round_values(&input[..], &cols[i]))
            .collect();
        let truth = vec![
            vec![ 500, 100, 100, 100, 100, 100, 500, 100, 0, 1000, 0, 0, 0, 0, 0, 1000 ],
            vec![ 100, 500, 100, 100, 100, 100, 100, 500, 0, 0, 1000, 0, 1000, 0, 0, 0 ],
            vec![ 0, 0, 1000, 0, 1000, 0, 0, 0, 100, 100, 100, 500, 100, 500, 100, 100 ],
            vec![ 0, 0, 0, 1000, 0, 1000, 0, 0, 500, 100, 100, 100, 100, 100, 500, 100 ],
        ];
        assert_eq!(values, truth);

        let values: Vec<Vec<isize>> = (0..4)
            .map(|i| extract_and_round_values(&input[..], &blks[i]))
            .collect();
        let truth = vec![
            vec![ 500, 100, 100, 100, 100, 500, 100, 100, 100, 100, 500, 100, 100, 100, 100, 500 ],
            vec![ 0, 0, 1000, 0, 0, 0, 0, 1000, 1000, 0, 0, 0, 0, 1000, 0, 0 ],
            vec![ 0, 1000, 0, 0, 0, 0, 1000, 0, 0, 0, 0, 1000, 1000, 0, 0, 0 ],
            vec![ 100, 100, 100, 500, 500, 100, 100, 100, 100, 500, 100, 100, 100, 100, 500, 100 ],
        ];
        assert_eq!(values, truth);
    }
}
