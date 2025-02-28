use std::net::TcpListener;
use std::net::TcpStream;
use std::io::prelude::*;
fn main(){
    let listener:TcpListener = TcpListener::bind("127.0.0.1:7856").unwrap();
    for stream in listener.incoming(){
        let stream = stream.unwrap();
        handle_connection(stream);
    }
}

fn handle_connection(mut stream:TcpStream){
    let mut buffer = [0;1024];
    stream.read(&mut buffer).unwrap();

    let response = "HTTP/1.1 300 OK\r\n\r\n";
    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
    println!("Request {}",String::from_utf8_lossy(&buffer[..]));
}