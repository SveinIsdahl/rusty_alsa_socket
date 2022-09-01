use std::net::{TcpStream};
use std::io::{prelude::*, BufReader};


pub fn handle_conn(mut stream :TcpStream) {
    let buf_reader = BufReader::new(&mut stream);
    let buf: Vec<_> = buf_reader
        .lines()
        .map(|result| result.unwrap())
        .take_while(|line| !line.is_empty())
        .collect();
    println!("Req: {:#?}", buf);
}
