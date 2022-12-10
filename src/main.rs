/**
 * @link https://rust-cli.github.io/book/index.html
 */
use std::{
    env,
    process::{exit},
    thread::{sleep, spawn},
    time::Duration,
};
use tempfile::NamedTempFile;
use nix::{
    unistd::{fork, ForkResult, getpid},
};
use signal_hook::{iterator::Signals, consts::SIGTERM};

fn main() {
    let command = env::args().nth(1);
    let name = env::args().nth(2);

    if command.is_none() || name.is_none() {
        println!("Not enough arguments: command and name are required");
        return;
    }

    match command.unwrap().as_str() {
        "init" => init(&name.unwrap()),
        "send" => {
            let message = env::args().nth(3);
            if message.is_none() {
                println!("Not enough arguments: message is required");
                return;
            }
            send(&name.unwrap(), &message.unwrap());
        }
        "stop" => stop(&name.unwrap()),
        _ => println!("Unknown command"),
    }
}

fn init(name: &String) {
    match unsafe { fork() } {
        Ok(ForkResult::Parent { child, .. }) => {
            println!("Filesystem {} successfully started with pid {}.", name, child);
        }
        Ok(ForkResult::Child) => {
            println!("Starting filesystem {} with pid {}.", name, getpid());
            fs_run(name);
        }
        Err(_) => {
            println!("Failed to start filesystem {}", name);
            exit(1);
        }
    }
}

fn fs_run(name: &String) {
    let tmpfile = NamedTempFile::new().unwrap();
    println!("Filesystem successfully created at {}", tmpfile.path().display());

    fs_catch_signal(name);
    loop {
        sleep(Duration::from_secs(1));
        println!("Running...");
    }
}

fn fs_catch_signal(name: &String) {
    let mut signals = Signals::new(&[SIGTERM]).unwrap();
    
    let fs_name = name.clone();
    spawn(move || {
        for signal in signals.forever() {
            match signal {
                SIGTERM => {
                    println!("Filesystem {} successfully stopped", fs_name);
                    exit(0);
                }
                _ => unreachable!(),
            }
        }
    });
}

fn send(name: &String, message: &String) {
    let tmpfile = "/tmp/file1234.tmp";
    println!("[{}] Data successfully written at {}: {}", name, tmpfile, message);
}

fn stop(name: &String) {
    println!("Filesystem {} successfully stopped", name);
}

// //https://www.nikbrendler.com/rust-process-communication/