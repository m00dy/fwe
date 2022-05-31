use std::io::{self, BufRead};
use std::path::Path;
extern crate notify;

use notify::{watcher, RecursiveMode, Watcher};
use std::sync::mpsc::channel;
use std::time::Duration;
use std::process::{Command, Stdio};

fn main() {
    let stdin = io::stdin();
    let mut _validfiles: Vec<String> = Vec::new();
    for line in stdin.lock().lines() {
        let p = line.unwrap();
        let validPath = Path::new(&p).is_file();
        if validPath {
            _validfiles.push(p.clone());
        }
    }

    let (tx, rx) = channel();
    let mut watcher = watcher(tx, Duration::from_secs(2)).unwrap();
    let fileiter = _validfiles.iter();
    for file in fileiter {
        watcher.watch(file, RecursiveMode::Recursive).unwrap();
    }

    loop {
        match rx.recv() {
            Ok(event) => {
                match event {
                    notify::DebouncedEvent::Write(path) => {
                        println!("File {:?} has been modified", path);
                        let args = std::env::args().collect::<Vec<String>>();
                        let argsLen = args.len();
                        if argsLen > 1 {
                            let mut execName = args[1].clone();
                            let mut execArgs = args[2..].to_vec();
                            Command::new(execName)
                                .args(execArgs)
                                .stdin(Stdio::piped())
                                .stdout(Stdio::piped())
                                .spawn()
                                .expect("failed to execute process");
                            //println!("{:?}", output);
                        }
                    }
                    _ => {}
                }
            },
            Err(e) => println!("watch error: {:?}", e),
        }
    }
}
