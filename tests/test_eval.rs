use embed_nu::{
    context::{CommandGroupConfig, Context},
    NewEmpty,
};
use nu_protocol::PipelineData;

#[test]
fn it_evals_strings() {
    let mut ctx = get_context();
    let pipeline = ctx
        .eval_raw(r#"echo "Hello World""#, PipelineData::empty())
        .unwrap();
    ctx.print_pipeline(pipeline).unwrap()
}

fn get_context() -> Context {
    Context::builder()
        .with_command_groups(CommandGroupConfig::default().all_groups(true))
        .add_parent_env_vars()
        .build()
        .unwrap()
}
