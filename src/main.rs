/**
 * @link https://rust-cli.github.io/book/index.html
 */
use std::env;

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
    println!("Filesystem {} successfully initialized", name);
}

fn send(name: &String, message: &String) {
    let tmpfile = "/tmp/file1234.tmp";
    println!("[{}] Data successfully written at {}: {}", name, tmpfile, message);
}

fn stop(name: &String) {
    println!("Filesystem {} successfully stopped", name);
}

// //https://www.nikbrendler.com/rust-process-communication/