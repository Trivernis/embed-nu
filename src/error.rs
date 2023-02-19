use miette::Diagnostic;
use nu_parser::ParseError;
use nu_protocol::ShellError;
use thiserror::Error;

pub type CrateResult<T> = std::result::Result<T, CrateError>;

#[derive(Clone, Debug, Error)]
pub enum CrateError {
    #[error("Shell Error {0}")]
    NuShellError(#[from] ShellError),

    #[error("Parse Error {0}")]
    NuParseError(#[from] ParseError),

    #[error("Could not find the function {0}")]
    FunctionNotFound(String),
}

impl Diagnostic for CrateError {
    fn code<'a>(&'a self) -> Option<Box<dyn std::fmt::Display + 'a>> {
        match self {
            CrateError::NuShellError(n) => n.code(),
            CrateError::NuParseError(n) => n.code(),
            Self::FunctionNotFound(_) => Some(Box::new("embed_nu::fn_not_found")),
        }
    }

    fn severity(&self) -> Option<miette::Severity> {
        match self {
            CrateError::NuShellError(n) => n.severity(),
            CrateError::NuParseError(n) => n.severity(),
            _ => None,
        }
    }

    fn help<'a>(&'a self) -> Option<Box<dyn std::fmt::Display + 'a>> {
        match self {
            CrateError::NuShellError(n) => n.help(),
            CrateError::NuParseError(n) => n.help(),
            CrateError::FunctionNotFound(_) => Some(Box::new(
                "Make sure the function you want to execute is defined at this point.",
            )),
        }
    }

    fn url<'a>(&'a self) -> Option<Box<dyn std::fmt::Display + 'a>> {
        match self {
            CrateError::NuShellError(n) => n.url(),
            CrateError::NuParseError(n) => n.url(),
            _ => None,
        }
    }

    fn source_code(&self) -> Option<&dyn miette::SourceCode> {
        match self {
            CrateError::NuShellError(n) => n.source_code(),
            CrateError::NuParseError(n) => n.source_code(),
            _ => None,
        }
    }

    fn labels(&self) -> Option<Box<dyn Iterator<Item = miette::LabeledSpan> + '_>> {
        match self {
            CrateError::NuShellError(n) => n.labels(),
            CrateError::NuParseError(n) => n.labels(),
            _ => None,
        }
    }

    fn related<'a>(&'a self) -> Option<Box<dyn Iterator<Item = &'a dyn Diagnostic> + 'a>> {
        match self {
            CrateError::NuShellError(n) => n.related(),
            CrateError::NuParseError(n) => n.related(),
            _ => None,
        }
    }

    fn diagnostic_source(&self) -> Option<&dyn Diagnostic> {
        match self {
            CrateError::NuShellError(n) => n.diagnostic_source(),
            CrateError::NuParseError(n) => n.diagnostic_source(),
            _ => None,
        }
    }
}
