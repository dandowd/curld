use std::process::Command;

pub fn run_with_args(args: Vec<String>) -> String {
    let output = Command::new("curl").args(args).output();
    match output {
        Ok(cmd_out) => {
            if !cmd_out.stdout.is_empty() {
                return String::from_utf8(cmd_out.stdout).unwrap();
            }

            String::from_utf8(cmd_out.stderr).unwrap()
        }
        Err(msg) => panic!("{:?}", msg),
    }
}
