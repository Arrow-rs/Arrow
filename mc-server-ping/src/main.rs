use std::{
    io::{Read, Write},
    net::TcpStream,
};

use arrow_protocol::{
    error::DeserializeError,
    handshake::{Handshake, NextState},
    status::{Status, StatusRequest},
    Bound, Protocol, State,
};
use bytes::BytesMut;
use clap::Parser;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// The address to connect to.
    #[clap(value_parser)]
    address: String,

    /// The port to connect to.
    #[clap(short, value_parser, default_value_t = 25565)]
    port: u16,

    /// The protocol version to use in the Handshake packet.
    #[clap(short = 'v', long, value_parser, default_value_t = 759)]
    protocol_version: i32,
}

fn main() {
    let args = Args::parse();

    match TcpStream::connect((args.address.as_str(), args.port)) {
        Ok(mut tcp_stream) => {
            tcp_stream
                .write_all(
                    &Protocol::Handshake(Handshake {
                        version: args.protocol_version.into(),
                        address: args.address,
                        port: args.port,
                        next_state: NextState::Status,
                    })
                    .serialize(Default::default(), None)
                    .unwrap(),
                )
                .unwrap();
            tcp_stream
                .write_all(
                    &Protocol::Status(Status::StatusRequest(StatusRequest))
                        .serialize(Default::default(), None)
                        .unwrap(),
                )
                .unwrap();

            let mut packet = BytesMut::new();

            let mut buf = [0; 256];

            let response = loop {
                let len = tcp_stream.read(&mut buf).unwrap();
                if len == 0 {
                    return;
                }

                packet.extend_from_slice(&buf[..len]);

                let response = Protocol::deserialize(
                    Bound::Clientbound,
                    State::Status,
                    Default::default(),
                    None,
                    &mut packet.clone(),
                );

                match response {
                    Err(DeserializeError::UnexpectedEof) => {}
                    _ => break response,
                }
            };

            match response {
                Ok(Protocol::Status(Status::StatusResponse(response))) => {
                    println!("{}", response.response)
                }
                Ok(_) => eprintln!("Unexpected packet received."),
                Err(e) => eprintln!("{e}"),
            }
        }
        Err(e) => eprintln!(
            "Failed to connect to {}:{}: {e}",
            args.address.as_str(),
            args.port
        ),
    }
}
