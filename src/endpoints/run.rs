use std::process::Command;

pub fn run(curl_cmd: &str) -> String {
    let curl_args: Vec<String> = curl_cmd.split(' ').map(|input| input.to_string()).collect();

    run_with_args(curl_args)
}

pub fn run_with_args(curl_args: Vec<String>) -> String {
    let output = Command::new("curl").args(curl_args).output();
    match output {
        Ok(cmd_out) => String::from_utf8(cmd_out.stdout).unwrap(),
        Err(msg) => panic!("{:?}", msg),
    }
}
