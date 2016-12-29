extern crate libdns;

use libdns::message::{Header, Question, QuestionBuilder, Message};
use libdns::wire::{FromWire, ToWire};

use std::env;
use std::net::UdpSocket;

fn main() {
    let args = env::args();
    let mut args = args.skip(1);
    let name = args.next().expect("No name was provided.");
    let server = args.next().expect("No server was provided.");

    // Create a message
    let header = Header {
        id: 123456,
        response: false,
        opcode: 0,
        aa: false,
        tc: false,
        rd: true,
        ra: false,
        z: 0,
        rcode: 0,
        qdcount: 1,
        ancount: 0,
        nscount: 0,
        arcount: 0,
    };

    let question = QuestionBuilder::no_name(1,1).set_name(name.to_string()).finish();

    let query = Message {
        header: header,
        question: vec![question],
        answer: vec![],
        authority:  vec![],
        additional:  vec![],
    };

    // Send it
    let mut socket = UdpSocket::bind("127.0.0.1:30000").unwrap();
    let server = format!("{}:53", server);
    socket.connect(&server.as_str()).unwrap();
    socket.send(&query.to_wire()).unwrap();

    // Read a response
    let mut buffer = [0u8;1024];
    let (length,_) = socket.recv_from(&mut buffer).unwrap();

    let response: Message = Message::from_wire(&buffer).unwrap();
    println!("Address of {} is {:?}", name, response.answer[0].rdata);

}

