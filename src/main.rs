use std::env;
use nix::{unistd::{fork, ForkResult}};

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 3 {
        println!("Not enough arguments");
        return;
    }

    let command = &args[1];
    let name = &args[2];

    if command == "init" {
        init(name);
    } else if command == "send" {
        if args.len() < 4 {
            println!("Not enough arguments");
            return;
        }
        let message = &args[3];
        send(name, message);
    } else if command == "stop" {
        stop(name);
    } else {
        println!("Unknown command");
    }
}

fn init(name: &String) {
    
    match unsafe{fork()} {
        Ok(ForkResult::Parent { child, .. }) => {
            //
            println!("Filesystem {} successfully with PID {}", name, child);
            //waitpid(child, None).unwrap();
        }
        Ok(ForkResult::Child) => {
            // let mut tmpfile = NamedTempFile::new().unwrap();
            // let tempfilename = tmpfile.path().to_str().unwrap();
            // let text = "Brian was here. Briefly.";
            // tmpfile.write_all(text.as_bytes());

            // Unsafe to use `println!` (or `unwrap`) here. See Safety.
            println!("I'm a new child process");

            loop {
                // Do nothing
            }
        }
        Err(_) => println!("Fork failed"),
     }    
}

fn send(name: &String, message: &String) {
    let tmpfile = "/tmp/file1234.tmp";
    println!("[{}] Data successfully written at {}: {}", name, tmpfile, message);
}

fn stop(name: &String) {
    println!("Filesystem {} successfully stopped", name);
}


//https://www.nikbrendler.com/rust-process-communication/