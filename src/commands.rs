use std::{
    collections::VecDeque,
    env,
    io,
    process::{exit, Command},
};

use crossterm::terminal::disable_raw_mode;

use crate::settings;

// takes a string consisting of
// "dir args..."
// splits on whitespace and uses any extra args after the dir
pub fn ls(arg: &str) -> Vec<String> {
    // use Deque to pop_front
    let mut args = arg.split(" ").collect::<VecDeque<&str>>();
    // get dir to execute ls
    let dir = args.pop_front().unwrap();
    let mut res = vec![];

    // start the process
    let mut cmd = Command::new("ls");
    // add args
    cmd.arg("-p").args(args).current_dir(dir);

    // prepare output
    let output = cmd.output().expect("failed to run");
    let res_string: String = String::from_utf8(output.stdout).unwrap();
    let lines = res_string.lines();
    // build result vector
    for l in lines {
        res.push(l.to_string());
    }

    return res;
}

pub fn enter_dir(path: String) -> io::Result<()> {
    return match env::set_current_dir(path) {
        Ok(_) => Ok(()),
        Err(e) => Err(e),
    };
}

pub fn enter_file(path: String) -> io::Result<()> {
    let config = settings::get_conf();
    // default editor to vim
    let default_editor = String::from("vim");
    // checks for "editor" key in config and opens file in its value
    let editor = config.get("editor").unwrap_or(&default_editor);

    Command::new(editor)
        .arg(path)
        .status()
        .expect("failed to open editor");

    disable_raw_mode()?;
    exit(0);
}
