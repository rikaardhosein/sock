#![allow(dead_code)]

use std::io::{Error, Read, Write, ErrorKind};
use std::net::{Ipv4Addr, SocketAddrV4, TcpListener, TcpStream};
use std::thread;

pub mod socklib;
use socklib::{client_messages, server_messages, AuthMethod, ProtocolVersion, AddressType, Command, server_messages::ReplyType};
use std::collections::VecDeque;


pub struct ClientHandler {
    stream: TcpStream,
}

fn recv_message<T: client_messages::ClientMessage>(
    stream: &mut TcpStream,
    buf: &mut VecDeque<u8>,
) -> Option<T> {
    loop {
        let mut tmpbuf: [u8; 4096] = [0; 4096];
        let bytes_read = stream.read(&mut tmpbuf).ok()?;

        if bytes_read == 0 {
            return None;
        }

        buf.extend(tmpbuf[..bytes_read].iter());

        let result = T::try_parse(&mut buf.iter());
        if let Some(msg) = result {
            buf.drain(..msg.size());
            return Some(msg);
        }
    }
}

fn create_pipe(read_from: &mut TcpStream, write_to: &mut TcpStream) -> Result<(), Error> {
    let mut buf: [u8; 4096] = [0; 4096];
    loop {
        let bytes_read = read_from.read(&mut buf)?;
        write_to.write_all(&buf[..bytes_read])?;
    }
}

fn create_bidirectional_pipe(a: &mut TcpStream, b: &mut TcpStream) -> Result<(), Error> {
    let mut a_reader = a.try_clone()?;
    let mut a_writer = a.try_clone()?;

    let mut b_reader = b.try_clone()?;
    let mut b_writer = b.try_clone()?;
    thread::spawn(move || {
        _ = create_pipe(&mut a_reader, &mut b_writer);
    });
    thread::spawn(move || {
        _ = create_pipe(&mut b_reader, &mut a_writer);
    });
    return Ok(());
}

impl ClientHandler {
    pub fn handle(&mut self) -> Result<(), Error> {
        let mut buf: VecDeque<u8> = VecDeque::with_capacity(16384);

        let client_ver_method: client_messages::VersionMethod =
            recv_message(&mut self.stream, &mut buf).ok_or(Error::new(ErrorKind::InvalidData, "Failed to receive VersionMethod from Client"))?;

        assert!(client_ver_method.methods.contains(&AuthMethod::NoAuth));

        let ver_method_resp = server_messages::version_method::VersionMethod {
            ver: ProtocolVersion::V5,
            method: AuthMethod::NoAuth,
        };

        self.stream.write_all(&ver_method_resp.serialize())?;

        let client_request: client_messages::ClientRequest =
            recv_message(&mut self.stream, &mut buf).ok_or(Error::new(ErrorKind::InvalidData, "Failed to receive ClientRequest from Client"))?;

        assert!(client_request.atyp == AddressType::IPv4);
        assert!(client_request.cmd == Command::Connect);

        let target_addr = SocketAddrV4::new(client_request.ipv4_addr, client_request.port);
        let target_host = TcpStream::connect(&target_addr);

        let client_request_resp = server_messages::RequestReply {
            ver: ProtocolVersion::V5,
            rsv: 0,
            rep: if target_host.is_ok() { ReplyType::Succeeded } else { ReplyType::ConnectionRefused },
            atyp: AddressType::IPv4,
            ipv4_bind_addr: Ipv4Addr::new(0, 0, 0, 0),
            bind_port: 0,
        };

        self.stream.write_all(&client_request_resp.serialize())?;

        create_bidirectional_pipe(&mut target_host?, &mut self.stream)?;

        return Ok(())
    }
}

pub struct Socks5Server {
    pub bind_addr: String,
}

impl Socks5Server {
    pub fn start(self) -> Result<(), Error> {
        let listener = TcpListener::bind(self.bind_addr)?;
        for stream in listener.incoming() {
            let mut ct = ClientHandler { stream: stream? };
            thread::spawn(move || { _ = ct.handle(); });
        }

        return Ok(());
    }
}
