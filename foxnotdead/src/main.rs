extern crate zmq;

use std::env;
use std::thread;
use std::time::Duration;
use std::string::String;
use std::thread::sleep;


fn main() {
    let context = zmq::Context::new();
    //let responder = context.socket(zmq::REP).unwrap();
    let socket = context.socket(zmq::PUB).unwrap();
    //- ZMQ_PUB_ADDRESS=tcp://server:3000
    let server_name =  (match  env::var("ZMQ_BIND_ADDRESS") {
        Ok(v) => v,
        Err(e) => String::from("tcp://*:3000")
    });

    println!("server is ready...");
    socket.bind(server_name.as_str());
    println!(".. and binded.");

    let user_channel: &[u8; 13] = b"users-service";
    //loop {
        println!("entering loop");
        //for i in 0..10 {
        loop {
            println!("sending partial data. channel: 9");
            //socket.send(user_channel, zmq::SNDMORE).unwrap();
            socket.send(user_channel, zmq::SNDMORE).unwrap();
            socket.send_str("Silago", 0).unwrap();
            sleep(Duration::from_secs(5));
        }
    //}

    //let mut msg = zmq::Message::new().unwrap();

    /*
    loop {
        responder.recv(&mut msg, 0).unwrap();
        println!("Received {}", msg.as_str().unwrap());
        thread::sleep(Duration::from_millis(1000));
        responder.send_str("World", 0).unwrap();
    }
    */



    /* publish to the user-service maybe it should be changed to the client-server messaging*/

}