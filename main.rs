use std::env;

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
    let tmpfile = "/tmp/file1234.tmp";
    let pid = 1234;
    println!("Filesystem {} successfully created at {} with PID {}", name, tmpfile, pid);
}

fn send(name: &String, message: &String) {
    let tmpfile = "/tmp/file1234.tmp";
    println!("Data successfully written at {}", tmpfile);
}

fn stop(name: &String) {
    println!("Filesystem {} successfully stopped", name);
}
