use nu_protocol::engine::{EngineState, Stack};

mod bindings;
mod command_group_config;
pub use command_group_config::CommandGroupConfig;

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
}
