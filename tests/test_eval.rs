use embed_nu::{Argument, CommandGroupConfig, Context, NewEmpty};
use nu_protocol::PipelineData;

#[test]
fn it_evals_strings() {
    let mut ctx = get_context();
    let pipeline = ctx
        .eval_raw(
            r#"echo "Hello World from this eval""#,
            PipelineData::empty(),
        )
        .unwrap();
    ctx.print_pipeline(pipeline).unwrap()
}

#[test]
fn it_executes_functions() {
    let mut ctx = get_context();
    ctx.eval_raw(
        r#"
    
        def hello [] {
            echo "Hello World from this script";
            echo # dummy echo so I don't have to print the output
        }        
        
    "#,
        PipelineData::empty(),
    )
    .unwrap();
    ctx.call_fn("hello", []).unwrap();
    assert!(ctx.has_fn("world") == false);

    let arg = Argument::positional("Hello from rust");
    let pipeline = ctx.call_fn("echo", [arg]).unwrap();
    ctx.print_pipeline(pipeline).unwrap();
}

fn get_context() -> Context {
    Context::builder()
        .with_command_groups(CommandGroupConfig::default().all_groups(true))
        .add_parent_env_vars()
        .build()
        .unwrap()
}
