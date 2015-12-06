use std::thread;
use std::net::{SocketAddrV4,UdpSocket};

fn discover() {
    // send a UTP broadcast
    let broadcast_port = 1898;

    let t = thread::spawn(move || {
        let broadcast_socket = UdpSocket::bind(("0.0.0.0", broadcast_port)).ok().unwrap();

        let mut buf = [0; 128];
        println!("Broadcast listener up on {:?}.", broadcast_socket);

        loop {
            let (byte_read, src) = broadcast_socket.recv_from(&mut buf).ok().unwrap();
            if byte_read > 0 {
                println!("{0}: {1}", src, String::from_utf8_lossy(&mut buf[0..byte_read]));
            }
        }
    });

    println!("Waiting for a broadcast to be heard.");
    t.join();
}

#[test]
#[ignore]
fn test_broadcast_listener() {
    discover();
}
