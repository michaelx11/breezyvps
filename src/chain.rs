use super::command;

// An item on the chain
#[derive(Clone)]
pub enum Item<'a> {
    FatalCommand(String),
    NonFatalCommand(String),
    ResultMappedCommand(&'a Fn(&command::Result, String) -> String, String, bool),
    ResultProcessor(&'a Fn(&command::Result) -> command::Result),
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

    pub fn result_proc(mut self, f: &'a Fn(&command::Result) -> command::Result) -> Self {
        self.commands.push(Item::ResultProcessor(f));
        self
    }

    pub fn result_mapped_cmd(mut self, f: &'a Fn(&command::Result, String) -> String, command_string: &str) -> Self {
        self.commands.push(Item::ResultMappedCommand(f, command_string.to_string(), true));
        self
    }

    pub fn result_mapped_cmd_nonfatal(mut self, f: &'a Fn(&command::Result, String) -> String, command_string: &str) -> Self {
        self.commands.push(Item::ResultMappedCommand(f, command_string.to_string(), false));
        self
    }

    pub fn cmd(mut self, command_string: &str) -> Self {
        self.commands.push(Item::FatalCommand(String::from(command_string)));
        self
    }

    pub fn cmd_nonfatal(mut self, command_string: &str) -> Self {
        self.commands.push(Item::NonFatalCommand(String::from(command_string)));
        self
    }

    fn run_command(cmd_str : &str) -> command::Result {
        let result = command::run_host_cmd(cmd_str);
        info!("Running: {}", cmd_str);
        if result.success {
            info!("stdout: {}", result.stdout);
            info!("stderr: {}", result.stderr);
        } else {
            warn!("stdout: {}", result.stdout);
            warn!("stderr: {}", result.stderr);
        }
        result
    }

    // Executes the chain and returns self, with vector reset
    pub fn execute(mut self) -> Self {
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
                &Item::ResultMappedCommand(f, ref s, is_fatal) => {
                    let mapped_command : String = {
                        if let Some(ref mut curr_res) = self.result {
                            f(&curr_res, s.to_string())
                        } else {
                            warn!("Executing ResultMappedCommand with no current result is a no-op!");
                            s.to_string()
                        }
                    };
                    let result = CommandChain::run_command(&mapped_command);
                    if is_fatal && !result.success {
                        self.result = Some(result);
                        break
                    }
                    self.result = Some(result);
                }
            }
        }
        self.old_commands.extend(self.commands.clone());
        self.commands = Vec::new();
        self
    }
}
