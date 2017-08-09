use super::command;

pub struct CommandChain {
    pub commands: Vec<String>,
    pub log_level: u32
}

impl CommandChain {

    pub fn new() -> CommandChain {
        CommandChain {
            commands: ::std::vec::Vec::new(),
            log_level: 0
        }
    }

    pub fn chain<'a>(&'a mut self, command_string: String) -> &'a mut CommandChain {
        self.commands.push(command_string);
        self
    }

    pub fn chain_nonfatal<'a>(&'a mut self, command_string: String) -> &'a mut CommandChain {
        self.commands.push(command_string);
        self
    }

    // TODO: Return an actual Result here
    pub fn execute(&self) {
        for cmd_str in self.commands.iter() {
            let result = command::run_host_cmd(&cmd_str);
            if result.success {
               info!("Stdout:{}", result.stdout);
            } else {
               warn!("Stderr:{}", result.stderr);
            }
        }
    }
}
