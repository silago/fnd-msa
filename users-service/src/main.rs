extern crate zmq;
use std::env;
use zmq::Message;
use std::string::String;


fn main() {
    println!("user-service us connecting to main server...\n");
    let channel = b"users-service-in";
    let server_name =  (match  env::var("ZMQ_PUB_ADDRESS") {
        Ok(v) => v,
        Err(e) => String::from("tcp://127.0.0.1:3000")
    });

    let context = zmq::Context::new();
    let requester = context.socket(zmq::SUB).unwrap();
    requester.set_subscribe(channel);
    requester.connect(server_name.as_str());

    loop {
        let message: String = requester
            .recv_string(0)
            .expect("failed receiving update")
            .unwrap();
        println!("{}",message);
        /*
        got user_name
        have to find or create user_id
        return user id to the channel
        */
    }

}
