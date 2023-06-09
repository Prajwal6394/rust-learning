#[allow(unused)]
use std::{net::{TcpListener}, io::Read};

fn main() {
    let listener = TcpListener::bind("127.0.0.1:8477").unwrap();
    println!("Server is listening at port 8477");
    for stream in listener.incoming() {
        let stream = stream.unwrap();
        println!("Connection happened");
    }
}