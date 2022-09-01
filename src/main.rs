use std::net::{TcpListener};

mod alsa_controller;
mod networking;

//use std::io;


fn main() {
    alsa_controller::init_mixer();
    //input eventually
    let address = String::from("127.0.0.1:7878");
    let listener: TcpListener =
        TcpListener::bind(address).unwrap();
    for stream in listener.incoming() { 
        let stream = stream.unwrap();
        networking::handle_conn(stream);
    }
}
