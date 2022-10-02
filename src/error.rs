use nu_protocol::ShellError;
use thiserror::Error;

pub type CrateResult<T> = std::result::Result<T, CrateError>;

#[derive(Clone, Debug, Error)]
pub enum CrateError {
    #[error("Shell Error {0}")]
    NuShellError(#[from] ShellError),
}
