#![doc=include_str!("../README.md")]
pub(crate) mod argument;
pub(crate) mod context;
pub(crate) mod error;
pub(crate) mod into_expression;
pub(crate) mod into_value;
pub(crate) mod utils;

pub use argument::Argument;
pub use context::{CommandGroupConfig, Context, ContextBuilder};
pub use into_value::*;
pub use nu_protocol::PipelineData;
pub use rusty_value;
pub use utils::NewEmpty;

pub type Error = error::CrateError;
