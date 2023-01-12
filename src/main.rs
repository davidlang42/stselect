use std::env;
use std::fs;

fn main() {
    let mut args = env::args();
    let cmd = args.next().unwrap();
    match args.next() {
        Some(s) => verb(cmd, &s, args.collect()),
        None => help(cmd)
    }
}

fn help(cmd: String) {
    println!("Usage: (omit PATH to use current directory)");
    println!("  {} [PATH] -- open an interactive editor for which folders to sync", cmd);
    println!("  {} list [PATH] -- list the folders currently selected to sync", cmd);
    println!("  {} include [PATH] SUB_FOLDER -- enable syncing of SUB_FOLDER", cmd);
    println!("  {} exclude [PATH] SUB_FOLDER -- disable syncing of SUB_FOLDER", cmd);
    println!("  {} all [PATH] -- enable syncing of all sub folders", cmd);
    println!("  {} none [PATH] -- disable syncing of all sub folders", cmd);
}

fn verb(cmd: String, verb: &str, args: Vec<String>) {
    match verb {
        "list" if args.len() == 0 => list("."),
        "list" if args.len() == 1 => list(&args[0]),
        "include" if args.len() == 1 => include_folder(".", &args[0]),
        "include" if args.len() == 2 => include_folder(&args[0], &args[1]),
        "exclude" if args.len() == 1 => exclude_folder(".", &args[0]),
        "exclude" if args.len() == 2 => exclude_folder(&args[0], &args[1]),
        "all" if args.len() == 0 => include_all("."),
        "all" if args.len() == 1 => include_all(&args[0]),
        "none" if args.len() == 0 => exclude_all("."),
        "none" if args.len() == 1 => exclude_all(&args[0]),
        _ => help(cmd)
    }
}

fn list(path: &str) {
    todo!("list")
}

fn include_folder(path: &str, sub_folder: &str) {
    todo!("include")
}

fn exclude_folder(path: &str, sub_folder: &str) {
    todo!("exclude")
}

fn include_all(path: &str) {
    todo!("include all")
}

fn exclude_all(path: &str) {
    todo!("exclude all")
}