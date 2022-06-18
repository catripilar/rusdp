use std::thread;
use std::net::UdpSocket;
fn main() {    
    let socket = UdpSocket::bind("0.0.0.0:3333").expect("Could not bind socket");
    //let sock = socket.try_clone().expect("Failed to clone socket"); 
    loop {
        let mut filesize = 1024;
        let mut buf = vec![0u8; filesize];   
        match socket.recv_from(&mut buf) {
            Ok((size, src)) => {                        
                    match std::str::from_utf8(&buf) {
                        Ok(ok) => {
                            println!("{}-{}",filesize,ok);
                            socket.send_to(ok.as_bytes(),src) .expect("Failed to send a response");
                            buf.clear();
                        }
                        Err(_) => println!("err"),
                    };                   
                },Err(_) =>println!("err")
            }
        }
    } 