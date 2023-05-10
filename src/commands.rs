use std::process::Command;

pub fn ls_arg(dir: &str, arg: &str) -> Vec<String> {
    let mut res = vec![];
    let mut cmd = Command::new("ls");
    cmd.arg("-p").arg(arg).current_dir(dir);
    let output = cmd.output().expect("failed to run");
    let res_string: String = String::from_utf8(output.stdout).unwrap();
    let lines = res_string.lines();
    for l in lines {
        res.push(l.to_string());
    }

    return res;
}

pub fn ls(dir: &str) -> Vec<String> {
    let mut res = vec![];
    let mut cmd = Command::new("ls");
    cmd.arg("-p").current_dir(dir);
    let output = cmd.output().expect("failed to run");
    let res_string: String = String::from_utf8(output.stdout).unwrap();
    let lines = res_string.lines();
    for l in lines {
        res.push(l.to_string());
    }

    return res;
}
