/// Copy of the nushell print command with a slight adjustment for pipelines
/// Source: https://github.com/nushell/nushell/blob/98525043edd20abb62da09726d75816d09d68f1e/crates/nu-cli/src/print.rs
use nu_engine::CallExt;
use nu_protocol::ast::Call;
use nu_protocol::engine::{Command, EngineState, Stack};
use nu_protocol::{
    Category, IntoPipelineData, PipelineData, ShellError, Signature, Span, SyntaxShape, Type, Value,
};

use crate::NewEmpty;

#[derive(Clone)]
pub struct PrintCommand;

impl Command for PrintCommand {
    fn name(&self) -> &str {
        "print"
    }

    fn signature(&self) -> Signature {
        Signature::build("print")
            .input_output_types(vec![
                (Type::Nothing, Type::Nothing),
                (Type::Any, Type::Nothing),
            ])
            .allow_variants_without_examples(true)
            .rest("rest", SyntaxShape::Any, "the values to print")
            .switch(
                "no-newline",
                "print without inserting a newline for the line ending",
                Some('n'),
            )
            .switch("stderr", "print to stderr instead of stdout", Some('e'))
            .category(Category::Strings)
    }

    fn usage(&self) -> &str {
        "Print the given values to stdout."
    }

    fn extra_usage(&self) -> &str {
        r#"Unlike `echo`, this command does not return any value (`print | describe` will return "nothing").
Since this command has no output, there is no point in piping it with other commands.
`print` may be used inside blocks of code (e.g.: hooks) to display text during execution without interfering with the pipeline.
When used inside a pipeline it passes the input forward as output without interfering with it.
"#
    }

    fn run(
        &self,
        engine_state: &EngineState,
        stack: &mut Stack,
        call: &Call,
        input: PipelineData,
    ) -> Result<PipelineData, ShellError> {
        let args: Vec<Value> = call.rest(engine_state, stack, 0)?;
        let no_newline = call.has_flag(engine_state, stack, "no-newline")?;
        let to_stderr = call.has_flag(engine_state, stack, "stderr")?;

        let input_val = input.into_value(Span::empty());

        // This will allow for easy printing of pipelines as well
        if !args.is_empty() {
            for arg in args {
                arg.into_pipeline_data()
                    .print(engine_state, stack, no_newline, to_stderr)?;
            }
        } else if !input_val.is_nothing() {
            input_val.clone().into_pipeline_data().print(
                engine_state,
                stack,
                no_newline,
                to_stderr,
            )?;
        }

        Ok(input_val.into_pipeline_data())
    }
}
