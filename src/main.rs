use std::env;
use crate::syncthing::IgnoreFile;

mod syncthing;

fn main() -> Result<(), String> {
    let mut args = env::args();
    let cmd = args.next().unwrap();
    match args.next() {
        Some(s) => verb(cmd, &s, args.collect()),
        None => help(cmd)
    }
}

fn help(cmd: String) -> Result<(), String> {
    println!("Usage: (omit PATH to use current directory)");
    println!("  {} [PATH] -- open an interactive editor for which folders to sync", cmd);
    println!("  {} list [PATH] -- list the folders currently selected to sync", cmd);
    println!("  {} include [PATH] SUB_FOLDER -- enable syncing of SUB_FOLDER", cmd);
    println!("  {} exclude [PATH] SUB_FOLDER -- disable syncing of SUB_FOLDER", cmd);
    println!("  {} all [PATH] -- enable syncing of all sub folders", cmd);
    println!("  {} none [PATH] -- disable syncing of all sub folders", cmd);
    Ok(())
}

fn verb(cmd: String, verb: &str, args: Vec<String>) -> Result<(), String> {
    match verb {
        "list" if args.len() == 0 => list("."),
        "list" if args.len() == 1 => list(&args[0]),
        "include" if args.len() == 1 => set_folder(".", &args[0], true),
        "include" if args.len() == 2 => set_folder(&args[0], &args[1], true),
        "exclude" if args.len() == 1 => set_folder(".", &args[0], false),
        "exclude" if args.len() == 2 => set_folder(&args[0], &args[1], false),
        "all" if args.len() == 0 => set_all(".", true),
        "all" if args.len() == 1 => set_all(&args[0], true),
        "none" if args.len() == 0 => set_all(".", false),
        "none" if args.len() == 1 => set_all(&args[0], false),
        _ => help(cmd)
    }
}

fn list(path: &str) -> Result<(), String> {
    let ignore = open(path)?;
    let mut count = 0;
    for folder in ignore.folders {
        let selected = if folder.selected {
            count += 1;
            'x'
        } else {
            ' '
        };
        println!("[{}] {}", selected, folder.name);
    }
    println!("{} of {} sub folders selected to sync", count, ignore.folders.len());
    Ok(())
}

fn set_folder(path: &str, sub_folder: &str, value: bool) -> Result<(), String> {
    let ignore = open(path)?;
    ignore.set(sub_folder, value)?;
    println!("{} '{}' for syncing", selected(value), sub_folder);
    ignore.save()
}

fn set_all(path: &str, value: bool) -> Result<(), String> {
    let ignore = open(path)?;
    for folder in ignore.folders.iter_mut() {
        folder.selected = value;
    }
    println!("{} all {} sub folders for syncing", selected(value), ignore.folders.len());
    ignore.save()
}

fn open(path: &str) -> Result<IgnoreFile, String> {
    let filename = IgnoreFile::find(path)?;
    IgnoreFile::open(&filename)
}

fn selected(value: bool) -> &'static str {
    if value {
        "Selected"
    } else {
        "Unselected"
    }
}