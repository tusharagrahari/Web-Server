use::std::{
    net::{TcpListener, TcpStream},
    io::{prelude::*, BufReader},
};

fn main () {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();
        println!("Connection Established");
    }
}