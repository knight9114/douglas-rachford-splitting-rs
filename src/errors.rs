#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("convergence error: failed to converge, delta={1}, after {0} steps")]
    Convergence(usize, f32),

    #[error("projection error: {0}")]
    Projection(Box<dyn std::error::Error>),

    #[error("unknown error: {0}")]
    Unknown(Box<dyn std::error::Error>),
}
