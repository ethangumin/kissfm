use std::{
    collections::VecDeque,
    env,
    fs::{self, File},
    io,
    process::{exit, Command},
};

use crossterm::terminal::disable_raw_mode;

use crate::{settings, state::App};

// takes a string consisting of
// "dir args..."
// splits on whitespace and uses any extra args after the dir
pub fn ls(arg: &str) -> Vec<String> {
    // use Deque to pop_front
    let args = arg.split(" ").collect::<VecDeque<&str>>();
    // get dir to execute ls
    // let dir = args.pop_front().unwrap();
    let mut res = vec![];

    // start the process
    let mut cmd = Command::new("ls");
    // add args
    cmd.arg("-p").arg("-a").args(args); //..current_dir(dir);

    // prepare output
    let output = cmd.output().expect("failed to run");
    let res_string: String = String::from_utf8(output.stdout).unwrap();
    let lines = res_string.lines();
    // build result vector
    for l in lines {
        res.push(l.to_owned());
    }

    return res;
}

pub fn enter_dir(path: String, app: &mut App, args: &str) -> io::Result<()> {
    return match env::set_current_dir(path) {
        Ok(_) => {
            app.new_cwd(args, true);
            return Ok(());
        }
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

pub fn create_file(path: String) {
    File::create(&path).expect("Failed to create file.");
    enter_file(path).expect("Failed to enter file");
}

pub fn create_dir(path: String, app: &mut App, hiding_dot_files: bool) {
    fs::create_dir(path).expect("Failed to create directory.");
    app.new_cwd("./", hiding_dot_files);
}

pub fn prev_file(path: String) -> String {
    let raw = Command::new("cat")
        .arg(path)
        .output()
        .expect("cannot preview file");

    let prev = String::from_utf8_lossy(&raw.stdout);
    prev.to_string()
}
