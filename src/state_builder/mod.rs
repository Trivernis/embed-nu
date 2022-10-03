use std::env;

use nu_protocol::{
    engine::{EngineState, Stack, StateWorkingSet},
    Span,
};

mod bindings;
mod command_group_config;
pub use command_group_config::CommandGroupConfig;

use crate::{error::CrateResult, into_value::IntoValue, utils::SpanEmpty};

/// Builder to create a new nu engine state
pub struct StateBuilder {
    engine_state: EngineState,
    stack: Stack,
}

impl Default for StateBuilder {
    fn default() -> Self {
        Self {
            engine_state: EngineState::new(),
            stack: Stack::new(),
        }
    }
}

impl StateBuilder {
    /// Enables certain command groups specified in the Config on the state
    pub fn with_command_groups(&mut self, group_config: CommandGroupConfig) -> &mut Self {
        macro_rules! toggle_command_groups {
            ($($group:ident),*) => {
                paste::item!(
                $(
                    if group_config.$group {
                        bindings::[<bind_ $group _commands>](&mut self.engine_state);
                    }
                )*
                )
            }
        }

        toggle_command_groups!(
            core,
            filter,
            chart,
            misc,
            path,
            system,
            string,
            bit,
            byte,
            file_system,
            platform,
            date,
            shell,
            format,
            viewer,
            conversion,
            environment,
            math,
            network,
            random,
            generator,
            hash,
            experimental
        );
        self
    }

    /// Adds a variable to the state
    pub fn add_var<S: ToString, V: IntoValue>(
        &mut self,
        name: S,
        value: V,
    ) -> CrateResult<&mut Self> {
        let mut working_set = StateWorkingSet::new(&self.engine_state);

        let var_id = working_set.add_variable(
            name.to_string().into_bytes(),
            Span::empty(),
            nu_protocol::Type::Any,
        );
        self.stack.add_var(var_id, value.into_value());
        let delta = working_set.render();
        self.engine_state.merge_delta(delta)?;

        Ok(self)
    }

    /// Adds an environment variable to the state
    pub fn add_env_var<S: ToString, V: IntoValue>(&mut self, name: S, value: V) -> &mut Self {
        self.engine_state
            .add_env_var(name.to_string(), value.into_value());

        self
    }

    /// Adds the environment variables of the parent process to the
    /// states env variables
    pub fn add_parent_env_vars(&mut self) -> &mut Self {
        for (name, val) in env::vars() {
            self.add_env_var(name, val);
        }

        self
    }
}
