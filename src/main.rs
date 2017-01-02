extern crate libdns;

use libdns::message::{Header, Question, QuestionBuilder, Message};
use libdns::wire::{FromWire, ToWire};

use std::env;
use std::fs::File;
use std::io::{Error, Result, Read};
use std::net::{Ipv4Addr, SocketAddr, SocketAddrV4, UdpSocket};
use std::str::FromStr;

#[cfg(target_os = "redox")]
fn get_local_socket() -> Result<SocketAddrV4> {

    println!("I'm running Redox! :-) ");

    let mut ip_string = String::new();
    File::open("/etc/net/ip")?.read_to_string(&mut ip_string)?;
    let ip: Vec<u8> = ip_string.trim().split(".").map(|part| part.parse::<u8>()
                               .unwrap_or(0)).collect();
    let local_ip = Ipv4Addr::new(ip[0], ip[1], ip[2], ip[3]);
    Ok(SocketAddrV4::new(local_ip, 0))
}

#[cfg(not(target_os = "redox"))]
fn get_local_socket() -> Result<SocketAddrV4> {
    let local_ip = Ipv4Addr::new(127, 0, 0, 1);
    Ok(SocketAddrV4::new(local_ip, 0))
}


fn main() {
    let args = env::args();
    let mut args = args.skip(1);
    let name = args.next().expect("No name was provided.");
    let server = args.next().expect("No server was provided.");

    // Create a message
    let header = Header {
        id: 1236,
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

    let question = QuestionBuilder::default().set_name(name.to_string()).finish();

    let query = Message {
        header: header,
        question: vec![question],
        answer: vec![],
        authority:  vec![],
        additional:  vec![],
    };

    // Send it
    let socket = UdpSocket::bind(get_local_socket()
                                .expect("Cannot get a local socket"))
                                .expect("Cannot bind to the local socket.");

    let dns: Vec<u8> = server.trim().split(".").map(|part| part.parse::<u8>()
                             .unwrap_or(0)).collect();
    if dns.len() != 4 {
        println!("Server address is not valid.");
        return
    }

    let server = SocketAddrV4::new(Ipv4Addr::new(dns[0], dns[1], dns[2], dns[3]), 53);

    socket.connect(&SocketAddr::V4(server)).expect("Cannot connect to the server");
    socket.send(&query.to_wire()).expect("Cannot send the query");

    // Read a response
    let mut buffer = [0u8;65536];
    let _  = socket.recv(&mut buffer).expect("Cannot read from the socket");
    // let (length,_) = socket.recv_from(&mut buffer).expect("Cannot read from the socket");

    let response: Message = Message::from_wire(&buffer).unwrap();
    println!("Address of {} is {:?}", name, response.answer[0].rdata);

}

