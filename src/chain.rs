use super::command;

// An item on the chain
#[derive(Clone)]
pub enum Item<'a> {
    FatalCommand(String),
    NonFatalCommand(String),
    ResultProcessor(&'a Fn(&command::Result) -> command::Result),
    CommandModifier(&'a Fn(&command::Result, Option<&'a mut Item>) -> Option<&'a mut Item<'a>>)
}

pub struct CommandChain<'a> {
    pub commands: Vec<Item<'a>>,
    pub old_commands: Vec<Item<'a>>,
    pub result: Option<command::Result>
}

impl<'a> CommandChain<'a> {

    pub fn new() -> Self {
        CommandChain {
            commands: Vec::new(),
            old_commands: Vec::new(),
            result: None
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

    // Executes the chain and returns self, with vector reset
    pub fn execute(&'a mut self) -> CommandChain {
        for item in self.commands.iter() {
            match item {
                &Item::FatalCommand(ref s) => {
                    let result = CommandChain::run_command(s);
                    if !result.success {
                        self.result = Some(result);
                        break
                    }
                    self.result = Some(result);
                },
                &Item::NonFatalCommand(ref s) => {
                    let result = CommandChain::run_command(s);
                    self.result = Some(result);
                },
                &Item::ResultProcessor(f) => {
                    self.result = {
                        if let Some(ref mut curr_res) = self.result {
                            Some(f(&curr_res))
                        } else {
                            warn!("Executing ResultProcessor with no current result is a no-op!");
                            None
                        }
                    };
                },
                &Item::CommandModifier(f) => {
                    // TODO: implement me!
                }
            }
        }
        CommandChain {
            commands: Vec::new(),
            old_commands: self.commands.clone(),
            result: self.result.clone()
        }
    }
}
