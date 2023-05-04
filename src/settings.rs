use std::path::Path;
use config::Config;
use dirs;
use std::collections::HashMap;

pub fn get_conf() {

    let home = match dirs::home_dir() {
        None => panic!("no home dir found"),
        Some(h) => h
    };

    let path = Path::new("").join(home).join(".config/kfm.toml");

    let settings = Config::builder()
            .add_source(config::File::with_name(path.to_str().unwrap()))
            .build()
            .unwrap()
            .try_deserialize::<HashMap<String, String>>()
            .unwrap();

    println!("{:?}", settings);
}
