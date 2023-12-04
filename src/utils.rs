use std::process::exit;

pub fn exit_with_err_msg(msg: &str) {
    println!("{}{}{}", "\x1b[31m", msg, "\x1b[0m");
    exit(1);
}

pub fn exit_with_help_msg(msg: &str) {
    println!("{}{}{}", "\x1b[33m", msg, "\x1b[0m");
    exit(0)
}
