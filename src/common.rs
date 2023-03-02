#[cfg(test)]
use mockall::predicate::*;

#[cfg(test)]
use mockall::*;

pub struct IO {}

#[cfg_attr(test, automock)]
impl IO {
    pub fn prompt(message: &str) -> String {
        use std::io::{stdin, stdout, Write};
        print!("{}", message);
        stdout().flush().expect("unable to flush stdout");

        let mut output = String::new();
        stdin().read_line(&mut output).expect("No input");
        //read_line will include the new line char, so output needs to be trimmed
        output.trim().to_string()
    }

    pub fn output(message: &str) {
        print!("{}", message);
    }
}
