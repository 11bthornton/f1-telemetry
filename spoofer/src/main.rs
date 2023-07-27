fn main() {
    loop {

        use std::net::UdpSocket;
        let socket = UdpSocket::bind("127.0.0.1:24235").unwrap();

        // use rand::Rng;
        let random_bytes: Vec<u8> = (0..948).map(|_| { rand::random::<u8>() }).collect();
        let buf = random_bytes.as_slice();

        println!("sent a packet");
        socket.send_to(&buf, "127.0.0.1:33333").unwrap();
        
    }
}
