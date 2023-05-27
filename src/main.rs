mod directory;
mod utility_error;
use directory::Directory;
use std::env;
use utility_error::collect_results;

const USAGE_STRING: &str = "Usage:\n  opu <command> [<args>]\nCommands:\n  add <feed url>\n  remove <podcast id>\n  verify";
const DEFAULT_PATH: &str = "public_podcasts.csv";

fn verify_command(_args: Vec<String>) {
    let mut dir = Directory::new(String::from(DEFAULT_PATH));

    let results = vec![
        dir.read(None),
        dir.verify()
    ];

    let unified_result = collect_results(results);

    match unified_result {
        Ok(_r) => println!("Verification succeeded"),
        Err(e) => println!("{}", e)
    }
}

/*
fn add_command() {
    dir.read();
    dir.add();
    dir.verify();
    dir.write();
}

fn remove_command() {
    dir.read();
    dir.remove();
    dir.verify();
    dir.write();
}*/

fn unknown_command(args: Vec<String>) {
    println!("Error: The command {} is invalid.\n", args[1]);
    println!("{}", USAGE_STRING);
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if (&args).len() < 2 {
        println!("Not enough arguments provided");
        println!("{}", USAGE_STRING);
    } else {
        match args[1].as_str() {
            /*"add" => add_command(args),
            "remove" => remove_command(args),*/
            "verify" => verify_command(args),
            _ => unknown_command(args)
        }
    }
}
