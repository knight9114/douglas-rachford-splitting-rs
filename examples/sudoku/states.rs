use crate::projectors::iroot;
use drs::State;
use rand::prelude::*;
use std::ops::{Add, Mul};

#[derive(Debug, Clone)]
pub struct ConstraintState(pub Vec<f32>);

impl Add for ConstraintState {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self(self.0.into_iter().zip(rhs.0).map(|(l, r)| l + r).collect())
    }
}

impl Mul<f32> for ConstraintState {
    type Output = Self;

    fn mul(self, rhs: f32) -> Self::Output {
        Self(self.0.into_iter().map(|l| l * rhs).collect())
    }
}

impl State for ConstraintState {}

#[derive(Debug, Clone)]
pub struct SudokuState {
    pub given: ConstraintState,
    pub states: Vec<ConstraintState>,
}

impl SudokuState {
    pub fn solution(&self) -> Vec<Vec<usize>> {
        let (s1, s2, s3) = (
            &self.states[0].0[..],
            &self.states[0].0[..],
            &self.states[0].0[..],
        );
        assert_eq!(s1, s2);
        assert_eq!(s2, s3);

        let n = iroot(s1.len(), 3);
        let mut output = vec![Vec::with_capacity(n); n];
        for r in 0..n {
            for c in 0..n {
                let offset = r * n.pow(2) + c * n;
                let mut max = f32::MIN;
                let mut idx = n;

                let (val, _) = s1[offset..offset + n]
                    .iter()
                    .enumerate()
                    .fold((0, s1[offset]), |(imax, vmax), (i, v)| {
                        if &vmax > v {
                            (imax, vmax)
                        } else {
                            (i, *v)
                        }
                    });

                output[r].push(val + 1);
            }
        }

        output
    }
}

impl Add for SudokuState {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        let given = self.given;
        let states = self
            .states
            .into_iter()
            .zip(rhs.states)
            .map(|(l, r)| l + r)
            .collect();
        Self { given, states }
    }
}

impl Mul<f32> for SudokuState {
    type Output = Self;

    fn mul(self, rhs: f32) -> Self::Output {
        let given = self.given;
        let states = self.states.into_iter().map(|l| l * rhs).collect();
        Self { given, states }
    }
}

impl State for SudokuState {}

impl From<[usize; 81]> for SudokuState {
    fn from(src: [usize; 81]) -> Self {
        let mut given = vec![0f32; 81 * 9];
        let mut rng = thread_rng();

        for (i, &val) in src.iter().enumerate() {
            let start = 9 * i;
            if val != 0 {
                given[start + val - 1] = 1f32;
            }
        }

        let given = ConstraintState(given) * 1000f32;
        let states = (0..3)
            .map(|_| {
                let mut state = vec![0f32; 81 * 9];
                rng.fill(&mut state[..]);
                ConstraintState(state)
            })
            .collect();

        Self { given, states }
    }
}
