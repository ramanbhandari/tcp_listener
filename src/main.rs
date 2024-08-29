use std::net::{TcpListener, TcpStream};
use std::io::{Read, Write};

fn handle_client(mut tcp_stream: TcpStream) -> Result<(), std::io::Error> {
    let mut buffer = [0; 512];
    //read value into the buffer
    match tcp_stream.read(&mut buffer) {
        Ok(size) => {
            //write the value back to client
            if let Err(e) = tcp_stream.write_all(&buffer[..size]) {
                eprintln!("Failed to send response : {}", e);
            }
        }
        Err(e) => eprintln!("Failed to read from stream {}", e),
    }
    Ok(())
}

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").expect("Failed to bind to port 7878.");

    println!("Server listening on port 7878");
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                println!("Connection established!");
                handle_client(stream).expect("Failed to read and write in handle_client");
            }
            Err(e) => eprintln!("Failed to accept connection: {}", e),
        }
    }
}
