use std::io::{Read, Write};
use std::net::TcpStream;
use std::str::from_utf8;

fn main() {
    // connect to server
    match TcpStream::connect("localhost:3333") {
        Ok(mut stream) => {
            println!("Successfully connected to server on port 3333");
            // create variable to hold data that is to be sent to the server
            let msg = b"What's up homie";

            // send data to the server
            stream.write(msg).unwrap();
            println!("Sent Hello, awaiting reply...");

            // create a buffer to hold the reponse of the server
            let mut data = [0 as u8; 15];

            // read the data from the response into the buffer
            match stream.read_exact(&mut data) {
                Ok(_) => {
                    // convert to string slice
                    let text = from_utf8(&data).unwrap();
                    println!("Sever says: {}", text);
                }
                Err(e) => println!("Failed to receive data: {}", e),
            }
        }
        Err(e) => println!("Failed to connect: {}", e),
    }
    println!("Terminated");
}
