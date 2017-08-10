use super::command;

// An item on the chain
#[derive(Clone)]
pub enum Item<'a> {
    FatalCommand(String),
    NonFatalCommand(String),
    ResultProcessor(&'a Fn(&command::Result) -> command::Result),
    CommandModifier(&'a Fn(&command::Result, Item) -> Item<'a>)
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

    pub fn command_modifier(mut self, f: &'a Fn(&command::Result, Item) -> Item<'a>) -> Self {
        self.commands.push(Item::CommandModifier(f));
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
        if result.success {
            info!("stdout:{}", result.stdout);
        } else {
            warn!("stderr:{}", result.stderr);
        }
        result
    }

    // Executes the chain and returns self, with vector reset
    pub fn execute(mut self) -> Self {
        for i in 0..commands.len() {
            let owned_commands_clone = commands.clone();
            let commands_len = owned_commands_clone.len();
            match item {
                &mut Item::FatalCommand(ref s) => {
                    let result = CommandChain::run_command(s);
                    if !result.success {
                        self.result = Some(result);
                        break
                    }
                    self.result = Some(result);
                },
                &mut Item::NonFatalCommand(ref s) => {
                    let result = CommandChain::run_command(s);
                    self.result = Some(result);
                },
                &mut Item::ResultProcessor(f) => {
                    self.result = {
                        if let Some(ref mut curr_res) = self.result {
                            Some(f(&curr_res))
                        } else {
                            warn!("Executing ResultProcessor with no current result is a no-op!");
                            None
                        }
                    };
                },
                &mut Item::CommandModifier(f) => {
                    if i >= commands_len - 1 {
                        warn!("No next Item, leaving it unmodified");
                    }
                    let next_item = &mut commands[i + 1];
                    commands[i + 1] = {
                        if let Some(ref mut curr_res) = self.result {
                            f(&curr_res, next_item.clone())
                        } else {
                            warn!("Current result is empty, no-op!");
                            next_item.clone()
                        }
                    };
                }
            }
        }
        self.old_commands.extend(self.commands.clone());
        self.commands = Vec::new();
        self
    }
}
