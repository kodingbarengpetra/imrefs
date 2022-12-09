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
    println!("init {}", name);
}

fn send(name: &String, message: &String) {
    println!("send {} {}", name, message);
}

fn stop(name: &String) {
    println!("stop {}", name);
}
