use std::process::Command;

pub struct Result {
    pub exit_code: Option<i32>,
    pub success: bool,
    pub stdout: String,
    pub stderr: String
}

pub fn run_ssh_cmd(host: &str, command_str: &str) {
    // unimplemented
}

pub fn run_host_cmd(command_str: &str) -> Result {
    let output = if cfg!(target_os = "windows") {
        Command::new("cmd")
                .args(&["/C", command_str])
                .output()
                .expect("failed to execute process")
    } else {
        Command::new("sh")
                .arg("-c")
                .arg(command_str)
                .output()
                .expect("failed to execute process")
    };
    let stdout = String::from_utf8(output.stdout).expect("Failed to unpack valid utf-8 from stdout");
    let stderr = String::from_utf8(output.stderr).expect("Failed to unpack valid utf-8 from stderr");
    let result = Result {
        exit_code: output.status.code(),
        success: output.status.success(),
        stdout: stdout,
        stderr: stderr
    };
    return result;
}
