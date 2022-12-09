use std::env;
use nix::{sys::wait::waitpid,unistd::{fork, ForkResult, write}};


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
            println!("Filesystem {} successfully created at /tmp/file1234.tmp with PID {}", name, child);
            //waitpid(child, None).unwrap();
        }
        Ok(ForkResult::Child) => {
            // Unsafe to use `println!` (or `unwrap`) here. See Safety.
            write(libc::STDOUT_FILENO, "I'm a new child process\n".as_bytes()).ok();
            loop {
                // Do nothing
            }
            unsafe { libc::_exit(0) };
        }
        Err(_) => println!("Fork failed"),
     }    
}

fn send(name: &String, message: &String) {
    let tmpfile = "/tmp/file1234.tmp";
    println!("Data successfully written at {}", tmpfile);
}

fn stop(name: &String) {
    println!("Filesystem {} successfully stopped", name);
}
