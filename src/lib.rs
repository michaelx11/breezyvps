#[macro_use]
extern crate log;

pub mod command;
pub mod digitalocean;
pub mod configure;
pub mod chain;

#[cfg(test)]
mod tests {

    use super::chain;
    use super::command;

    #[test]
    fn basic_fatal_test_should_succeed() {
        let res = chain::CommandChain::new()
            .cmd("echo hello")
            .execute();

        assert!(res.result.unwrap().success);
    }

    #[test]
    fn basic_fatal_test_should_fail() {
        let res = chain::CommandChain::new()
            .cmd("echo hello")
            .cmd("test2")
            .cmd("test3")
            .execute();

        assert!(!res.result.unwrap().success);
    }

    #[test]
    fn basic_nonfatal_should_succeed() {
        let res = chain::CommandChain::new()
            .cmd_nonfatal("echo hello")
            .cmd_nonfatal("restinpeace")
            .cmd("echo yo")
            .execute();

        assert!(res.result.unwrap().success);
    }


    #[test]
    fn basic_processing_func_test() {

        let processing_func = |res: &command::Result| -> command::Result {
            let mut extra_stdout = res.stdout.clone();
            extra_stdout.push_str("+processing");

            command::Result {
                exit_code: res.exit_code,
                success: res.success,
                stdout: extra_stdout,
                stderr: res.stderr.clone()
            }
        };

        let res = chain::CommandChain::new()
            .cmd("echo hello")
            .result_proc(&processing_func)
            .execute()
            .execute();

        assert!(res.result.unwrap().success);
    }
}
