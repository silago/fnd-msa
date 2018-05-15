extern crate zmq;

use std::env;
use std::thread;
use std::time::Duration;
use std::string::String;

fn main() {
    let context = zmq::Context::new();
    let responder = context.socket(zmq::REP).unwrap();
    //- ZMQ_PUB_ADDRESS=tcp://server:3000
    let server_name =  (match  env::var("ZMQ_BIND_ADDRESS") {
        Ok(v) => v,
        Err(e) => String::from("tcp://*:3000")
    });
    assert!(responder.bind(server_name.as_str()).is_ok());

    let mut msg = zmq::Message::new().unwrap();
    loop {
        responder.recv(&mut msg, 0).unwrap();
        println!("Received {}", msg.as_str().unwrap());
        thread::sleep(Duration::from_millis(1000));
        responder.send_str("World", 0).unwrap();
    }
}