use crate::prompt::{draw_prompt, print_result};
use cli-core::{dispatch, init_registry, parse_line};
use std::io::{stdin, Read};

pub async fn run_repl() {
    init_registry();
    loop {
        draw_prompt();
        let mut buf = String::new();
        if stdin().read_line(&mut buf).is_err() {
            break;
        }
        let line = buf.trim();
        if line.is_empty() {
            continue;
        }
        if line == "exit" || line == "quit" {
            break;
        }
        match parse_line(line) {
            Ok(cmd) => {
                let res = dispatch(&cmd);
                print_result(res.ok, &res.message, &res.payload);
            }
            Err(e) => {
                print_result(false, &e, &None);
            }
        }
    }
}
