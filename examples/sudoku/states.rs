use drs::State;
use rand::prelude::*;
use std::ops::{Add, Mul};

#[derive(Debug, Clone)]
pub struct ConstraintState(pub Vec<f32>);

impl Add for ConstraintState {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self(
            self.0
                .into_iter()
                .zip(rhs.0.into_iter())
                .map(|(l, r)| l + r)
                .collect(),
        )
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
pub struct SudokuState(pub Vec<ConstraintState>);

impl Add for SudokuState {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self(
            self.0
                .into_iter()
                .zip(rhs.0.into_iter())
                .map(|(l, r)| l + r)
                .collect(),
        )
    }
}

impl Mul<f32> for SudokuState {
    type Output = Self;

    fn mul(self, rhs: f32) -> Self::Output {
        Self(self.0.into_iter().map(|l| l * rhs).collect())
    }
}

impl State for SudokuState {}

impl From<[usize; 81]> for SudokuState {
    fn from(src: [usize; 81]) -> Self {
        let mut data = vec![0f32; 81 * 9];
        let mut rng = thread_rng();

        for (i, val) in src.into_iter().enumerate() {
            let start = 9 * i;
            let end = start + 9;
            rng.fill(&mut data[start..end]);

            if val != 0 {
                data[start + val - 1] = 1f32;
            }
        }

        Self(vec![ConstraintState(data); 3])
    }
}
