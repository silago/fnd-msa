extern crate zmq;
use std::env;
use zmq::Message;
use std::string::String;


fn main() {
    println!("Connecting to hello world server...\n");

    let context = zmq::Context::new();
    let requester = context.socket(zmq::REQ).unwrap();
    let server_name =  (match  env::var("ZMQ_PUB_ADDRESS") {
        Ok(v) => v,
        Err(e) => String::from("tcp://127.0.0.1:3000")
    });
    assert!(requester.connect(server_name.as_str()).is_ok());

    let mut msg = Message::new().unwrap();

    for request_nbr in 0..10 {
        println!("Sending Hello {}...", request_nbr);
        requester.send_str("Hello", 0).unwrap();

        requester.recv(&mut msg, 0).unwrap();
        println!("Received World {}: {}", msg.as_str().unwrap(), request_nbr);
    }
}
