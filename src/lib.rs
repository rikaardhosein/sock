#![allow(dead_code)]

use std::io::{Read, Write};
use std::net::{Ipv4Addr, SocketAddrV4, TcpListener, TcpStream};
use std::thread;

pub mod socklib;
use socklib::{client_messages, server_messages};
use std::collections::VecDeque;

use crate::socklib::AddressType;

pub struct ClientHandler {
    stream: TcpStream,
}

impl ClientHandler {
    pub fn handle(&mut self) {
        let mut buf: VecDeque<u8> = VecDeque::with_capacity(16384);

        fn recv_message<T: client_messages::ClientMessage>(
            stream: &mut TcpStream,
            buf: &mut VecDeque<u8>,
        ) -> Option<T> {
            loop {
                let mut tmpbuf: [u8; 4096] = [0; 4096];
                let read_result = stream.read(&mut tmpbuf);
                if read_result.is_err() {
                    return None;
                }

                let bytes_read = read_result.unwrap();

                if bytes_read == 0 {
                    return None;
                }

                buf.extend(tmpbuf[..bytes_read].iter());

                let result = T::try_parse(&mut buf.iter());
                if let Some(client_ver_method) = result {
                    buf.drain(..client_ver_method.size());
                    return Some(client_ver_method);
                }
            }
        }

        let client_ver_method: Option<client_messages::version_method::VersionMethod> =
            recv_message(&mut self.stream, &mut buf);
        if client_ver_method.is_none() {
            return;
        }
        let client_ver_method = client_ver_method.unwrap();

        println!("{:?}", &client_ver_method);

        assert!(client_ver_method
            .methods
            .contains(&socklib::AuthMethod::NoAuth));

        let ver_method_resp = server_messages::version_method::VersionMethod {
            ver: socklib::ProtocolVersion::V5,
            method: socklib::AuthMethod::NoAuth,
        };

        println!("{:?}", &ver_method_resp);

        let ver_method_resp = ver_method_resp.serialize();
        let write_result = self.stream.write_all(&ver_method_resp);
        if write_result.is_err() {
            return;
        }

        let client_request: Option<client_messages::request::ClientRequest> =
            recv_message(&mut self.stream, &mut buf);
        if client_request.is_none() {
            return;
        }
        let client_request = client_request.unwrap();

        assert!(client_request.atyp == socklib::AddressType::IPv4);
        assert!(client_request.cmd == socklib::Command::Connect);

        println!("{:?}", &client_request);

        let client_request_resp = server_messages::request_reply::RequestReply {
            ver: socklib::ProtocolVersion::V5,
            rsv: 0,
            rep: server_messages::request_reply::ReplyType::Succeeded,
            atyp: AddressType::IPv4,
            ipv4_bind_addr: Ipv4Addr::new(127, 0, 0, 1),
            bind_port: 1337,
        };

        println!("{:?}", &client_request_resp);

        let client_request_resp = client_request_resp.serialize();
        let write_result = self.stream.write_all(&client_request_resp);
        if write_result.is_err() {
            return;
        }

        let target_addr = SocketAddrV4::new(client_request.ipv4_addr, client_request.port);
        let target_host = TcpStream::connect(&target_addr);
        if target_host.is_err() {
            return;
        }
        let mut target_host = target_host.unwrap();
        let mut target_host_read = target_host.try_clone();
        if target_host_read.is_err() {
            return;
        }
        let mut target_host_read = target_host_read.unwrap();
        let mut client_write = self.stream.try_clone();
        if client_write.is_err() {
            return;
        }
        let mut client_write = client_write.unwrap();

        thread::spawn(move || {
            let mut buf: VecDeque<u8> = VecDeque::with_capacity(16384);

            loop {
                let mut tmpbuf: [u8; 4096] = [0; 4096];
                let read_result = target_host_read.read(&mut tmpbuf);
                if read_result.is_err() {
                    return;
                }

                let bytes_read = read_result.unwrap();

                if bytes_read == 0 {
                    return;
                }

                buf.extend(&tmpbuf[..bytes_read]);
                buf.make_contiguous();

                if let (slice, &[]) = buf.as_slices() {
                    let write_result = client_write.write(slice);
                    if write_result.is_err() {
                        return;
                    }
                    let bytes_written = write_result.unwrap();
                    buf.drain(..bytes_written);
                }
            }
        });

        loop {
            let mut tmpbuf: [u8; 4096] = [0; 4096];
            let read_result = self.stream.read(&mut tmpbuf);
            if read_result.is_err() {
                return;
            }

            let bytes_read = read_result.unwrap();

            if bytes_read == 0 {
                return;
            }

            buf.extend(&tmpbuf[..bytes_read]);

            buf.make_contiguous();
            if let (slice, &[]) = buf.as_slices() {
                let write_result = target_host.write(slice);
                if write_result.is_err() {
                    return;
                }
                let bytes_written = write_result.unwrap();
                buf.drain(..bytes_written);
            }
        }
    }
}

pub struct Socks5Server {
    pub bind_addr: String,
    //pub client_threads: Vec<ClientThread>,
}

impl Socks5Server {
    pub fn start(self) {
        let listener = TcpListener::bind(self.bind_addr).unwrap();
        for stream in listener.incoming() {
            if let Ok(client_stream) = stream {
                let mut ct = ClientHandler {
                    stream: client_stream,
                };
                thread::spawn(move || {
                    ct.handle();
                });
                //self.client_threads.push(ct);
            }
        }
    }
}
