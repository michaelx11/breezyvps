use super::command;

// An item on the chain
pub enum Item<'a> {
    FatalCommand(String),
    NonFatalCommand(String),
    ResultProcessor(&'a Fn(&command::Result) -> command::Result),
    CommandModifier(&'a Fn(&'a command::Result, Option<&'a mut Item>) -> Option<&'a mut Item<'a>>)
}

pub struct CommandChain<'a> {
    pub commands: Vec<Item<'a>>,
    current_result: Option<command::Result>
}

impl<'a> CommandChain<'a> {

    pub fn new() -> CommandChain<'a> {
        CommandChain {
            commands: ::std::vec::Vec::new(),
            current_result: None
        }
    }

    pub fn result_proc(&'a mut self, f: &'a Fn(&command::Result) -> command::Result) -> &'a mut CommandChain {
        self.commands.push(Item::ResultProcessor(f));
        self
    }

    pub fn cmd(&'a mut self, command_string: &str) -> &'a mut CommandChain {
        self.commands.push(Item::FatalCommand(String::from(command_string)));
        self
    }

    pub fn cmd_nonfatal(&'a mut self, command_string: &str) -> &'a mut CommandChain {
        self.commands.push(Item::NonFatalCommand(String::from(command_string)));
        self
    }

    fn run_command(cmd_str : &str) -> command::Result {
        let result = command::run_host_cmd(cmd_str);
        if result.success {
            info!("stdout:{}", result.stdout);
        } else {
            warn!("stderr:{}", result.stderr);
        }
        result
    }

    // TODO: Return an actual Result here
    pub fn execute(&'a mut self) -> Option<command::Result> {
        for item in self.commands.iter() {
            match item {
                &Item::FatalCommand(ref s) => {
                    let result = CommandChain::run_command(s);
                    if !result.success {
                        return None;
                    }
                    self.current_result = Some(result);
                },
                &Item::NonFatalCommand(ref s) => {
                    let result = CommandChain::run_command(s);
                    self.current_result = Some(result);
                },
                &Item::ResultProcessor(f) => {
                    self.current_result = {
                        if let Some(ref mut curr_res) = self.current_result {
                            Some(f(&curr_res))
                        } else {
                            warn!("Executing ResultProcessor with no current result is a no-op!");
                            None
                        }
                    };
                },
                &Item::CommandModifier(f) => {
                }
            }
        }
        if let Some(ref curr_res) = self.current_result {
            return Some(curr_res.clone())
        } else {
            None
        }
    }
}
