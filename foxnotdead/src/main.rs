extern crate zmq;

use std::env;
use std::thread;
use std::time::Duration;
use std::string::String;
use std::thread::sleep;


/* this service should provide auth and forward requests */
/* also this module shall answer client, i guees */
fn main() {
    let context = zmq::Context::new();
    let responder = context.socket(zmq::REP).unwrap();
    let socket = context.socket(zmq::PUB).unwrap();
    let server_name =  (match  env::var("ZMQ_BIND_ADDRESS") {
        Ok(v) => v,
        Err(e) => String::from("tcp://*:3000")
    });

    socket.bind(server_name.as_str());

    let user_channel: &[u8; 13] = b"users-service";
    loop {
        socket.send(user_channel, zmq::SNDMORE).unwrap();
        socket.send_str("Huilago", zmq::SNDMORE).unwrap();
        socket.send_str("i", 0).unwrap();
        sleep(Duration::from_secs(5));
    }
}