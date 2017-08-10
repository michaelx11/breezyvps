#[macro_use]
extern crate log;

pub mod command;
pub mod digitalocean;
pub mod configure;
pub mod chain;

#[cfg(test)]
mod tests {

    extern crate simplelog;
    use std::sync::{Once, ONCE_INIT};
    static SYNC_OBJ: Once = ONCE_INIT;
    use super::chain;
    use super::command;
    use self::simplelog::{Config, TermLogger, WriteLogger, CombinedLogger, LogLevelFilter};
    use ::std::fs::File;

    fn setup_logger() {
        SYNC_OBJ.call_once(|| {
            // Configure logging with simplelogger
            CombinedLogger::init(
                vec![
                TermLogger::new(LogLevelFilter::Info, Config::default()).unwrap(),
                WriteLogger::new(LogLevelFilter::Info, Config::default(), File::create("breezyvps_test.log").unwrap()),
                ]
                ).unwrap();
        });
    }

    #[test]
    fn basic_fatal_test_should_succeed() {
        setup_logger();
        let res = chain::CommandChain::new()
            .cmd("echo hello")
            .execute();

        assert!(res.result.unwrap().success);
    }

    #[test]
    fn basic_fatal_test_should_fail() {
        setup_logger();
        let res = chain::CommandChain::new()
            .cmd("echo hello")
            .cmd("test2")
            .cmd("test3")
            .execute();

        assert!(!res.result.unwrap().success);
    }

    #[test]
    fn basic_nonfatal_should_succeed() {
        setup_logger();
        let res = chain::CommandChain::new()
            .cmd_nonfatal("echo hello")
            .cmd_nonfatal("restinpeace")
            .cmd("echo yo")
            .execute();

        assert!(res.result.unwrap().success);
    }


    #[test]
    fn basic_processing_func_test() {
        setup_logger();

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

    #[test]
    fn basic_result_mapped_cmd_test() {
        setup_logger();

        let mapping_func = |res: &command::Result, cmd_str: String| -> String {
            let new_cmd = str::replace(&cmd_str, "%stdout%", &res.stdout);
            new_cmd.to_string()
        };

        let res = chain::CommandChain::new()
            .cmd("echo hello")
            .result_mapped_cmd(&mapping_func, "echo sup_%stdout%")
            .execute();

        let stdout = &res.result.unwrap().stdout;
        let trimmed = stdout.trim();
        println!("{}", trimmed);
        assert!(trimmed == "sup_hello");
    }
}
