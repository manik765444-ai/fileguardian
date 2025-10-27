use notify::{watcher, DebouncedEvent, RecursiveMode, Watcher};
use std::sync::mpsc::channel;
use std::time::Duration;
use std::{env, process};

fn main() {
    // Get the directory to watch from the first command-line argument
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: {} <path-to-watch>", args[0]);
        process::exit(1);
    }

    let path_to_watch = &args[1];

    // Create a channel to receive file events
    let (tx, rx) = channel();

    // Create a watcher object, with a debounce delay of 2 seconds
    let mut watcher = match watcher(tx, Duration::from_secs(2)) {
        Ok(watcher) => watcher,
        Err(e) => {
            eprintln!("Error creating watcher: {}", e);
            process::exit(1);
        }
    };

    // Add the directory we want to watch
    if let Err(e) = watcher.watch(path_to_watch, RecursiveMode::Recursive) {
        eprintln!("Error watching directory {}: {}", path_to_watch, e);
        process::exit(1);
    }

    println!("Watching directory: {}", path_to_watch);

    // Process events received on the channel
    loop {
        match rx.recv() {
            Ok(event) => match event {
                DebouncedEvent::Create(path) => println!("File created: {:?}", path),
                DebouncedEvent::Write(path) => println!("File modified: {:?}", path),
                DebouncedEvent::Rename(src, dest) => {
                    println!("File renamed from {:?} to {:?}", src, dest)
                }
                DebouncedEvent::Remove(path) => println!("File deleted: {:?}", path),
                DebouncedEvent::Rescan => println!("Rescan event occurred"),
                _ => println!("Other event: {:?}", event),
            },
            Err(e) => {
                eprintln!("Error receiving file event: {}", e);
                process::exit(1);
            }
        }
    }
}